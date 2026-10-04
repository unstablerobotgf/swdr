#[doc = "Register `I2C_RXDR` reader"]
pub type R = crate::R<I2cRxdrSpec>;
#[doc = "Field `RXDATA` reader - Eight bit (8-bit) receive data Data byte received from the I2C bus."]
pub type RxdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Eight bit (8-bit) receive data Data byte received from the I2C bus."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C_RXDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cRxdrSpec;
impl crate::RegisterSpec for I2cRxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_rxdr::R`](R) reader structure"]
impl crate::Readable for I2cRxdrSpec {}
#[doc = "`reset()` method sets I2C_RXDR to value 0"]
impl crate::Resettable for I2cRxdrSpec {}
