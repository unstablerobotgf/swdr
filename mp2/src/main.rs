//! swdr-tap: read the WL33 freq-tap stream over SPI and decode P25 TSBKs on the DK, with the
//! same decoder source the firmware runs. TSBK lines go to stdout as `unix_time\tline`
//! (the swdr-p25 raw.log format); link and decode stats go to stderr.
//! usage: swdr-tap [--hz 16000000] [--stats 30] [--dump FILE.i8]
#[path = "../../firmware/src/bch.rs"]
mod bch;
mod link;
#[path = "../../firmware/src/p25.rs"]
#[allow(dead_code)]
mod p25;
#[path = "../../firmware/src/tsbk.rs"]
mod tsbk;

use std::io::Write;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let hz: u32 = arg(&args, "--hz", 16_000_000);
    let stats_s: u64 = arg(&args, "--stats", 30);
    let mut dump = match args.iter().position(|a| a == "--dump").and_then(|i| args.get(i + 1)) {
        Some(p) => Some(std::fs::File::create(p)?),
        None => None,
    };
    let mut link = link::Link::open("/dev/spidev0.0", "/dev/gpiochip9", 0, hz)?;
    let mut dec = p25::Decoder::new();
    let mut idens = tsbk::Idens::new();
    let out = std::io::stdout();

    let (mut frames, mut bad, mut gaps, mut lost, mut tsbks) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut last_seq, mut drops0): (Option<u16>, Option<u32>) = (None, None);
    let (mut win_t, mut win_tsbk) = (Instant::now(), 0u64);
    eprintln!("swdr-tap: SPI {} MHz, stats every {} s", hz / 1_000_000, stats_s);
    loop {
        if !link.wait_ready(2000)? {
            eprintln!("swdr-tap: no DRDY for 2 s");
            continue;
        }
        let f = link.transfer()?;
        let Some(h) = link::Header::parse(f) else {
            bad += 1;
            continue;
        };
        if h.kind != link::KIND_TAP {
            bad += 1;
            continue;
        }
        if let Some(s) = last_seq {
            let d = h.seq.wrapping_sub(s.wrapping_add(1));
            if d != 0 {
                gaps += 1;
                lost += d as u64;
            }
        }
        last_seq = Some(h.seq);
        drops0.get_or_insert(h.aux0);
        frames += 1;
        let payload = &f[link::HDR..];
        if let Some(d) = dump.as_mut() {
            d.write_all(payload)?;
        }
        let samples: Vec<i8> = payload.iter().map(|&b| b as i8).collect();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let mut o = out.lock();
        dec.push(&samples, &mut |t| {
            let l = tsbk::format(t.nac, &t.bytes, t.trellis_errs, t.nid_errs, &mut idens);
            let _ = writeln!(o, "{:.3}\t{}", now, String::from_utf8_lossy(&l.buf[..l.len]));
            tsbks += 1;
            win_tsbk += 1;
        });
        o.flush()?;
        drop(o);
        if win_t.elapsed().as_secs() >= stats_s {
            let dt = win_t.elapsed().as_secs_f64();
            eprintln!(
                "swdr-tap: frames={} bad={} gaps={} lost={} fw_drops={} tsbk={} ({:.1}/s) syncs={} nid_rej={}",
                frames, bad, gaps, lost, h.aux0.wrapping_sub(drops0.unwrap_or(0)), tsbks, win_tsbk as f64 / dt, dec.frames_seen, dec.nid_rejects
            );
            (win_t, win_tsbk) = (Instant::now(), 0);
        }
    }
}
