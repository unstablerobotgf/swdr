#[doc = "Register `TIME_CAPTURE` reader"]
pub type R = crate::R<TimeCaptureSpec>;
#[doc = "Field `TIME_CAPTURE` reader - Interpolated absolute time value captured on specific programmable event through TIME_CAPTURESEL\\[2:0\\] bit field."]
pub type TimeCaptureR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interpolated absolute time value captured on specific programmable event through TIME_CAPTURESEL\\[2:0\\] bit field."]
    #[inline(always)]
    pub fn time_capture(&self) -> TimeCaptureR {
        TimeCaptureR::new(self.bits)
    }
}
#[doc = "TIME_CAPTURE register\n\nYou can [`read`](crate::Reg::read) this register and get [`time_capture::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeCaptureSpec;
impl crate::RegisterSpec for TimeCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_capture::R`](R) reader structure"]
impl crate::Readable for TimeCaptureSpec {}
#[doc = "`reset()` method sets TIME_CAPTURE to value 0"]
impl crate::Resettable for TimeCaptureSpec {}
