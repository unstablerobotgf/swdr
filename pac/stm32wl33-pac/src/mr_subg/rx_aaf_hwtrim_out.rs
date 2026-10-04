#[doc = "Register `RX_AAF_HWTRIM_OUT` reader"]
pub type R = crate::R<RxAafHwtrimOutSpec>;
#[doc = "Field `AAF_HW_FCTRIM` reader - AAF calibration information loaded by HW."]
pub type AafHwFctrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AAF calibration information loaded by HW."]
    #[inline(always)]
    pub fn aaf_hw_fctrim(&self) -> AafHwFctrimR {
        AafHwFctrimR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RX_AAF_HWTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_aaf_hwtrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxAafHwtrimOutSpec;
impl crate::RegisterSpec for RxAafHwtrimOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_aaf_hwtrim_out::R`](R) reader structure"]
impl crate::Readable for RxAafHwtrimOutSpec {}
#[doc = "`reset()` method sets RX_AAF_HWTRIM_OUT to value 0x06"]
impl crate::Resettable for RxAafHwtrimOutSpec {
    const RESET_VALUE: u32 = 0x06;
}
