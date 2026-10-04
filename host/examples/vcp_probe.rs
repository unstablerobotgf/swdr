//! Capture the VCP for a few seconds: frame count, sequence gaps, payload stats.
use std::io::Read;
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    let port = std::env::args().nth(1).unwrap_or("COM11".into());
    let secs_total: u64 = std::env::args().nth(2).and_then(|v| v.parse().ok()).unwrap_or(3);
    // With REPLUG=1, wait for the VCP to vanish and come back (COM list only, no probe sweep).
    if std::env::var("REPLUG").is_ok() {
        let present = |p: &str| serialport::available_ports().map(|v| v.iter().any(|x| x.port_name == p)).unwrap_or(false);
        println!("waiting for {port} to disappear (unplug the board)...");
        while present(&port) { std::thread::sleep(Duration::from_millis(250)); }
        println!("gone; waiting for it to come back...");
        while !present(&port) { std::thread::sleep(Duration::from_millis(250)); }
    }
    let mut sp = loop {
        match serialport::new(&port, 2_000_000).timeout(Duration::from_millis(100)).open() {
            Ok(sp) => break sp,
            Err(_) => std::thread::sleep(Duration::from_millis(250)),
        }
    };
    println!("{port} open");
    let (mut raw, mut buf, t) = (Vec::new(), [0u8; 8192], Instant::now());
    let mut per_sec = vec![0usize; secs_total as usize + 1];
    while t.elapsed() < Duration::from_secs(secs_total) {
        if let Ok(n) = sp.read(&mut buf) { raw.extend_from_slice(&buf[..n]); per_sec[t.elapsed().as_secs() as usize] += n; }
    }
    println!("kB per second: {:?}", per_sec.iter().map(|b| b / 1000).collect::<Vec<_>>());
    let secs = t.elapsed().as_secs_f64();
    println!("{} bytes in {secs:.2}s = {:.1} kB/s", raw.len(), raw.len() as f64 / secs / 1e3);
    // Boot markers are ASCII text before the first frame.
    let first = raw.windows(2).position(|w| w == [0xA5, 0x5A]).unwrap_or(raw.len());
    println!("text before first frame: {:?}", String::from_utf8_lossy(&raw[..first.min(600)]));
    let (mut i, mut frames, mut gaps, mut last, mut hist) = (0, 0, 0, None::<u16>, [0u64; 256]);
    while i + 8 <= raw.len() {
        if raw[i] == 0xA5 && raw[i + 1] == 0x5A {
            let seq = u16::from_le_bytes([raw[i + 2], raw[i + 3]]);
            let len = u16::from_le_bytes([raw[i + 4], raw[i + 5]]) as usize;
            if len == 1024 && i + 8 + len <= raw.len() {
                if let Some(l) = last { if seq != l.wrapping_add(1) { gaps += 1 } }
                if frames == 0 { println!("first frame: seq {seq} rate_exp {} shift {}", raw[i + 6], raw[i + 7]); }
                for &b in &raw[i + 8..i + 8 + len] { hist[b as usize] += 1 }
                last = Some(seq); frames += 1; i += 8 + len; continue;
            }
        }
        i += 1;
    }
    let total: u64 = hist.iter().sum();
    let mean = hist.iter().enumerate().map(|(v, &c)| v as f64 * c as f64).sum::<f64>() / total.max(1) as f64;
    let (lo, hi) = (hist.iter().position(|&c| c > 0), hist.iter().rposition(|&c| c > 0));
    println!("{frames} frames, {gaps} seq gaps; payload mean {mean:.1}, min {lo:?}, max {hi:?}, clipped {} / {}", hist[0] + hist[255], total);
    Ok(())
}
