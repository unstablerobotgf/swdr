#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rfip_wakeuptime: RfipWakeuptime,
    cpu_wakeuptime: CpuWakeuptime,
    wakeup_ctrl: WakeupCtrl,
    rrm_cmdlist_ptr: RrmCmdlistPtr,
    seq_globaltable_ptr: SeqGlobaltablePtr,
}
impl RegisterBlock {
    #[doc = "0x00 - RFIP_WAKEUPTIME register"]
    #[inline(always)]
    pub const fn rfip_wakeuptime(&self) -> &RfipWakeuptime {
        &self.rfip_wakeuptime
    }
    #[doc = "0x04 - CPU_WAKEUPTIME register"]
    #[inline(always)]
    pub const fn cpu_wakeuptime(&self) -> &CpuWakeuptime {
        &self.cpu_wakeuptime
    }
    #[doc = "0x08 - WAKEUP_CTRL register"]
    #[inline(always)]
    pub const fn wakeup_ctrl(&self) -> &WakeupCtrl {
        &self.wakeup_ctrl
    }
    #[doc = "0x0c - RRM_CMDLIST_PTR register"]
    #[inline(always)]
    pub const fn rrm_cmdlist_ptr(&self) -> &RrmCmdlistPtr {
        &self.rrm_cmdlist_ptr
    }
    #[doc = "0x10 - SEQ_GLOBALTABLE_PTR register"]
    #[inline(always)]
    pub const fn seq_globaltable_ptr(&self) -> &SeqGlobaltablePtr {
        &self.seq_globaltable_ptr
    }
}
#[doc = "RFIP_WAKEUPTIME (r) register accessor: RFIP_WAKEUPTIME register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_wakeuptime::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfip_wakeuptime`] module"]
#[doc(alias = "RFIP_WAKEUPTIME")]
pub type RfipWakeuptime = crate::Reg<rfip_wakeuptime::RfipWakeuptimeSpec>;
#[doc = "RFIP_WAKEUPTIME register"]
pub mod rfip_wakeuptime;
#[doc = "CPU_WAKEUPTIME (rw) register accessor: CPU_WAKEUPTIME register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wakeuptime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeuptime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_wakeuptime`] module"]
#[doc(alias = "CPU_WAKEUPTIME")]
pub type CpuWakeuptime = crate::Reg<cpu_wakeuptime::CpuWakeuptimeSpec>;
#[doc = "CPU_WAKEUPTIME register"]
pub mod cpu_wakeuptime;
#[doc = "WAKEUP_CTRL (rw) register accessor: WAKEUP_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_ctrl`] module"]
#[doc(alias = "WAKEUP_CTRL")]
pub type WakeupCtrl = crate::Reg<wakeup_ctrl::WakeupCtrlSpec>;
#[doc = "WAKEUP_CTRL register"]
pub mod wakeup_ctrl;
#[doc = "RRM_CMDLIST_PTR (rw) register accessor: RRM_CMDLIST_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrm_cmdlist_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_cmdlist_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrm_cmdlist_ptr`] module"]
#[doc(alias = "RRM_CMDLIST_PTR")]
pub type RrmCmdlistPtr = crate::Reg<rrm_cmdlist_ptr::RrmCmdlistPtrSpec>;
#[doc = "RRM_CMDLIST_PTR register"]
pub mod rrm_cmdlist_ptr;
#[doc = "SEQ_GLOBALTABLE_PTR (rw) register accessor: SEQ_GLOBALTABLE_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_globaltable_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_globaltable_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_globaltable_ptr`] module"]
#[doc(alias = "SEQ_GLOBALTABLE_PTR")]
pub type SeqGlobaltablePtr = crate::Reg<seq_globaltable_ptr::SeqGlobaltablePtrSpec>;
#[doc = "SEQ_GLOBALTABLE_PTR register"]
pub mod seq_globaltable_ptr;
