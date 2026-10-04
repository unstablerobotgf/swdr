//! Print text lines from the VCP for N seconds and summarize TSBK lines. usage: vcp_lines [COM] [SECONDS]
use std::io::Read;
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let port = a.first().cloned().unwrap_or("COM11".into());
    let secs: u64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(30);
    let mut sp = serialport::new(&port, 2_000_000).timeout(Duration::from_millis(100)).open()?;
    let (mut text, mut buf, t) = (String::new(), [0u8; 4096], Instant::now());
    while t.elapsed() < Duration::from_secs(secs) {
        if let Ok(n) = sp.read(&mut buf) {
            text.push_str(&String::from_utf8_lossy(&buf[..n]));
        }
    }
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    let tsbk: Vec<&&str> = lines.iter().filter(|l| l.starts_with("TSBK")).collect();
    for l in lines.iter().filter(|l| !l.starts_with("TSBK")).take(10) {
        println!("other: {l}");
    }
    for l in tsbk.iter().take(8) {
        println!("{l}");
    }
    println!("{} TSBK lines in {secs} s ({:.1}/s)", tsbk.len(), tsbk.len() as f64 / secs as f64);
    Ok(())
}
