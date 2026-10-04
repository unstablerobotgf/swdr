#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    die_id: DieId,
    jtag_id: JtagId,
    i2c_fmp_ctrl: I2cFmpCtrl,
    io_dtr: IoDtr,
    io_iber: IoIber,
    io_ievr: IoIevr,
    io_ier: IoIer,
    io_iscr: IoIscr,
    pwrc_ier: PwrcIer,
    pwrc_iscr: PwrcIscr,
    gpio_swa_ctrl: GpioSwaCtrl,
    intai_dtr: IntaiDtr,
    intai_iber: IntaiIber,
    intai_ievr: IntaiIevr,
    intai_ier: IntaiIer,
    intai_iscr: IntaiIscr,
    syscfg_sr1: SyscfgSr1,
    rf_dtb_config: RfDtbConfig,
}
impl RegisterBlock {
    #[doc = "0x00 - DIE_ID register"]
    #[inline(always)]
    pub const fn die_id(&self) -> &DieId {
        &self.die_id
    }
    #[doc = "0x04 - JTAG_ID register"]
    #[inline(always)]
    pub const fn jtag_id(&self) -> &JtagId {
        &self.jtag_id
    }
    #[doc = "0x08 - I2C_FMP_CTRL register"]
    #[inline(always)]
    pub const fn i2c_fmp_ctrl(&self) -> &I2cFmpCtrl {
        &self.i2c_fmp_ctrl
    }
    #[doc = "0x0c - IO_DTR register"]
    #[inline(always)]
    pub const fn io_dtr(&self) -> &IoDtr {
        &self.io_dtr
    }
    #[doc = "0x10 - IO_IBER register"]
    #[inline(always)]
    pub const fn io_iber(&self) -> &IoIber {
        &self.io_iber
    }
    #[doc = "0x14 - IO_IEVR register"]
    #[inline(always)]
    pub const fn io_ievr(&self) -> &IoIevr {
        &self.io_ievr
    }
    #[doc = "0x18 - IO_IER register"]
    #[inline(always)]
    pub const fn io_ier(&self) -> &IoIer {
        &self.io_ier
    }
    #[doc = "0x1c - IO_ISCR register"]
    #[inline(always)]
    pub const fn io_iscr(&self) -> &IoIscr {
        &self.io_iscr
    }
    #[doc = "0x20 - PWRC_IER register"]
    #[inline(always)]
    pub const fn pwrc_ier(&self) -> &PwrcIer {
        &self.pwrc_ier
    }
    #[doc = "0x24 - PWRC_ISCR register"]
    #[inline(always)]
    pub const fn pwrc_iscr(&self) -> &PwrcIscr {
        &self.pwrc_iscr
    }
    #[doc = "0x28 - GPIO_SWA_CTRL register"]
    #[inline(always)]
    pub const fn gpio_swa_ctrl(&self) -> &GpioSwaCtrl {
        &self.gpio_swa_ctrl
    }
    #[doc = "0x2c - INTAI_DTR register"]
    #[inline(always)]
    pub const fn intai_dtr(&self) -> &IntaiDtr {
        &self.intai_dtr
    }
    #[doc = "0x30 - INTAI_IBER register"]
    #[inline(always)]
    pub const fn intai_iber(&self) -> &IntaiIber {
        &self.intai_iber
    }
    #[doc = "0x34 - INTAI_IEVR register"]
    #[inline(always)]
    pub const fn intai_ievr(&self) -> &IntaiIevr {
        &self.intai_ievr
    }
    #[doc = "0x38 - INTAI_IER register"]
    #[inline(always)]
    pub const fn intai_ier(&self) -> &IntaiIer {
        &self.intai_ier
    }
    #[doc = "0x3c - INTAI_ISCR register"]
    #[inline(always)]
    pub const fn intai_iscr(&self) -> &IntaiIscr {
        &self.intai_iscr
    }
    #[doc = "0x40 - SYSCFG_SR1 register"]
    #[inline(always)]
    pub const fn syscfg_sr1(&self) -> &SyscfgSr1 {
        &self.syscfg_sr1
    }
    #[doc = "0x44 - RF_DTB_CONFIG register"]
    #[inline(always)]
    pub const fn rf_dtb_config(&self) -> &RfDtbConfig {
        &self.rf_dtb_config
    }
}
#[doc = "DIE_ID (r) register accessor: DIE_ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`die_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_id`] module"]
#[doc(alias = "DIE_ID")]
pub type DieId = crate::Reg<die_id::DieIdSpec>;
#[doc = "DIE_ID register"]
pub mod die_id;
#[doc = "JTAG_ID (r) register accessor: JTAG_ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_id`] module"]
#[doc(alias = "JTAG_ID")]
pub type JtagId = crate::Reg<jtag_id::JtagIdSpec>;
#[doc = "JTAG_ID register"]
pub mod jtag_id;
#[doc = "I2C_FMP_CTRL (rw) register accessor: I2C_FMP_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fmp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fmp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fmp_ctrl`] module"]
#[doc(alias = "I2C_FMP_CTRL")]
pub type I2cFmpCtrl = crate::Reg<i2c_fmp_ctrl::I2cFmpCtrlSpec>;
#[doc = "I2C_FMP_CTRL register"]
pub mod i2c_fmp_ctrl;
#[doc = "IO_DTR (rw) register accessor: IO_DTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_dtr`] module"]
#[doc(alias = "IO_DTR")]
pub type IoDtr = crate::Reg<io_dtr::IoDtrSpec>;
#[doc = "IO_DTR register"]
pub mod io_dtr;
#[doc = "IO_IBER (rw) register accessor: IO_IBER register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_iber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_iber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_iber`] module"]
#[doc(alias = "IO_IBER")]
pub type IoIber = crate::Reg<io_iber::IoIberSpec>;
#[doc = "IO_IBER register"]
pub mod io_iber;
#[doc = "IO_IEVR (rw) register accessor: IO_IEVR register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ievr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ievr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ievr`] module"]
#[doc(alias = "IO_IEVR")]
pub type IoIevr = crate::Reg<io_ievr::IoIevrSpec>;
#[doc = "IO_IEVR register"]
pub mod io_ievr;
#[doc = "IO_IER (rw) register accessor: IO_IER register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ier`] module"]
#[doc(alias = "IO_IER")]
pub type IoIer = crate::Reg<io_ier::IoIerSpec>;
#[doc = "IO_IER register"]
pub mod io_ier;
#[doc = "IO_ISCR (rw) register accessor: IO_ISCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_iscr`] module"]
#[doc(alias = "IO_ISCR")]
pub type IoIscr = crate::Reg<io_iscr::IoIscrSpec>;
#[doc = "IO_ISCR register"]
pub mod io_iscr;
#[doc = "PWRC_IER (rw) register accessor: PWRC_IER register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrc_ier`] module"]
#[doc(alias = "PWRC_IER")]
pub type PwrcIer = crate::Reg<pwrc_ier::PwrcIerSpec>;
#[doc = "PWRC_IER register"]
pub mod pwrc_ier;
#[doc = "PWRC_ISCR (rw) register accessor: PWRC_ISCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrc_iscr`] module"]
#[doc(alias = "PWRC_ISCR")]
pub type PwrcIscr = crate::Reg<pwrc_iscr::PwrcIscrSpec>;
#[doc = "PWRC_ISCR register"]
pub mod pwrc_iscr;
#[doc = "GPIO_SWA_CTRL (rw) register accessor: GPIO_SWA_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_swa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_swa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_swa_ctrl`] module"]
#[doc(alias = "GPIO_SWA_CTRL")]
pub type GpioSwaCtrl = crate::Reg<gpio_swa_ctrl::GpioSwaCtrlSpec>;
#[doc = "GPIO_SWA_CTRL register"]
pub mod gpio_swa_ctrl;
#[doc = "INTAI_DTR (rw) register accessor: INTAI_DTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intai_dtr`] module"]
#[doc(alias = "INTAI_DTR")]
pub type IntaiDtr = crate::Reg<intai_dtr::IntaiDtrSpec>;
#[doc = "INTAI_DTR register"]
pub mod intai_dtr;
#[doc = "INTAI_IBER (rw) register accessor: INTAI_IBER register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_iber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intai_iber`] module"]
#[doc(alias = "INTAI_IBER")]
pub type IntaiIber = crate::Reg<intai_iber::IntaiIberSpec>;
#[doc = "INTAI_IBER register"]
pub mod intai_iber;
#[doc = "INTAI_IEVR (rw) register accessor: INTAI_IEVR register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_ievr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ievr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intai_ievr`] module"]
#[doc(alias = "INTAI_IEVR")]
pub type IntaiIevr = crate::Reg<intai_ievr::IntaiIevrSpec>;
#[doc = "INTAI_IEVR register"]
pub mod intai_ievr;
#[doc = "INTAI_IER (rw) register accessor: INTAI_IER register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intai_ier`] module"]
#[doc(alias = "INTAI_IER")]
pub type IntaiIer = crate::Reg<intai_ier::IntaiIerSpec>;
#[doc = "INTAI_IER register"]
pub mod intai_ier;
#[doc = "INTAI_ISCR (rw) register accessor: INTAI_ISCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intai_iscr`] module"]
#[doc(alias = "INTAI_ISCR")]
pub type IntaiIscr = crate::Reg<intai_iscr::IntaiIscrSpec>;
#[doc = "INTAI_ISCR register"]
pub mod intai_iscr;
#[doc = "SYSCFG_SR1 (r) register accessor: SYSCFG_SR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_sr1`] module"]
#[doc(alias = "SYSCFG_SR1")]
pub type SyscfgSr1 = crate::Reg<syscfg_sr1::SyscfgSr1Spec>;
#[doc = "SYSCFG_SR1 register"]
pub mod syscfg_sr1;
#[doc = "RF_DTB_CONFIG (rw) register accessor: RF_DTB_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_dtb_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_dtb_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_dtb_config`] module"]
#[doc(alias = "RF_DTB_CONFIG")]
pub type RfDtbConfig = crate::Reg<rf_dtb_config::RfDtbConfigSpec>;
#[doc = "RF_DTB_CONFIG register"]
pub mod rf_dtb_config;
