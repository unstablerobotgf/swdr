#[doc = "Register `I2C_PEC` reader"]
pub type R = crate::R<I2cPecSpec>;
#[doc = "Field `PEC` reader - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
pub type PecR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C_PEC register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_pec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cPecSpec;
impl crate::RegisterSpec for I2cPecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_pec::R`](R) reader structure"]
impl crate::Readable for I2cPecSpec {}
#[doc = "`reset()` method sets I2C_PEC to value 0"]
impl crate::Resettable for I2cPecSpec {}
