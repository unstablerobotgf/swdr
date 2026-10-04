#[doc = "Register `PA_HWTRIM_OUT` reader"]
pub type R = crate::R<PaHwtrimOutSpec>;
#[doc = "Field `PA_HW_DEGEN_TRIM` reader - MSB part meaning:"]
pub type PaHwDegenTrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - MSB part meaning:"]
    #[inline(always)]
    pub fn pa_hw_degen_trim(&self) -> PaHwDegenTrimR {
        PaHwDegenTrimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "PA_HWTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_hwtrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaHwtrimOutSpec;
impl crate::RegisterSpec for PaHwtrimOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_hwtrim_out::R`](R) reader structure"]
impl crate::Readable for PaHwtrimOutSpec {}
#[doc = "`reset()` method sets PA_HWTRIM_OUT to value 0x88"]
impl crate::Resettable for PaHwtrimOutSpec {
    const RESET_VALUE: u32 = 0x88;
}
