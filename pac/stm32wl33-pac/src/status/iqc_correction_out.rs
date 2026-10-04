#[doc = "Register `IQC_CORRECTION_OUT` reader"]
pub type R = crate::R<IqcCorrectionOutSpec>;
#[doc = "Field `IQC_CORRECT_OUT` reader - Final correction value output from IQC (compensation engine)."]
pub type IqcCorrectOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Final correction value output from IQC (compensation engine)."]
    #[inline(always)]
    pub fn iqc_correct_out(&self) -> IqcCorrectOutR {
        IqcCorrectOutR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "IQC_CORRECTION_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_correction_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqcCorrectionOutSpec;
impl crate::RegisterSpec for IqcCorrectionOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqc_correction_out::R`](R) reader structure"]
impl crate::Readable for IqcCorrectionOutSpec {}
#[doc = "`reset()` method sets IQC_CORRECTION_OUT to value 0"]
impl crate::Resettable for IqcCorrectionOutSpec {}
