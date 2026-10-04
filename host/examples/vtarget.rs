//! Read the target supply voltage measured by the ST-Link (no target attach needed).
use probe_rs::probe::list::Lister;
fn main() -> anyhow::Result<()> {
    for _ in 0..5 {
        let mut p = Lister::new().list_all()[0].open()?;
        println!("target voltage: {:?}", p.get_target_voltage()?);
        drop(p);
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    Ok(())
}
