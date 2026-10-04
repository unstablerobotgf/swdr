#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    icscr: Icscr,
    cfgr: Cfgr,
    csswcr: Csswcr,
    krmr: Krmr,
    _reserved5: [u8; 0x04],
    cier: Cier,
    cifr: Cifr,
    cscmdr: Cscmdr,
    _reserved8: [u8; 0x0c],
    ahbrstr: Ahbrstr,
    apb0rstr: Apb0rstr,
    apb1rstr: Apb1rstr,
    _reserved11: [u8; 0x04],
    apb2rstr: Apb2rstr,
    _reserved12: [u8; 0x0c],
    ahbenr: Ahbenr,
    apb0enr: Apb0enr,
    apb1enr: Apb1enr,
    _reserved15: [u8; 0x04],
    apb2enr: Apb2enr,
    _reserved16: [u8; 0x1c],
    dbgr: Dbgr,
    _reserved17: [u8; 0x10],
    csr: Csr,
    rfswhsecr: Rfswhsecr,
    rfhsecr: Rfhsecr,
    ahbsmenr: Ahbsmenr,
    apb0smenr: Apb0smenr,
    apb1smenr: Apb1smenr,
}
impl RegisterBlock {
    #[doc = "0x00 - CR register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - ICSCR register"]
    #[inline(always)]
    pub const fn icscr(&self) -> &Icscr {
        &self.icscr
    }
    #[doc = "0x08 - CFGR register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x0c - CSSWCR register"]
    #[inline(always)]
    pub const fn csswcr(&self) -> &Csswcr {
        &self.csswcr
    }
    #[doc = "0x10 - KRMR register"]
    #[inline(always)]
    pub const fn krmr(&self) -> &Krmr {
        &self.krmr
    }
    #[doc = "0x18 - CIER register"]
    #[inline(always)]
    pub const fn cier(&self) -> &Cier {
        &self.cier
    }
    #[doc = "0x1c - CIFR register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &Cifr {
        &self.cifr
    }
    #[doc = "0x20 - CSCMDR register"]
    #[inline(always)]
    pub const fn cscmdr(&self) -> &Cscmdr {
        &self.cscmdr
    }
    #[doc = "0x30 - AHBRSTR register"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &Ahbrstr {
        &self.ahbrstr
    }
    #[doc = "0x34 - APB0RSTR register"]
    #[inline(always)]
    pub const fn apb0rstr(&self) -> &Apb0rstr {
        &self.apb0rstr
    }
    #[doc = "0x38 - APB1RSTR register"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &Apb1rstr {
        &self.apb1rstr
    }
    #[doc = "0x40 - APB2RSTR register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x50 - AHBENR register"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &Ahbenr {
        &self.ahbenr
    }
    #[doc = "0x54 - APB0ENR register"]
    #[inline(always)]
    pub const fn apb0enr(&self) -> &Apb0enr {
        &self.apb0enr
    }
    #[doc = "0x58 - APB1ENR register"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &Apb1enr {
        &self.apb1enr
    }
    #[doc = "0x60 - APB2ENR register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0x80 - DBGR register"]
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
    #[doc = "0x94 - CSR register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x98 - RFSWHSECR register"]
    #[inline(always)]
    pub const fn rfswhsecr(&self) -> &Rfswhsecr {
        &self.rfswhsecr
    }
    #[doc = "0x9c - RFHSECR register"]
    #[inline(always)]
    pub const fn rfhsecr(&self) -> &Rfhsecr {
        &self.rfhsecr
    }
    #[doc = "0xa0 - AHBSMENR register"]
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &Ahbsmenr {
        &self.ahbsmenr
    }
    #[doc = "0xa4 - APB0SMENR register"]
    #[inline(always)]
    pub const fn apb0smenr(&self) -> &Apb0smenr {
        &self.apb0smenr
    }
    #[doc = "0xa8 - APB1SMENR register"]
    #[inline(always)]
    pub const fn apb1smenr(&self) -> &Apb1smenr {
        &self.apb1smenr
    }
}
#[doc = "CR (rw) register accessor: CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: ICSCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icscr`] module"]
#[doc(alias = "ICSCR")]
pub type Icscr = crate::Reg<icscr::IcscrSpec>;
#[doc = "ICSCR register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: CFGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "CFGR register"]
pub mod cfgr;
#[doc = "CSSWCR (rw) register accessor: CSSWCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csswcr`] module"]
#[doc(alias = "CSSWCR")]
pub type Csswcr = crate::Reg<csswcr::CsswcrSpec>;
#[doc = "CSSWCR register"]
pub mod csswcr;
#[doc = "KRMR (rw) register accessor: KRMR register\n\nYou can [`read`](crate::Reg::read) this register and get [`krmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krmr`] module"]
#[doc(alias = "KRMR")]
pub type Krmr = crate::Reg<krmr::KrmrSpec>;
#[doc = "KRMR register"]
pub mod krmr;
#[doc = "CIER (rw) register accessor: CIER register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`] module"]
#[doc(alias = "CIER")]
pub type Cier = crate::Reg<cier::CierSpec>;
#[doc = "CIER register"]
pub mod cier;
#[doc = "CIFR (rw) register accessor: CIFR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`] module"]
#[doc(alias = "CIFR")]
pub type Cifr = crate::Reg<cifr::CifrSpec>;
#[doc = "CIFR register"]
pub mod cifr;
#[doc = "CSCMDR (rw) register accessor: CSCMDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cscmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cscmdr`] module"]
#[doc(alias = "CSCMDR")]
pub type Cscmdr = crate::Reg<cscmdr::CscmdrSpec>;
#[doc = "CSCMDR register"]
pub mod cscmdr;
#[doc = "AHBRSTR (rw) register accessor: AHBRSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`] module"]
#[doc(alias = "AHBRSTR")]
pub type Ahbrstr = crate::Reg<ahbrstr::AhbrstrSpec>;
#[doc = "AHBRSTR register"]
pub mod ahbrstr;
#[doc = "APB0RSTR (rw) register accessor: APB0RSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb0rstr`] module"]
#[doc(alias = "APB0RSTR")]
pub type Apb0rstr = crate::Reg<apb0rstr::Apb0rstrSpec>;
#[doc = "APB0RSTR register"]
pub mod apb0rstr;
#[doc = "APB1RSTR (rw) register accessor: APB1RSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`] module"]
#[doc(alias = "APB1RSTR")]
pub type Apb1rstr = crate::Reg<apb1rstr::Apb1rstrSpec>;
#[doc = "APB1RSTR register"]
pub mod apb1rstr;
#[doc = "APB2RSTR (rw) register accessor: APB2RSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = "APB2RSTR register"]
pub mod apb2rstr;
#[doc = "AHBENR (rw) register accessor: AHBENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`] module"]
#[doc(alias = "AHBENR")]
pub type Ahbenr = crate::Reg<ahbenr::AhbenrSpec>;
#[doc = "AHBENR register"]
pub mod ahbenr;
#[doc = "APB0ENR (rw) register accessor: APB0ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb0enr`] module"]
#[doc(alias = "APB0ENR")]
pub type Apb0enr = crate::Reg<apb0enr::Apb0enrSpec>;
#[doc = "APB0ENR register"]
pub mod apb0enr;
#[doc = "APB1ENR (rw) register accessor: APB1ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`] module"]
#[doc(alias = "APB1ENR")]
pub type Apb1enr = crate::Reg<apb1enr::Apb1enrSpec>;
#[doc = "APB1ENR register"]
pub mod apb1enr;
#[doc = "APB2ENR (rw) register accessor: APB2ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = "APB2ENR register"]
pub mod apb2enr;
#[doc = "DBGR (rw) register accessor: DBGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgr`] module"]
#[doc(alias = "DBGR")]
pub type Dbgr = crate::Reg<dbgr::DbgrSpec>;
#[doc = "DBGR register"]
pub mod dbgr;
#[doc = "CSR (rw) register accessor: CSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "CSR register"]
pub mod csr;
#[doc = "RFSWHSECR (rw) register accessor: RFSWHSECR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfswhsecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfswhsecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfswhsecr`] module"]
#[doc(alias = "RFSWHSECR")]
pub type Rfswhsecr = crate::Reg<rfswhsecr::RfswhsecrSpec>;
#[doc = "RFSWHSECR register"]
pub mod rfswhsecr;
#[doc = "RFHSECR (r) register accessor: RFHSECR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfhsecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfhsecr`] module"]
#[doc(alias = "RFHSECR")]
pub type Rfhsecr = crate::Reg<rfhsecr::RfhsecrSpec>;
#[doc = "RFHSECR register"]
pub mod rfhsecr;
#[doc = "AHBSMENR (rw) register accessor: AHBSMENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbsmenr`] module"]
#[doc(alias = "AHBSMENR")]
pub type Ahbsmenr = crate::Reg<ahbsmenr::AhbsmenrSpec>;
#[doc = "AHBSMENR register"]
pub mod ahbsmenr;
#[doc = "APB0SMENR (rw) register accessor: APB0SMENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb0smenr`] module"]
#[doc(alias = "APB0SMENR")]
pub type Apb0smenr = crate::Reg<apb0smenr::Apb0smenrSpec>;
#[doc = "APB0SMENR register"]
pub mod apb0smenr;
#[doc = "APB1SMENR (rw) register accessor: APB1SMENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1smenr`] module"]
#[doc(alias = "APB1SMENR")]
pub type Apb1smenr = crate::Reg<apb1smenr::Apb1smenrSpec>;
#[doc = "APB1SMENR register"]
pub mod apb1smenr;
