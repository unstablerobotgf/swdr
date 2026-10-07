//! Streaming differential QPSK demodulator for P25: Phase 1 CQPSK (LSM, 4800 sym/s) and
//! Phase 2 H-DQPSK (6000 sym/s); ported from tools/cqpsk_demod.py.
//! u8 offset-binary I/Q in. `push` emits a 15625 S/s i8 stream shaped like the WL33 freq tap
//! (each level held for 15625/sym samples) so the firmware's p25::Decoder runs unchanged;
//! `push_dibits` emits hard dibit decisions instead (P25 mapping: +3 -> 1, +1 -> 0, -1 -> 2, -3 -> 3).

use std::f32::consts::{FRAC_PI_4, PI, TAU};

pub const P1_SYM: f32 = 4800.0;
pub const P2_SYM: f32 = 6000.0;
const TAP_FS: f32 = 15625.0;
/// i8 counts per symbol unit (+-1, +-3 -> +-20, +-60); the decoder fits its own scale.
const TAP_GAIN: f32 = 20.0;

#[derive(Clone, Copy, Default)]
struct C(f32, f32);

impl C {
    fn mul(self, o: C) -> C {
        C(self.0 * o.0 - self.1 * o.1, self.0 * o.1 + self.1 * o.0)
    }
    fn conj(self) -> C {
        C(self.0, -self.1)
    }
    fn arg(self) -> f32 {
        self.1.atan2(self.0)
    }
    fn norm2(self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }
    fn lerp(self, o: C, t: f32) -> C {
        C(self.0 + (o.0 - self.0) * t, self.1 + (o.1 - self.1) * t)
    }
}

fn rrc(sps: f32, alpha: f32, span: f32) -> Vec<f32> {
    let n = (span * sps / 2.0) as i32;
    let mut h: Vec<f32> = (-n..=n)
        .map(|i| {
            let x = i as f32 / sps;
            if x.abs() < 1e-6 {
                1.0 - alpha + 4.0 * alpha / PI
            } else if (x.abs() - 1.0 / (4.0 * alpha)).abs() < 1e-6 {
                alpha / 2f32.sqrt() * ((1.0 + 2.0 / PI) * (PI / (4.0 * alpha)).sin() + (1.0 - 2.0 / PI) * (PI / (4.0 * alpha)).cos())
            } else {
                ((PI * x * (1.0 - alpha)).sin() + 4.0 * alpha * x * (PI * x * (1.0 + alpha)).cos())
                    / (PI * x * (1.0 - (4.0 * alpha * x).powi(2)))
            }
        })
        .collect();
    let e = h.iter().map(|v| v * v).sum::<f32>().sqrt();
    h.iter_mut().for_each(|v| *v /= e);
    h
}

pub struct Cqpsk {
    fs: f32,
    sym: f32,
    sps: f32,
    taps: Vec<f32>,
    hist: Vec<C>, // FIR delay line (ring)
    hpos: usize,
    // carrier: slow mean of the per-sample discriminator, then an NCO
    last_in: C,
    off: f32, // rad/sample
    nsamp: f32,
    nco: f32,
    agc: f32,
    // Gardner over filtered samples kept in `y` (front trimmed as symbols are taken)
    y: Vec<C>,
    k: f32,
    mu: f32,
    prev: Option<C>,
    // 4th-power rotation estimate of the differential product (running vector average)
    rot4: C,
    hold: f32,
    pub symbols: u64,
    pub level_err: f32, // running mean |level - nearest ideal|
}

impl Cqpsk {
    pub fn new(fs: f32) -> Self {
        Self::with_rate(fs, P1_SYM)
    }

    pub fn with_rate(fs: f32, sym: f32) -> Self {
        let sps = fs / sym;
        let taps = rrc(sps, 0.2, 8.0);
        Self {
            fs,
            sym,
            sps,
            hist: vec![C::default(); taps.len()],
            taps,
            hpos: 0,
            last_in: C(1.0, 0.0),
            off: 0.0,
            nsamp: 0.0,
            nco: 0.0,
            agc: 1.0,
            y: Vec::with_capacity(4096),
            k: 2.0 * sps,
            mu: 0.0,
            prev: None,
            rot4: C(-1.0, 0.0),
            hold: 0.0,
            symbols: 0,
            level_err: 0.0,
        }
    }

    pub fn carrier_hz(&self) -> f32 {
        self.off * self.fs / TAU
    }

    /// Feed interleaved u8 I/Q; appends freq-tap-like i8 samples to `out`.
    pub fn push(&mut self, iq: &[u8], out: &mut Vec<i8>) {
        let mut lv = Vec::new();
        self.levels(iq, &mut lv);
        for v in lv {
            self.hold += TAP_FS / self.sym;
            let v = (v * TAP_GAIN).clamp(-127.0, 127.0) as i8;
            while self.hold >= 1.0 {
                out.push(v);
                self.hold -= 1.0;
            }
        }
    }

