#[doc = "Register `RTC_TSSSR` reader"]
pub type R = crate::R<RtcTsssrSpec>;
#[doc = "Field `SS` reader - Sub second value SS\\[15:0\\] is the value of the synchronous prescalers counter when the timestamp event occurred."]
pub type SsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value SS\\[15:0\\] is the value of the synchronous prescalers counter when the timestamp event occurred."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC_TSSSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTsssrSpec;
impl crate::RegisterSpec for RtcTsssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tsssr::R`](R) reader structure"]
impl crate::Readable for RtcTsssrSpec {}
#[doc = "`reset()` method sets RTC_TSSSR to value 0"]
impl crate::Resettable for RtcTsssrSpec {}
