//! P25 Phase 1 control-channel (TSBK) decoder over MRSUBG frequency-detector samples.
//!
//! The radio does channel filtering and FM discrimination (RX_MODE=100, i8 at 15625 S/s with
//! CHFLT_E=7). Here: symbol timing, slicing, framing and FEC. Validated offline against an
//! I/Q reference decoder (tools/p25_ref.py, tools/p25_stream.py) on a live control channel.
//! Tables: OP25 p25p1_fdma.cc and SDRTrunk P25P1Interleave / P25_1_2_Node.

/// Timing phases per symbol. Phase p has boundaries at (p + 16k) * sps / 16, so all phases share
/// one boundary grid and each symbol is C(b_j) - C(b_{j-16}) on the running sample sum C.
const PHASES: usize = 16;
/// sps / PHASES in Q24 samples: 15625 / 4800 / 16 * 2^24.
const STEP_Q24: u32 = 3_413_333;
const RING: usize = 48;
const FRAME_MAX: usize = 360;

/// Frame sync 0x5575F5FF77FF as signs: +3 -> 1, -3 -> 0 (all 24 FS symbols are outer).
const SYNC_SIGNS: u32 = 0b1111_1011_0011_0000_1010_0000;
/// Sync symbols in units of one inner step: +-3. sum(e) = -6, sum(e^2) = 216.
const SYNC_E: [i8; 24] = [3, 3, 3, 3, 3, -3, 3, 3, -3, -3, 3, 3, -3, -3, -3, -3, 3, -3, 3, -3, -3, -3, -3, -3];

const TB_DIBIT: [u8; 98] = [
    0, 1, 26, 27, 50, 51, 74, 75, 2, 3, 28, 29, 52, 53, 76, 77, 4, 5, 30, 31, 54, 55, 78, 79, 6, 7, 32, 33, 56, 57,
    80, 81, 8, 9, 34, 35, 58, 59, 82, 83, 10, 11, 36, 37, 60, 61, 84, 85, 12, 13, 38, 39, 62, 63, 86, 87, 14, 15,
    40, 41, 64, 65, 88, 89, 16, 17, 42, 43, 66, 67, 90, 91, 18, 19, 44, 45, 68, 69, 92, 93, 20, 21, 46, 47, 70, 71,
    94, 95, 22, 23, 48, 49, 72, 73, 96, 97, 24, 25,
];
/// Rate-1/2 trellis: NEXT_WORDS[state][input] = transmitted nibble; next state = input.
const NEXT_WORDS: [[u8; 4]; 4] = [[0x2, 0xC, 0x1, 0xF], [0xE, 0x0, 0xD, 0x3], [0x9, 0x7, 0xA, 0x4], [0x5, 0xB, 0x6, 0x8]];

/// One decoded, CRC-valid TSBK.
pub struct Tsbk {
    pub nac: u16,
    pub bytes: [u8; 12],
    pub trellis_errs: u8,
    /// NID bits corrected by BCH(63,16).
    pub nid_errs: u8,
}

#[derive(Clone, Copy)]
struct Phase {
    signs: u32,
    ring: [i32; RING],
    pos: usize,
}

pub struct Decoder {
    // Sample bookkeeping: n = samples consumed, sum = running sum (wrapping), b = next boundary (Q24).
    n: u32,
    sum: i32,
    b_int: u32,
    b_frac: u32,
    j: u32,
    c_hist: [i32; PHASES],
    ph: [Phase; PHASES],
    // Sync selection window: first detection opens it; best LS fit among phases wins.
    window_end: Option<u32>,
    best: Option<(usize, f32, i32, i32)>, // phase, normalized residual, a, dc
    // Frame capture on the chosen phase.
    lock: Option<(usize, i32, i32)>,
    frame: [i32; FRAME_MAX],
    len: usize,
    blocks_done: usize,
    pub frames_seen: u32,
    pub nid_rejects: u32,
}

impl Decoder {
    pub const fn new() -> Self {
        Self {
            n: 0,
            sum: 0,
            b_int: 0,
            b_frac: 0,
            j: 0,
            c_hist: [0; PHASES],
            ph: [Phase { signs: 0, ring: [0; RING], pos: 0 }; PHASES],
            window_end: None,
            best: None,
            lock: None,
            frame: [0; FRAME_MAX],
            len: 0,
            blocks_done: 0,
            frames_seen: 0,
            nid_rejects: 0,
        }
    }

