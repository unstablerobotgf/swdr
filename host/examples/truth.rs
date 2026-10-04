//! Block reads vs ground truth (flash contents from the ELF image on disk).
use probe_rs::{config::Registry, probe::list::Lister, MemoryInterface, Permissions};

fn main() -> anyhow::Result<()> {
    let elf = std::fs::read("../firmware/target/thumbv6m-none-eabi/release/swdr-fw")?;
    let mut reg = Registry::from_builtin_families();
    reg.add_target_family_from_yaml(&std::fs::read_to_string("../probe/STM32WL3_Series.yaml")?)?;
    let mut s = loop {
        let mut p = Lister::new().list_all()[0].open()?;
        p.set_speed(std::env::var("KHZ").ok().and_then(|v| v.parse().ok()).unwrap_or(24000))?;
        if let Ok(s) = p.attach_with_registry("STM32WL33CCVx", Permissions::default(), &reg) {
            break s;
        }
    };
    let mut c = s.core(0)?;
    // Ground truth: single-word reads of the first 64 words of flash.
    let mut truth = vec![];
    for i in 0..64u64 {
        truth.push(loop { if let Ok(v) = c.read_word_32(0x1004_0000 + 4 * i) { break v } });
    }
    println!("truth[0..4] {:x?} (elf vec[0..2] {:x?})", &truth[..4], &elf.windows(8).position(|w| w == [0, 0x80, 0, 0x20, 0xb9, 0x00, 0x04, 0x10]).is_some());
    for len in [16usize, 32, 64, 128, 256] {
        let mut w = vec![0u32; len / 4];
        let mut tries = 0;
        while c.read_32(0x1004_0000, &mut w).is_err() { tries += 1 }
        let bad: Vec<usize> = (0..w.len()).filter(|&i| w[i] != truth[i]).collect();
        let map: String = (0..w.len()).map(|i| if w[i] == truth[i] { '.' } else if w[i] == 0 { '0' } else { 'X' }).collect();
        println!("read_32 len {len:3} (retries {tries}): {:2} bad  {map}", bad.len());
        let mut b = vec![0u8; len];
        while c.read_8(0x1004_0000, &mut b).is_err() {}
        let badb = (0..len / 4).filter(|&i| u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap()) != truth[i]).count();
        let _ = badb;
    }
    Ok(())
}
