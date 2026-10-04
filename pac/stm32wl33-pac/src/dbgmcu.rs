#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    dbg_apb0_fz: DbgApb0Fz,
    dbg_apb1_fz: DbgApb1Fz,
}
impl RegisterBlock {
    #[doc = "0x00 - CR register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DBG_APB0_FZ register"]
    #[inline(always)]
    pub const fn dbg_apb0_fz(&self) -> &DbgApb0Fz {
        &self.dbg_apb0_fz
    }
    #[doc = "0x08 - DBG_APB1_FZ register"]
    #[inline(always)]
    pub const fn dbg_apb1_fz(&self) -> &DbgApb1Fz {
        &self.dbg_apb1_fz
    }
}
#[doc = "CR (rw) register accessor: CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR register"]
pub mod cr;
#[doc = "DBG_APB0_FZ (rw) register accessor: DBG_APB0_FZ register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_apb0_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb0_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_apb0_fz`] module"]
#[doc(alias = "DBG_APB0_FZ")]
pub type DbgApb0Fz = crate::Reg<dbg_apb0_fz::DbgApb0FzSpec>;
#[doc = "DBG_APB0_FZ register"]
pub mod dbg_apb0_fz;
#[doc = "DBG_APB1_FZ (rw) register accessor: DBG_APB1_FZ register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_apb1_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb1_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_apb1_fz`] module"]
#[doc(alias = "DBG_APB1_FZ")]
pub type DbgApb1Fz = crate::Reg<dbg_apb1_fz::DbgApb1FzSpec>;
#[doc = "DBG_APB1_FZ register"]
pub mod dbg_apb1_fz;