    /// Feed interleaved u8 I/Q; appends one hard dibit per symbol to `out`.
    pub fn push_dibits(&mut self, iq: &[u8], out: &mut Vec<u8>) {
        let mut lv = Vec::new();
        self.levels(iq, &mut lv);
        out.extend(lv.iter().map(|&v| if v >= 2.0 { 1 } else if v >= 0.0 { 0 } else if v >= -2.0 { 2 } else { 3 }));
    }

    /// Feed interleaved u8 I/Q; appends one symbol level (+-1, +-3 ideal) per symbol.
    pub fn levels(&mut self, iq: &[u8], out: &mut Vec<f32>) {
        for p in iq.chunks_exact(2) {
            let x = C(p[0] as f32 - 127.5, p[1] as f32 - 127.5);
            // Carrier offset: the discriminator's mean (data is symmetric). For the first second a
            // ~0.1 s time constant (forgets the retune transient), then ~2 s.
            let d = x.mul(self.last_in.conj()).arg();
            self.last_in = x;
            self.nsamp += 1.0;
            let a = if self.nsamp < self.fs { (1.0 / self.nsamp).max(3e-4) } else { 1.6e-5 };
            self.off += (d - self.off) * a;
            self.nco = (self.nco - self.off) % TAU;
            let z = x.mul(C(self.nco.cos(), self.nco.sin()));
            self.hist[self.hpos] = z;
            self.hpos = (self.hpos + 1) % self.hist.len();
            let mut acc = C::default();
            for (i, &t) in self.taps.iter().enumerate() {
                let h = self.hist[(self.hpos + i) % self.hist.len()];
                acc = C(acc.0 + h.0 * t, acc.1 + h.1 * t);
            }
            self.agc += (acc.norm2().sqrt() - self.agc) * 1e-3;
            let g = 1.0 / self.agc.max(1e-3);
            self.y.push(C(acc.0 * g, acc.1 * g));
        }
        self.gardner(out);
    }

    fn at(&self, p: f32) -> C {
        let i = p as usize;
        self.y[i].lerp(self.y[i + 1], p - i as f32)
    }

    fn gardner(&mut self, out: &mut Vec<f32>) {
        const BW: f32 = 0.01;
        let (g1, g2) = (4.0 * BW, 4.0 * BW * BW);
        while self.k + self.sps + 2.0 < self.y.len() as f32 {
            let cur = self.at(self.k);
            let mid = self.at(self.k - self.sps / 2.0);
            if let Some(prev) = self.prev {
                let e = (C(prev.0 - cur.0, prev.1 - cur.1).mul(mid.conj())).0 / (mid.norm2() + cur.norm2() + 1e-9);
                // Clock-rate integral, clamped to +-0.2%: real clocks are ppm apart, and unbounded
                // it can run off to a false lock where the Gardner error averages zero while slipping.
                self.mu = (self.mu + g2 * e).clamp(-2e-3, 2e-3);
                self.k += self.sps + self.sps * (g1 * e + self.mu);
                self.symbol(cur.mul(prev.conj()), out);
            } else {
                self.k += self.sps;
            }
            self.prev = Some(cur);
        }
        // Keep a few symbols of history for interpolation, drop the rest.
        let keep = (self.k - 2.0 * self.sps).max(0.0) as usize;
        if keep > 0 {
            self.y.drain(..keep);
            self.k -= keep as f32;
        }
    }

    fn symbol(&mut self, d: C, out: &mut Vec<f32>) {
        // 4th power of the unit differential folds +-pi/4, +-3pi/4 onto -1; its angle/4 is the residual rotation.
        let n = d.norm2().sqrt().max(1e-9);
        let u = C(d.0 / n, d.1 / n);
        let u4 = u.mul(u).mul(u.mul(u));
        self.rot4 = C(self.rot4.0 + (u4.0 - self.rot4.0) * 0.002, self.rot4.1 + (u4.1 - self.rot4.1) * 0.002);
        let rot = C(-self.rot4.0, -self.rot4.1).arg() / 4.0;
        let lv = (d.arg() - rot) / FRAC_PI_4; // +-1, +-3
        let lv = if lv > 4.0 { lv - 8.0 } else if lv < -4.0 { lv + 8.0 } else { lv };
        let ideal = (((lv - 1.0) / 2.0).round() * 2.0 + 1.0).clamp(-3.0, 3.0);
        self.level_err += ((lv - ideal).abs() - self.level_err) * 1e-3;
        self.symbols += 1;
        out.push(lv);
    }
}
