//! Minimal rtl_tcp client: header check, set freq/rate/gain, measure stream for a few seconds.
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

fn cmd(s: &mut TcpStream, c: u8, p: u32) -> std::io::Result<()> {
    let mut b = [c, 0, 0, 0, 0];
    b[1..].copy_from_slice(&p.to_be_bytes());
    s.write_all(&b)
}

fn main() -> anyhow::Result<()> {
    let mut s = TcpStream::connect("127.0.0.1:1234")?;
    s.set_read_timeout(Some(Duration::from_millis(500)))?;
    let mut hdr = [0u8; 12];
    s.read_exact(&mut hdr)?;
    println!("header {:?} tuner {} gains {}", String::from_utf8_lossy(&hdr[..4]), u32::from_be_bytes(hdr[4..8].try_into()?), u32::from_be_bytes(hdr[8..12].try_into()?));
    cmd(&mut s, 0x01, 868_000_000)?;
    cmd(&mut s, 0x02, std::env::var("RATE").ok().and_then(|v| v.parse().ok()).unwrap_or(250_000))?;
    cmd(&mut s, 0x0d, 4)?;
    let (mut n, mut sum, t, mut buf) = (0usize, 0u64, Instant::now(), vec![0u8; 8192]);
    while t.elapsed() < Duration::from_secs(5) {
        if let Ok(k) = s.read(&mut buf) {
            n += k;
            sum += buf[..k].iter().map(|&b| b as u64).sum::<u64>();
        }
    }
    println!("{} bytes in 5 s = {:.1} kS/s, mean {:.1}", n, n as f64 / 5.0 / 2e3, sum as f64 / n.max(1) as f64);
    Ok(())
}
