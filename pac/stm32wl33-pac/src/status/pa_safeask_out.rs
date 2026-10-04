#[doc = "Register `PA_SAFEASK_OUT` reader"]
pub type R = crate::R<PaSafeaskOutSpec>;
#[doc = "Field `PA_CODEMAX` reader - Safe ASK level (provided after a CALIB_SAFEASK command), indicating the maximum PA Power to program before reaching ohmic saturation."]
pub type PaCodemaxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Safe ASK level (provided after a CALIB_SAFEASK command), indicating the maximum PA Power to program before reaching ohmic saturation."]
    #[inline(always)]
    pub fn pa_codemax(&self) -> PaCodemaxR {
        PaCodemaxR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PA_SAFEASK_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_safeask_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaSafeaskOutSpec;
impl crate::RegisterSpec for PaSafeaskOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_safeask_out::R`](R) reader structure"]
impl crate::Readable for PaSafeaskOutSpec {}
#[doc = "`reset()` method sets PA_SAFEASK_OUT to value 0"]
impl crate::Resettable for PaSafeaskOutSpec {}
