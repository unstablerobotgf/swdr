#[doc = "Register `I2C_OAR1` reader"]
pub type R = crate::R<I2cOar1Spec>;
#[doc = "Register `I2C_OAR1` writer"]
pub type W = crate::W<I2cOar1Spec>;
#[doc = "Field `OA1` reader - Interface address"]
pub type Oa1R = crate::FieldReader<u16>;
#[doc = "Field `OA1` writer - Interface address"]
pub type Oa1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address."]
pub type Oa1modeR = crate::BitReader;
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address."]
pub type Oa1modeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1EN` reader - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed."]
pub type Oa1enR = crate::BitReader;
#[doc = "Field `OA1EN` writer - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed."]
pub type Oa1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn oa1(&self) -> Oa1R {
        Oa1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn oa1mode(&self) -> Oa1modeR {
        Oa1modeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn oa1en(&self) -> Oa1enR {
        Oa1enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn oa1(&mut self) -> Oa1W<'_, I2cOar1Spec> {
        Oa1W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode - 0: Own address 1 is a 7-bit address. - 1: Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> Oa1modeW<'_, I2cOar1Spec> {
        Oa1modeW::new(self, 10)
    }
    #[doc = "Bit 15 - Own Address 1 enable - 0: Own address 1 disabled. The received slave address OA1 is NACKed. - 1: Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn oa1en(&mut self) -> Oa1enW<'_, I2cOar1Spec> {
        Oa1enW::new(self, 15)
    }
}
#[doc = "I2C_OAR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cOar1Spec;
impl crate::RegisterSpec for I2cOar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_oar1::R`](R) reader structure"]
impl crate::Readable for I2cOar1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_oar1::W`](W) writer structure"]
impl crate::Writable for I2cOar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_OAR1 to value 0"]
impl crate::Resettable for I2cOar1Spec {}
