//! Tuning-range sweep over SWD/RTT: one attach, step the synth, record lock + VCO cal code.
//! usage: sweep START_MHZ STOP_MHZ STEP_MHZ
#[path = "../src/rtt.rs"]
mod rtt;
use probe_rs::{config::Registry, probe::list::Lister, Permissions};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let a: Vec<f64> = std::env::args().skip(1).map(|v| v.parse().unwrap()).collect();
    let (start, stop, step) = (a[0], a[1], a[2]);
    let mut reg = Registry::from_builtin_families();
    reg.add_target_family_from_yaml(&std::fs::read_to_string("../probe/STM32WL3_Series.yaml")?)?;
    let probes = Lister::new().list_all(); // single USB enumeration
    let mut p = probes[0].open()?;
    p.set_speed(24000)?;
    let mut s = p.attach_with_registry("STM32WL33CCVx", Permissions::default(), &reg)?;
    let mut c = s.core(0)?;
    let r = rtt::Rtt::attach(&mut c, 0x2000_0100..0x2000_8000)?;
    let (mut log, mut junk) = (vec![0u8; 512], vec![0u8; 32768]);
    let mut text = String::new();
    // Firmware needs a few seconds after reset (LSE restart) before it services commands.
    let t0 = std::time::Instant::now();
    while !text.contains("tune ") && t0.elapsed() < Duration::from_secs(30) {
        let n = r.up[0].read(&mut c, &mut log)?;
        text.push_str(&String::from_utf8_lossy(&log[..n]));
        let _ = r.up[1].read(&mut c, &mut junk)?;
        std::thread::sleep(Duration::from_millis(100));
    }
    println!("firmware ready after {:.1} s: {:?}", t0.elapsed().as_secs_f64(), text.trim());
    text.clear();
    let mut f = start;
    while f <= stop + 1e-9 {
        let hz = (f * 1e6).round() as u32;
        let a = hz.to_le_bytes();
        r.down[0].write(&mut c, &[0xC5, 1, a[0], a[1], a[2], a[3]])?;
        std::thread::sleep(Duration::from_millis(120));
        let _ = r.up[1].read(&mut c, &mut junk)?; // keep the I/Q ring drained
        let n = r.up[0].read(&mut c, &mut log)?;
        text.push_str(&String::from_utf8_lossy(&log[..n]));
        let line = text.lines().rev().find(|l| l.starts_with(&format!("tune {hz} Hz"))).map(str::to_string).unwrap_or_else(|| format!("(no report; log: {:?})", text.trim()));
        println!("{f:8.2} MHz  {line}");
        text.clear();
        f += step;
    }
    Ok(())
}
