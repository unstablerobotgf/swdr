#[doc = "Register `RTC_SSR` reader"]
pub type R = crate::R<RtcSsrSpec>;
#[doc = "Field `SS` reader - Sub second value SS\\[15:0\\] is the value in the synchronous prescalers counter. The fraction of a second is given by the formula below: Second fraction = ( PREDIV_S - SS ) / ( PREDIV_S + 1 )"]
pub type SsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value SS\\[15:0\\] is the value in the synchronous prescalers counter. The fraction of a second is given by the formula below: Second fraction = ( PREDIV_S - SS ) / ( PREDIV_S + 1 )"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC_SSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcSsrSpec;
impl crate::RegisterSpec for RtcSsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_ssr::R`](R) reader structure"]
impl crate::Readable for RtcSsrSpec {}
#[doc = "`reset()` method sets RTC_SSR to value 0"]
impl crate::Resettable for RtcSsrSpec {}
