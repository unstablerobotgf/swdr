//! Halt and dump the HardFault exception frame (Cortex-M0+ has no CFSR).
use probe_rs::{config::Registry, probe::list::Lister, MemoryInterface, Permissions, RegisterId};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut reg = Registry::from_builtin_families();
    reg.add_target_family_from_yaml(&std::fs::read_to_string("../probe/STM32WL3_Series.yaml")?)?;
    let mut s = loop {
        let mut p = Lister::new().list_all()[0].open()?;
        p.set_speed(3300)?;
        if let Ok(s) = p.attach_with_registry("STM32WL33CCVx", Permissions::default(), &reg) {
            break s;
        }
    };
    let mut c = s.core(0)?;
    c.halt(Duration::from_millis(500))?;
    let pc: u32 = c.read_core_reg(c.program_counter())?;
    let lr: u32 = c.read_core_reg(c.return_address())?;
    let sp: u32 = c.read_core_reg(RegisterId(13))?;
    println!("halted PC={pc:#x} LR={lr:#x} SP={sp:#x}");
    let mut w = [0u32; 16];
    c.read_32(sp as u64, &mut w)?;
    for (i, v) in w.iter().enumerate() {
        println!("  [sp+{:#04x}] {v:#010x}", i * 4);
    }
    c.run()?;
    Ok(())
}
