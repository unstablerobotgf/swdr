#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcktlen_config: PcktlenConfig,
    mod0_config: Mod0Config,
    mod1_config: Mod1Config,
    snyth_freq: SnythFreq,
    vco_cal_config: VcoCalConfig,
    rx_timer: RxTimer,
    databuffer_thr: DatabufferThr,
    rfseq_irq_enable: RfseqIrqEnable,
    additional_ctrl: AdditionalCtrl,
    fast_rx_timer: FastRxTimer,
    command: Command,
}
impl RegisterBlock {
    #[doc = "0x00 - PCKTLEN_CONFIG register"]
    #[inline(always)]
    pub const fn pcktlen_config(&self) -> &PcktlenConfig {
        &self.pcktlen_config
    }
    #[doc = "0x04 - MOD0_CONFIG register"]
    #[inline(always)]
    pub const fn mod0_config(&self) -> &Mod0Config {
        &self.mod0_config
    }
    #[doc = "0x08 - MOD1_CONFIG register"]
    #[inline(always)]
    pub const fn mod1_config(&self) -> &Mod1Config {
        &self.mod1_config
    }
    #[doc = "0x0c - SNYTH_FREQ register"]
    #[inline(always)]
    pub const fn snyth_freq(&self) -> &SnythFreq {
        &self.snyth_freq
    }
    #[doc = "0x10 - VCO_CAL_CONFIG register"]
    #[inline(always)]
    pub const fn vco_cal_config(&self) -> &VcoCalConfig {
        &self.vco_cal_config
    }
    #[doc = "0x14 - RX_TIMER register"]
    #[inline(always)]
    pub const fn rx_timer(&self) -> &RxTimer {
        &self.rx_timer
    }
    #[doc = "0x18 - DATABUFFER_THR register"]
    #[inline(always)]
    pub const fn databuffer_thr(&self) -> &DatabufferThr {
        &self.databuffer_thr
    }
    #[doc = "0x1c - RFSEQ_IRQ_ENABLE register"]
    #[inline(always)]
    pub const fn rfseq_irq_enable(&self) -> &RfseqIrqEnable {
        &self.rfseq_irq_enable
    }
    #[doc = "0x20 - ADDITIONAL_CTRL register"]
    #[inline(always)]
    pub const fn additional_ctrl(&self) -> &AdditionalCtrl {
        &self.additional_ctrl
    }
    #[doc = "0x24 - FAST_RX_TIMER register"]
    #[inline(always)]
    pub const fn fast_rx_timer(&self) -> &FastRxTimer {
        &self.fast_rx_timer
    }
    #[doc = "0x28 - COMMAND register"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
}
#[doc = "PCKTLEN_CONFIG (rw) register accessor: PCKTLEN_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcktlen_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcktlen_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcktlen_config`] module"]
#[doc(alias = "PCKTLEN_CONFIG")]
pub type PcktlenConfig = crate::Reg<pcktlen_config::PcktlenConfigSpec>;
#[doc = "PCKTLEN_CONFIG register"]
pub mod pcktlen_config;
#[doc = "MOD0_CONFIG (rw) register accessor: MOD0_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod0_config`] module"]
#[doc(alias = "MOD0_CONFIG")]
pub type Mod0Config = crate::Reg<mod0_config::Mod0ConfigSpec>;
#[doc = "MOD0_CONFIG register"]
pub mod mod0_config;
#[doc = "MOD1_CONFIG (rw) register accessor: MOD1_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod1_config`] module"]
#[doc(alias = "MOD1_CONFIG")]
pub type Mod1Config = crate::Reg<mod1_config::Mod1ConfigSpec>;
#[doc = "MOD1_CONFIG register"]
pub mod mod1_config;
#[doc = "SNYTH_FREQ (rw) register accessor: SNYTH_FREQ register\n\nYou can [`read`](crate::Reg::read) this register and get [`snyth_freq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snyth_freq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snyth_freq`] module"]
#[doc(alias = "SNYTH_FREQ")]
pub type SnythFreq = crate::Reg<snyth_freq::SnythFreqSpec>;
#[doc = "SNYTH_FREQ register"]
pub mod snyth_freq;
#[doc = "VCO_CAL_CONFIG (rw) register accessor: VCO_CAL_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`vco_cal_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco_cal_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vco_cal_config`] module"]
#[doc(alias = "VCO_CAL_CONFIG")]
pub type VcoCalConfig = crate::Reg<vco_cal_config::VcoCalConfigSpec>;
#[doc = "VCO_CAL_CONFIG register"]
pub mod vco_cal_config;
#[doc = "RX_TIMER (rw) register accessor: RX_TIMER register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_timer`] module"]
#[doc(alias = "RX_TIMER")]
pub type RxTimer = crate::Reg<rx_timer::RxTimerSpec>;
#[doc = "RX_TIMER register"]
pub mod rx_timer;
#[doc = "DATABUFFER_THR (rw) register accessor: DATABUFFER_THR register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databuffer_thr`] module"]
#[doc(alias = "DATABUFFER_THR")]
pub type DatabufferThr = crate::Reg<databuffer_thr::DatabufferThrSpec>;
#[doc = "DATABUFFER_THR register"]
pub mod databuffer_thr;
#[doc = "RFSEQ_IRQ_ENABLE (rw) register accessor: RFSEQ_IRQ_ENABLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfseq_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfseq_irq_enable`] module"]
#[doc(alias = "RFSEQ_IRQ_ENABLE")]
pub type RfseqIrqEnable = crate::Reg<rfseq_irq_enable::RfseqIrqEnableSpec>;
#[doc = "RFSEQ_IRQ_ENABLE register"]
pub mod rfseq_irq_enable;
#[doc = "ADDITIONAL_CTRL (rw) register accessor: ADDITIONAL_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`additional_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`additional_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@additional_ctrl`] module"]
#[doc(alias = "ADDITIONAL_CTRL")]
pub type AdditionalCtrl = crate::Reg<additional_ctrl::AdditionalCtrlSpec>;
#[doc = "ADDITIONAL_CTRL register"]
pub mod additional_ctrl;
#[doc = "FAST_RX_TIMER (rw) register accessor: FAST_RX_TIMER register\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_rx_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_rx_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_rx_timer`] module"]
#[doc(alias = "FAST_RX_TIMER")]
pub type FastRxTimer = crate::Reg<fast_rx_timer::FastRxTimerSpec>;
#[doc = "FAST_RX_TIMER register"]
pub mod fast_rx_timer;
#[doc = "COMMAND (rw) register accessor: COMMAND register\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "COMMAND register"]
pub mod command;
