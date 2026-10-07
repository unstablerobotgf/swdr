//! Streaming P25 Phase 2 (TDMA) outbound MAC decoder; port of tools/p2_mac.py.
//! Dibits in (from cqpsk::Cqpsk at 6000 sym/s). S-ISCH pairs (slots 2/3, 6/7, 10/11) anchor the
//! 180-dibit slot grid, re-anchored at every pair so a timing slip costs only the slots before
//! the next one. The superframe position (which pair) is resolved by CRC: each ACCH burst is
//! tried under all three, and a CRC-12 pass after RS(63,35) correction is accepted.

use crate::rs63;

const BURST: u64 = 180;
const SYNC: u64 = 0x575D_57F7_FF;
/// Vocoder frame bit k (MSB of each dibit first) -> (codeword, bit), bit 0 = LSB of c0..c3.
const VCW: [(u8, u8); 72] = [
    (0, 23), (0, 5), (1, 10), (2, 3), (0, 22), (0, 4), (1, 9), (2, 2), (0, 21), (0, 3), (1, 8), (2, 1),
    (0, 20), (0, 2), (1, 7), (2, 0), (0, 19), (0, 1), (1, 6), (3, 13), (0, 18), (0, 0), (1, 5), (3, 12),
    (0, 17), (1, 22), (1, 4), (3, 11), (0, 16), (1, 21), (1, 3), (3, 10), (0, 15), (1, 20), (1, 2), (3, 9),
    (0, 14), (1, 19), (1, 1), (3, 8), (0, 13), (1, 18), (1, 0), (3, 7), (0, 12), (1, 17), (2, 10), (3, 6),
    (0, 11), (1, 16), (2, 9), (3, 5), (0, 10), (1, 15), (2, 8), (3, 4), (0, 9), (1, 14), (2, 7), (3, 3),
    (0, 8), (1, 13), (2, 6), (3, 2), (0, 7), (1, 12), (2, 5), (3, 1), (0, 6), (1, 11), (2, 4), (3, 0),
];

/// Superframe slot -> logical channel; slots 10 and 11 are swapped.
const LCH: [u8; 12] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0];
const DUID_CW: [u8; 16] = [0x00, 0x17, 0x2E, 0x39, 0x4B, 0x5C, 0x65, 0x72, 0x8D, 0x9A, 0xA3, 0xB4, 0xC6, 0xD1, 0xE8, 0xFF];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Signal,
    Ptt,
    EndPtt,
    Idle,
    Active,
    Hangtime,
    Other(u8),
}

#[derive(Debug, Clone)]
pub struct Pdu {
    pub kind: Kind,
    pub tg: Option<u16>,
    pub src: Option<u32>,
    pub mco: u8,
    pub slot: u8,
    /// Logical channel (0/1) of the burst; matches the grant's channel-number LSB.
    pub lch: u8,
    pub fast: bool,
    /// MAC_PTT encryption algorithm (0x80 = clear).
    pub alg: Option<u8>,
}

/// Encryption sync from one voice superframe on a logical channel.
#[derive(Debug, Clone, Copy)]
pub struct Ess {
    pub lch: u8,
    pub alg: u8,
    pub key: u16,
}

/// One 20 ms AMBE+2 3600x2450 frame as codewords c0 (24), c1 (23), c2 (11), c3 (14).
#[derive(Debug, Clone, Copy)]
pub struct VoiceFrame {
    pub lch: u8,
    pub c: [u32; 4],
}

/// Superframe scrambling dibits: 44-bit Galois LFSR (taps 40,35,29,24,10,0) seeded with
/// WACN|SYSID|NAC spread by the fixed offsets {0,4,9,15,20,34}.
fn scrambler(nac: u16, sysid: u16, wacn: u32) -> Vec<u8> {
    let v = ((wacn as u64) << 24) | ((sysid as u64) << 12) | nac as u64;
    let mut r = 0u64;
    for i in 0..44 {
        if (v >> (43 - i)) & 1 == 1 {
            for o in [0, 4, 9, 15, 20, 34] {
                if i + o < 44 {
                    r ^= 1 << (43 - i - o);
                }
            }
        }
    }
    let taps = (1u64 << 40) | (1 << 35) | (1 << 29) | (1 << 24) | (1 << 10) | 1;
    let mut bits = Vec::with_capacity(4320);
    for _ in 0..4320 {
        let b = ((r >> 43) & 1) as u8;
        bits.push(b);
        r = ((r << 1) & ((1 << 44) - 1)) ^ if b == 1 { taps } else { 0 };
    }
    bits.chunks(2).map(|p| (p[0] << 1) | p[1]).collect()
}

