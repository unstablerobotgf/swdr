#[doc = "Register `RFIP_WAKEUPTIME` reader"]
pub type R = crate::R<RfipWakeuptimeSpec>;
#[doc = "Field `RFIP_WAKEUPTIME` reader - (Absolute) Target time to wakeup the RFIP."]
pub type RfipWakeuptimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - (Absolute) Target time to wakeup the RFIP."]
    #[inline(always)]
    pub fn rfip_wakeuptime(&self) -> RfipWakeuptimeR {
        RfipWakeuptimeR::new(self.bits)
    }
}
#[doc = "RFIP_WAKEUPTIME register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_wakeuptime::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfipWakeuptimeSpec;
impl crate::RegisterSpec for RfipWakeuptimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfip_wakeuptime::R`](R) reader structure"]
impl crate::Readable for RfipWakeuptimeSpec {}
#[doc = "`reset()` method sets RFIP_WAKEUPTIME to value 0"]
impl crate::Resettable for RfipWakeuptimeSpec {}
