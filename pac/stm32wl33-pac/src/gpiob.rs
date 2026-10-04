#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    moder: Moder,
    otyper: Otyper,
    ospeedr: Ospeedr,
    pupdr: Pupdr,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    lckr: Lckr,
    afrl: Afrl,
    afrh: Afrh,
    brr: Brr,
}
impl RegisterBlock {
    #[doc = "0x00 - MODER register"]
    #[inline(always)]
    pub const fn moder(&self) -> &Moder {
        &self.moder
    }
    #[doc = "0x04 - OTYPER register"]
    #[inline(always)]
    pub const fn otyper(&self) -> &Otyper {
        &self.otyper
    }
    #[doc = "0x08 - OSPEEDR register"]
    #[inline(always)]
    pub const fn ospeedr(&self) -> &Ospeedr {
        &self.ospeedr
    }
    #[doc = "0x0c - PUPDR register"]
    #[inline(always)]
    pub const fn pupdr(&self) -> &Pupdr {
        &self.pupdr
    }
    #[doc = "0x10 - IDR register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x14 - ODR register"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x18 - BSRR register"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x1c - LCKR register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &Lckr {
        &self.lckr
    }
    #[doc = "0x20 - AFRL register"]
    #[inline(always)]
    pub const fn afrl(&self) -> &Afrl {
        &self.afrl
    }
    #[doc = "0x24 - AFRH register"]
    #[inline(always)]
    pub const fn afrh(&self) -> &Afrh {
        &self.afrh
    }
    #[doc = "0x28 - BRR register"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
}
#[doc = "MODER (rw) register accessor: MODER register\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder`] module"]
#[doc(alias = "MODER")]
pub type Moder = crate::Reg<moder::ModerSpec>;
#[doc = "MODER register"]
pub mod moder;
#[doc = "OTYPER (rw) register accessor: OTYPER register\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper`] module"]
#[doc(alias = "OTYPER")]
pub type Otyper = crate::Reg<otyper::OtyperSpec>;
#[doc = "OTYPER register"]
pub mod otyper;
#[doc = "OSPEEDR (rw) register accessor: OSPEEDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr`] module"]
#[doc(alias = "OSPEEDR")]
pub type Ospeedr = crate::Reg<ospeedr::OspeedrSpec>;
#[doc = "OSPEEDR register"]
pub mod ospeedr;
#[doc = "PUPDR (rw) register accessor: PUPDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr`] module"]
#[doc(alias = "PUPDR")]
pub type Pupdr = crate::Reg<pupdr::PupdrSpec>;
#[doc = "PUPDR register"]
pub mod pupdr;
#[doc = "IDR (r) register accessor: IDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "IDR register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: ODR register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "ODR register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: BSRR register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "BSRR register"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: LCKR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`] module"]
#[doc(alias = "LCKR")]
pub type Lckr = crate::Reg<lckr::LckrSpec>;
#[doc = "LCKR register"]
pub mod lckr;
#[doc = "AFRL (rw) register accessor: AFRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`] module"]
#[doc(alias = "AFRL")]
pub type Afrl = crate::Reg<afrl::AfrlSpec>;
#[doc = "AFRL register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: AFRH register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`] module"]
#[doc(alias = "AFRH")]
pub type Afrh = crate::Reg<afrh::AfrhSpec>;
#[doc = "AFRH register"]
pub mod afrh;
#[doc = "BRR (rw) register accessor: BRR register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "BRR register"]
pub mod brr;
