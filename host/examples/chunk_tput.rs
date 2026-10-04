//! Effective SRAM read throughput with retry-on-fault, per chunk size, at 24 MHz SWD.
use probe_rs::{config::Registry, probe::list::Lister, MemoryInterface, Permissions};
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    let mut reg = Registry::from_builtin_families();
    reg.add_target_family_from_yaml(&std::fs::read_to_string("../probe/STM32WL3_Series.yaml")?)?;
    let mut s = loop {
        let mut p = Lister::new().list_all()[0].open()?;
        p.set_speed(24000)?;
        if let Ok(s) = p.attach_with_registry("STM32WL33CCVx", Permissions::default(), &reg) {
            break s;
        }
    };
    let mut c = s.core(0)?;
    for chunk in [128usize, 256, 512, 1024, 2048, 4096] {
        let mut b = vec![0u8; chunk];
        let (mut ok, mut bad, t) = (0usize, 0usize, Instant::now());
        while t.elapsed() < Duration::from_secs(2) {
            let a = 0x2000_2000 + (ok * chunk % 0x4000) as u64;
            if c.read(a, &mut b).is_ok() { ok += 1 } else { bad += 1 }
        }
        let secs = t.elapsed().as_secs_f64();
        println!("chunk {chunk:5}: {:7.1} kB/s good, fault rate {:5.1}%", (ok * chunk) as f64 / secs / 1e3, 100.0 * bad as f64 / (ok + bad) as f64);
    }
    Ok(())
}
