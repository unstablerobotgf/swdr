//! One SWD attach: print the firmware's RTT log (up-channel 0) for N seconds. usage: rtt_log [SECONDS]
#[path = "../src/rtt.rs"]
mod rtt;
use probe_rs::{config::Registry, probe::list::Lister, Permissions};
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    let secs: u64 = std::env::args().nth(1).and_then(|v| v.parse().ok()).unwrap_or(5);
    let mut reg = Registry::from_builtin_families();
    reg.add_target_family_from_yaml(&std::fs::read_to_string("../probe/STM32WL3_Series.yaml")?)?;
    let probes = Lister::new().list_all(); // single USB enumeration
    let mut p = probes[0].open()?;
    p.set_speed(24000)?;
    let mut s = p.attach_with_registry("STM32WL33CCVx", Permissions::default(), &reg)?;
    let mut c = s.core(0)?;
    let r = rtt::Rtt::attach(&mut c, 0x2000_0100..0x2000_8000)?;
    let (mut log, mut junk, t) = (vec![0u8; 512], vec![0u8; 32768], Instant::now());
    while t.elapsed() < Duration::from_secs(secs) {
        let n = r.up[0].read(&mut c, &mut log)?;
        print!("{}", String::from_utf8_lossy(&log[..n]));
        let _ = r.up[1].read(&mut c, &mut junk)?;
        std::thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}