    /// Feed frequency-detector samples; `emit` gets each CRC-valid TSBK.
    pub fn push(&mut self, samples: &[i8], emit: &mut impl FnMut(&Tsbk)) {
        for &x in samples {
            let x = x as i32;
            // Every boundary inside this sample: C(b) = sum_before + frac * x (Q8).
            while self.b_int == self.n {
                let c = self.sum.wrapping_mul(256).wrapping_add(((self.b_frac >> 16) as i32) * x);
                let p = (self.j as usize) % PHASES;
                let v = c.wrapping_sub(self.c_hist[p]);
                self.c_hist[p] = c;
                if self.j >= PHASES as u32 {
                    self.symbol(p, v, emit);
                }
                self.j = self.j.wrapping_add(1);
                self.b_frac += STEP_Q24;
                self.b_int = self.b_int.wrapping_add(self.b_frac >> 24);
                self.b_frac &= 0xFF_FFFF;
            }
            self.sum = self.sum.wrapping_add(x);
            self.n = self.n.wrapping_add(1);
        }
    }

    fn symbol(&mut self, p: usize, v: i32, emit: &mut impl FnMut(&Tsbk)) {
        let ph = &mut self.ph[p];
        ph.signs = ((ph.signs << 1) | (v > 0) as u32) & 0xFF_FFFF;
        ph.ring[ph.pos] = v;
        ph.pos = (ph.pos + 1) % RING;

        if let Some((lp, a, dc)) = self.lock {
            if lp == p {
                self.frame[self.len] = v;
                self.len += 1;
                self.advance_frame(a, dc, emit);
            }
            return;
        }
        if (ph.signs ^ SYNC_SIGNS).count_ones() <= 2 {
            if let Some((a, dc, resid)) = fit(&self.ph[p]) {
                if self.best.map_or(true, |b| resid < b.1) {
                    self.best = Some((p, resid, a, dc));
                }
                self.window_end.get_or_insert(self.j + PHASES as u32);
            }
        }
        if self.window_end.is_some_and(|w| self.j >= w) {
            self.window_end = None;
            if let Some((bp, _, a, dc)) = self.best.take() {
                // Seed the frame with the winner's sync and any symbols it has produced since.
                let ph = &self.ph[bp];
                let newest = (ph.pos + RING - 1) % RING;
                let n_since = self.since_sync(bp).min(RING - 24);
                let start = (newest + RING + 1 - 24 - n_since) % RING;
                self.len = 0;
                for k in 0..24 + n_since {
                    self.frame[self.len] = ph.ring[(start + k) % RING];
                    self.len += 1;
                }
                self.lock = Some((bp, a, dc));
                self.blocks_done = 0;
                self.frames_seen = self.frames_seen.wrapping_add(1);
            }
        }
    }

    /// Symbols the phase produced after its sync pattern ended (0 or 1 within a 16-boundary window).
    fn since_sync(&self, p: usize) -> usize {
        let ph = &self.ph[p];
        for back in 0..2usize {
            let mut signs = 0u32;
            for k in 0..24 {
                let idx = (ph.pos + RING - 1 - back - (23 - k)) % RING;
                signs = (signs << 1) | (ph.ring[idx] > 0) as u32;
            }
            if (signs ^ SYNC_SIGNS).count_ones() <= 2 {
                return back;
            }
        }
        0
    }