fn duid(burst: &[u8]) -> Option<u8> {
    let cw = (burst[10] << 6) | (burst[47] << 4) | (burst[132] << 2) | burst[169];
    let (d, dist) = (0..16u8).map(|k| (k, (cw ^ DUID_CW[k as usize]).count_ones())).min_by_key(|x| x.1)?;
    (dist <= 1).then_some(d)
}

/// CRC-12, x^12+x^11+x^7+x^4+x^2+x+1, complemented.
fn crc12(bits: &[u8]) -> u16 {
    const POLY: [u8; 13] = [1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1];
    let mut reg = bits.to_vec();
    reg.extend([0u8; 12]);
    for i in 0..bits.len() {
        if reg[i] == 1 {
            for j in 0..13 {
                reg[i + j] ^= POLY[j];
            }
        }
    }
    reg[bits.len()..].iter().fold(0u16, |c, &b| (c << 1) | b as u16) ^ 0xFFF
}

/// SACCH (fast = false) or FACCH bits, RS-corrected, CRC-checked; the MAC PDU bytes.
fn acch(b: &[u8], fast: bool) -> Option<Vec<u8>> {
    let spans: &[(usize, usize)] = if fast { &[(11, 36), (48, 31), (100, 32), (133, 36)] } else { &[(11, 36), (48, 84), (133, 36)] };
    let x: Vec<u8> = spans.iter().flat_map(|&(a, n)| b[a..a + n].iter().flat_map(|&d| [(d >> 1) & 1, d & 1])).collect();
    // First codeword symbol, data bits, punctured trailing parity (leading positions are shortened zeros).
    let (j0, n, punct) = if fast { (9, 144, 54..63) } else { (5, 168, 57..63) };
    let mut hb = [0u8; 63];
    for (i, c) in x.chunks(6).enumerate() {
        hb[j0 + i] = c.iter().fold(0, |v, &bit| (v << 1) | bit);
    }
    let erasures: Vec<usize> = punct.collect();
    rs63::decode(&mut hb, &erasures)?;
    let data: Vec<u8> = hb[j0..j0 + (n + 12) / 6].iter().flat_map(|&v| (0..6).rev().map(move |k| (v >> k) & 1)).collect();
    let crc = data[n..n + 12].iter().fold(0u16, |c, &b| (c << 1) | b as u16);
    (crc12(&data[..n]) == crc).then(|| data[..n].chunks(8).map(|c| c.iter().fold(0, |v, &b| (v << 1) | b)).collect())
}

fn parse(p: &[u8], slot: u8, fast: bool) -> Pdu {
    let op = p[0] >> 5;
    let kind = match op {
        0 => Kind::Signal,
        1 => Kind::Ptt,
        2 => Kind::EndPtt,
        3 => Kind::Idle,
        4 => Kind::Active,
        6 => Kind::Hangtime,
        o => Kind::Other(o),
    };
    let be = |a: usize, n: usize| p[a..a + n].iter().fold(0u32, |v, &b| (v << 8) | b as u32);
    let (mut tg, mut src) = (None, None);
    if matches!(kind, Kind::Ptt | Kind::EndPtt) {
        (tg, src) = (Some(be(16, 2) as u16), Some(be(13, 3)));
    } else if matches!(kind, Kind::Idle | Kind::Active | Kind::Hangtime) && p[1] == 0x01 {
        (tg, src) = (Some(be(3, 2) as u16), Some(be(5, 3))); // Group Voice Channel User (abbreviated)
    }
    let alg = matches!(kind, Kind::Ptt).then(|| p[10]);
    Pdu { kind, tg, src, mco: p[1], slot, lch: LCH[slot as usize], fast, alg }
}

