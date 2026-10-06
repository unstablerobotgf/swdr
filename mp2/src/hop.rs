//! Voice-channel hop test: follow a group grant off the control channel, record the voice
//! channel's I/Q for a fixed dwell, then return. Field offsets as in firmware/src/tsbk.rs.

use crate::tsbk::{field, Idens};

pub struct Grant {
    pub tg: u32,
    pub ch: u32,
    pub hz: u32,
    /// Phase 2 logical channel (TDMA timeslot) the call is on.
    pub slot: u8,
}

/// GRP_VCH_GRANT (0x00) or the first slot of GRP_VCH_GRANT_UPD (0x02), standard MFID only.
pub fn grant(b: &[u8; 12], idens: &Idens) -> Option<Grant> {
    if b[1] != 0 {
        return None;
    }
    let (tg, ch) = match b[0] & 0x3F {
        0x00 => (field(b, 40, 16), field(b, 56, 16)),
        0x02 => (field(b, 48, 16), field(b, 64, 16)),
        _ => return None,
    };
    Some(Grant { tg, ch, hz: idens.freq(ch)?, slot: idens.slot(ch)? })
}
