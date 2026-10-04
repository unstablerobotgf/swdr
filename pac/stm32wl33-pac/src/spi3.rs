#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_sspcr1: SpiSspcr1,
    spi_sspcr2: SpiSspcr2,
    spi_sspsr: SpiSspsr,
    spi_sspdr: SpiSspdr,
    spi_sspcrcpr: SpiSspcrcpr,
    spi_ssprxcrcr: SpiSsprxcrcr,
    spi_ssptxcrcr: SpiSsptxcrcr,
    spi2s_i2scfgr: Spi2sI2scfgr,
    spi2s_i2spr: Spi2sI2spr,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI_SSPCR1 register"]
    #[inline(always)]
    pub const fn spi_sspcr1(&self) -> &SpiSspcr1 {
        &self.spi_sspcr1
    }
    #[doc = "0x04 - SPI_SSPCR2 register"]
    #[inline(always)]
    pub const fn spi_sspcr2(&self) -> &SpiSspcr2 {
        &self.spi_sspcr2
    }
    #[doc = "0x08 - SPI_SSPSR register"]
    #[inline(always)]
    pub const fn spi_sspsr(&self) -> &SpiSspsr {
        &self.spi_sspsr
    }
    #[doc = "0x0c - SPI_SSPDR register"]
    #[inline(always)]
    pub const fn spi_sspdr(&self) -> &SpiSspdr {
        &self.spi_sspdr
    }
    #[doc = "0x10 - SPI_SSPCRCPR register"]
    #[inline(always)]
    pub const fn spi_sspcrcpr(&self) -> &SpiSspcrcpr {
        &self.spi_sspcrcpr
    }
    #[doc = "0x14 - SPI_SSPRXCRCR register"]
    #[inline(always)]
    pub const fn spi_ssprxcrcr(&self) -> &SpiSsprxcrcr {
        &self.spi_ssprxcrcr
    }
    #[doc = "0x18 - SPI_SSPTXCRCR register"]
    #[inline(always)]
    pub const fn spi_ssptxcrcr(&self) -> &SpiSsptxcrcr {
        &self.spi_ssptxcrcr
    }
    #[doc = "0x1c - SPI2S_I2SCFGR register"]
    #[inline(always)]
    pub const fn spi2s_i2scfgr(&self) -> &Spi2sI2scfgr {
        &self.spi2s_i2scfgr
    }
    #[doc = "0x20 - SPI2S_I2SPR register"]
    #[inline(always)]
    pub const fn spi2s_i2spr(&self) -> &Spi2sI2spr {
        &self.spi2s_i2spr
    }
}
#[doc = "SPI_SSPCR1 (rw) register accessor: SPI_SSPCR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sspcr1`] module"]
#[doc(alias = "SPI_SSPCR1")]
pub type SpiSspcr1 = crate::Reg<spi_sspcr1::SpiSspcr1Spec>;
#[doc = "SPI_SSPCR1 register"]
pub mod spi_sspcr1;
#[doc = "SPI_SSPCR2 (rw) register accessor: SPI_SSPCR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sspcr2`] module"]
#[doc(alias = "SPI_SSPCR2")]
pub type SpiSspcr2 = crate::Reg<spi_sspcr2::SpiSspcr2Spec>;
#[doc = "SPI_SSPCR2 register"]
pub mod spi_sspcr2;
#[doc = "SPI_SSPSR (rw) register accessor: SPI_SSPSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sspsr`] module"]
#[doc(alias = "SPI_SSPSR")]
pub type SpiSspsr = crate::Reg<spi_sspsr::SpiSspsrSpec>;
#[doc = "SPI_SSPSR register"]
pub mod spi_sspsr;
#[doc = "SPI_SSPDR (rw) register accessor: SPI_SSPDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sspdr`] module"]
#[doc(alias = "SPI_SSPDR")]
pub type SpiSspdr = crate::Reg<spi_sspdr::SpiSspdrSpec>;
#[doc = "SPI_SSPDR register"]
pub mod spi_sspdr;
#[doc = "SPI_SSPCRCPR (rw) register accessor: SPI_SSPCRCPR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspcrcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcrcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sspcrcpr`] module"]
#[doc(alias = "SPI_SSPCRCPR")]
pub type SpiSspcrcpr = crate::Reg<spi_sspcrcpr::SpiSspcrcprSpec>;
#[doc = "SPI_SSPCRCPR register"]
pub mod spi_sspcrcpr;
#[doc = "SPI_SSPRXCRCR (r) register accessor: SPI_SSPRXCRCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ssprxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ssprxcrcr`] module"]
#[doc(alias = "SPI_SSPRXCRCR")]
pub type SpiSsprxcrcr = crate::Reg<spi_ssprxcrcr::SpiSsprxcrcrSpec>;
#[doc = "SPI_SSPRXCRCR register"]
pub mod spi_ssprxcrcr;
#[doc = "SPI_SSPTXCRCR (r) register accessor: SPI_SSPTXCRCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ssptxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ssptxcrcr`] module"]
#[doc(alias = "SPI_SSPTXCRCR")]
pub type SpiSsptxcrcr = crate::Reg<spi_ssptxcrcr::SpiSsptxcrcrSpec>;
#[doc = "SPI_SSPTXCRCR register"]
pub mod spi_ssptxcrcr;
#[doc = "SPI2S_I2SCFGR (rw) register accessor: SPI2S_I2SCFGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2s_i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_i2scfgr`] module"]
#[doc(alias = "SPI2S_I2SCFGR")]
pub type Spi2sI2scfgr = crate::Reg<spi2s_i2scfgr::Spi2sI2scfgrSpec>;
#[doc = "SPI2S_I2SCFGR register"]
pub mod spi2s_i2scfgr;
#[doc = "SPI2S_I2SPR (rw) register accessor: SPI2S_I2SPR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2s_i2spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_i2spr`] module"]
#[doc(alias = "SPI2S_I2SPR")]
pub type Spi2sI2spr = crate::Reg<spi2s_i2spr::Spi2sI2sprSpec>;
#[doc = "SPI2S_I2SPR register"]
pub mod spi2s_i2spr;
