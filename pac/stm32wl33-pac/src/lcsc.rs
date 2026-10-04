#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcsc_cr0: LcscCr0,
    lcsc_cr1: LcscCr1,
    lcsc_cr2: LcscCr2,
    lcsc_pulse_cr: LcscPulseCr,
    lcsc_enr: LcscEnr,
    lcsc_wheel_sr: LcscWheelSr,
    lcsc_confr: LcscConfr,
    lcsc_comp_ctn: LcscCompCtn,
    lcsc_sr: LcscSr,
    lcsc_stat: LcscStat,
    lcsc_tst_cfg: LcscTstCfg,
    lcsc_anatst_cfg: LcscAnatstCfg,
    _reserved12: [u8; 0x10],
    lcsc_ver: LcscVer,
    lcsc_isr: LcscIsr,
}
impl RegisterBlock {
    #[doc = "0x00 - LCSC_CR0 register"]
    #[inline(always)]
    pub const fn lcsc_cr0(&self) -> &LcscCr0 {
        &self.lcsc_cr0
    }
    #[doc = "0x04 - LCSC_CR1 register"]
    #[inline(always)]
    pub const fn lcsc_cr1(&self) -> &LcscCr1 {
        &self.lcsc_cr1
    }
    #[doc = "0x08 - LCSC_CR2 register"]
    #[inline(always)]
    pub const fn lcsc_cr2(&self) -> &LcscCr2 {
        &self.lcsc_cr2
    }
    #[doc = "0x0c - LCSC_PULSE_CR register"]
    #[inline(always)]
    pub const fn lcsc_pulse_cr(&self) -> &LcscPulseCr {
        &self.lcsc_pulse_cr
    }
    #[doc = "0x10 - LCSC_ENR register"]
    #[inline(always)]
    pub const fn lcsc_enr(&self) -> &LcscEnr {
        &self.lcsc_enr
    }
    #[doc = "0x14 - LCSC_WHEEL_SR register"]
    #[inline(always)]
    pub const fn lcsc_wheel_sr(&self) -> &LcscWheelSr {
        &self.lcsc_wheel_sr
    }
    #[doc = "0x18 - LCSC_CONFR register"]
    #[inline(always)]
    pub const fn lcsc_confr(&self) -> &LcscConfr {
        &self.lcsc_confr
    }
    #[doc = "0x1c - LCSC_COMP_CTN register"]
    #[inline(always)]
    pub const fn lcsc_comp_ctn(&self) -> &LcscCompCtn {
        &self.lcsc_comp_ctn
    }
    #[doc = "0x20 - LCSC_SR register"]
    #[inline(always)]
    pub const fn lcsc_sr(&self) -> &LcscSr {
        &self.lcsc_sr
    }
    #[doc = "0x24 - LCSC_STAT register"]
    #[inline(always)]
    pub const fn lcsc_stat(&self) -> &LcscStat {
        &self.lcsc_stat
    }
    #[doc = "0x28 - LCSC Test Configuration Register"]
    #[inline(always)]
    pub const fn lcsc_tst_cfg(&self) -> &LcscTstCfg {
        &self.lcsc_tst_cfg
    }
    #[doc = "0x2c - LCSC ANA Test Configuration Register"]
    #[inline(always)]
    pub const fn lcsc_anatst_cfg(&self) -> &LcscAnatstCfg {
        &self.lcsc_anatst_cfg
    }
    #[doc = "0x40 - LCSC_VER register"]
    #[inline(always)]
    pub const fn lcsc_ver(&self) -> &LcscVer {
        &self.lcsc_ver
    }
    #[doc = "0x44 - LCSC_ISR register"]
    #[inline(always)]
    pub const fn lcsc_isr(&self) -> &LcscIsr {
        &self.lcsc_isr
    }
}
#[doc = "LCSC_CR0 (rw) register accessor: LCSC_CR0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_cr0`] module"]
#[doc(alias = "LCSC_CR0")]
pub type LcscCr0 = crate::Reg<lcsc_cr0::LcscCr0Spec>;
#[doc = "LCSC_CR0 register"]
pub mod lcsc_cr0;
#[doc = "LCSC_CR1 (rw) register accessor: LCSC_CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_cr1`] module"]
#[doc(alias = "LCSC_CR1")]
pub type LcscCr1 = crate::Reg<lcsc_cr1::LcscCr1Spec>;
#[doc = "LCSC_CR1 register"]
pub mod lcsc_cr1;
#[doc = "LCSC_CR2 (rw) register accessor: LCSC_CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_cr2`] module"]
#[doc(alias = "LCSC_CR2")]
pub type LcscCr2 = crate::Reg<lcsc_cr2::LcscCr2Spec>;
#[doc = "LCSC_CR2 register"]
pub mod lcsc_cr2;
#[doc = "LCSC_PULSE_CR (rw) register accessor: LCSC_PULSE_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_pulse_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_pulse_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_pulse_cr`] module"]
#[doc(alias = "LCSC_PULSE_CR")]
pub type LcscPulseCr = crate::Reg<lcsc_pulse_cr::LcscPulseCrSpec>;
#[doc = "LCSC_PULSE_CR register"]
pub mod lcsc_pulse_cr;
#[doc = "LCSC_ENR (rw) register accessor: LCSC_ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_enr`] module"]
#[doc(alias = "LCSC_ENR")]
pub type LcscEnr = crate::Reg<lcsc_enr::LcscEnrSpec>;
#[doc = "LCSC_ENR register"]
pub mod lcsc_enr;
#[doc = "LCSC_WHEEL_SR (r) register accessor: LCSC_WHEEL_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_wheel_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_wheel_sr`] module"]
#[doc(alias = "LCSC_WHEEL_SR")]
pub type LcscWheelSr = crate::Reg<lcsc_wheel_sr::LcscWheelSrSpec>;
#[doc = "LCSC_WHEEL_SR register"]
pub mod lcsc_wheel_sr;
#[doc = "LCSC_CONFR (rw) register accessor: LCSC_CONFR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_confr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_confr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_confr`] module"]
#[doc(alias = "LCSC_CONFR")]
pub type LcscConfr = crate::Reg<lcsc_confr::LcscConfrSpec>;
#[doc = "LCSC_CONFR register"]
pub mod lcsc_confr;
#[doc = "LCSC_COMP_CTN (r) register accessor: LCSC_COMP_CTN register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_comp_ctn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_comp_ctn`] module"]
#[doc(alias = "LCSC_COMP_CTN")]
pub type LcscCompCtn = crate::Reg<lcsc_comp_ctn::LcscCompCtnSpec>;
#[doc = "LCSC_COMP_CTN register"]
pub mod lcsc_comp_ctn;
#[doc = "LCSC_SR (r) register accessor: LCSC_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_sr`] module"]
#[doc(alias = "LCSC_SR")]
pub type LcscSr = crate::Reg<lcsc_sr::LcscSrSpec>;
#[doc = "LCSC_SR register"]
pub mod lcsc_sr;
#[doc = "LCSC_STAT (rw) register accessor: LCSC_STAT register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_stat`] module"]
#[doc(alias = "LCSC_STAT")]
pub type LcscStat = crate::Reg<lcsc_stat::LcscStatSpec>;
#[doc = "LCSC_STAT register"]
pub mod lcsc_stat;
#[doc = "LCSC_TST_CFG (rw) register accessor: LCSC Test Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_tst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_tst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_tst_cfg`] module"]
#[doc(alias = "LCSC_TST_CFG")]
pub type LcscTstCfg = crate::Reg<lcsc_tst_cfg::LcscTstCfgSpec>;
#[doc = "LCSC Test Configuration Register"]
pub mod lcsc_tst_cfg;
#[doc = "LCSC_ANATST_CFG (rw) register accessor: LCSC ANA Test Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_anatst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_anatst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_anatst_cfg`] module"]
#[doc(alias = "LCSC_ANATST_CFG")]
pub type LcscAnatstCfg = crate::Reg<lcsc_anatst_cfg::LcscAnatstCfgSpec>;
#[doc = "LCSC ANA Test Configuration Register"]
pub mod lcsc_anatst_cfg;
#[doc = "LCSC_VER (r) register accessor: LCSC_VER register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_ver`] module"]
#[doc(alias = "LCSC_VER")]
pub type LcscVer = crate::Reg<lcsc_ver::LcscVerSpec>;
#[doc = "LCSC_VER register"]
pub mod lcsc_ver;
#[doc = "LCSC_ISR (rw) register accessor: LCSC_ISR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsc_isr`] module"]
#[doc(alias = "LCSC_ISR")]
pub type LcscIsr = crate::Reg<lcsc_isr::LcscIsrSpec>;
#[doc = "LCSC_ISR register"]
pub mod lcsc_isr;
