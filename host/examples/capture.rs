//! Tune, capture u8 I/Q over SWD/RTT to capture_iq.u8 (rtl_sdr format), and hunt POCSAG sync words.
//! usage: capture [FREQ_HZ] [SECONDS] [SHIFT]   (defaults 929612500, 60, 4). Check tones with tools/fsk_check.py.
#[path = "../src/rtt.rs"]
mod rtt;
use probe_rs::{config::Registry, probe::list::Lister, Permissions};
use std::time::{Duration, Instant};

const SYNC: u32 = 0x7CD2_15D8;
const FS: f64 = 250_000.0;

fn cmd(op: u8, arg: u32) -> [u8; 6] {
    let a = arg.to_le_bytes();
    [0xC5, op, a[0], a[1], a[2], a[3]]
}

fn main() -> anyhow::Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let hz: u32 = a.first().map(|v| v.parse().unwrap()).unwrap_or(929_612_500);
    let secs: u64 = a.get(1).map(|v| v.parse().unwrap()).unwrap_or(60);
    let shift: u32 = a.get(2).map(|v| v.parse().unwrap()).unwrap_or(4);

    let mut reg = Registry::from_builtin_families();
    reg.add_target_family_from_yaml(&std::fs::read_to_string("../probe/STM32WL3_Series.yaml")?)?;
    let probes = Lister::new().list_all(); // single USB enumeration
    let mut p = probes[0].open()?;
    p.set_speed(24000)?;
    let mut s = p.attach_with_registry("STM32WL33CCVx", Permissions::default(), &reg)?;
    let mut c = s.core(0)?;
    let r = rtt::Rtt::attach(&mut c, 0x2000_0100..0x2000_8000)?;
    for k in [cmd(2, 3), cmd(1, hz), cmd(3, shift)] {
        r.down[0].write(&mut c, &k)?;
        std::thread::sleep(Duration::from_millis(150));
    }
    let mut log = vec![0u8; 512];
    let n = r.up[0].read(&mut c, &mut log)?;
    let tune = String::from_utf8_lossy(&log[..n]).lines().filter(|l| l.starts_with("tune")).last().unwrap_or("?").to_string();
    println!("{tune}");

    // Capture and deframe (A5 5A seq len rate shift | 1024 B).
    let (mut raw, mut buf, t0) = (Vec::new(), vec![0u8; 32768], Instant::now());
    let _ = r.up[1].read(&mut c, &mut buf)?; // drop pre-retune samples
    while t0.elapsed() < Duration::from_secs(secs) {
        let n = r.up[1].read(&mut c, &mut buf)?;
        raw.extend_from_slice(&buf[..n]);
        if n < 4096 { std::thread::sleep(Duration::from_millis(2)); }
    }
    let (mut iq, mut i, mut gaps, mut last) = (Vec::<u8>::new(), 0usize, 0, None::<u16>);
    while i + 1032 <= raw.len() {
        if raw[i] == 0xA5 && raw[i + 1] == 0x5A {
            let seq = u16::from_le_bytes([raw[i + 2], raw[i + 3]]);
            if last.is_some_and(|l| seq != l.wrapping_add(1)) { gaps += 1; }
            last = Some(seq);
            iq.extend_from_slice(&raw[i + 8..i + 1032]);
            i += 1032;
        } else { i += 1; }
    }
    std::fs::write("capture_iq.u8", &iq)?; // rtl_sdr-style u8 I/Q for offline analysis
    let ns = iq.len() / 2;
    println!("captured {:.1} s of I/Q ({} samples), frame gaps {}", ns as f64 / FS, ns, gaps);

    // FM discriminator: arg(z[n] * conj(z[n-1])).
    let z: Vec<(f32, f32)> = iq.chunks_exact(2).map(|p| (p[0] as f32 - 127.5, p[1] as f32 - 127.5)).collect();
    let clip = iq.iter().filter(|&&b| b == 0 || b == 255).count();
    let pwr = z.iter().map(|(i, q)| i * i + q * q).sum::<f32>() / ns as f32;
    println!("mean |z|^2 {pwr:.1}, clipped {:.3}%", 100.0 * clip as f64 / iq.len() as f64);
    let fm: Vec<f32> = z.windows(2).map(|w| {
        let (a, b) = (w[1], w[0]);
        (a.1 * b.0 - a.0 * b.1).atan2(a.0 * b.0 + a.1 * b.1)
    }).collect();

    let mut total = 0;
    for baud in [512.0f64, 1200.0, 2400.0] {
        let spb = FS / baud;
        // Integrate-and-dump per bit at 8 clock phases; search both polarities.
        let mut hits = Vec::new();
        for ph in 0..8 {
            let off = ph as f64 * spb / 8.0;
            let mut word: u32 = 0;
            let mut k = 0usize;
            loop {
                let (s0, s1) = ((off + k as f64 * spb) as usize, (off + (k + 1) as f64 * spb) as usize);
                if s1 >= fm.len() { break; }
                let v: f32 = fm[s0..s1].iter().sum();
                word = (word << 1) | (v < 0.0) as u32; // POCSAG: higher freq = 0
                if k >= 31 && (word == SYNC || word == !SYNC) {
                    hits.push((s1 as f64 / FS, word == SYNC));
                }
                k += 1;
            }
        }
        hits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        hits.dedup_by(|a, b| (a.0 - b.0).abs() < 0.01);
        total += hits.len();
        let show: Vec<String> = hits.iter().take(8).map(|(t, pol)| format!("{t:.2}s{}", if *pol { "" } else { "(inv)" })).collect();
        println!("{baud:>6} baud: {} sync codewords {:?}", hits.len(), show);
    }
    println!("{}", if total > 0 { "PASS: POCSAG sync found" } else { "NO SYNC (no traffic in window, or a fault)" });
    Ok(())
}
