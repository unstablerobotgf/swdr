#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rng_cr: RngCr,
    rng_sr: RngSr,
    rng_val: RngVal,
    _reserved3: [u8; 0x74],
    rng_tcr: RngTcr,
    rng_itip: RngItip,
    _reserved5: [u8; 0x0f58],
    rngperiph_id0: RngperiphId0,
    rngperiph_id1: RngperiphId1,
    rngperiph_id2: RngperiphId2,
    rngperiph_id3: RngperiphId3,
    rngpcell_id0: RngpcellId0,
    rngpcell_id1: RngpcellId1,
    rngpcell_id2: RngpcellId2,
    rngpcell_id3: RngpcellId3,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG_CR register"]
    #[inline(always)]
    pub const fn rng_cr(&self) -> &RngCr {
        &self.rng_cr
    }
    #[doc = "0x04 - RNG_SR register"]
    #[inline(always)]
    pub const fn rng_sr(&self) -> &RngSr {
        &self.rng_sr
    }
    #[doc = "0x08 - RNG_VAL register"]
    #[inline(always)]
    pub const fn rng_val(&self) -> &RngVal {
        &self.rng_val
    }
    #[doc = "0x80 - RNG_TCR register"]
    #[inline(always)]
    pub const fn rng_tcr(&self) -> &RngTcr {
        &self.rng_tcr
    }
    #[doc = "0x84 - RNG_ITIP register"]
    #[inline(always)]
    pub const fn rng_itip(&self) -> &RngItip {
        &self.rng_itip
    }
    #[doc = "0xfe0 - RNGPeriphID0 register"]
    #[inline(always)]
    pub const fn rngperiph_id0(&self) -> &RngperiphId0 {
        &self.rngperiph_id0
    }
    #[doc = "0xfe4 - RNGPeriphID1 register"]
    #[inline(always)]
    pub const fn rngperiph_id1(&self) -> &RngperiphId1 {
        &self.rngperiph_id1
    }
    #[doc = "0xfe8 - RNGPeriphID2 register"]
    #[inline(always)]
    pub const fn rngperiph_id2(&self) -> &RngperiphId2 {
        &self.rngperiph_id2
    }
    #[doc = "0xfec - RNGPeriphID3 register"]
    #[inline(always)]
    pub const fn rngperiph_id3(&self) -> &RngperiphId3 {
        &self.rngperiph_id3
    }
    #[doc = "0xff0 - RNGPCellID0 register"]
    #[inline(always)]
    pub const fn rngpcell_id0(&self) -> &RngpcellId0 {
        &self.rngpcell_id0
    }
    #[doc = "0xff4 - RNGPCellID1 register"]
    #[inline(always)]
    pub const fn rngpcell_id1(&self) -> &RngpcellId1 {
        &self.rngpcell_id1
    }
    #[doc = "0xff8 - RNGPCellID2 register"]
    #[inline(always)]
    pub const fn rngpcell_id2(&self) -> &RngpcellId2 {
        &self.rngpcell_id2
    }
    #[doc = "0xffc - RNGPCellID3 register"]
    #[inline(always)]
    pub const fn rngpcell_id3(&self) -> &RngpcellId3 {
        &self.rngpcell_id3
    }
}
#[doc = "RNG_CR (rw) register accessor: RNG_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cr`] module"]
#[doc(alias = "RNG_CR")]
pub type RngCr = crate::Reg<rng_cr::RngCrSpec>;
#[doc = "RNG_CR register"]
pub mod rng_cr;
#[doc = "RNG_SR (rw) register accessor: RNG_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_sr`] module"]
#[doc(alias = "RNG_SR")]
pub type RngSr = crate::Reg<rng_sr::RngSrSpec>;
#[doc = "RNG_SR register"]
pub mod rng_sr;
#[doc = "RNG_VAL (r) register accessor: RNG_VAL register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_val`] module"]
#[doc(alias = "RNG_VAL")]
pub type RngVal = crate::Reg<rng_val::RngValSpec>;
#[doc = "RNG_VAL register"]
pub mod rng_val;
#[doc = "RNG_TCR (rw) register accessor: RNG_TCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_tcr`] module"]
#[doc(alias = "RNG_TCR")]
pub type RngTcr = crate::Reg<rng_tcr::RngTcrSpec>;
#[doc = "RNG_TCR register"]
pub mod rng_tcr;
#[doc = "RNG_ITIP (rw) register accessor: RNG_ITIP register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_itip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_itip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_itip`] module"]
#[doc(alias = "RNG_ITIP")]
pub type RngItip = crate::Reg<rng_itip::RngItipSpec>;
#[doc = "RNG_ITIP register"]
pub mod rng_itip;
#[doc = "RNGPeriphID0 (r) register accessor: RNGPeriphID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngperiph_id0`] module"]
#[doc(alias = "RNGPeriphID0")]
pub type RngperiphId0 = crate::Reg<rngperiph_id0::RngperiphId0Spec>;
#[doc = "RNGPeriphID0 register"]
pub mod rngperiph_id0;
#[doc = "RNGPeriphID1 (r) register accessor: RNGPeriphID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngperiph_id1`] module"]
#[doc(alias = "RNGPeriphID1")]
pub type RngperiphId1 = crate::Reg<rngperiph_id1::RngperiphId1Spec>;
#[doc = "RNGPeriphID1 register"]
pub mod rngperiph_id1;
#[doc = "RNGPeriphID2 (r) register accessor: RNGPeriphID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngperiph_id2`] module"]
#[doc(alias = "RNGPeriphID2")]
pub type RngperiphId2 = crate::Reg<rngperiph_id2::RngperiphId2Spec>;
#[doc = "RNGPeriphID2 register"]
pub mod rngperiph_id2;
#[doc = "RNGPeriphID3 (r) register accessor: RNGPeriphID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngperiph_id3`] module"]
#[doc(alias = "RNGPeriphID3")]
pub type RngperiphId3 = crate::Reg<rngperiph_id3::RngperiphId3Spec>;
#[doc = "RNGPeriphID3 register"]
pub mod rngperiph_id3;
#[doc = "RNGPCellID0 (r) register accessor: RNGPCellID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpcell_id0`] module"]
#[doc(alias = "RNGPCellID0")]
pub type RngpcellId0 = crate::Reg<rngpcell_id0::RngpcellId0Spec>;
#[doc = "RNGPCellID0 register"]
pub mod rngpcell_id0;
#[doc = "RNGPCellID1 (r) register accessor: RNGPCellID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpcell_id1`] module"]
#[doc(alias = "RNGPCellID1")]
pub type RngpcellId1 = crate::Reg<rngpcell_id1::RngpcellId1Spec>;
#[doc = "RNGPCellID1 register"]
pub mod rngpcell_id1;
#[doc = "RNGPCellID2 (r) register accessor: RNGPCellID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpcell_id2`] module"]
#[doc(alias = "RNGPCellID2")]
pub type RngpcellId2 = crate::Reg<rngpcell_id2::RngpcellId2Spec>;
#[doc = "RNGPCellID2 register"]
pub mod rngpcell_id2;
#[doc = "RNGPCellID3 (r) register accessor: RNGPCellID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngpcell_id3`] module"]
#[doc(alias = "RNGPCellID3")]
pub type RngpcellId3 = crate::Reg<rngpcell_id3::RngpcellId3Spec>;
#[doc = "RNGPCellID3 register"]
pub mod rngpcell_id3;
