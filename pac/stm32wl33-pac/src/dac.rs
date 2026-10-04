#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    swtrigr: Swtrigr,
    _reserved2: [u8; 0x08],
    dhr: Dhr,
    _reserved3: [u8; 0x18],
    dor: Dor,
    _reserved4: [u8; 0x04],
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - CR register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - SWTRIGR register"]
    #[inline(always)]
    pub const fn swtrigr(&self) -> &Swtrigr {
        &self.swtrigr
    }
    #[doc = "0x10 - DHR register"]
    #[inline(always)]
    pub const fn dhr(&self) -> &Dhr {
        &self.dhr
    }
    #[doc = "0x2c - DOR register"]
    #[inline(always)]
    pub const fn dor(&self) -> &Dor {
        &self.dor
    }
    #[doc = "0x34 - SR register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "CR (rw) register accessor: CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR register"]
pub mod cr;
#[doc = "SWTRIGR (rw) register accessor: SWTRIGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrigr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrigr`] module"]
#[doc(alias = "SWTRIGR")]
pub type Swtrigr = crate::Reg<swtrigr::SwtrigrSpec>;
#[doc = "SWTRIGR register"]
pub mod swtrigr;
#[doc = "DHR (rw) register accessor: DHR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr`] module"]
#[doc(alias = "DHR")]
pub type Dhr = crate::Reg<dhr::DhrSpec>;
#[doc = "DHR register"]
pub mod dhr;
#[doc = "DOR (r) register accessor: DOR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor`] module"]
#[doc(alias = "DOR")]
pub type Dor = crate::Reg<dor::DorSpec>;
#[doc = "DOR register"]
pub mod dor;
#[doc = "SR (rw) register accessor: SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SR register"]
pub mod sr;
