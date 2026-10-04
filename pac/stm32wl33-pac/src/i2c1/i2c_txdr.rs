#[doc = "Register `I2C_TXDR` reader"]
pub type R = crate::R<I2cTxdrSpec>;
#[doc = "Register `I2C_TXDR` writer"]
pub type W = crate::W<I2cTxdrSpec>;
#[doc = "Field `TXDATA` reader - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1."]
pub type TxdataR = crate::FieldReader;
#[doc = "Field `TXDATA` writer - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1."]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Eight bits (8-bit) transmit data Data byte to be transmitted to the I2C bus. Note: These bits can be written only when TXE=1."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<'_, I2cTxdrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "I2C_TXDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cTxdrSpec;
impl crate::RegisterSpec for I2cTxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_txdr::R`](R) reader structure"]
impl crate::Readable for I2cTxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_txdr::W`](W) writer structure"]
impl crate::Writable for I2cTxdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_TXDR to value 0"]
impl crate::Resettable for I2cTxdrSpec {}
