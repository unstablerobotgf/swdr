//! Streaming P25 Phase 2 (TDMA) outbound MAC decoder; port of tools/p2_mac.py.
//! Dibits in (from cqpsk::Cqpsk at 6000 sym/s). S-ISCH pairs (slots 2/3, 6/7, 10/11) anchor the
//! 180-dibit slot grid, re-anchored at every pair so a timing slip costs only the slots before
//! the next one. The superframe position (which pair) is resolved by CRC: each ACCH burst is
//! tried under all three, and a CRC-12 pass after RS(63,35) correction is accepted.

use crate::rs63;

const BURST: u64 = 180;
const SYNC: u64 = 0x575D_57F7_FF;
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
    pub fast: bool,
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
    Pdu { kind, tg, src, mco: p[1], slot, fast }
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
}

impl P2 {
    pub fn new(nac: u16, sysid: u16, wacn: u32) -> Self {
        Self { xm: scrambler(nac, sysid, wacn), hist: Vec::new(), hist0: 0, n: 0, acc: 0, hits: Vec::new(), anchor: None, next: (0, 0), good: [0; 3], slots: 0, acch: 0, acch_good: 0 }
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

    fn unit(&mut self, unit: &[u8], rel: i64, out: &mut Vec<Pdu>) {
        let burst = &unit[10..];
        let Some(d) = duid(burst) else { return };
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
