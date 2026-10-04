//! Run the firmware's P25 decoder (firmware/src/p25.rs, same file) on a captured freq-detector tap.
//! usage: p25_offline FREQ_TAP_FILE
#[path = "../../firmware/src/p25.rs"]
#[allow(dead_code)]
mod p25;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).unwrap_or("capture_freq.bin".into());
    let raw = std::fs::read(&path)?;
    let samples: Vec<i8> = raw.iter().map(|&b| b as i8).collect();
    let mut dec = p25::Decoder::new();
    let mut out: Vec<(u16, [u8; 12], u8)> = Vec::new();
    for chunk in samples.chunks(1024) {
        dec.push(chunk, &mut |t| out.push((t.nac, t.bytes, t.trellis_errs)));
    }
    println!("{} CRC-valid TSBKs from {} frame syncs in {:.1} s", out.len(), dec.frames_seen, samples.len() as f64 / 15625.0);
    for (nac, b, e) in out.iter().take(6) {
        println!("  nac={nac:03x} op={:02x} {} e={e}", b[0] & 0x3f, b.iter().map(|x| format!("{x:02x}")).collect::<String>());
    }
    Ok(())
}
