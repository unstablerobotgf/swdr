#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rfip_version: RfipVersion,
    rrm_udra_ctrl: RrmUdraCtrl,
    sequencer_ctrl: SequencerCtrl,
    absolute_time: AbsoluteTime,
    scm_counter_val: ScmCounterVal,
    scm_min_max: ScmMinMax,
    wakeup_irq_status: WakeupIrqStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - RFIP_VERSION register"]
    #[inline(always)]
    pub const fn rfip_version(&self) -> &RfipVersion {
        &self.rfip_version
    }
    #[doc = "0x04 - RRM_UDRA_CTRL register"]
    #[inline(always)]
    pub const fn rrm_udra_ctrl(&self) -> &RrmUdraCtrl {
        &self.rrm_udra_ctrl
    }
    #[doc = "0x08 - SEQUENCER_CTRL register"]
    #[inline(always)]
    pub const fn sequencer_ctrl(&self) -> &SequencerCtrl {
        &self.sequencer_ctrl
    }
    #[doc = "0x0c - ABSOLUTE_TIME register"]
    #[inline(always)]
    pub const fn absolute_time(&self) -> &AbsoluteTime {
        &self.absolute_time
    }
    #[doc = "0x10 - SCM_COUNTER_VAL register"]
    #[inline(always)]
    pub const fn scm_counter_val(&self) -> &ScmCounterVal {
        &self.scm_counter_val
    }
    #[doc = "0x14 - SCM_MIN_MAX register"]
    #[inline(always)]
    pub const fn scm_min_max(&self) -> &ScmMinMax {
        &self.scm_min_max
    }
    #[doc = "0x18 - WAKEUP_IRQ_STATUS register"]
    #[inline(always)]
    pub const fn wakeup_irq_status(&self) -> &WakeupIrqStatus {
        &self.wakeup_irq_status
    }
}
#[doc = "RFIP_VERSION (r) register accessor: RFIP_VERSION register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfip_version`] module"]
#[doc(alias = "RFIP_VERSION")]
pub type RfipVersion = crate::Reg<rfip_version::RfipVersionSpec>;
#[doc = "RFIP_VERSION register"]
pub mod rfip_version;
#[doc = "RRM_UDRA_CTRL (w) register accessor: RRM_UDRA_CTRL register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_udra_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrm_udra_ctrl`] module"]
#[doc(alias = "RRM_UDRA_CTRL")]
pub type RrmUdraCtrl = crate::Reg<rrm_udra_ctrl::RrmUdraCtrlSpec>;
#[doc = "RRM_UDRA_CTRL register"]
pub mod rrm_udra_ctrl;
#[doc = "SEQUENCER_CTRL (rw) register accessor: SEQUENCER_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`sequencer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sequencer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sequencer_ctrl`] module"]
#[doc(alias = "SEQUENCER_CTRL")]
pub type SequencerCtrl = crate::Reg<sequencer_ctrl::SequencerCtrlSpec>;
#[doc = "SEQUENCER_CTRL register"]
pub mod sequencer_ctrl;
#[doc = "ABSOLUTE_TIME (r) register accessor: ABSOLUTE_TIME register\n\nYou can [`read`](crate::Reg::read) this register and get [`absolute_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@absolute_time`] module"]
#[doc(alias = "ABSOLUTE_TIME")]
pub type AbsoluteTime = crate::Reg<absolute_time::AbsoluteTimeSpec>;
#[doc = "ABSOLUTE_TIME register"]
pub mod absolute_time;
#[doc = "SCM_COUNTER_VAL (r) register accessor: SCM_COUNTER_VAL register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm_counter_val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm_counter_val`] module"]
#[doc(alias = "SCM_COUNTER_VAL")]
pub type ScmCounterVal = crate::Reg<scm_counter_val::ScmCounterValSpec>;
#[doc = "SCM_COUNTER_VAL register"]
pub mod scm_counter_val;
#[doc = "SCM_MIN_MAX (rw) register accessor: SCM_MIN_MAX register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm_min_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm_min_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm_min_max`] module"]
#[doc(alias = "SCM_MIN_MAX")]
pub type ScmMinMax = crate::Reg<scm_min_max::ScmMinMaxSpec>;
#[doc = "SCM_MIN_MAX register"]
pub mod scm_min_max;
#[doc = "WAKEUP_IRQ_STATUS (rw) register accessor: WAKEUP_IRQ_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_irq_status`] module"]
#[doc(alias = "WAKEUP_IRQ_STATUS")]
pub type WakeupIrqStatus = crate::Reg<wakeup_irq_status::WakeupIrqStatusSpec>;
#[doc = "WAKEUP_IRQ_STATUS register"]
pub mod wakeup_irq_status;