pub struct P2 {
    xm: Vec<u8>,
    hist: Vec<u8>,
    hist0: u64, // absolute index of hist[0]
    n: u64,
    acc: u64,
    hits: Vec<u64>, // recent S-ISCH starts
    anchor: Option<(u64, i64)>, // pair start, relative slot number
    next: (u64, i64),           // next unit to cut
    good: [u32; 3],
    pub slots: u64,
    pub acch: u64,
    pub acch_good: u64,
    /// Voice frames from 4V/2V bursts, appended as they are cut; callers drain it.
    pub voice: Vec<VoiceFrame>,
    /// ESS decoded at each 2V burst; callers drain it.
    pub ess: Vec<Ess>,
    /// Last bursts per logical channel: DUID (or None) and descrambled 4V bits for ESS-B.
    recent: [std::collections::VecDeque<(Option<u8>, Option<[u8; 12]>)>; 2],
}

impl P2 {
    pub fn new(nac: u16, sysid: u16, wacn: u32) -> Self {
        Self { xm: scrambler(nac, sysid, wacn), hist: Vec::new(), hist0: 0, n: 0, acc: 0, hits: Vec::new(), anchor: None, next: (0, 0), good: [0; 3], slots: 0, acch: 0, acch_good: 0, voice: Vec::new(), ess: Vec::new(), recent: Default::default() }
    }

    pub fn locked(&self) -> bool {
        self.anchor.is_some()
    }

    pub fn push(&mut self, dibits: &[u8], out: &mut Vec<Pdu>) {
        for &d in dibits {
            self.hist.push(d);
            self.n += 1;
            self.acc = ((self.acc << 2) | d as u64) & 0xFF_FFFF_FFFF;
            if (self.acc ^ SYNC).count_ones() <= 4 {
                let s = self.n - 20;
                // First of a pair: the previous hit is exactly one slot back, and none before it.
                if self.hits.contains(&(s.wrapping_sub(BURST))) && !self.hits.contains(&(s.wrapping_sub(2 * BURST))) {
                    self.reanchor(s - BURST);
                }
                self.hits.push(s);
                if self.hits.len() > 16 {
                    self.hits.remove(0);
                }
            }
            self.cut(out);
        }
        if self.hist.len() > 8000 {
            let keep_from = if self.anchor.is_some() { self.next.0.min(self.n - 4000) } else { self.n - 4000 }.max(self.hist0);
            let drop = (keep_from - self.hist0) as usize;
            self.hist.drain(..drop);
            self.hist0 += drop as u64;
        }
    }

    fn reanchor(&mut self, p: u64) {
        let rel = match self.anchor {
            None => 0,
            Some((a, r)) => r + 4 * ((p as i64 - a as i64) as f64 / (4 * BURST) as f64).round() as i64,
        };
        self.anchor = Some((p, rel));
        // Continue after the last slot already cut, positioned on the new grid.
        let (_, nrel) = self.next;
        let from = if self.slots == 0 || nrel < rel { rel } else { nrel };
        let pos = p as i64 + (from - rel) * BURST as i64;
        self.next = (pos.max(self.hist0 as i64) as u64, from);
    }

    fn cut(&mut self, out: &mut Vec<Pdu>) {
        let Some((_, arel)) = self.anchor else { return };
        while self.next.0 + BURST <= self.n {
            let (pos, rel) = self.next;
            if rel - arel > 24 {
                self.anchor = None; // two superframes without a pair: lost
                return;
            }
            let i = (pos - self.hist0) as usize;
            let unit: Vec<u8> = self.hist[i..i + BURST as usize].to_vec();
            self.unit(&unit, rel, out);
            self.next = (pos + BURST, rel + 1);
            self.slots += 1;
        }
    }

