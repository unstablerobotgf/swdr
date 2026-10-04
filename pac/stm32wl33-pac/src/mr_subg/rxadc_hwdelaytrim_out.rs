#[doc = "Register `RXADC_HWDELAYTRIM_OUT` reader"]
pub type R = crate::R<RxadcHwdelaytrimOutSpec>;
#[doc = "Field `RXADC_HW_DELAYTRIM_I` reader - Control bits of the RX ADC loop delay for I channel (from SoC Flash)."]
pub type RxadcHwDelaytrimIR = crate::FieldReader;
#[doc = "Field `RXADC_HW_DELAYTRIM_Q` reader - Control bits of the RX ADC loop delay for Q channel (from SoC Flash)."]
pub type RxadcHwDelaytrimQR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Control bits of the RX ADC loop delay for I channel (from SoC Flash)."]
    #[inline(always)]
    pub fn rxadc_hw_delaytrim_i(&self) -> RxadcHwDelaytrimIR {
        RxadcHwDelaytrimIR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Control bits of the RX ADC loop delay for Q channel (from SoC Flash)."]
    #[inline(always)]
    pub fn rxadc_hw_delaytrim_q(&self) -> RxadcHwDelaytrimQR {
        RxadcHwDelaytrimQR::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "RXADC_HWDELAYTRIM_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxadc_hwdelaytrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxadcHwdelaytrimOutSpec;
impl crate::RegisterSpec for RxadcHwdelaytrimOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxadc_hwdelaytrim_out::R`](R) reader structure"]
impl crate::Readable for RxadcHwdelaytrimOutSpec {}
#[doc = "`reset()` method sets RXADC_HWDELAYTRIM_OUT to value 0x1b"]
impl crate::Resettable for RxadcHwdelaytrimOutSpec {
    const RESET_VALUE: u32 = 0x1b;
}
