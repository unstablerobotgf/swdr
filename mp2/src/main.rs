//! swdr-tap: read the WL33 freq-tap stream over SPI and decode P25 TSBKs on the DK, with the
//! same decoder source the firmware runs. TSBK lines go to stdout as `unix_time\tline`
//! (the swdr-p25 raw.log format); link and decode stats go to stderr.
//! usage: swdr-tap [--hz 16000000] [--stats 30] [--secs N] [--dump FILE.i8] [--iq FILE.u8] [--cmd OP:ARG]...
//! --cmd sends C5 commands to the WL33 over the link, one every 16 frames, in order.
//! Kind-2 (I/Q) frames go to --iq as interleaved u8 offset-binary I/Q; with --demod cqpsk they
//! are also CQPSK-demodulated (Fs from the frame's rate_exp) into the same decoder.
//! --input FILE.u8 [--fs 31250] replays a recorded I/Q file through --demod instead of the link.
//! --hop TG|any --cc HZ [--dwell 4] [--cooldown 20] [--hopdir DIR]: with --demod cqpsk on the
//! control channel, follow a group grant to its voice channel, record --dwell s of I/Q there to
//! DIR/hop_N_tgT_HZ.u8 (--hopdir none: no recording), then return to --cc. --ppm corrects every retune for the WL33 crystal
//! (the firmware's AFC held -1005 Hz at 851.975 MHz, i.e. about -1.18 ppm).
//! With --nac --sysid --wacn (hex, from the CC's NET/RFSS_STATUS) each hop decodes the Phase 2
//! MAC live and follows the call: --dwell is the acquisition window, each MAC_ACTIVE/PTT for the
//! granted talkgroup extends the stay by --hold s, MAC_END_PTT for it leaves, --maxdwell caps it.
#[path = "../../firmware/src/bch.rs"]
mod bch;
mod cqpsk;
mod hop;
mod link;
mod p2;
#[path = "../../firmware/src/p25.rs"]
#[allow(dead_code)]
mod p25;
mod rs63;
#[path = "../../firmware/src/tsbk.rs"]
mod tsbk;

use std::io::Write;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// Decode freq-tap-like samples, printing TSBKs as `unix_time	line`; returns how many.
fn decode(dec: &mut p25::Decoder, idens: &mut tsbk::Idens, samples: &[i8], got: &mut Vec<[u8; 12]>) -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    let mut o = std::io::stdout().lock();
    let mut n = 0;
    dec.push(samples, &mut |t| {
        let l = tsbk::format(t.nac, &t.bytes, t.trellis_errs, t.nid_errs, idens);
        let _ = writeln!(o, "{:.3}	{}", now, String::from_utf8_lossy(&l.buf[..l.len]));
        got.push(t.bytes);
        n += 1;
    });
    let _ = o.flush();
    n
}

fn hexarg(args: &[String], name: &str) -> Option<u32> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|v| u32::from_str_radix(v, 16).ok())
}

