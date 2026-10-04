#[doc = "Register `SYSCFG_SR1` reader"]
pub type R = crate::R<SyscfgSr1Spec>;
#[doc = "Field `RFIP_BUSY_STATUS` reader - RFIP_BUSY_STATUS: MR_SUBG BUSY status: Software should check that MR_SUBG IP is not busy (or relay on the related interrupt) before to initiate any system clock frequency switch to operate the switching in a safe way. 0: MR_SUBG is not busy. 1: MR_SUBG is busy"]
pub type RfipBusyStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - RFIP_BUSY_STATUS: MR_SUBG BUSY status: Software should check that MR_SUBG IP is not busy (or relay on the related interrupt) before to initiate any system clock frequency switch to operate the switching in a safe way. 0: MR_SUBG is not busy. 1: MR_SUBG is busy"]
    #[inline(always)]
    pub fn rfip_busy_status(&self) -> RfipBusyStatusR {
        RfipBusyStatusR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "SYSCFG_SR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgSr1Spec;
impl crate::RegisterSpec for SyscfgSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_sr1::R`](R) reader structure"]
impl crate::Readable for SyscfgSr1Spec {}
#[doc = "`reset()` method sets SYSCFG_SR1 to value 0"]
impl crate::Resettable for SyscfgSr1Spec {}
