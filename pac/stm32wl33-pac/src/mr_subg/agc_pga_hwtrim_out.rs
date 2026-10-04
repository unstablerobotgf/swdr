#[doc = "Register `AGC_PGA_HWTRIM_OUT` reader"]
pub type R = crate::R<AgcPgaHwtrimOutSpec>;
#[doc = "Field `AGC_HW_PGA_TRIM` reader - AGC PGA calibration information loaded by HW from the SoC flash."]
pub type AgcHwPgaTrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AGC PGA calibration information loaded by HW from the SoC flash."]
    #[inline(always)]
    pub fn agc_hw_pga_trim(&self) -> AgcHwPgaTrimR {
        AgcHwPgaTrimR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "AGC_PGA_HWTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_pga_hwtrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcPgaHwtrimOutSpec;
impl crate::RegisterSpec for AgcPgaHwtrimOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_pga_hwtrim_out::R`](R) reader structure"]
impl crate::Readable for AgcPgaHwtrimOutSpec {}
#[doc = "`reset()` method sets AGC_PGA_HWTRIM_OUT to value 0x08"]
impl crate::Resettable for AgcPgaHwtrimOutSpec {
    const RESET_VALUE: u32 = 0x08;
}