    fn advance_frame(&mut self, a: i32, dc: i32, emit: &mut impl FnMut(&Tsbk)) {
        const ENDS: [usize; 3] = [180, 288, 360];
        if self.len < ENDS[self.blocks_done] {
            return;
        }
        let mut dib = [0u8; FRAME_MAX];
        let mut nd = 0;
        for i in 0..self.len {
            if (i + 1) % 36 != 0 {
                dib[nd] = slice(self.frame[i], a, dc);
                nd += 1;
            }
        }
        let mut nid: u64 = 0;
        for &d in &dib[24..56] {
            nid = (nid << 2) | d as u64;
        }
        // 63-bit BCH codeword plus one overall parity bit (ignored). Uncorrectable: drop the frame.
        let Some((info, nid_errs)) = crate::bch::decode(nid >> 1) else {
            self.nid_rejects = self.nid_rejects.wrapping_add(1);
            self.lock = None;
            self.len = 0;
            return;
        };
        let (nac, duid) = (info >> 4, (info & 0xF) as u8);
        let k = self.blocks_done;
        let mut more = false;
        if duid == 0x7 {
            let blk: &[u8; 98] = dib[56 + 98 * k..56 + 98 * (k + 1)].try_into().unwrap();
            let (bytes, errs) = decode_block(blk);
            if crc16_gsm(&bytes[..10]) == u16::from_be_bytes([bytes[10], bytes[11]]) {
                emit(&Tsbk { nac, bytes, trellis_errs: errs, nid_errs });
                more = bytes[0] & 0x80 == 0 && k < 2;
            }
        }
        self.blocks_done += 1;
        if !more {
            self.lock = None;
            self.len = 0;
        }
    }
}

/// Least-squares fit v = a*e + dc over the phase's last 24 symbols (the sync). Returns (a, dc, residual / a^2).
fn fit(ph: &Phase) -> Option<(i32, i32, f32)> {
    let (mut sv, mut sve, mut svv) = (0i64, 0i64, 0i64);
    for k in 0..24 {
        let v = ph.ring[(ph.pos + RING - 24 + k) % RING] as i64;
        sv += v;
        sve += v * SYNC_E[k] as i64;
        svv += v * v;
    }
    // n=24, sum(e)=-6, sum(e^2)=216: a = (24*sve + 6*sv) / (24*216 - 36), dc = (sv + 6a) / 24.
    let a = (24 * sve + 6 * sv) / 5148;
    if a <= 0 {
        return None;
    }
    let dc = (sv + 6 * a) / 24;
    let resid = svv - a * sve - dc * sv;
    Some((a as i32, dc as i32, resid as f32 / (a as f32 * a as f32)))
}

/// Dibit for a symbol value given the sync fit: thresholds at dc and dc +- 2a.
fn slice(v: i32, a: i32, dc: i32) -> u8 {
    let d = v - dc;
    if d >= 2 * a {
        1
    } else if d >= 0 {
        0
    } else if d >= -2 * a {
        2
    } else {
        3
    }
}

/// Deinterleave, hard-decision Viterbi, pack 48 dibits into 12 bytes.
fn decode_block(blk: &[u8; 98]) -> ([u8; 12], u8) {
    let mut nib = [0u8; 49];
    for i in 0..49 {
        nib[i] = (blk[TB_DIBIT[2 * i] as usize] << 2) | blk[TB_DIBIT[2 * i + 1] as usize];
    }
    const INF: u16 = u16::MAX / 2;
    let mut metric = [0, INF, INF, INF];
    let mut prev = [[0u8; 4]; 49];
    for t in 0..49 {
        let mut nm = [INF; 4];
        for s in 0..4 {
            if metric[s] >= INF {
                continue;
            }
            for d in 0..4 {
                let m = metric[s] + (NEXT_WORDS[s][d] ^ nib[t]).count_ones() as u16;
                if m < nm[d] {
                    nm[d] = m;
                    prev[t][d] = s as u8;
                }
            }
        }
        metric = nm;
    }
    // Flush input is 0, so trace back from state 0; the state after step t is that step's input.
    let mut data = [0u8; 49];
    let mut s = 0u8;
    for t in (0..49).rev() {
        data[t] = s;
        s = prev[t][s as usize];
    }
    let mut out = [0u8; 12];
    for (i, &d) in data[..48].iter().enumerate() {
        out[i / 4] |= d << (6 - 2 * (i % 4));
    }
    (out, metric[0].min(255) as u8)
}

/// CRC-16/GSM: poly 0x1021, init 0, xorout 0xFFFF, MSB first.
fn crc16_gsm(data: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &b in data {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 { (crc << 1) ^ 0x1021 } else { crc << 1 };
        }
    }
    crc ^ 0xFFFF
}