fn p2_line(p: &p2::Pdu) -> String {
    let mut s = format!("P2 slot={:2} lch={} {} {:?}", p.slot, p.lch, if p.fast { "FACCH" } else { "SACCH" }, p.kind);
    match (p.tg, p.src) {
        (Some(t), Some(r)) => s += &format!(" tg={t} src={r}"),
        _ => s += &format!(" mco=0x{:02x}", p.mco),
    }
    s
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
    let use_cqpsk = args.windows(2).any(|w| w[0] == "--demod" && w[1] == "cqpsk");
    let mut demod: Option<cqpsk::Cqpsk> = None;
    let mut tapbuf: Vec<i8> = Vec::with_capacity(4096);
    if let Some(path) = args.iter().position(|a| a == "--input").and_then(|i| args.get(i + 1)) {
        let fs: f32 = arg(&args, "--fs", 31250.0);
        let data = std::fs::read(path)?;
        if args.windows(2).any(|w| w[0] == "--demod" && w[1] == "p2") {
            let ids = (hexarg(&args, "--nac"), hexarg(&args, "--sysid"), hexarg(&args, "--wacn"));
            let (Some(nac), Some(sysid), Some(wacn)) = ids else {
                eprintln!("swdr-tap: --demod p2 needs --nac --sysid --wacn (hex)");
                return Ok(());
            };
            let (mut d, mut p) = (cqpsk::Cqpsk::with_rate(fs, cqpsk::P2_SYM), p2::P2::new(nac as u16, sysid as u16, wacn));
            let (mut dib, mut pdus, t) = (Vec::new(), Vec::new(), Instant::now());
            for chunk in data.chunks(1024) {
                dib.clear();
                d.push_dibits(chunk, &mut dib);
                p.push(&dib, &mut pdus);
            }
            for x in &pdus {
                println!("{}", p2_line(x));
            }
            let mut tgs: Vec<u16> = pdus.iter().filter_map(|x| x.tg).collect();
            tgs.sort();
            tgs.dedup();
            eprintln!(
                "swdr-tap: {path}: {:.1}s in {:.2}s, {} symbols, level err {:.3}, slots {}, ACCH {}/{} CRC good, talkgroups {:?}",
                data.len() as f32 / 2.0 / fs, t.elapsed().as_secs_f32(), d.symbols, d.level_err, p.slots, p.acch_good, p.acch, tgs
            );
            return Ok(());
        }
        let (mut d, mut dec, mut idens) = (cqpsk::Cqpsk::new(fs), p25::Decoder::new(), tsbk::Idens::new());
        let t = Instant::now();
        let mut n = 0;
        for chunk in data.chunks(1024) {
            tapbuf.clear();
            d.push(chunk, &mut tapbuf);
            n += decode(&mut dec, &mut idens, &tapbuf, &mut Vec::new());
        }
        eprintln!(
            "swdr-tap: {path}: {:.1}s of I/Q in {:.2}s, {} symbols, carrier {:+.0} Hz, level err {:.3}, tsbk={n} syncs={} nid_rej={}",
            data.len() as f32 / 2.0 / fs, t.elapsed().as_secs_f32(), d.symbols, d.carrier_hz(), d.level_err, dec.frames_seen, dec.nid_rejects
        );
        return Ok(());
    }
    // Hop test state. A retune leaves ~1 frame of the old channel in flight, so skip 3 after each.
    let hop_tg = args.iter().position(|a| a == "--hop").and_then(|i| args.get(i + 1)).cloned();
    let cc_hz: u32 = arg(&args, "--cc", 851_975_000);
    let ppm: f64 = arg(&args, "--ppm", 0.0);
    let lo = |hz: u32| (hz as f64 * (1.0 + ppm * 1e-6)).round() as u32;
    let (dwell, cooldown): (f64, f64) = (arg(&args, "--dwell", 4.0), arg(&args, "--cooldown", 20.0));
    let hopdir: String = arg(&args, "--hopdir", "/root/hops".to_string());
    let record = hopdir != "none";
    if hop_tg.is_some() && record {
        std::fs::create_dir_all(&hopdir)?;
    }
    struct Voice {
        until: Instant,
        start: Instant,
        file: Option<std::fs::File>,
        tg: u32,
        hz: u32,
        lch: u8,
        frames: u64,
        sum2: f64,
        p2: Option<(cqpsk::Cqpsk, p2::P2)>,
        seen: u32,
        why: &'static str,
    }
    let ids = (hexarg(&args, "--nac"), hexarg(&args, "--sysid"), hexarg(&args, "--wacn"));
    let (hold, maxdwell): (f64, f64) = (arg(&args, "--hold", 1.0), arg(&args, "--maxdwell", 30.0));
    let mut dib: Vec<u8> = Vec::with_capacity(1024);
    let mut pdus: Vec<p2::Pdu> = Vec::new();
    let (mut voice, mut skip, mut hops, mut last_hop): (Option<Voice>, u32, u32, Option<Instant>) = (None, 0, 0, None);
    let mut got: Vec<[u8; 12]> = Vec::new();
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
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if let Some(v) = voice.as_mut() {
                if let Some(f) = v.file.as_mut() {
                    f.write_all(payload)?;
                }
                v.frames += 1;
                v.sum2 += payload.iter().map(|&b| (b as f64 - 127.5).powi(2)).sum::<f64>() / payload.len() as f64;
                if let Some((d, p)) = v.p2.as_mut() {
                    dib.clear();
                    pdus.clear();
                    d.push_dibits(payload, &mut dib);
                    p.push(&dib, &mut pdus);
                    let now_s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                    for x in &pdus {
                        println!("{now_s:.3}	hop={hops} {} Hz {}", v.hz, p2_line(x));
                        // Only our timeslot's signalling; the other slot carries a different call.
                        if x.tg != Some(v.tg as u16) || x.lch != v.lch {
                            continue;
                        }
                        v.seen += 1;
                        match x.kind {
                            p2::Kind::Active | p2::Kind::Ptt => {
                                v.until = v.until.max(Instant::now() + std::time::Duration::from_secs_f64(hold));
                                v.why = "hold expired";
                            }
                            p2::Kind::EndPtt => (v.until, v.why) = (Instant::now(), "END_PTT"),
                            _ => {}
                        }
                    }
                    let cap = v.start + std::time::Duration::from_secs_f64(maxdwell);
                    if Instant::now() >= cap {
                        (v.until, v.why) = (cap, "maxdwell");
                    }
                }
                if Instant::now() >= v.until {
                    link.command(1, lo(cc_hz));
                    let (slots, good, acch) = v.p2.as_ref().map_or((0, 0, 0), |(_, p)| (p.slots, p.acch_good, p.acch));
                    eprintln!(
                        "swdr-tap: hop {hops} back to cc after {:.1}s ({}): tg={} {} Hz, {} frames, rms {:.1}, slots {slots}, ACCH {good}/{acch}, tg seen {}x",
                        v.start.elapsed().as_secs_f64(), v.why, v.tg, v.hz, v.frames, (v.sum2 / v.frames as f64).sqrt(), v.seen
                    );
                    voice = None;
                    skip = 3;
                }
                continue;
            }
            if use_cqpsk {
                let fs = 2_000_000.0 / (1u32 << (h.flags & 0xF)) as f32;
                let d = demod.get_or_insert_with(|| cqpsk::Cqpsk::new(fs));
                tapbuf.clear();
                d.push(payload, &mut tapbuf);
                got.clear();
                let n = decode(&mut dec, &mut idens, &tapbuf, &mut got);
                (tsbks, win_tsbk) = (tsbks + n, win_tsbk + n);
                let free = cmds.is_empty() && last_hop.is_none_or(|t| t.elapsed().as_secs_f64() >= cooldown);
                if let (Some(want), true) = (hop_tg.as_deref(), free) {
                    let g = got.iter().filter_map(|b| hop::grant(b, &idens)).find(|g| want == "any" || want.parse() == Ok(g.tg));
                    if let Some(g) = g.filter(|g| g.hz != cc_hz) {
                        hops += 1;
                        let path = format!("{hopdir}/hop_{hops}_tg{}_{}.u8", g.tg, g.hz);
                        link.command(1, lo(g.hz));
                        eprintln!("swdr-tap: hop {hops} tg={} ch={}-{} {} Hz slot {} -> {path}", g.tg, g.ch >> 12, g.ch & 0xFFF, g.hz, g.slot);
                        let until = Instant::now() + std::time::Duration::from_secs_f64(dwell);
                        let p2 = match ids {
                            (Some(n), Some(s), Some(w)) => Some((cqpsk::Cqpsk::with_rate(fs, cqpsk::P2_SYM), p2::P2::new(n as u16, s as u16, w))),
                            _ => None,
                        };
                        let why = if p2.is_some() { "no MAC for tg in acquisition window" } else { "dwell" };
                        voice = Some(Voice { until, start: Instant::now(), file: if record { Some(std::fs::File::create(&path)?) } else { None }, tg: g.tg, hz: g.hz, lch: g.slot, frames: 0, sum2: 0.0, p2, seen: 0, why });
                        (skip, last_hop) = (3, Some(Instant::now()));
                    }
                }
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
