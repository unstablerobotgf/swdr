#[doc = "Register `ABSOLUTE_TIME` reader"]
pub type R = crate::R<AbsoluteTimeSpec>;
#[doc = "Field `ABSOLUTE_TIME` reader - Indicate the interpolated absolute."]
pub type AbsoluteTimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Indicate the interpolated absolute."]
    #[inline(always)]
    pub fn absolute_time(&self) -> AbsoluteTimeR {
        AbsoluteTimeR::new(self.bits)
    }
}
#[doc = "ABSOLUTE_TIME register\n\nYou can [`read`](crate::Reg::read) this register and get [`absolute_time::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbsoluteTimeSpec;
impl crate::RegisterSpec for AbsoluteTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`absolute_time::R`](R) reader structure"]
impl crate::Readable for AbsoluteTimeSpec {}
#[doc = "`reset()` method sets ABSOLUTE_TIME to value 0"]
impl crate::Resettable for AbsoluteTimeSpec {}
