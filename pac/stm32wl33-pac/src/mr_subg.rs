#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rf_fsm0_timeout: RfFsm0Timeout,
    rf_fsm1_timeout: RfFsm1Timeout,
    rf_fsm2_timeout: RfFsm2Timeout,
    rf_fsm3_timeout: RfFsm3Timeout,
    rf_fsm4_timeout: RfFsm4Timeout,
    rf_fsm5_timeout: RfFsm5Timeout,
    rf_fsm6_timeout: RfFsm6Timeout,
    rf_fsm7_timeout: RfFsm7Timeout,
    afc0_config: Afc0Config,
    afc1_config: Afc1Config,
    afc2_config: Afc2Config,
    afc3_config: Afc3Config,
    clkrec_ctrl0: ClkrecCtrl0,
    clkrec_ctrl1: ClkrecCtrl1,
    dcrem_ctrl0: DcremCtrl0,
    _reserved15: [u8; 0x04],
    iqc_ctrl0: IqcCtrl0,
    iqc_ctrl1: IqcCtrl1,
    iqc_ctrl2: IqcCtrl2,
    iqc_ctrl3: IqcCtrl3,
    agc_ana_eng: AgcAnaEng,
    agc0_ctrl: Agc0Ctrl,
    agc1_ctrl: Agc1Ctrl,
    agc2_ctrl: Agc2Ctrl,
    agc3_ctrl: Agc3Ctrl,
    agc4_ctrl: Agc4Ctrl,
    agc_atten0: AgcAtten0,
    agc_atten1: AgcAtten1,
    agc_atten2: AgcAtten2,
    agc_atten3: AgcAtten3,
    agc_atten4: AgcAtten4,
    agc_atten5: AgcAtten5,
    agc_atten6: AgcAtten6,
    agc_atten7: AgcAtten7,
    agc_atten8: AgcAtten8,
    agc_atten9: AgcAtten9,
    _reserved35: [u8; 0x10],
    agc_pga_hwtrim_out: AgcPgaHwtrimOut,
    _reserved36: [u8; 0x04],
    pa_reg: PaReg,
    pa_hwtrim_out: PaHwtrimOut,
    _reserved38: [u8; 0x0c],
    rssi_flt: RssiFlt,
    _reserved39: [u8; 0x08],
    synth2_ana_eng: Synth2AnaEng,
    _reserved40: [u8; 0x1c],
    rxadc_hwdelaytrim_out: RxadcHwdelaytrimOut,
    _reserved41: [u8; 0x08],
    rx_aaf_hwtrim_out: RxAafHwtrimOut,
    _reserved42: [u8; 0x08],
    singen_ana_eng: SingenAnaEng,
    _reserved43: [u8; 0x04],
    rf_info_out: RfInfoOut,
    _reserved44: [u8; 0x18],
    rf_fsm8_timeout: RfFsm8Timeout,
    rf_fsm9_timeout: RfFsm9Timeout,
    rf_fsm10_timeout: RfFsm10Timeout,
    _reserved47: [u8; 0x14],
    subg_dig_ctrl0: SubgDigCtrl0,
    rx_chain_eng: RxChainEng,
    demod_dig_eng: DemodDigEng,
}
impl RegisterBlock {
    #[doc = "0x00 - RF_FSM0_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm0_timeout(&self) -> &RfFsm0Timeout {
        &self.rf_fsm0_timeout
    }
    #[doc = "0x04 - RF_FSM1_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm1_timeout(&self) -> &RfFsm1Timeout {
        &self.rf_fsm1_timeout
    }
    #[doc = "0x08 - RF_FSM2_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm2_timeout(&self) -> &RfFsm2Timeout {
        &self.rf_fsm2_timeout
    }
    #[doc = "0x0c - RF_FSM3_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm3_timeout(&self) -> &RfFsm3Timeout {
        &self.rf_fsm3_timeout
    }
    #[doc = "0x10 - RF_FSM4_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm4_timeout(&self) -> &RfFsm4Timeout {
        &self.rf_fsm4_timeout
    }
    #[doc = "0x14 - RF_FSM5_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm5_timeout(&self) -> &RfFsm5Timeout {
        &self.rf_fsm5_timeout
    }
    #[doc = "0x18 - RF_FSM6_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm6_timeout(&self) -> &RfFsm6Timeout {
        &self.rf_fsm6_timeout
    }
    #[doc = "0x1c - RF_FSM7_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm7_timeout(&self) -> &RfFsm7Timeout {
        &self.rf_fsm7_timeout
    }
    #[doc = "0x20 - AFC0_CONFIG register"]
    #[inline(always)]
    pub const fn afc0_config(&self) -> &Afc0Config {
        &self.afc0_config
    }
    #[doc = "0x24 - AFC1_CONFIG register"]
    #[inline(always)]
    pub const fn afc1_config(&self) -> &Afc1Config {
        &self.afc1_config
    }
    #[doc = "0x28 - AFC2_CONFIG register"]
    #[inline(always)]
    pub const fn afc2_config(&self) -> &Afc2Config {
        &self.afc2_config
    }
    #[doc = "0x2c - AFC3_CONFIG register"]
    #[inline(always)]
    pub const fn afc3_config(&self) -> &Afc3Config {
        &self.afc3_config
    }
    #[doc = "0x30 - CLKREC_CTRL0 register"]
    #[inline(always)]
    pub const fn clkrec_ctrl0(&self) -> &ClkrecCtrl0 {
        &self.clkrec_ctrl0
    }
    #[doc = "0x34 - CLKREC_CTRL1 register"]
    #[inline(always)]
    pub const fn clkrec_ctrl1(&self) -> &ClkrecCtrl1 {
        &self.clkrec_ctrl1
    }
    #[doc = "0x38 - DCREM_CTRL0 register"]
    #[inline(always)]
    pub const fn dcrem_ctrl0(&self) -> &DcremCtrl0 {
        &self.dcrem_ctrl0
    }
    #[doc = "0x40 - IQC_CTRL0 register"]
    #[inline(always)]
    pub const fn iqc_ctrl0(&self) -> &IqcCtrl0 {
        &self.iqc_ctrl0
    }
    #[doc = "0x44 - IQC_CTRL1 register"]
    #[inline(always)]
    pub const fn iqc_ctrl1(&self) -> &IqcCtrl1 {
        &self.iqc_ctrl1
    }
    #[doc = "0x48 - IQC_CTRL2 register"]
    #[inline(always)]
    pub const fn iqc_ctrl2(&self) -> &IqcCtrl2 {
        &self.iqc_ctrl2
    }
    #[doc = "0x4c - IQC_CTRL3 register"]
    #[inline(always)]
    pub const fn iqc_ctrl3(&self) -> &IqcCtrl3 {
        &self.iqc_ctrl3
    }
    #[doc = "0x50 - AGC_ANA_ENG register"]
    #[inline(always)]
    pub const fn agc_ana_eng(&self) -> &AgcAnaEng {
        &self.agc_ana_eng
    }
    #[doc = "0x54 - AGC0_CTRL register"]
    #[inline(always)]
    pub const fn agc0_ctrl(&self) -> &Agc0Ctrl {
        &self.agc0_ctrl
    }
    #[doc = "0x58 - AGC1_CTRL register"]
    #[inline(always)]
    pub const fn agc1_ctrl(&self) -> &Agc1Ctrl {
        &self.agc1_ctrl
    }
    #[doc = "0x5c - AGC2_CTRL register"]
    #[inline(always)]
    pub const fn agc2_ctrl(&self) -> &Agc2Ctrl {
        &self.agc2_ctrl
    }
    #[doc = "0x60 - AGC3_CTRL register"]
    #[inline(always)]
    pub const fn agc3_ctrl(&self) -> &Agc3Ctrl {
        &self.agc3_ctrl
    }
    #[doc = "0x64 - AGC4_CTRL register"]
    #[inline(always)]
    pub const fn agc4_ctrl(&self) -> &Agc4Ctrl {
        &self.agc4_ctrl
    }
    #[doc = "0x68 - AGC_ATTEN0 register"]
    #[inline(always)]
    pub const fn agc_atten0(&self) -> &AgcAtten0 {
        &self.agc_atten0
    }
    #[doc = "0x6c - AGC_ATTEN1 register"]
    #[inline(always)]
    pub const fn agc_atten1(&self) -> &AgcAtten1 {
        &self.agc_atten1
    }
    #[doc = "0x70 - AGC_ATTEN2 register"]
    #[inline(always)]
    pub const fn agc_atten2(&self) -> &AgcAtten2 {
        &self.agc_atten2
    }
    #[doc = "0x74 - AGC_ATTEN3 register"]
    #[inline(always)]
    pub const fn agc_atten3(&self) -> &AgcAtten3 {
        &self.agc_atten3
    }
    #[doc = "0x78 - AGC_ATTEN4 register"]
    #[inline(always)]
    pub const fn agc_atten4(&self) -> &AgcAtten4 {
        &self.agc_atten4
    }
    #[doc = "0x7c - AGC_ATTEN5 register"]
    #[inline(always)]
    pub const fn agc_atten5(&self) -> &AgcAtten5 {
        &self.agc_atten5
    }
    #[doc = "0x80 - AGC_ATTEN6 register"]
    #[inline(always)]
    pub const fn agc_atten6(&self) -> &AgcAtten6 {
        &self.agc_atten6
    }
    #[doc = "0x84 - AGC_ATTEN7 register"]
    #[inline(always)]
    pub const fn agc_atten7(&self) -> &AgcAtten7 {
        &self.agc_atten7
    }
    #[doc = "0x88 - AGC_ATTEN8 register"]
    #[inline(always)]
    pub const fn agc_atten8(&self) -> &AgcAtten8 {
        &self.agc_atten8
    }
    #[doc = "0x8c - AGC_ATTEN9 register"]
    #[inline(always)]
    pub const fn agc_atten9(&self) -> &AgcAtten9 {
        &self.agc_atten9
    }
    #[doc = "0xa0 - AGC_PGA_HWTRIM_OUT register"]
    #[inline(always)]
    pub const fn agc_pga_hwtrim_out(&self) -> &AgcPgaHwtrimOut {
        &self.agc_pga_hwtrim_out
    }
    #[doc = "0xa8 - PA_REG register"]
    #[inline(always)]
    pub const fn pa_reg(&self) -> &PaReg {
        &self.pa_reg
    }
    #[doc = "0xac - PA_HWTRIM_OUT register"]
    #[inline(always)]
    pub const fn pa_hwtrim_out(&self) -> &PaHwtrimOut {
        &self.pa_hwtrim_out
    }
    #[doc = "0xbc - RSSI_FLT register"]
    #[inline(always)]
    pub const fn rssi_flt(&self) -> &RssiFlt {
        &self.rssi_flt
    }
    #[doc = "0xc8 - SYNTH2_ANA_ENG register"]
    #[inline(always)]
    pub const fn synth2_ana_eng(&self) -> &Synth2AnaEng {
        &self.synth2_ana_eng
    }
    #[doc = "0xe8 - RXADC_HWDELAYTRIM_OUT register"]
    #[inline(always)]
    pub const fn rxadc_hwdelaytrim_out(&self) -> &RxadcHwdelaytrimOut {
        &self.rxadc_hwdelaytrim_out
    }
    #[doc = "0xf4 - RX_AAF_HWTRIM_OUT register"]
    #[inline(always)]
    pub const fn rx_aaf_hwtrim_out(&self) -> &RxAafHwtrimOut {
        &self.rx_aaf_hwtrim_out
    }
    #[doc = "0x100 - SINGEN_ANA_ENG register"]
    #[inline(always)]
    pub const fn singen_ana_eng(&self) -> &SingenAnaEng {
        &self.singen_ana_eng
    }
    #[doc = "0x108 - RF_INFO_OUT register"]
    #[inline(always)]
    pub const fn rf_info_out(&self) -> &RfInfoOut {
        &self.rf_info_out
    }
    #[doc = "0x124 - RF_FSM8_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm8_timeout(&self) -> &RfFsm8Timeout {
        &self.rf_fsm8_timeout
    }
    #[doc = "0x128 - RF_FSM9_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm9_timeout(&self) -> &RfFsm9Timeout {
        &self.rf_fsm9_timeout
    }
    #[doc = "0x12c - RF_FSM10_TIMEOUT register"]
    #[inline(always)]
    pub const fn rf_fsm10_timeout(&self) -> &RfFsm10Timeout {
        &self.rf_fsm10_timeout
    }
    #[doc = "0x144 - SUBG_DIG_CTRL0 register"]
    #[inline(always)]
    pub const fn subg_dig_ctrl0(&self) -> &SubgDigCtrl0 {
        &self.subg_dig_ctrl0
    }
    #[doc = "0x148 - RX_CHAIN_ENG register"]
    #[inline(always)]
    pub const fn rx_chain_eng(&self) -> &RxChainEng {
        &self.rx_chain_eng
    }
    #[doc = "0x14c - DEMOD_DIG_ENG register"]
    #[inline(always)]
    pub const fn demod_dig_eng(&self) -> &DemodDigEng {
        &self.demod_dig_eng
    }
}
#[doc = "RF_FSM0_TIMEOUT (rw) register accessor: RF_FSM0_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm0_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm0_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm0_timeout`] module"]
#[doc(alias = "RF_FSM0_TIMEOUT")]
pub type RfFsm0Timeout = crate::Reg<rf_fsm0_timeout::RfFsm0TimeoutSpec>;
#[doc = "RF_FSM0_TIMEOUT register"]
pub mod rf_fsm0_timeout;
#[doc = "RF_FSM1_TIMEOUT (rw) register accessor: RF_FSM1_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm1_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm1_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm1_timeout`] module"]
#[doc(alias = "RF_FSM1_TIMEOUT")]
pub type RfFsm1Timeout = crate::Reg<rf_fsm1_timeout::RfFsm1TimeoutSpec>;
#[doc = "RF_FSM1_TIMEOUT register"]
pub mod rf_fsm1_timeout;
#[doc = "RF_FSM2_TIMEOUT (rw) register accessor: RF_FSM2_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm2_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm2_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm2_timeout`] module"]
#[doc(alias = "RF_FSM2_TIMEOUT")]
pub type RfFsm2Timeout = crate::Reg<rf_fsm2_timeout::RfFsm2TimeoutSpec>;
#[doc = "RF_FSM2_TIMEOUT register"]
pub mod rf_fsm2_timeout;
#[doc = "RF_FSM3_TIMEOUT (rw) register accessor: RF_FSM3_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm3_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm3_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm3_timeout`] module"]
#[doc(alias = "RF_FSM3_TIMEOUT")]
pub type RfFsm3Timeout = crate::Reg<rf_fsm3_timeout::RfFsm3TimeoutSpec>;
#[doc = "RF_FSM3_TIMEOUT register"]
pub mod rf_fsm3_timeout;
#[doc = "RF_FSM4_TIMEOUT (rw) register accessor: RF_FSM4_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm4_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm4_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm4_timeout`] module"]
#[doc(alias = "RF_FSM4_TIMEOUT")]
pub type RfFsm4Timeout = crate::Reg<rf_fsm4_timeout::RfFsm4TimeoutSpec>;
#[doc = "RF_FSM4_TIMEOUT register"]
pub mod rf_fsm4_timeout;
#[doc = "RF_FSM5_TIMEOUT (rw) register accessor: RF_FSM5_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm5_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm5_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm5_timeout`] module"]
#[doc(alias = "RF_FSM5_TIMEOUT")]
pub type RfFsm5Timeout = crate::Reg<rf_fsm5_timeout::RfFsm5TimeoutSpec>;
#[doc = "RF_FSM5_TIMEOUT register"]
pub mod rf_fsm5_timeout;
#[doc = "RF_FSM6_TIMEOUT (rw) register accessor: RF_FSM6_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm6_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm6_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm6_timeout`] module"]
#[doc(alias = "RF_FSM6_TIMEOUT")]
pub type RfFsm6Timeout = crate::Reg<rf_fsm6_timeout::RfFsm6TimeoutSpec>;
#[doc = "RF_FSM6_TIMEOUT register"]
pub mod rf_fsm6_timeout;
#[doc = "RF_FSM7_TIMEOUT (rw) register accessor: RF_FSM7_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm7_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm7_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm7_timeout`] module"]
#[doc(alias = "RF_FSM7_TIMEOUT")]
pub type RfFsm7Timeout = crate::Reg<rf_fsm7_timeout::RfFsm7TimeoutSpec>;
#[doc = "RF_FSM7_TIMEOUT register"]
pub mod rf_fsm7_timeout;
#[doc = "AFC0_CONFIG (rw) register accessor: AFC0_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afc0_config`] module"]
#[doc(alias = "AFC0_CONFIG")]
pub type Afc0Config = crate::Reg<afc0_config::Afc0ConfigSpec>;
#[doc = "AFC0_CONFIG register"]
pub mod afc0_config;
#[doc = "AFC1_CONFIG (rw) register accessor: AFC1_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afc1_config`] module"]
#[doc(alias = "AFC1_CONFIG")]
pub type Afc1Config = crate::Reg<afc1_config::Afc1ConfigSpec>;
#[doc = "AFC1_CONFIG register"]
pub mod afc1_config;
#[doc = "AFC2_CONFIG (rw) register accessor: AFC2_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc2_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc2_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afc2_config`] module"]
#[doc(alias = "AFC2_CONFIG")]
pub type Afc2Config = crate::Reg<afc2_config::Afc2ConfigSpec>;
#[doc = "AFC2_CONFIG register"]
pub mod afc2_config;
#[doc = "AFC3_CONFIG (rw) register accessor: AFC3_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc3_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc3_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afc3_config`] module"]
#[doc(alias = "AFC3_CONFIG")]
pub type Afc3Config = crate::Reg<afc3_config::Afc3ConfigSpec>;
#[doc = "AFC3_CONFIG register"]
pub mod afc3_config;
#[doc = "CLKREC_CTRL0 (rw) register accessor: CLKREC_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkrec_ctrl0`] module"]
#[doc(alias = "CLKREC_CTRL0")]
pub type ClkrecCtrl0 = crate::Reg<clkrec_ctrl0::ClkrecCtrl0Spec>;
#[doc = "CLKREC_CTRL0 register"]
pub mod clkrec_ctrl0;
#[doc = "CLKREC_CTRL1 (rw) register accessor: CLKREC_CTRL1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkrec_ctrl1`] module"]
#[doc(alias = "CLKREC_CTRL1")]
pub type ClkrecCtrl1 = crate::Reg<clkrec_ctrl1::ClkrecCtrl1Spec>;
#[doc = "CLKREC_CTRL1 register"]
pub mod clkrec_ctrl1;
#[doc = "DCREM_CTRL0 (rw) register accessor: DCREM_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrem_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrem_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrem_ctrl0`] module"]
#[doc(alias = "DCREM_CTRL0")]
pub type DcremCtrl0 = crate::Reg<dcrem_ctrl0::DcremCtrl0Spec>;
#[doc = "DCREM_CTRL0 register"]
pub mod dcrem_ctrl0;
#[doc = "IQC_CTRL0 (rw) register accessor: IQC_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqc_ctrl0`] module"]
#[doc(alias = "IQC_CTRL0")]
pub type IqcCtrl0 = crate::Reg<iqc_ctrl0::IqcCtrl0Spec>;
#[doc = "IQC_CTRL0 register"]
pub mod iqc_ctrl0;
#[doc = "IQC_CTRL1 (rw) register accessor: IQC_CTRL1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqc_ctrl1`] module"]
#[doc(alias = "IQC_CTRL1")]
pub type IqcCtrl1 = crate::Reg<iqc_ctrl1::IqcCtrl1Spec>;
#[doc = "IQC_CTRL1 register"]
pub mod iqc_ctrl1;
#[doc = "IQC_CTRL2 (rw) register accessor: IQC_CTRL2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqc_ctrl2`] module"]
#[doc(alias = "IQC_CTRL2")]
pub type IqcCtrl2 = crate::Reg<iqc_ctrl2::IqcCtrl2Spec>;
#[doc = "IQC_CTRL2 register"]
pub mod iqc_ctrl2;
#[doc = "IQC_CTRL3 (rw) register accessor: IQC_CTRL3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqc_ctrl3`] module"]
#[doc(alias = "IQC_CTRL3")]
pub type IqcCtrl3 = crate::Reg<iqc_ctrl3::IqcCtrl3Spec>;
#[doc = "IQC_CTRL3 register"]
pub mod iqc_ctrl3;
#[doc = "AGC_ANA_ENG (rw) register accessor: AGC_ANA_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_ana_eng`] module"]
#[doc(alias = "AGC_ANA_ENG")]
pub type AgcAnaEng = crate::Reg<agc_ana_eng::AgcAnaEngSpec>;
#[doc = "AGC_ANA_ENG register"]
pub mod agc_ana_eng;
#[doc = "AGC0_CTRL (rw) register accessor: AGC0_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc0_ctrl`] module"]
#[doc(alias = "AGC0_CTRL")]
pub type Agc0Ctrl = crate::Reg<agc0_ctrl::Agc0CtrlSpec>;
#[doc = "AGC0_CTRL register"]
pub mod agc0_ctrl;
#[doc = "AGC1_CTRL (rw) register accessor: AGC1_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc1_ctrl`] module"]
#[doc(alias = "AGC1_CTRL")]
pub type Agc1Ctrl = crate::Reg<agc1_ctrl::Agc1CtrlSpec>;
#[doc = "AGC1_CTRL register"]
pub mod agc1_ctrl;
#[doc = "AGC2_CTRL (rw) register accessor: AGC2_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc2_ctrl`] module"]
#[doc(alias = "AGC2_CTRL")]
pub type Agc2Ctrl = crate::Reg<agc2_ctrl::Agc2CtrlSpec>;
#[doc = "AGC2_CTRL register"]
pub mod agc2_ctrl;
#[doc = "AGC3_CTRL (rw) register accessor: AGC3_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc3_ctrl`] module"]
#[doc(alias = "AGC3_CTRL")]
pub type Agc3Ctrl = crate::Reg<agc3_ctrl::Agc3CtrlSpec>;
#[doc = "AGC3_CTRL register"]
pub mod agc3_ctrl;
#[doc = "AGC4_CTRL (rw) register accessor: AGC4_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc4_ctrl`] module"]
#[doc(alias = "AGC4_CTRL")]
pub type Agc4Ctrl = crate::Reg<agc4_ctrl::Agc4CtrlSpec>;
#[doc = "AGC4_CTRL register"]
pub mod agc4_ctrl;
#[doc = "AGC_ATTEN0 (rw) register accessor: AGC_ATTEN0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten0`] module"]
#[doc(alias = "AGC_ATTEN0")]
pub type AgcAtten0 = crate::Reg<agc_atten0::AgcAtten0Spec>;
#[doc = "AGC_ATTEN0 register"]
pub mod agc_atten0;
#[doc = "AGC_ATTEN1 (rw) register accessor: AGC_ATTEN1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten1`] module"]
#[doc(alias = "AGC_ATTEN1")]
pub type AgcAtten1 = crate::Reg<agc_atten1::AgcAtten1Spec>;
#[doc = "AGC_ATTEN1 register"]
pub mod agc_atten1;
#[doc = "AGC_ATTEN2 (rw) register accessor: AGC_ATTEN2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten2`] module"]
#[doc(alias = "AGC_ATTEN2")]
pub type AgcAtten2 = crate::Reg<agc_atten2::AgcAtten2Spec>;
#[doc = "AGC_ATTEN2 register"]
pub mod agc_atten2;
#[doc = "AGC_ATTEN3 (rw) register accessor: AGC_ATTEN3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten3`] module"]
#[doc(alias = "AGC_ATTEN3")]
pub type AgcAtten3 = crate::Reg<agc_atten3::AgcAtten3Spec>;
#[doc = "AGC_ATTEN3 register"]
pub mod agc_atten3;
#[doc = "AGC_ATTEN4 (rw) register accessor: AGC_ATTEN4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten4`] module"]
#[doc(alias = "AGC_ATTEN4")]
pub type AgcAtten4 = crate::Reg<agc_atten4::AgcAtten4Spec>;
#[doc = "AGC_ATTEN4 register"]
pub mod agc_atten4;
#[doc = "AGC_ATTEN5 (rw) register accessor: AGC_ATTEN5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten5`] module"]
#[doc(alias = "AGC_ATTEN5")]
pub type AgcAtten5 = crate::Reg<agc_atten5::AgcAtten5Spec>;
#[doc = "AGC_ATTEN5 register"]
pub mod agc_atten5;
#[doc = "AGC_ATTEN6 (rw) register accessor: AGC_ATTEN6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten6`] module"]
#[doc(alias = "AGC_ATTEN6")]
pub type AgcAtten6 = crate::Reg<agc_atten6::AgcAtten6Spec>;
#[doc = "AGC_ATTEN6 register"]
pub mod agc_atten6;
#[doc = "AGC_ATTEN7 (rw) register accessor: AGC_ATTEN7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten7`] module"]
#[doc(alias = "AGC_ATTEN7")]
pub type AgcAtten7 = crate::Reg<agc_atten7::AgcAtten7Spec>;
#[doc = "AGC_ATTEN7 register"]
pub mod agc_atten7;
#[doc = "AGC_ATTEN8 (rw) register accessor: AGC_ATTEN8 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten8`] module"]
#[doc(alias = "AGC_ATTEN8")]
pub type AgcAtten8 = crate::Reg<agc_atten8::AgcAtten8Spec>;
#[doc = "AGC_ATTEN8 register"]
pub mod agc_atten8;
#[doc = "AGC_ATTEN9 (rw) register accessor: AGC_ATTEN9 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_atten9`] module"]
#[doc(alias = "AGC_ATTEN9")]
pub type AgcAtten9 = crate::Reg<agc_atten9::AgcAtten9Spec>;
#[doc = "AGC_ATTEN9 register"]
pub mod agc_atten9;
#[doc = "AGC_PGA_HWTRIM_OUT (r) register accessor: AGC_PGA_HWTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_pga_hwtrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_pga_hwtrim_out`] module"]
#[doc(alias = "AGC_PGA_HWTRIM_OUT")]
pub type AgcPgaHwtrimOut = crate::Reg<agc_pga_hwtrim_out::AgcPgaHwtrimOutSpec>;
#[doc = "AGC_PGA_HWTRIM_OUT register"]
pub mod agc_pga_hwtrim_out;
#[doc = "PA_REG (rw) register accessor: PA_REG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_reg`] module"]
#[doc(alias = "PA_REG")]
pub type PaReg = crate::Reg<pa_reg::PaRegSpec>;
#[doc = "PA_REG register"]
pub mod pa_reg;
#[doc = "PA_HWTRIM_OUT (r) register accessor: PA_HWTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_hwtrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_hwtrim_out`] module"]
#[doc(alias = "PA_HWTRIM_OUT")]
pub type PaHwtrimOut = crate::Reg<pa_hwtrim_out::PaHwtrimOutSpec>;
#[doc = "PA_HWTRIM_OUT register"]
pub mod pa_hwtrim_out;
#[doc = "RSSI_FLT (rw) register accessor: RSSI_FLT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rssi_flt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rssi_flt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssi_flt`] module"]
#[doc(alias = "RSSI_FLT")]
pub type RssiFlt = crate::Reg<rssi_flt::RssiFltSpec>;
#[doc = "RSSI_FLT register"]
pub mod rssi_flt;
#[doc = "SYNTH2_ANA_ENG (rw) register accessor: SYNTH2_ANA_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`synth2_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synth2_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synth2_ana_eng`] module"]
#[doc(alias = "SYNTH2_ANA_ENG")]
pub type Synth2AnaEng = crate::Reg<synth2_ana_eng::Synth2AnaEngSpec>;
#[doc = "SYNTH2_ANA_ENG register"]
pub mod synth2_ana_eng;
#[doc = "RXADC_HWDELAYTRIM_OUT (r) register accessor: RXADC_HWDELAYTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxadc_hwdelaytrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxadc_hwdelaytrim_out`] module"]
#[doc(alias = "RXADC_HWDELAYTRIM_OUT")]
pub type RxadcHwdelaytrimOut = crate::Reg<rxadc_hwdelaytrim_out::RxadcHwdelaytrimOutSpec>;
#[doc = "RXADC_HWDELAYTRIM_OUT register"]
pub mod rxadc_hwdelaytrim_out;
#[doc = "RX_AAF_HWTRIM_OUT (r) register accessor: RX_AAF_HWTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_aaf_hwtrim_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_aaf_hwtrim_out`] module"]
#[doc(alias = "RX_AAF_HWTRIM_OUT")]
pub type RxAafHwtrimOut = crate::Reg<rx_aaf_hwtrim_out::RxAafHwtrimOutSpec>;
#[doc = "RX_AAF_HWTRIM_OUT register"]
pub mod rx_aaf_hwtrim_out;
#[doc = "SINGEN_ANA_ENG (rw) register accessor: SINGEN_ANA_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ana_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ana_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singen_ana_eng`] module"]
#[doc(alias = "SINGEN_ANA_ENG")]
pub type SingenAnaEng = crate::Reg<singen_ana_eng::SingenAnaEngSpec>;
#[doc = "SINGEN_ANA_ENG register"]
pub mod singen_ana_eng;
#[doc = "RF_INFO_OUT (r) register accessor: RF_INFO_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_info_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_info_out`] module"]
#[doc(alias = "RF_INFO_OUT")]
pub type RfInfoOut = crate::Reg<rf_info_out::RfInfoOutSpec>;
#[doc = "RF_INFO_OUT register"]
pub mod rf_info_out;
#[doc = "RF_FSM8_TIMEOUT (rw) register accessor: RF_FSM8_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm8_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm8_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm8_timeout`] module"]
#[doc(alias = "RF_FSM8_TIMEOUT")]
pub type RfFsm8Timeout = crate::Reg<rf_fsm8_timeout::RfFsm8TimeoutSpec>;
#[doc = "RF_FSM8_TIMEOUT register"]
pub mod rf_fsm8_timeout;
#[doc = "RF_FSM9_TIMEOUT (rw) register accessor: RF_FSM9_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm9_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm9_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm9_timeout`] module"]
#[doc(alias = "RF_FSM9_TIMEOUT")]
pub type RfFsm9Timeout = crate::Reg<rf_fsm9_timeout::RfFsm9TimeoutSpec>;
#[doc = "RF_FSM9_TIMEOUT register"]
pub mod rf_fsm9_timeout;
#[doc = "RF_FSM10_TIMEOUT (rw) register accessor: RF_FSM10_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm10_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm10_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm10_timeout`] module"]
#[doc(alias = "RF_FSM10_TIMEOUT")]
pub type RfFsm10Timeout = crate::Reg<rf_fsm10_timeout::RfFsm10TimeoutSpec>;
#[doc = "RF_FSM10_TIMEOUT register"]
pub mod rf_fsm10_timeout;
#[doc = "SUBG_DIG_CTRL0 (rw) register accessor: SUBG_DIG_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`subg_dig_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subg_dig_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subg_dig_ctrl0`] module"]
#[doc(alias = "SUBG_DIG_CTRL0")]
pub type SubgDigCtrl0 = crate::Reg<subg_dig_ctrl0::SubgDigCtrl0Spec>;
#[doc = "SUBG_DIG_CTRL0 register"]
pub mod subg_dig_ctrl0;
#[doc = "RX_CHAIN_ENG (rw) register accessor: RX_CHAIN_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_chain_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_chain_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_chain_eng`] module"]
#[doc(alias = "RX_CHAIN_ENG")]
pub type RxChainEng = crate::Reg<rx_chain_eng::RxChainEngSpec>;
#[doc = "RX_CHAIN_ENG register"]
pub mod rx_chain_eng;
#[doc = "DEMOD_DIG_ENG (rw) register accessor: DEMOD_DIG_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`demod_dig_eng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demod_dig_eng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demod_dig_eng`] module"]
#[doc(alias = "DEMOD_DIG_ENG")]
pub type DemodDigEng = crate::Reg<demod_dig_eng::DemodDigEngSpec>;
#[doc = "DEMOD_DIG_ENG register"]
pub mod demod_dig_eng;
