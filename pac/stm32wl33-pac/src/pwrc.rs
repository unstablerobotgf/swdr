#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    iewu: Iewu,
    iwup: Iwup,
    iwuf: Iwuf,
    sr2: Sr2,
    _reserved6: [u8; 0x04],
    cr5: Cr5,
    pucra: Pucra,
    pdcra: Pdcra,
    pucrb: Pucrb,
    pdcrb: Pdcrb,
    ewua: Ewua,
    wupa: Wupa,
    wufa: Wufa,
    _reserved14: [u8; 0x04],
    ewub: Ewub,
    wupb: Wupb,
    wufb: Wufb,
    sdwn_wuen: SdwnWuen,
    sdwn_wupol: SdwnWupol,
    sdwn_wuf: SdwnWuf,
    bof_tune: BofTune,
    _reserved21: [u8; 0x28],
    dbgr: Dbgr,
    extsrr: Extsrr,
    dbgsmps: Dbgsmps,
    trimr: Trimr,
    engtrim: Engtrim,
    dbg_status_reg1: DbgStatusReg1,
    dbg_status_reg2: DbgStatusReg2,
    engtrim2: Engtrim2,
}
impl RegisterBlock {
    #[doc = "0x00 - CR1 register"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - CR2 register"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - IEWU register"]
    #[inline(always)]
    pub const fn iewu(&self) -> &Iewu {
        &self.iewu
    }
    #[doc = "0x0c - IWUP register"]
    #[inline(always)]
    pub const fn iwup(&self) -> &Iwup {
        &self.iwup
    }
    #[doc = "0x10 - IWUF register"]
    #[inline(always)]
    pub const fn iwuf(&self) -> &Iwuf {
        &self.iwuf
    }
    #[doc = "0x14 - SR2 register"]
    #[inline(always)]
    pub const fn sr2(&self) -> &Sr2 {
        &self.sr2
    }
    #[doc = "0x1c - CR5 register"]
    #[inline(always)]
    pub const fn cr5(&self) -> &Cr5 {
        &self.cr5
    }
    #[doc = "0x20 - PUCRA register"]
    #[inline(always)]
    pub const fn pucra(&self) -> &Pucra {
        &self.pucra
    }
    #[doc = "0x24 - PDCRA register"]
    #[inline(always)]
    pub const fn pdcra(&self) -> &Pdcra {
        &self.pdcra
    }
    #[doc = "0x28 - PUCRB register"]
    #[inline(always)]
    pub const fn pucrb(&self) -> &Pucrb {
        &self.pucrb
    }
    #[doc = "0x2c - PDCRB register"]
    #[inline(always)]
    pub const fn pdcrb(&self) -> &Pdcrb {
        &self.pdcrb
    }
    #[doc = "0x30 - EWUA register"]
    #[inline(always)]
    pub const fn ewua(&self) -> &Ewua {
        &self.ewua
    }
    #[doc = "0x34 - WUPA register"]
    #[inline(always)]
    pub const fn wupa(&self) -> &Wupa {
        &self.wupa
    }
    #[doc = "0x38 - WUFA register"]
    #[inline(always)]
    pub const fn wufa(&self) -> &Wufa {
        &self.wufa
    }
    #[doc = "0x40 - EWUB register"]
    #[inline(always)]
    pub const fn ewub(&self) -> &Ewub {
        &self.ewub
    }
    #[doc = "0x44 - WUPB register"]
    #[inline(always)]
    pub const fn wupb(&self) -> &Wupb {
        &self.wupb
    }
    #[doc = "0x48 - WUFB register"]
    #[inline(always)]
    pub const fn wufb(&self) -> &Wufb {
        &self.wufb
    }
    #[doc = "0x4c - SDWN_WUEN register"]
    #[inline(always)]
    pub const fn sdwn_wuen(&self) -> &SdwnWuen {
        &self.sdwn_wuen
    }
    #[doc = "0x50 - SDWN_WUPOL register"]
    #[inline(always)]
    pub const fn sdwn_wupol(&self) -> &SdwnWupol {
        &self.sdwn_wupol
    }
    #[doc = "0x54 - SDWN_WUF register"]
    #[inline(always)]
    pub const fn sdwn_wuf(&self) -> &SdwnWuf {
        &self.sdwn_wuf
    }
    #[doc = "0x58 - BOF_TUNE register"]
    #[inline(always)]
    pub const fn bof_tune(&self) -> &BofTune {
        &self.bof_tune
    }
    #[doc = "0x84 - DBGR register"]
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
    #[doc = "0x88 - EXTSRR register"]
    #[inline(always)]
    pub const fn extsrr(&self) -> &Extsrr {
        &self.extsrr
    }
    #[doc = "0x8c - DBGSMPS register"]
    #[inline(always)]
    pub const fn dbgsmps(&self) -> &Dbgsmps {
        &self.dbgsmps
    }
    #[doc = "0x90 - TRIMR register"]
    #[inline(always)]
    pub const fn trimr(&self) -> &Trimr {
        &self.trimr
    }
    #[doc = "0x94 - ENGTRIM register"]
    #[inline(always)]
    pub const fn engtrim(&self) -> &Engtrim {
        &self.engtrim
    }
    #[doc = "0x98 - DBG_STATUS_REG1 register"]
    #[inline(always)]
    pub const fn dbg_status_reg1(&self) -> &DbgStatusReg1 {
        &self.dbg_status_reg1
    }
    #[doc = "0x9c - DBG_STATUS_REG2 register"]
    #[inline(always)]
    pub const fn dbg_status_reg2(&self) -> &DbgStatusReg2 {
        &self.dbg_status_reg2
    }
    #[doc = "0xa0 - ENGTRIM2 register"]
    #[inline(always)]
    pub const fn engtrim2(&self) -> &Engtrim2 {
        &self.engtrim2
    }
}
#[doc = "CR1 (rw) register accessor: CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "CR1 register"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "CR2 register"]
pub mod cr2;
#[doc = "IEWU (rw) register accessor: IEWU register\n\nYou can [`read`](crate::Reg::read) this register and get [`iewu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iewu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iewu`] module"]
#[doc(alias = "IEWU")]
pub type Iewu = crate::Reg<iewu::IewuSpec>;
#[doc = "IEWU register"]
pub mod iewu;
#[doc = "IWUP (rw) register accessor: IWUP register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwup`] module"]
#[doc(alias = "IWUP")]
pub type Iwup = crate::Reg<iwup::IwupSpec>;
#[doc = "IWUP register"]
pub mod iwup;
#[doc = "IWUF (rw) register accessor: IWUF register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwuf`] module"]
#[doc(alias = "IWUF")]
pub type Iwuf = crate::Reg<iwuf::IwufSpec>;
#[doc = "IWUF register"]
pub mod iwuf;
#[doc = "SR2 (r) register accessor: SR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`] module"]
#[doc(alias = "SR2")]
pub type Sr2 = crate::Reg<sr2::Sr2Spec>;
#[doc = "SR2 register"]
pub mod sr2;
#[doc = "CR5 (rw) register accessor: CR5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr5`] module"]
#[doc(alias = "CR5")]
pub type Cr5 = crate::Reg<cr5::Cr5Spec>;
#[doc = "CR5 register"]
pub mod cr5;
#[doc = "PUCRA (rw) register accessor: PUCRA register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucra`] module"]
#[doc(alias = "PUCRA")]
pub type Pucra = crate::Reg<pucra::PucraSpec>;
#[doc = "PUCRA register"]
pub mod pucra;
#[doc = "PDCRA (rw) register accessor: PDCRA register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcra`] module"]
#[doc(alias = "PDCRA")]
pub type Pdcra = crate::Reg<pdcra::PdcraSpec>;
#[doc = "PDCRA register"]
pub mod pdcra;
#[doc = "PUCRB (rw) register accessor: PUCRB register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrb`] module"]
#[doc(alias = "PUCRB")]
pub type Pucrb = crate::Reg<pucrb::PucrbSpec>;
#[doc = "PUCRB register"]
pub mod pucrb;
#[doc = "PDCRB (rw) register accessor: PDCRB register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrb`] module"]
#[doc(alias = "PDCRB")]
pub type Pdcrb = crate::Reg<pdcrb::PdcrbSpec>;
#[doc = "PDCRB register"]
pub mod pdcrb;
#[doc = "EWUA (rw) register accessor: EWUA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewua::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewua::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewua`] module"]
#[doc(alias = "EWUA")]
pub type Ewua = crate::Reg<ewua::EwuaSpec>;
#[doc = "EWUA register"]
pub mod ewua;
#[doc = "WUPA (rw) register accessor: WUPA register\n\nYou can [`read`](crate::Reg::read) this register and get [`wupa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupa`] module"]
#[doc(alias = "WUPA")]
pub type Wupa = crate::Reg<wupa::WupaSpec>;
#[doc = "WUPA register"]
pub mod wupa;
#[doc = "WUFA (rw) register accessor: WUFA register\n\nYou can [`read`](crate::Reg::read) this register and get [`wufa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wufa`] module"]
#[doc(alias = "WUFA")]
pub type Wufa = crate::Reg<wufa::WufaSpec>;
#[doc = "WUFA register"]
pub mod wufa;
#[doc = "EWUB (rw) register accessor: EWUB register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewub`] module"]
#[doc(alias = "EWUB")]
pub type Ewub = crate::Reg<ewub::EwubSpec>;
#[doc = "EWUB register"]
pub mod ewub;
#[doc = "WUPB (rw) register accessor: WUPB register\n\nYou can [`read`](crate::Reg::read) this register and get [`wupb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wupb`] module"]
#[doc(alias = "WUPB")]
pub type Wupb = crate::Reg<wupb::WupbSpec>;
#[doc = "WUPB register"]
pub mod wupb;
#[doc = "WUFB (rw) register accessor: WUFB register\n\nYou can [`read`](crate::Reg::read) this register and get [`wufb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wufb`] module"]
#[doc(alias = "WUFB")]
pub type Wufb = crate::Reg<wufb::WufbSpec>;
#[doc = "WUFB register"]
pub mod wufb;
#[doc = "SDWN_WUEN (rw) register accessor: SDWN_WUEN register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdwn_wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdwn_wuen`] module"]
#[doc(alias = "SDWN_WUEN")]
pub type SdwnWuen = crate::Reg<sdwn_wuen::SdwnWuenSpec>;
#[doc = "SDWN_WUEN register"]
pub mod sdwn_wuen;
#[doc = "SDWN_WUPOL (rw) register accessor: SDWN_WUPOL register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdwn_wupol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wupol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdwn_wupol`] module"]
#[doc(alias = "SDWN_WUPOL")]
pub type SdwnWupol = crate::Reg<sdwn_wupol::SdwnWupolSpec>;
#[doc = "SDWN_WUPOL register"]
pub mod sdwn_wupol;
#[doc = "SDWN_WUF (rw) register accessor: SDWN_WUF register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdwn_wuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdwn_wuf`] module"]
#[doc(alias = "SDWN_WUF")]
pub type SdwnWuf = crate::Reg<sdwn_wuf::SdwnWufSpec>;
#[doc = "SDWN_WUF register"]
pub mod sdwn_wuf;
#[doc = "BOF_TUNE (rw) register accessor: BOF_TUNE register\n\nYou can [`read`](crate::Reg::read) this register and get [`bof_tune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bof_tune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bof_tune`] module"]
#[doc(alias = "BOF_TUNE")]
pub type BofTune = crate::Reg<bof_tune::BofTuneSpec>;
#[doc = "BOF_TUNE register"]
pub mod bof_tune;
#[doc = "DBGR (rw) register accessor: DBGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgr`] module"]
#[doc(alias = "DBGR")]
pub type Dbgr = crate::Reg<dbgr::DbgrSpec>;
#[doc = "DBGR register"]
pub mod dbgr;
#[doc = "EXTSRR (rw) register accessor: EXTSRR register\n\nYou can [`read`](crate::Reg::read) this register and get [`extsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extsrr`] module"]
#[doc(alias = "EXTSRR")]
pub type Extsrr = crate::Reg<extsrr::ExtsrrSpec>;
#[doc = "EXTSRR register"]
pub mod extsrr;
#[doc = "DBGSMPS (rw) register accessor: DBGSMPS register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgsmps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsmps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgsmps`] module"]
#[doc(alias = "DBGSMPS")]
pub type Dbgsmps = crate::Reg<dbgsmps::DbgsmpsSpec>;
#[doc = "DBGSMPS register"]
pub mod dbgsmps;
#[doc = "TRIMR (r) register accessor: TRIMR register\n\nYou can [`read`](crate::Reg::read) this register and get [`trimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trimr`] module"]
#[doc(alias = "TRIMR")]
pub type Trimr = crate::Reg<trimr::TrimrSpec>;
#[doc = "TRIMR register"]
pub mod trimr;
#[doc = "ENGTRIM (rw) register accessor: ENGTRIM register\n\nYou can [`read`](crate::Reg::read) this register and get [`engtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@engtrim`] module"]
#[doc(alias = "ENGTRIM")]
pub type Engtrim = crate::Reg<engtrim::EngtrimSpec>;
#[doc = "ENGTRIM register"]
pub mod engtrim;
#[doc = "DBG_STATUS_REG1 (r) register accessor: DBG_STATUS_REG1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_status_reg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_status_reg1`] module"]
#[doc(alias = "DBG_STATUS_REG1")]
pub type DbgStatusReg1 = crate::Reg<dbg_status_reg1::DbgStatusReg1Spec>;
#[doc = "DBG_STATUS_REG1 register"]
pub mod dbg_status_reg1;
#[doc = "DBG_STATUS_REG2 (r) register accessor: DBG_STATUS_REG2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_status_reg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_status_reg2`] module"]
#[doc(alias = "DBG_STATUS_REG2")]
pub type DbgStatusReg2 = crate::Reg<dbg_status_reg2::DbgStatusReg2Spec>;
#[doc = "DBG_STATUS_REG2 register"]
pub mod dbg_status_reg2;
#[doc = "ENGTRIM2 (rw) register accessor: ENGTRIM2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`engtrim2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@engtrim2`] module"]
#[doc(alias = "ENGTRIM2")]
pub type Engtrim2 = crate::Reg<engtrim2::Engtrim2Spec>;
#[doc = "ENGTRIM2 register"]
pub mod engtrim2;
