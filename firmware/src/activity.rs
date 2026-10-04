//! On-board control-channel aggregates for standalone use: per-talkgroup grant activity, site
//! health and an activity alarm, printed as summary lines every `interval_s`.
//! Bounded tables (least-recently-active entry is evicted); full aggregation lives on the host.

use crate::tsbk::{field, Line};
use core::fmt::Write;

const MAX_TGS: usize = 96;
/// Windows of history before a talkgroup's baseline is trusted for alarms.
const BASELINE_WINDOWS: u16 = 4;

#[derive(Clone, Copy)]
struct Tg {
    tg: u16,
    grants: u16,      // new grants (GRP_VCH_GRANT) this window
    updates: u16,     // grant updates this window (call still active)
    total: u32,       // grants since boot
    last_s: u32,      // last grant or update
    baseline_q8: u32, // EWMA of grants per window, x256
    windows: u16,     // windows observed since first seen
}

pub struct Activity {
    tgs: [Tg; MAX_TGS],
    n: usize,
    pub interval_s: u32,
    window_start_s: u32,
    tsbk: u32,
    trellis_errs: u32,
    nid_fixed: u32,
    vendor: u32,
    site: Option<(u16, u8, u8, u32)>, // sys, rfss, site, control channel id
}

impl Activity {
    pub const fn new() -> Self {
        const Z: Tg = Tg { tg: 0, grants: 0, updates: 0, total: 0, last_s: 0, baseline_q8: 0, windows: 0 };
        Self {
            tgs: [Z; MAX_TGS],
            n: 0,
            interval_s: 30,
            window_start_s: 0,
            tsbk: 0,
            trellis_errs: 0,
            nid_fixed: 0,
            vendor: 0,
            site: None,
        }
    }

    fn slot(&mut self, tg: u16) -> &mut Tg {
        if let Some(i) = self.tgs[..self.n].iter().position(|t| t.tg == tg) {
            return &mut self.tgs[i];
        }
        let i = if self.n < MAX_TGS {
            self.n += 1;
            self.n - 1
        } else {
            // Evict the least recently active talkgroup.
            (0..MAX_TGS).min_by_key(|&i| self.tgs[i].last_s).unwrap()
        };
        self.tgs[i] = Tg { tg, grants: 0, updates: 0, total: 0, last_s: 0, baseline_q8: 0, windows: 0 };
        &mut self.tgs[i]
    }

    /// Account one CRC-valid TSBK (standard MFID only for field decoding).
    pub fn on_tsbk(&mut self, b: &[u8; 12], trellis_errs: u8, nid_errs: u8, now_s: u32) {
        self.tsbk += 1;
        self.trellis_errs += trellis_errs as u32;
        self.nid_fixed += (nid_errs > 0) as u32;
        if b[1] != 0 {
            self.vendor += 1;
            return;
        }
        match b[0] & 0x3F {
            0x00 => {
                let t = self.slot(field(b, 40, 16) as u16);
                t.grants += 1;
                t.total += 1;
                t.last_s = now_s;
            }
            0x02 => {
                // Two (channel, talkgroup) pairs; systems often repeat the same pair, so count once.
                let (tg1, tg2) = (field(b, 48, 16), field(b, 16, 16));
                for tg in if tg1 == tg2 { [tg1, u32::MAX] } else { [tg1, tg2] } {
                    if tg == u32::MAX {
                        continue;
                    }
                    let t = self.slot(tg as u16);
                    t.updates += 1;
                    t.last_s = now_s;
                }
            }
            0x03 => {
                let t = self.slot(field(b, 16, 16) as u16);
                t.updates += 1;
                t.last_s = now_s;
            }
            0x3A => {
                self.site = Some((field(b, 56, 12) as u16, field(b, 48, 8) as u8, field(b, 40, 8) as u8, field(b, 24, 16)));
            }
            _ => {}
        }
    }

    /// If the window has elapsed, emit summary lines and roll the window.
    pub fn poll(&mut self, now_s: u32, frames_seen: u32, nid_rejects: u32, emit: &mut impl FnMut(&Line)) {
        if self.interval_s == 0 || now_s < self.window_start_s + self.interval_s {
            return;
        }
        let secs = (now_s - self.window_start_s).max(1);
        let mut l = Line { buf: [0; 192], len: 0 };
        let _ = write!(
            l,
            "SUM t={now_s}s window={secs}s tsbk={} ({}.{}/s) frames={frames_seen} nid_fixed={} nid_rej={nid_rejects} trellis_e={} vendor={}",
            self.tsbk,
            self.tsbk / secs,
            self.tsbk * 10 / secs % 10,
            self.nid_fixed,
            self.trellis_errs,
            self.vendor
        );
        if let Some((sys, rfss, site, cc)) = self.site {
            let _ = write!(l, " sys={sys:03x} rfss={rfss} site={site} cc={}-{}", cc >> 12, cc & 0xFFF);
        }
        emit(&l);

        // Talkgroups active this window, busiest first (top 10).
        let mut order: [usize; MAX_TGS] = core::array::from_fn(|i| i);
        let n = self.n;
        order[..n].sort_unstable_by_key(|&i| core::cmp::Reverse((self.tgs[i].grants, self.tgs[i].updates)));
        for &i in order[..n].iter().take(10) {
            let t = self.tgs[i];
            if t.grants == 0 && t.updates == 0 {
                break;
            }
            let mut l = Line { buf: [0; 192], len: 0 };
            let base = t.baseline_q8 as f32 / 256.0;
            let _ = write!(
                l,
                "SUM tg={} grants={} updates={} total={} last={}s ago baseline={:.1}/window",
                t.tg,
                t.grants,
                t.updates,
                t.total,
                now_s - t.last_s,
                base
            );
            emit(&l);
        }

        // Alarm: grants well above this talkgroup's own baseline, once the baseline is established.
        for t in self.tgs[..n].iter() {
            if t.windows < BASELINE_WINDOWS || t.grants < 3 {
                continue;
            }
            let base_x256 = t.baseline_q8.max(64); // floor of 0.25 grants/window
            if (t.grants as u32) * 256 >= 4 * base_x256 + 2 * 256 {
                let mut l = Line { buf: [0; 192], len: 0 };
                let _ = write!(
                    l,
                    "ALERT tg={} grants={} in {secs}s vs baseline {:.1}/window",
                    t.tg,
                    t.grants,
                    t.baseline_q8 as f32 / 256.0
                );
                emit(&l);
            }
        }

        // Roll: baseline EWMA (alpha 1/8), reset window counters.
        for t in self.tgs[..n].iter_mut() {
            t.baseline_q8 = if t.windows == 0 {
                (t.grants as u32) << 8
            } else {
                t.baseline_q8 - (t.baseline_q8 >> 3) + ((t.grants as u32) << 5)
            };
            t.windows = t.windows.saturating_add(1);
            t.grants = 0;
            t.updates = 0;
        }
        self.window_start_s = now_s;
        self.tsbk = 0;
        self.trellis_errs = 0;
        self.nid_fixed = 0;
        self.vendor = 0;
    }
}
