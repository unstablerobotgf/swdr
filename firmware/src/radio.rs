//! MRSUBG bring-up (port of HAL_MRSubG_Init) plus I/Q sampling mode (RX_MODE=0b011, RM0511 29.3.6).

use stm32wl33_pac as pac;

pub const XO_HZ: u64 = 48_000_000;
const F_DIG: u64 = XO_HZ / 3;

// RFSEQ_IRQ_STATUS / _ENABLE bit positions (SVD + CMSIS).
pub const IRQ_SABORT_DONE: u32 = 1 << 8;
pub const IRQ_DB0_USED: u32 = 1 << 16;
pub const IRQ_DB1_USED: u32 = 1 << 17;
pub const IRQ_AHB_ERR: u32 = 1 << 22;

const CMD_RX: u8 = 0x02;
const CMD_SABORT: u8 = 0x05;

/// Undocumented SYNTH0_ANA_ENG (MR_SUBG+0xC0); ST's strobe macro pokes it around TX/RX commands.
const SYNTH0_ANA_ENG: *mut u32 = 0x4900_00C0 as *mut u32;

const MR_SUBG_BASE: u32 = 0x4900_0000;

pub struct Radio {
    p: pac::Peripherals,
}

impl Radio {
    /// Clocks must already be up (rcc::init), since MRSUBG needs the 16 MHz XO/3 domain.
    pub fn new() -> Self {
        let p = unsafe { pac::Peripherals::steal() };
        let rcc = &p.rcc;

        // HAL_MRSubG_MspInit: LSE slow clock, reset + clock the IP.
        rcc.cfgr().modify(|_, w| unsafe { w.clkslowsel().bits(0b01) });
        if rcc.apb2enr().read().mrsubgen().bit_is_clear() {
            rcc.apb2rstr().modify(|_, w| w.mrsubgrst().set_bit());
            rcc.apb2rstr().modify(|_, w| w.mrsubgrst().clear_bit());
            rcc.apb2enr().modify(|_, w| w.mrsubgen().set_bit());
            let _ = rcc.apb2enr().read();
        }

        // HAL_MRSubG_Init design values. AFC1/CLKREC writes equal reset values; kept for parity.
        p.mr_subg.afc1_config().modify(|_, w| unsafe { w.afc_fast_period().bits(0x18) });
        p.mr_subg.clkrec_ctrl0().modify(|_, w| unsafe {
            w.pstflt_len().set_bit().clkrec_p_gain_fast().bits(3).clkrec_i_gain_fast().bits(8)
        });
        p.mr_subg.clkrec_ctrl1().modify(|_, w| unsafe {
            w.clkrec_algo_sel().clear_bit().clkrec_p_gain_slow().bits(5).clkrec_i_gain_slow().bits(0xC)
        });
        // DSSS off for plain 2FSK: ACQ_HITS=3, ACQ_WINDOW=4, everything else 0.
        p.static_.dsss_ctrl().write(|w| unsafe { w.bits(0x0000_0304) });
        p.dynamic_reg.vco_cal_config().modify(|_, w| w.vco_calib_req().set_bit());
        p.mr_subg.rf_fsm7_timeout().write(|w| unsafe { w.bits(0x0F) }); // "avoid AGC glitches"

        // Modulation 2FSK, no gaussian: equalizer off, BT_SEL=0, MOD_TYPE=0.
        p.static_.as_qi_ctrl().modify(|_, w| unsafe { w.as_equ_ctrl().bits(0) });
        p.dynamic_reg.mod0_config().modify(|_, w| unsafe { w.bt_sel().clear_bit().mod_type().bits(0) });

        let mut r = Self { p };
        r.set_frequency(868_000_000);
        r.set_rate_exp(3);
        r
    }

    /// Synth word per MRSubG_ComputeSynthWord; f_rf = f_vco / B, VCO nominally 3.30..3.83 GHz.
    /// ST guarantees 413..479 MHz (B=8) and 826..958 MHz (B=4); the split sits mid-gap.
    pub fn set_frequency(&mut self, hz: u32) {
        let band: u64 = if hz >= 600_000_000 { 4 } else { 8 };
        let word = band * hz as u64 * (1 << 20) / XO_HZ;
        let (int, frac) = ((word >> 20) as u8, (word & 0xF_FFFF) as u32);
        self.p.dynamic_reg.snyth_freq().modify(|_, w| unsafe {
            w.synth_int().bits(int).synth_frac().bits(frac).bs().bit(band == 8)
        });
        self.p.dynamic_reg.additional_ctrl().modify(|_, w| unsafe { w.ch_num().bits(0) });
    }

