#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crc_dr: CrcDr,
    crc_idr: CrcIdr,
    crc_cr: CrcCr,
    _reserved3: [u8; 0x04],
    crc_init: CrcInit,
    crc_pol: CrcPol,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_DR register"]
    #[inline(always)]
    pub const fn crc_dr(&self) -> &CrcDr {
        &self.crc_dr
    }
    #[doc = "0x04 - CRC_IDR register"]
    #[inline(always)]
    pub const fn crc_idr(&self) -> &CrcIdr {
        &self.crc_idr
    }
    #[doc = "0x08 - CRC_CR register"]
    #[inline(always)]
    pub const fn crc_cr(&self) -> &CrcCr {
        &self.crc_cr
    }
    #[doc = "0x10 - CRC_INIT register"]
    #[inline(always)]
    pub const fn crc_init(&self) -> &CrcInit {
        &self.crc_init
    }
    #[doc = "0x14 - CRC_POL register"]
    #[inline(always)]
    pub const fn crc_pol(&self) -> &CrcPol {
        &self.crc_pol
    }
}
#[doc = "CRC_DR (rw) register accessor: CRC_DR register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_dr`] module"]
#[doc(alias = "CRC_DR")]
pub type CrcDr = crate::Reg<crc_dr::CrcDrSpec>;
#[doc = "CRC_DR register"]
pub mod crc_dr;
#[doc = "CRC_IDR (rw) register accessor: CRC_IDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_idr`] module"]
#[doc(alias = "CRC_IDR")]
pub type CrcIdr = crate::Reg<crc_idr::CrcIdrSpec>;
#[doc = "CRC_IDR register"]
pub mod crc_idr;
#[doc = "CRC_CR (rw) register accessor: CRC_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cr`] module"]
#[doc(alias = "CRC_CR")]
pub type CrcCr = crate::Reg<crc_cr::CrcCrSpec>;
#[doc = "CRC_CR register"]
pub mod crc_cr;
#[doc = "CRC_INIT (rw) register accessor: CRC_INIT register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_init`] module"]
#[doc(alias = "CRC_INIT")]
pub type CrcInit = crate::Reg<crc_init::CrcInitSpec>;
#[doc = "CRC_INIT register"]
pub mod crc_init;
#[doc = "CRC_POL (rw) register accessor: CRC_POL register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pol`] module"]
#[doc(alias = "CRC_POL")]
pub type CrcPol = crate::Reg<crc_pol::CrcPolSpec>;
#[doc = "CRC_POL register"]
pub mod crc_pol;
