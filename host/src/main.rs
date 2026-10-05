//! rtl_tcp server for the STM32WL3x SDR firmware. Framed u8 I/Q arrives over SWD/RTT or the STLINK VCP.

use anyhow::{Context, Result};
use probe_rs::{config::Registry, probe::list::Lister, Permissions, Session};
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

mod rtt;

const CHIP: &str = "STM32WL33CCVx";
const RAM: std::ops::Range<u64> = 0x2000_0100..0x2000_8000;
/// rtl_tcp gain table (tenths of dB): index i drops the 8-bit window by i bits from shift 8.
const GAINS: [u32; 9] = [0, 60, 120, 180, 240, 300, 360, 420, 480];
const PAYLOAD: usize = 1024;

struct Args {
    listen: String,
    port: String,
    swd: bool,
    yaml: String,
    probe: Option<String>,
}

fn args() -> Args {
    let mut a = Args {
        listen: "127.0.0.1:1234".into(),
        port: "COM11".into(),
        swd: false,
        yaml: "../probe/STM32WL3_Series.yaml".into(),
        probe: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(k) = it.next() {
        match k.as_str() {
            "--swd" => a.swd = true,
            "--listen" => a.listen = it.next().unwrap_or_default(),
            "--port" => a.port = it.next().unwrap_or_default(),
            "--yaml" => a.yaml = it.next().unwrap_or_default(),
            "--probe" => a.probe = it.next(),
            _ => panic!("usage: swdr [--swd [--yaml target.yaml] [--probe SERIAL] | --port COMx] [--listen addr:port]"),
        }
    }
    a
}

/// Firmware command: C5 op arg:u32 LE. Ops: 1 freq Hz, 2 rate exponent, 3 shift.
fn fw_cmd(op: u8, arg: u32) -> [u8; 6] {
    let a = arg.to_le_bytes();
    [0xC5, op, a[0], a[1], a[2], a[3]]
}

/// rtl_tcp client command -> firmware command. Fs = 2 MHz >> e on the WL3x.
fn translate(cmd: u8, p: u32, min_rate_exp: u32) -> Option<[u8; 6]> {
    match cmd {
        0x01 => Some(fw_cmd(1, p)),
        0x02 => {
            let e = (min_rate_exp..=9).min_by_key(|e| (2_000_000u32 >> e).abs_diff(p)).unwrap();
            if 2_000_000u32 >> e != p {
                eprintln!("note: client asked {p} S/s, WL3x delivers {} S/s", 2_000_000u32 >> e);
            }
            Some(fw_cmd(2, e))
        }
        0x04 => Some(fw_cmd(3, 8 - (p.min(480) + 30) / 60)),
        0x0d => Some(fw_cmd(3, 8 - p.min(8))),
        _ => None,
    }
}

fn client_reader(mut s: TcpStream, tx: mpsc::Sender<[u8; 6]>, min_rate_exp: u32) {
    let mut b = [0u8; 5];
    while s.read_exact(&mut b).is_ok() {
        let p = u32::from_be_bytes([b[1], b[2], b[3], b[4]]);
        eprintln!("rtl_tcp: cmd {:#04x} param {}", b[0], p);
        if let Some(c) = translate(b[0], p, min_rate_exp) {
            if tx.send(c).is_err() {
                return;
            }
        }
    }
}

/// Pulls whole frames out of the byte stream, resyncing on A5 5A.
struct Deframer {
    buf: Vec<u8>,
    last_seq: Option<u16>,
    gaps: u64,
    rate_exp: u8,
}

impl Deframer {
    fn push(&mut self, data: &[u8], mut out: impl FnMut(&[u8])) {
        self.buf.extend_from_slice(data);
        let mut i = 0;
        while i + 8 <= self.buf.len() {
            let b = &self.buf[i..];
            if b[0] != 0xA5 || b[1] != 0x5A || u16::from_le_bytes([b[4], b[5]]) as usize != PAYLOAD {
                i += 1;
                continue;
            }
            if b.len() < 8 + PAYLOAD {
                break;
            }
            let seq = u16::from_le_bytes([b[2], b[3]]);
            if self.last_seq.is_some_and(|l| seq != l.wrapping_add(1)) {
                self.gaps += 1;
            }
            self.last_seq = Some(seq);
            self.rate_exp = b[6];
            out(&b[8..8 + PAYLOAD]);
            i += 8 + PAYLOAD;
        }
        self.buf.drain(..i);
    }
}

/// I/Q source: framed bytes plus a command sink, over the VCP or SWD/RTT.
enum Link {
    Vcp(Box<dyn serialport::SerialPort>),
    Swd(Session, rtt::Rtt),
}

impl Link {
    /// Lowest rate exponent the transport sustains: VCP ~200 kB/s (E=5), SWD ~630 kB/s (E=3).
    fn min_rate_exp(&self) -> u32 {
        match self {
            Link::Vcp(_) => 5,
            Link::Swd(..) => 3,
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Link::Vcp(p) => match p.read(buf) {
                Ok(n) => Ok(n),
                Err(e) if e.kind() == ErrorKind::TimedOut => Ok(0),
                Err(e) => Err(e.into()),
            },
            Link::Swd(s, r) => {
                let n = r.up[1].read(&mut s.core(0)?, buf)?;
                if n < 4096 {
                    std::thread::sleep(Duration::from_millis(2));
                }
                Ok(n)
            }
        }
    }

    fn command(&mut self, c: &[u8; 6]) -> Result<()> {
        match self {
            Link::Vcp(p) => Ok(p.write_all(c)?),
            Link::Swd(s, r) => {
                if !r.down[0].write(&mut s.core(0)?, c)? {
                    eprintln!("cmd channel full, dropped {c:02x?}");
                }
                Ok(())
            }
        }
    }
}

/// Attach over SWD once. USB is listed a single time: repeated probe sweeps upset other USB devices.
fn open_swd(yaml_path: &str, serial: Option<&str>) -> Result<Link> {
    let mut reg = Registry::from_builtin_families();
    let yaml = std::fs::read_to_string(yaml_path).with_context(|| format!("reading {yaml_path}"))?;
    reg.add_target_family_from_yaml(&yaml)?;
    let probes = Lister::new().list_all();
    // With several ST-LINKs attached, --probe picks one by serial; otherwise take the first.
    let info = probes
        .iter()
        .find(|p| serial.is_none_or(|s| p.serial_number.as_deref() == Some(s)))
        .ok_or_else(|| anyhow::anyhow!("no matching debug probe found"))?;
    let mut last = None;
    for _ in 0..3 {
        let mut probe = info.open()?;
        probe.set_speed(24000)?;
        match probe.attach_with_registry(CHIP, Permissions::default(), &reg) {
            Ok(mut s) => {
                let mut core = s.core(0)?;
                if core.status()?.is_halted() {
                    core.run()?;
                }
                let r = rtt::Rtt::attach(&mut core, RAM).context("is the firmware running?")?;
                drop(core);
                anyhow::ensure!(r.up.len() >= 2 && !r.down.is_empty(), "firmware RTT layout unexpected");
                return Ok(Link::Swd(s, r));
            }
            Err(e) => last = Some(e),
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    Err(last.unwrap().into())
}

fn main() -> Result<()> {
    let a = args();
    // Ctrl-C must not kill us mid-USB-transfer: that wedges the ST-Link until a replug.
    let run = Arc::new(AtomicBool::new(true));
    let r = run.clone();
    ctrlc::set_handler(move || r.store(false, Ordering::Relaxed))?;
    let mut link = if a.swd {
        open_swd(&a.yaml, a.probe.as_deref())?
    } else {
        let p = serialport::new(&a.port, 2_000_000).timeout(Duration::from_millis(20)).open();
        Link::Vcp(p.with_context(|| format!("opening {}", a.port))?)
    };
    let min_rate_exp = link.min_rate_exp();
    let listener = TcpListener::bind(&a.listen)?;
    listener.set_nonblocking(true)?;
    eprintln!("{} link up; rtl_tcp listening on {}", if a.swd { "SWD/RTT" } else { &a.port }, a.listen);

    let mut df = Deframer { buf: Vec::new(), last_seq: None, gaps: 0, rate_exp: 0 };
    let mut rx_buf = vec![0u8; 32 * 1024];
    let mut client: Option<(TcpStream, mpsc::Receiver<[u8; 6]>)> = None;
    let (mut bytes, mut t0) = (0usize, Instant::now());

    while run.load(Ordering::Relaxed) {
        if client.is_none() {
            match listener.accept() {
                Ok((mut s, peer)) => {
                    s.set_nonblocking(false)?;
                    s.set_nodelay(true)?;
                    let mut hdr = *b"RTL0\0\0\0\0\0\0\0\0";
                    hdr[4..8].copy_from_slice(&5u32.to_be_bytes()); // R820T, so clients offer gain control
                    hdr[8..12].copy_from_slice(&(GAINS.len() as u32).to_be_bytes());
                    s.write_all(&hdr)?;
                    let (tx, rx) = mpsc::channel();
                    let rs = s.try_clone()?;
                    std::thread::spawn(move || client_reader(rs, tx, min_rate_exp));
                    eprintln!("client {peer} connected");
                    client = Some((s, rx));
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => return Err(e.into()),
            }
        }

        if let Some((_, rx)) = &client {
            while let Ok(c) = rx.try_recv() {
                link.command(&c)?;
            }
        }

        let n = link.read(&mut rx_buf)?;
        let mut dropped = false;
        df.push(&rx_buf[..n], |iq| {
            bytes += iq.len();
            if let Some((s, _)) = &mut client {
                dropped |= s.write_all(iq).is_err();
            }
        });
        if dropped {
            eprintln!("client disconnected");
            client = None;
        }

        if t0.elapsed() >= Duration::from_secs(2) {
            let secs = t0.elapsed().as_secs_f64();
            eprintln!("iq: {:.1} kS/s (Fs {} S/s), seq gaps {}", bytes as f64 / secs / 2e3, 2_000_000u32 >> df.rate_exp, df.gaps);
            (bytes, t0) = (0, Instant::now());
        }
    }
    eprintln!("shutting down cleanly");
    drop(link);
    Ok(())
}
