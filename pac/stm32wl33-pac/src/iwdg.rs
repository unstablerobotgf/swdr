#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iwdg_kr: IwdgKr,
    iwdg_pr: IwdgPr,
    iwdg_rlr: IwdgRlr,
    iwdg_sr: IwdgSr,
    iwdg_winr: IwdgWinr,
}
impl RegisterBlock {
    #[doc = "0x00 - IWDG_KR register"]
    #[inline(always)]
    pub const fn iwdg_kr(&self) -> &IwdgKr {
        &self.iwdg_kr
    }
    #[doc = "0x04 - IWDG_PR register"]
    #[inline(always)]
    pub const fn iwdg_pr(&self) -> &IwdgPr {
        &self.iwdg_pr
    }
    #[doc = "0x08 - IWDG_RLR register"]
    #[inline(always)]
    pub const fn iwdg_rlr(&self) -> &IwdgRlr {
        &self.iwdg_rlr
    }
    #[doc = "0x0c - IWDG_SR register"]
    #[inline(always)]
    pub const fn iwdg_sr(&self) -> &IwdgSr {
        &self.iwdg_sr
    }
    #[doc = "0x10 - IWDG_WINR register"]
    #[inline(always)]
    pub const fn iwdg_winr(&self) -> &IwdgWinr {
        &self.iwdg_winr
    }
}
#[doc = "IWDG_KR (rw) register accessor: IWDG_KR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_kr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_kr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_kr`] module"]
#[doc(alias = "IWDG_KR")]
pub type IwdgKr = crate::Reg<iwdg_kr::IwdgKrSpec>;
#[doc = "IWDG_KR register"]
pub mod iwdg_kr;
#[doc = "IWDG_PR (rw) register accessor: IWDG_PR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_pr`] module"]
#[doc(alias = "IWDG_PR")]
pub type IwdgPr = crate::Reg<iwdg_pr::IwdgPrSpec>;
#[doc = "IWDG_PR register"]
pub mod iwdg_pr;
#[doc = "IWDG_RLR (rw) register accessor: IWDG_RLR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_rlr`] module"]
#[doc(alias = "IWDG_RLR")]
pub type IwdgRlr = crate::Reg<iwdg_rlr::IwdgRlrSpec>;
#[doc = "IWDG_RLR register"]
pub mod iwdg_rlr;
#[doc = "IWDG_SR (r) register accessor: IWDG_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_sr`] module"]
#[doc(alias = "IWDG_SR")]
pub type IwdgSr = crate::Reg<iwdg_sr::IwdgSrSpec>;
#[doc = "IWDG_SR register"]
pub mod iwdg_sr;
#[doc = "IWDG_WINR (rw) register accessor: IWDG_WINR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_winr`] module"]
#[doc(alias = "IWDG_WINR")]
pub type IwdgWinr = crate::Reg<iwdg_winr::IwdgWinrSpec>;
#[doc = "IWDG_WINR register"]
pub mod iwdg_winr;
