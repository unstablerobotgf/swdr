#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtc_tr: RtcTr,
    rtc_dr: RtcDr,
    rtc_cr: RtcCr,
    rtc_isr: RtcIsr,
    rtc_prer: RtcPrer,
    rtc_wutr: RtcWutr,
    _reserved6: [u8; 0x04],
    rtc_alrmar: RtcAlrmar,
    _reserved7: [u8; 0x04],
    rtc_wpr: RtcWpr,
    rtc_ssr: RtcSsr,
    rtc_shiftr: RtcShiftr,
    rtc_tstr: RtcTstr,
    rtc_tsdr: RtcTsdr,
    rtc_tsssr: RtcTsssr,
    rtc_calr: RtcCalr,
    rtc_tampcr: RtcTampcr,
    rtc_alrmassr: RtcAlrmassr,
    _reserved16: [u8; 0x04],
    rtc_or: RtcOr,
    rtc_bkp0r: RtcBkp0r,
    rtc_bkp1r: RtcBkp1r,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC_TR register"]
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RtcTr {
        &self.rtc_tr
    }
    #[doc = "0x04 - RTC_DR register"]
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RtcDr {
        &self.rtc_dr
    }
    #[doc = "0x08 - RTC_CR register"]
    #[inline(always)]
    pub const fn rtc_cr(&self) -> &RtcCr {
        &self.rtc_cr
    }
    #[doc = "0x0c - RTC_ISR register"]
    #[inline(always)]
    pub const fn rtc_isr(&self) -> &RtcIsr {
        &self.rtc_isr
    }
    #[doc = "0x10 - RTC_PRER register"]
    #[inline(always)]
    pub const fn rtc_prer(&self) -> &RtcPrer {
        &self.rtc_prer
    }
    #[doc = "0x14 - RTC_WUTR register"]
    #[inline(always)]
    pub const fn rtc_wutr(&self) -> &RtcWutr {
        &self.rtc_wutr
    }
    #[doc = "0x1c - RTC_ALRMAR register"]
    #[inline(always)]
    pub const fn rtc_alrmar(&self) -> &RtcAlrmar {
        &self.rtc_alrmar
    }
    #[doc = "0x24 - RTC_WPR register"]
    #[inline(always)]
    pub const fn rtc_wpr(&self) -> &RtcWpr {
        &self.rtc_wpr
    }
    #[doc = "0x28 - RTC_SSR register"]
    #[inline(always)]
    pub const fn rtc_ssr(&self) -> &RtcSsr {
        &self.rtc_ssr
    }
    #[doc = "0x2c - RTC_SHIFTR register"]
    #[inline(always)]
    pub const fn rtc_shiftr(&self) -> &RtcShiftr {
        &self.rtc_shiftr
    }
    #[doc = "0x30 - RTC_TSTR register"]
    #[inline(always)]
    pub const fn rtc_tstr(&self) -> &RtcTstr {
        &self.rtc_tstr
    }
    #[doc = "0x34 - RTC_TSDR register"]
    #[inline(always)]
    pub const fn rtc_tsdr(&self) -> &RtcTsdr {
        &self.rtc_tsdr
    }
    #[doc = "0x38 - RTC_TSSSR register"]
    #[inline(always)]
    pub const fn rtc_tsssr(&self) -> &RtcTsssr {
        &self.rtc_tsssr
    }
    #[doc = "0x3c - RTC_CALR register"]
    #[inline(always)]
    pub const fn rtc_calr(&self) -> &RtcCalr {
        &self.rtc_calr
    }
    #[doc = "0x40 - RTC_TAMPCR register"]
    #[inline(always)]
    pub const fn rtc_tampcr(&self) -> &RtcTampcr {
        &self.rtc_tampcr
    }
    #[doc = "0x44 - RTC_ALRMASSR register"]
    #[inline(always)]
    pub const fn rtc_alrmassr(&self) -> &RtcAlrmassr {
        &self.rtc_alrmassr
    }
    #[doc = "0x4c - RTC_OR register"]
    #[inline(always)]
    pub const fn rtc_or(&self) -> &RtcOr {
        &self.rtc_or
    }
    #[doc = "0x50 - RTC_BKPxR register"]
    #[inline(always)]
    pub const fn rtc_bkp0r(&self) -> &RtcBkp0r {
        &self.rtc_bkp0r
    }
    #[doc = "0x54 - RTC_BKPxR register"]
    #[inline(always)]
    pub const fn rtc_bkp1r(&self) -> &RtcBkp1r {
        &self.rtc_bkp1r
    }
}
#[doc = "RTC_TR (rw) register accessor: RTC_TR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tr`] module"]
#[doc(alias = "RTC_TR")]
pub type RtcTr = crate::Reg<rtc_tr::RtcTrSpec>;
#[doc = "RTC_TR register"]
pub mod rtc_tr;
#[doc = "RTC_DR (rw) register accessor: RTC_DR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_dr`] module"]
#[doc(alias = "RTC_DR")]
pub type RtcDr = crate::Reg<rtc_dr::RtcDrSpec>;
#[doc = "RTC_DR register"]
pub mod rtc_dr;
#[doc = "RTC_CR (rw) register accessor: RTC_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cr`] module"]
#[doc(alias = "RTC_CR")]
pub type RtcCr = crate::Reg<rtc_cr::RtcCrSpec>;
#[doc = "RTC_CR register"]
pub mod rtc_cr;
#[doc = "RTC_ISR (rw) register accessor: RTC_ISR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_isr`] module"]
#[doc(alias = "RTC_ISR")]
pub type RtcIsr = crate::Reg<rtc_isr::RtcIsrSpec>;
#[doc = "RTC_ISR register"]
pub mod rtc_isr;
#[doc = "RTC_PRER (rw) register accessor: RTC_PRER register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_prer`] module"]
#[doc(alias = "RTC_PRER")]
pub type RtcPrer = crate::Reg<rtc_prer::RtcPrerSpec>;
#[doc = "RTC_PRER register"]
pub mod rtc_prer;
#[doc = "RTC_WUTR (rw) register accessor: RTC_WUTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wutr`] module"]
#[doc(alias = "RTC_WUTR")]
pub type RtcWutr = crate::Reg<rtc_wutr::RtcWutrSpec>;
#[doc = "RTC_WUTR register"]
pub mod rtc_wutr;
#[doc = "RTC_ALRMAR (rw) register accessor: RTC_ALRMAR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmar`] module"]
#[doc(alias = "RTC_ALRMAR")]
pub type RtcAlrmar = crate::Reg<rtc_alrmar::RtcAlrmarSpec>;
#[doc = "RTC_ALRMAR register"]
pub mod rtc_alrmar;
#[doc = "RTC_WPR (rw) register accessor: RTC_WPR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_wpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wpr`] module"]
#[doc(alias = "RTC_WPR")]
pub type RtcWpr = crate::Reg<rtc_wpr::RtcWprSpec>;
#[doc = "RTC_WPR register"]
pub mod rtc_wpr;
#[doc = "RTC_SSR (r) register accessor: RTC_SSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ssr`] module"]
#[doc(alias = "RTC_SSR")]
pub type RtcSsr = crate::Reg<rtc_ssr::RtcSsrSpec>;
#[doc = "RTC_SSR register"]
pub mod rtc_ssr;
#[doc = "RTC_SHIFTR (rw) register accessor: RTC_SHIFTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_shiftr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_shiftr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_shiftr`] module"]
#[doc(alias = "RTC_SHIFTR")]
pub type RtcShiftr = crate::Reg<rtc_shiftr::RtcShiftrSpec>;
#[doc = "RTC_SHIFTR register"]
pub mod rtc_shiftr;
#[doc = "RTC_TSTR (rw) register accessor: RTC_TSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tstr`] module"]
#[doc(alias = "RTC_TSTR")]
pub type RtcTstr = crate::Reg<rtc_tstr::RtcTstrSpec>;
#[doc = "RTC_TSTR register"]
pub mod rtc_tstr;
#[doc = "RTC_TSDR (rw) register accessor: RTC_TSDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tsdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsdr`] module"]
#[doc(alias = "RTC_TSDR")]
pub type RtcTsdr = crate::Reg<rtc_tsdr::RtcTsdrSpec>;
#[doc = "RTC_TSDR register"]
pub mod rtc_tsdr;
#[doc = "RTC_TSSSR (r) register accessor: RTC_TSSSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsssr`] module"]
#[doc(alias = "RTC_TSSSR")]
pub type RtcTsssr = crate::Reg<rtc_tsssr::RtcTsssrSpec>;
#[doc = "RTC_TSSSR register"]
pub mod rtc_tsssr;
#[doc = "RTC_CALR (rw) register accessor: RTC_CALR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_calr`] module"]
#[doc(alias = "RTC_CALR")]
pub type RtcCalr = crate::Reg<rtc_calr::RtcCalrSpec>;
#[doc = "RTC_CALR register"]
pub mod rtc_calr;
#[doc = "RTC_TAMPCR (rw) register accessor: RTC_TAMPCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tampcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tampcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tampcr`] module"]
#[doc(alias = "RTC_TAMPCR")]
pub type RtcTampcr = crate::Reg<rtc_tampcr::RtcTampcrSpec>;
#[doc = "RTC_TAMPCR register"]
pub mod rtc_tampcr;
#[doc = "RTC_ALRMASSR (rw) register accessor: RTC_ALRMASSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmassr`] module"]
#[doc(alias = "RTC_ALRMASSR")]
pub type RtcAlrmassr = crate::Reg<rtc_alrmassr::RtcAlrmassrSpec>;
#[doc = "RTC_ALRMASSR register"]
pub mod rtc_alrmassr;
#[doc = "RTC_OR (rw) register accessor: RTC_OR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_or`] module"]
#[doc(alias = "RTC_OR")]
pub type RtcOr = crate::Reg<rtc_or::RtcOrSpec>;
#[doc = "RTC_OR register"]
pub mod rtc_or;
#[doc = "RTC_BKP0R (rw) register accessor: RTC_BKPxR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp0r`] module"]
#[doc(alias = "RTC_BKP0R")]
pub type RtcBkp0r = crate::Reg<rtc_bkp0r::RtcBkp0rSpec>;
#[doc = "RTC_BKPxR register"]
pub mod rtc_bkp0r;
#[doc = "RTC_BKP1R (rw) register accessor: RTC_BKPxR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp1r`] module"]
#[doc(alias = "RTC_BKP1R")]
pub type RtcBkp1r = crate::Reg<rtc_bkp1r::RtcBkp1rSpec>;
#[doc = "RTC_BKPxR register"]
pub mod rtc_bkp1r;