    /// Fs = 16 MHz / (8 * 2^e). CHFLT_M=0 makes the channel BW 0.8 * Fs on every row of Table 115.
    pub fn set_rate_exp(&mut self, e: u8) {
        let e = e.min(9);
        self.p.dynamic_reg.mod1_config().modify(|_, w| unsafe { w.chflt_e().bits(e).chflt_m().bits(0) });
        // HAL_MRSubG_SetChannelBW: IF 600 kHz above 400 kHz BW, else 300 kHz.
        let bw = 1_600_000u64 >> e;
        let (if_mode, f_if_khz) = if bw > 400_000 { (true, 600u64) } else { (false, 300u64) };
        let off = ((f_if_khz * 100 * 65536) / F_DIG * 10) as u16;
        self.p.static_.if_ctrl().modify(|_, w| unsafe {
            w.if_mode().bit(if_mode).if_offset_ana().bits(off).if_offset_dig().bits(off)
        });
    }

    /// Lock report after a (re)start: FSM state, RFSEQ_STATUS_DETAIL, VCO_CALFREQ_OUT.
    /// DETAIL bit 8 PLL_LOCK_FAIL, 9 PLL_UNLOCK, 10 CALFREQ_ERROR, 11 CALAMP_ERROR (RM0511 29.14.5).
    pub fn tune_status(&self) -> (u8, u32, u8) {
        let s = &self.p.status;
        let fsm = s.radio_fsm_info().read().bits() as u8 & 0x1F;
        let detail = s.rfseq_status_detail().read().bits();
        let cal = s.vco_calib_out().read().bits() as u8 & 0x7F;
        (fsm, detail, cal)
    }

    pub fn sample_rate(&self) -> u32 {
        let e = self.p.dynamic_reg.mod1_config().read().chflt_e().bits();
        (F_DIG as u32 / 8) >> e
    }

    /// P25 Phase 1 C4FM demod in the IP: 4-FSK, 4800 sym/s, outer deviation 1800 Hz (inner = FDEV/3).
    /// CONST_MAP 2 is 00:+F/3 01:+F 10:-F/3 11:-F, i.e. the P25 dibit mapping (RM0511 Table 110).
    /// Register values from ST's MRSubG_SearchDatarateME / SearchFreqDevME (B=4).
    pub fn configure_p25(&mut self) {
        let (s, d) = (&self.p.static_, &self.p.dynamic_reg);
        d.mod0_config().modify(|_, w| unsafe {
            w.mod_type().bits(1).const_map().bits(2).bt_sel().clear_bit().datarate_m().bits(14995).datarate_e().bits(5)
        });
        // fdev M=157 E=0 = 1785 Hz; channel filter E=7 M=0 = 12.5 kHz, low-IF (300 kHz) path.
        d.mod1_config().modify(|_, w| unsafe { w.fdev_m().bits(157).fdev_e().bits(0) });
        // 4-level FSK: post-filter length 8 (value 0), as HAL_MRSubG_SetModulation does for 4FSK.
        self.p.mr_subg.clkrec_ctrl0().modify(|_, w| w.pstflt_len().clear_bit());
        s.pckt_ctrl().modify(|_, w| unsafe { w.whit_en().clear_bit().coding_sel().bits(0).four_fsk_sym_swap().clear_bit() });
        self.set_rate_exp(7);
    }

    /// Undo configure_p25 for I/Q capture (only the channel filter matters there).
    pub fn configure_iq(&mut self, rate_exp: u8) {
        self.p.dynamic_reg.mod0_config().modify(|_, w| unsafe { w.mod_type().bits(0).const_map().bits(0) });
        self.p.mr_subg.clkrec_ctrl0().modify(|_, w| w.pstflt_len().set_bit());
        self.set_rate_exp(rate_exp);
    }