    /// The channel's superframe is 4V 4V SACCH 4V 4V 2V: ESS-B comes from the 4V bursts 5, 4, 2
    /// and 1 back (a missing one becomes four erasures), ESS-A from this 2V, as RS(63,35)
    /// shortened to [19 zero | 16 ESS-B | 28 ESS-A].
    fn ess_decode(&self, lch: usize, b: &[u8]) -> Option<Ess> {
        let h = &self.recent[lch];
        if h.len() < 6 {
            return None;
        }
        let hex = |d: &[u8]| (d[0] << 4) | (d[1] << 2) | d[2];
        let mut cw = [0u8; 63];
        let mut era = Vec::new();
        for (k, back) in [5usize, 4, 2, 1].into_iter().enumerate() {
            match h[h.len() - 1 - back] {
                (Some(0), Some(e)) => {
                    for i in 0..4 {
                        cw[19 + 4 * k + i] = hex(&e[3 * i..3 * i + 3]);
                    }
                }
                _ => era.extend(19 + 4 * k..23 + 4 * k),
            }
        }
        for i in 0..28 {
            let j = 84 + 3 * i + (i > 15) as usize; // skip the DUID dibit at 132
            cw[35 + i] = hex(&b[j..j + 3]);
        }
        rs63::decode(&mut cw, &era)?;
        let e = &cw[19..35];
        Some(Ess { lch: lch as u8, alg: (e[0] << 2) | (e[1] >> 4), key: (((e[1] & 15) as u16) << 12) | ((e[2] as u16) << 6) | e[3] as u16 })
    }

    fn unit(&mut self, unit: &[u8], rel: i64, out: &mut Vec<Pdu>) {
        let burst = &unit[10..];
        let d = duid(burst);
        // Burst history per logical channel, on the best-supported superframe position.
        let kb = (0..3).max_by_key(|&k| self.good[k]).unwrap();
        let lch = LCH[((2 + 4 * kb as i64 + rel).rem_euclid(12)) as usize] as usize;
        let mut ess_b = None;
        if d == Some(0) && self.good[kb] > 0 {
            let slot = ((2 + 4 * kb as i64 + rel).rem_euclid(12)) as usize;
            let mut e = [0u8; 12];
            for (i, v) in e.iter_mut().enumerate() {
                *v = burst[84 + i] ^ self.xm[slot * BURST as usize + 84 + i];
            }
            ess_b = Some(e);
        }
        if self.recent[lch].len() == 6 {
            self.recent[lch].pop_front();
        }
        self.recent[lch].push_back((d, ess_b));
        let Some(d) = d else { return };
        if d == 6 && self.good[kb] > 0 {
            let slot = ((2 + 4 * kb as i64 + rel).rem_euclid(12)) as usize;
            let b: Vec<u8> = burst.iter().zip(&self.xm[slot * BURST as usize..]).map(|(a, m)| a ^ m).collect();
            if let Some(e) = self.ess_decode(lch, &b) {
                self.ess.push(e);
            }
        }
        if d == 0 || d == 6 {
            // Voice is always scrambled; use the superframe position the ACCH CRCs settled on.
            let k = (0..3).max_by_key(|&k| self.good[k]).unwrap();
            if self.good[k] == 0 {
                return;
            }
            let slot = ((2 + 4 * k as i64 + rel).rem_euclid(12)) as usize;
            let b: Vec<u8> = burst.iter().zip(&self.xm[slot * BURST as usize..]).map(|(a, m)| a ^ m).collect();
            let starts: &[usize] = if d == 0 { &[11, 48, 96, 133] } else { &[11, 48] };
            for &s in starts {
                let mut c = [0u32; 4];
                for (i, &(w, bit)) in VCW.iter().enumerate() {
                    let v = (b[s + i / 2] >> (1 - i % 2)) & 1;
                    c[w as usize] |= (v as u32) << bit;
                }
                self.voice.push(VoiceFrame { lch: LCH[slot], c });
            }
            return;
        }
        let (fast, scrambled) = match d {
            3 => (false, true),
            12 => (false, false),
            9 => (true, true),
            15 => (true, false),
            _ => return,
        };
        self.acch += 1;
        // Most successful superframe position first.
        let mut order = [0usize, 1, 2];
        order.sort_by_key(|&k| std::cmp::Reverse(self.good[k]));
        for k in order {
            let slot = ((2 + 4 * k as i64 + rel).rem_euclid(12)) as usize;
            let b: Vec<u8> = if scrambled {
                burst.iter().zip(&self.xm[slot * BURST as usize..]).map(|(a, m)| a ^ m).collect()
            } else {
                burst.to_vec()
            };
            if let Some(p) = acch(&b, fast) {
                self.good[k] += 1;
                self.acch_good += 1;
                out.push(parse(&p, slot as u8, fast));
                return;
            }
            if !scrambled {
                return;
            }
        }
    }
}
