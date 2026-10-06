//! Run the firmware's P25 decoder (firmware/src/{bch,p25,tsbk}.rs, same files) on a captured
//! frequency-detector tap, optionally with added Gaussian noise; also self-tests the BCH decoder.
//! usage: p25_offline [FREQ_TAP_FILE] [NOISE_SD_COUNTS]
#[path = "../../firmware/src/bch.rs"]
mod bch;
#[path = "../../firmware/src/p25.rs"]
#[allow(dead_code)]
mod p25;
#[path = "../../firmware/src/tsbk.rs"]
mod tsbk;

/// Deterministic xorshift for repeatable tests.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn gauss(&mut self) -> f64 {
        let (u1, u2) = ((self.next() >> 11) as f64 / (1u64 << 53) as f64, (self.next() >> 11) as f64 / (1u64 << 53) as f64);
        (-2.0 * u1.max(1e-12).ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }
}

/// Reference systematic encoder (generator octal 6331141367235453).
fn bch_encode(info: u16) -> u64 {
    const GEN: u64 = 0o6331141367235453;
    let msg = (info as u64) << 47;
    let mut rem = msg;
    for i in (47..63).rev() {
        if rem >> i & 1 != 0 {
            rem ^= GEN << (i - 47);
        }
    }
    msg | rem
}

fn bch_self_test() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..20_000 {
        let info = rng.next() as u16;
        let mut w = bch_encode(info);
        let n = (rng.next() % 12) as u8;
        let mut used = 0u64;
        while used.count_ones() < n as u32 {
            used |= 1 << (rng.next() % 63);
        }
        w ^= used;
        assert_eq!(bch::decode(w), Some((info, n)), "info {info:04x} errs {n}");
    }
    println!("bch.rs self-test: 20000 random codewords with 0..11 errors decoded exactly");
}

fn main() -> anyhow::Result<()> {
    bch_self_test();
    let a: Vec<String> = std::env::args().skip(1).collect();
    let path = a.first().cloned().unwrap_or("capture_freq.bin".into());
    let sd: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let raw = std::fs::read(&path)?;
    let mut rng = Rng(7);
    let samples: Vec<i8> = raw
        .iter()
        .map(|&b| ((b as i8) as f64 + sd * rng.gauss()).round().clamp(-128.0, 127.0) as i8)
        .collect();
    let mut dec = p25::Decoder::new();
    let mut out: Vec<(u16, [u8; 12], u8, u8)> = Vec::new();
    for chunk in samples.chunks(1024) {
        dec.push(chunk, &mut |t| out.push((t.nac, t.bytes, t.trellis_errs, t.nid_errs)));
    }
    let fixed = out.iter().filter(|o| o.3 > 0).count();
    let max_nid = out.iter().map(|o| o.3).max().unwrap_or(0);
    println!(
        "noise sd {sd}: {} CRC-valid TSBKs from {} frame syncs; {fixed} needed NID correction (max {max_nid} bits); {} NIDs uncorrectable",
        out.len(),
        dec.frames_seen,
        dec.nid_rejects
    );
    let mut idens = tsbk::Idens::new();
    let mut seen = std::collections::BTreeMap::new();
    for (nac, b, e, ne) in &out {
        let l = tsbk::format(*nac, b, *e, *ne, &mut idens);
        let text = String::from_utf8_lossy(&l.buf[..l.len]).to_string();
        // P25_LINES=1 prints every TSBK, for diffing against the on-board decoder's VCP lines.
        if std::env::var_os("P25_LINES").is_some() {
            println!("{text}");
        }
        seen.entry(text.split_whitespace().nth(1).unwrap_or("").to_string()).or_insert(text);
    }
    if sd == 0.0 {
        for (_, line) in seen.iter().take(4) {
            println!("  {line}");
        }
    }
    Ok(())
}
