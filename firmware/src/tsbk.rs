//! Human-readable TSBK lines. Opcode names from SDRTrunk (p25/phase1/message/tsbk/Opcode.java);
//! field positions from OP25 tk_p25.py decode_tsbk (96-bit TSBK incl. CRC, byte 0 = bits 95..88).
//! Channel IDs (4-bit iden + 12-bit channel) become frequencies once IDEN_UP messages are seen.

use core::fmt::Write;

/// Standard (MFID 0x00) outbound opcode short names, indexed by opcode.
const NAMES: [&str; 64] = [
    "GRP_VCH_GRANT", "RESERVED_01", "GRP_VCH_GRANT_UPD", "GRP_VCH_GRANT_UPX", "UU_VCH_GRANT", "UU_ANS_REQ",
    "UU_VCH_GRANT_UPD", "RESERVED_07", "TEL_INT_VCH_GRANT", "TEL_INT_VCH_GRANT_UPD", "TEL_INT_ANS_REQ",
    "RESERVED_0B", "RESERVED_0C", "RESERVED_0D", "RESERVED_0E", "RESERVED_0F", "IND_DCH_GRANT", "GRP_DCH_GRANT",
    "GRP_DCH_ANN", "GRP_DCH_ANN_EXP", "SNDCP_DCH_GRANT", "SNDCP_DCH_PAGE_REQ", "SNDCP_DCH_ANN_EXP",
    "RESERVED_17", "STATUS_UPDATE", "RESERVED_19", "STATUS_QUERY", "RESERVED_1B", "MESSAGE_UPDATE",
    "RADIO_MONITOR_CMD", "RADIO_MONITOR_ENH_CMD", "CALL_ALERT", "ACK_RESPONSE", "QUEUED_RESPONSE",
    "RESERVED_22", "RESERVED_23", "EXT_FUNCTION_CMD", "RESERVED_25", "RESERVED_26", "DENY_RESPONSE",
    "GRP_AFFIL_RESPONSE", "SEC_CCH_BCAST_EXP", "GRP_AFFIL_QUERY", "LOC_REG_RESPONSE", "UNIT_REG_RESPONSE",
    "UNIT_REG_CMD", "AUTH_CMD", "DEREG_ACK", "TDMA_SYNC_BCAST", "AUTH_DEMAND", "AUTH_FNE_RESPONSE",
    "IDEN_UPDATE_TDMA", "IDEN_UPDATE_VUHF", "TIME_DATE_ANN", "ROAM_ADDR_CMD", "ROAM_ADDR_UPDATE",
    "SYS_SVC_BCAST", "SEC_CCH_BCAST", "RFSS_STATUS_BCAST", "NET_STATUS_BCAST", "ADJ_STATUS_BCAST",
    "IDEN_UPDATE", "ADJ_STATUS_BCAST_UNC", "RESERVED_3F",
];

#[derive(Clone, Copy)]
struct Iden {
    base_hz: u32,
    step_hz: u32,
    slots: u8, // TDMA slots per carrier (1 for FDMA)
}

/// Channel identifier table learned from IDEN_UPDATE messages.
pub struct Idens([Option<Iden>; 16]);

impl Idens {
    pub const fn new() -> Self {
        Self([None; 16])
    }

    fn freq(&self, ch: u32) -> Option<u32> {
        let i = self.0[(ch >> 12) as usize & 0xF]?;
        Some(i.base_hz + i.step_hz * ((ch & 0xFFF) / i.slots.max(1) as u32))
    }
}

/// Fixed-capacity text line for the VCP.
pub struct Line {
    pub buf: [u8; 192],
    pub len: usize,
}

impl Write for Line {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let n = s.len().min(self.buf.len() - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&s.as_bytes()[..n]);
        self.len += n;
        Ok(())
    }
}

/// Field `width` bits wide whose LSB is `shift` bits up from the end of the 96-bit TSBK (OP25 numbering).
pub fn field(b: &[u8; 12], shift: u32, width: u32) -> u32 {
    f(b, shift, width)
}

fn f(b: &[u8; 12], shift: u32, width: u32) -> u32 {
    let mut v: u128 = 0;
    for &x in b {
        v = (v << 8) | x as u128;
    }
    ((v >> shift) & ((1u128 << width) - 1)) as u32
}

fn ch(line: &mut Line, idens: &Idens, key: &str, id: u32) {
    let _ = write!(line, " {key}={}-{}", id >> 12, id & 0xFFF);
    if let Some(hz) = idens.freq(id) {
        let _ = write!(line, "({}.{:06}MHz)", hz / 1_000_000, hz % 1_000_000);
    }
}

