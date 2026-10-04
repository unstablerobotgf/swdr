#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rfip_version: RfipVersion,
    irq_enable: IrqEnable,
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x00 - RFIP_VERSION register"]
    #[inline(always)]
    pub const fn rfip_version(&self) -> &RfipVersion {
        &self.rfip_version
    }
    #[doc = "0x04 - IRQ_ENABLE register"]
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IrqEnable {
        &self.irq_enable
    }
    #[doc = "0x08 - STATUS register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "RFIP_VERSION (r) register accessor: RFIP_VERSION register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfip_version`] module"]
#[doc(alias = "RFIP_VERSION")]
pub type RfipVersion = crate::Reg<rfip_version::RfipVersionSpec>;
#[doc = "RFIP_VERSION register"]
pub mod rfip_version;
#[doc = "IRQ_ENABLE (rw) register accessor: IRQ_ENABLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`] module"]
#[doc(alias = "IRQ_ENABLE")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
#[doc = "IRQ_ENABLE register"]
pub mod irq_enable;
#[doc = "STATUS (rw) register accessor: STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "STATUS register"]
pub mod status;
