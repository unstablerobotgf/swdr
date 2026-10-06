//! swdr-tap: read the WL33 freq-tap stream over SPI and decode P25 TSBKs on the DK, with the
//! same decoder source the firmware runs. TSBK lines go to stdout as `unix_time\tline`
//! (the swdr-p25 raw.log format); link and decode stats go to stderr.
//! usage: swdr-tap [--hz 16000000] [--stats 30] [--secs N] [--dump FILE.i8] [--iq FILE.u8] [--cmd OP:ARG]...
//! --cmd sends C5 commands to the WL33 over the link, one every 16 frames, in order.
//! Kind-2 (I/Q) frames go to --iq as interleaved u8 offset-binary I/Q.
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
    let mut iq = match args.iter().position(|a| a == "--iq").and_then(|i| args.get(i + 1)) {
        Some(p) => Some(std::fs::File::create(p)?),
        None => None,
    };
    let secs: f64 = arg(&args, "--secs", f64::INFINITY);
    let mut cmds: std::collections::VecDeque<(u8, u32)> = args
        .windows(2)
        .filter(|w| w[0] == "--cmd")
        .filter_map(|w| w[1].split_once(':').and_then(|(o, a)| Some((o.parse().ok()?, a.parse().ok()?))))
        .collect();
    let started = Instant::now();
    let (mut iq_frames, mut xfers) = (0u64, 0u64);
    let mut link = link::Link::open("/dev/spidev0.0", "/dev/gpiochip9", 0, hz)?;
    let mut dec = p25::Decoder::new();
    let mut idens = tsbk::Idens::new();
    let out = std::io::stdout();

    let (mut frames, mut bad, mut gaps, mut lost, mut tsbks) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut last_seq, mut drops0): (Option<u16>, Option<u32>) = (None, None);
    let (mut win_t, mut win_tsbk) = (Instant::now(), 0u64);
    eprintln!("swdr-tap: SPI {} MHz, stats every {} s", hz / 1_000_000, stats_s);
    while started.elapsed().as_secs_f64() < secs {
        // Paced on transfers, not good frames, so a bad header can't fire two commands back to back.
        if xfers % 16 == 0 {
            if let Some((op, a)) = cmds.pop_front() {
                eprintln!("swdr-tap: cmd {op} {a}");
                link.command(op, a);
            }
        }
        if !link.wait_ready(2000)? {
            eprintln!("swdr-tap: no DRDY for 2 s");
            continue;
        }
        let f = link.transfer()?;
        xfers += 1;
        let Some(h) = link::Header::parse(f) else {
            bad += 1;
            continue;
        };
        if h.kind != link::KIND_TAP && h.kind != link::KIND_IQ {
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
        if h.kind == link::KIND_IQ {
            iq_frames += 1;
            if let Some(w) = iq.as_mut() {
                w.write_all(payload)?;
            }
            continue;
        }
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
    eprintln!("swdr-tap: done: frames={frames} iq_frames={iq_frames} bad={bad} gaps={gaps} lost={lost} tsbk={tsbks}");
    Ok(())
}