/// Decode one TSBK into `line`, learning identifier tables along the way.
pub fn format(nac: u16, b: &[u8; 12], trellis_errs: u8, nid_errs: u8, idens: &mut Idens) -> Line {
    let mut l = Line { buf: [0; 192], len: 0 };
    let (op, mfid) = (b[0] & 0x3F, b[1]);
    let _ = write!(l, "nac={nac:03x} ");
    if mfid != 0 {
        let _ = write!(l, "VENDOR mfid={mfid:02x} op={op:02x}");
    } else {
        let _ = write!(l, "{}", NAMES[op as usize]);
        match op {
            0x00 => {
                let _ = write!(l, " tg={} src={}", f(b, 40, 16), f(b, 16, 24));
                ch(&mut l, idens, "ch", f(b, 56, 16));
            }
            0x02 => {
                ch(&mut l, idens, "ch1", f(b, 64, 16));
                let _ = write!(l, " tg1={}", f(b, 48, 16));
                ch(&mut l, idens, "ch2", f(b, 32, 16));
                let _ = write!(l, " tg2={}", f(b, 16, 16));
            }
            0x03 => {
                ch(&mut l, idens, "ch_tx", f(b, 48, 16));
                ch(&mut l, idens, "ch_rx", f(b, 32, 16));
                let _ = write!(l, " tg={}", f(b, 16, 16));
            }
            0x16 => {
                ch(&mut l, idens, "ch_tx", f(b, 48, 16));
                ch(&mut l, idens, "ch_rx", f(b, 32, 16));
            }
            0x28 => {
                let _ = write!(l, " result={} tg={} unit={}", f(b, 72, 2), f(b, 40, 16), f(b, 16, 24));
            }
            0x2C => {
                let _ = write!(l, " result={} sys={:03x} src={}", f(b, 76, 2), f(b, 64, 12), f(b, 16, 24));
            }
            0x2F => {
                let _ = write!(l, " wacn={:05x} sys={:03x} src={}", f(b, 52, 20), f(b, 40, 12), f(b, 16, 24));
            }
            0x33 | 0x34 | 0x3D => {
                let iden = f(b, 76, 4) as usize;
                let spac = f(b, 48, 10);
                let base_hz = f(b, 16, 32).wrapping_mul(5);
                let (tx_offset_hz, slots) = match op {
                    // 9-bit offset, bit 8 = sign (1 = mobile transmits above), units of 250 kHz.
                    0x3D => {
                        let t = f(b, 58, 9);
                        let mag = (t & 0xFF) as i32 * 250_000;
                        (if t >> 8 & 1 == 1 { mag } else { -mag }, 1)
                    }
                    // 14-bit offset, bit 13 = sign, units of the channel spacing.
                    _ => {
                        let t = f(b, 58, 14);
                        let mag = (t & 0x1FFF) as i32 * spac as i32 * 125;
                        let s = if op == 0x33 { [1, 1, 1, 2, 4, 2][f(b, 72, 4).min(5) as usize] } else { 1 };
                        (if t >> 13 & 1 == 1 { mag } else { -mag }, s)
                    }
                };
                idens.0[iden] = Some(Iden { base_hz, step_hz: spac * 125, slots });
                let _ = write!(
                    l,
                    " id={iden} base={}.{:06}MHz step={}Hz tx_off={}kHz",
                    base_hz / 1_000_000,
                    base_hz % 1_000_000,
                    spac * 125,
                    tx_offset_hz / 1000
                );
                if slots > 1 {
                    let _ = write!(l, " tdma_slots={slots}");
                }
            }
            0x38 => {
                let _ = write!(l, " avail={:06x} supp={:06x}", f(b, 48, 24), f(b, 24, 24));
            }
            0x39 => {
                let _ = write!(l, " rfss={} site={}", f(b, 72, 8), f(b, 64, 8));
                ch(&mut l, idens, "ch1", f(b, 48, 16));
                ch(&mut l, idens, "ch2", f(b, 24, 16));
            }
            0x3A => {
                let _ = write!(l, " lra={:02x} sys={:03x} rfss={} site={}", f(b, 72, 8), f(b, 56, 12), f(b, 48, 8), f(b, 40, 8));
                ch(&mut l, idens, "ch", f(b, 24, 16));
            }
            0x3B => {
                let _ = write!(l, " lra={:02x} wacn={:05x} sys={:03x}", f(b, 72, 8), f(b, 52, 20), f(b, 40, 12));
                ch(&mut l, idens, "ch", f(b, 24, 16));
            }
            0x3C => {
                let _ = write!(l, " lra={:02x} flags={:x} rfss={} site={}", f(b, 72, 8), f(b, 68, 4), f(b, 48, 8), f(b, 40, 8));
                ch(&mut l, idens, "ch", f(b, 24, 16));
            }
            _ => {}
        }
    }
    let _ = write!(l, " [");
    for x in b {
        let _ = write!(l, "{x:02x}");
    }
    let _ = write!(l, "] e={trellis_errs} nid_e={nid_errs}");
    l
}
