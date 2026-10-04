//! Deterministic check of firmware/src/activity.rs: steady talkgroup, then a burst -> ALERT.
#[path = "../../firmware/src/tsbk.rs"]
#[allow(dead_code)]
mod tsbk;
#[path = "../../firmware/src/activity.rs"]
mod activity;

/// GRP_VCH_GRANT (op 0x00, MFID 0) for talkgroup `tg`; fields per OP25 positions.
fn grant(tg: u16) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[0] = 0x80;
    b[5] = (tg >> 8) as u8; // tg at bits 55..40 = bytes 5..6 of the 96-bit word
    b[6] = tg as u8;
    b
}

fn main() {
    let mut a = activity::Activity::new();
    let mut out = Vec::new();
    let mut now = 0u32;
    let script: Vec<u16> = [1u16, 1, 1, 1, 1, 1, 12, 1].to_vec(); // grants per 30 s window
    for (w, &n) in script.iter().enumerate() {
        for _ in 0..n {
            a.on_tsbk(&grant(4201), 0, 0, now);
        }
        now += 30;
        a.poll(now, 0, 0, &mut |l| out.push((w, String::from_utf8_lossy(&l.buf[..l.len]).to_string())));
    }
    for (w, l) in &out {
        if l.starts_with("ALERT") || l.contains("tg=") {
            println!("window {w}: {l}");
        }
    }
    let alerts: Vec<usize> = out.iter().filter(|(_, l)| l.starts_with("ALERT")).map(|(w, _)| *w).collect();
    assert_eq!(alerts, vec![6], "alarm must fire only on the burst window");
    println!("activity test: ALERT fired only in burst window 6");
}