    /// Raw demod taps into the ping-pong buffers (RM0511 29.3.6): 0b001 hard bits,
    /// 0b100 frequency-detector samples (i8 at the channel-filter rate), 0b101 soft symbols (i8 at symbol rate).
    pub fn start_raw(&mut self, rx_mode: u8, buf0: *mut u8, buf1: *mut u8, len: u16) {
        self.start_rx(rx_mode, buf0, buf1, len);
    }

    /// Start continuous I/Q capture into two ping-pong buffers of `len` bytes (multiple of 4).
    pub fn start_iq(&mut self, buf0: *mut u8, buf1: *mut u8, len: u16) {
        self.start_rx(0b011, buf0, buf1, len);
    }

    fn start_rx(&mut self, rx_mode: u8, buf0: *mut u8, buf1: *mut u8, len: u16) {
        let (s, d) = (&self.p.static_, &self.p.dynamic_reg);
        s.pckt_ctrl().modify(|_, w| unsafe { w.rx_mode().bits(rx_mode) });
        s.databuffer0_ptr().write(|w| unsafe { w.bits(buf0 as u32) });
        s.databuffer1_ptr().write(|w| unsafe { w.bits(buf1 as u32) });
        s.databuffer_size().write(|w| unsafe { w.bits(len as u32) });
        d.pcktlen_config().write(|w| unsafe { w.bits(0) });
        self.clear_irq(!0);
        // DETAIL flags are sticky (rc_w1); clear so the next tune report is about this tune only.
        self.p.status.rfseq_status_detail().write(|w| unsafe { w.bits(!0) });
        d.rfseq_irq_enable().write(|w| unsafe { w.bits(IRQ_DB0_USED | IRQ_DB1_USED | IRQ_AHB_ERR) });
        self.strobe(CMD_RX);
    }

    /// SABORT never reports DONE if the FSM already dropped out of RX (e.g. PLL lock fail), so bound the wait.
    pub fn abort(&mut self) {
        self.strobe(CMD_SABORT);
        for _ in 0..100_000 {
            if self.irq_status() & IRQ_SABORT_DONE != 0 {
                break;
            }
        }
        self.clear_irq(!0);
    }

    pub fn irq_status(&self) -> u32 {
        self.p.status.rfseq_irq_status().read().bits()
    }

    pub fn clear_irq(&self, mask: u32) {
        self.p.status.rfseq_irq_status().write(|w| unsafe { w.bits(mask) });
    }

    /// RSSI_LEVEL_RUN in dBm, same scale as HAL_MRSubG_GetRssidBm (raw/2 - 160); scale inferred.
    pub fn rssi_dbm(&self) -> i16 {
        let raw = (self.p.status.rx_indicator().read().bits() >> 12) & 0x1FF;
        raw as i16 / 2 - 160
    }

    /// (RSSI dBm, AGC attenuation step, AFC estimate) for the per-window RF health line.
    pub fn rf_sample(&self) -> (i16, u8, i8) {
        let agc = self.p.status.rx_indicator().read().agc_word().bits();
        (self.rssi_dbm(), agc, self.p.status.qi_info().read().afc_correction().bits() as i8)
    }

    /// Raw MR_SUBG register access for AGC/AFC experiments (ops 6/7); offsets per RM0511 29.10.
    pub fn mr_write(&mut self, off: u32, v: u32) {
        unsafe { ((MR_SUBG_BASE + (off & 0x3FC)) as *mut u32).write_volatile(v) }
    }

    pub fn mr_read(&self, off: u32) -> u32 {
        unsafe { ((MR_SUBG_BASE + (off & 0x3FC)) as *const u32).read_volatile() }
    }

    /// __HAL_MRSUBG_STROBE_CMD, including ST's undocumented 0xC0 dance for TX/RX commands.
    fn strobe(&mut self, cmd: u8) {
        let c = &self.p.dynamic_reg.command();
        if (1..=4).contains(&cmd) {
            let b = c.read().bits();
            c.write(|w| unsafe { w.bits((b & (1 << 26)) | (1 << 25)) });
            unsafe { SYNTH0_ANA_ENG.write_volatile(0x87) };
            c.write(|w| unsafe { w.bits((b & !0xF) | cmd as u32) });
            for _ in 0..3 {
                cortex_m::asm::nop();
            }
            unsafe { SYNTH0_ANA_ENG.write_volatile(0x07) };
        } else {
            c.modify(|_, w| unsafe { w.command_id().bits(cmd) });
        }
    }
}
