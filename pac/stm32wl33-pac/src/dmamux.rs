#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c0cr: C0cr,
    c1cr: C1cr,
    c2cr: C2cr,
    c3cr: C3cr,
    c4cr: C4cr,
    c5cr: C5cr,
    c6cr: C6cr,
    c7cr: C7cr,
}
impl RegisterBlock {
    #[doc = "0x00 - CxCR register"]
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0cr {
        &self.c0cr
    }
    #[doc = "0x04 - CxCR register"]
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1cr {
        &self.c1cr
    }
    #[doc = "0x08 - CxCR register"]
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2cr {
        &self.c2cr
    }
    #[doc = "0x0c - CxCR register"]
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3cr {
        &self.c3cr
    }
    #[doc = "0x10 - CxCR register"]
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4cr {
        &self.c4cr
    }
    #[doc = "0x14 - CxCR register"]
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5cr {
        &self.c5cr
    }
    #[doc = "0x18 - CxCR register"]
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6cr {
        &self.c6cr
    }
    #[doc = "0x1c - CxCR register"]
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7cr {
        &self.c7cr
    }
}
#[doc = "C0CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0cr`] module"]
#[doc(alias = "C0CR")]
pub type C0cr = crate::Reg<c0cr::C0crSpec>;
#[doc = "CxCR register"]
pub mod c0cr;
#[doc = "C1CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cr`] module"]
#[doc(alias = "C1CR")]
pub type C1cr = crate::Reg<c1cr::C1crSpec>;
#[doc = "CxCR register"]
pub mod c1cr;
#[doc = "C2CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cr`] module"]
#[doc(alias = "C2CR")]
pub type C2cr = crate::Reg<c2cr::C2crSpec>;
#[doc = "CxCR register"]
pub mod c2cr;
#[doc = "C3CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3cr`] module"]
#[doc(alias = "C3CR")]
pub type C3cr = crate::Reg<c3cr::C3crSpec>;
#[doc = "CxCR register"]
pub mod c3cr;
#[doc = "C4CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4cr`] module"]
#[doc(alias = "C4CR")]
pub type C4cr = crate::Reg<c4cr::C4crSpec>;
#[doc = "CxCR register"]
pub mod c4cr;
#[doc = "C5CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5cr`] module"]
#[doc(alias = "C5CR")]
pub type C5cr = crate::Reg<c5cr::C5crSpec>;
#[doc = "CxCR register"]
pub mod c5cr;
#[doc = "C6CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6cr`] module"]
#[doc(alias = "C6CR")]
pub type C6cr = crate::Reg<c6cr::C6crSpec>;
#[doc = "CxCR register"]
pub mod c6cr;
#[doc = "C7CR (rw) register accessor: CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7cr`] module"]
#[doc(alias = "C7CR")]
pub type C7cr = crate::Reg<c7cr::C7crSpec>;
#[doc = "CxCR register"]
pub mod c7cr;
