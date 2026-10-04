#[doc = "Register `I2C_OAR2` reader"]
pub type R = crate::R<I2cOar2Spec>;
#[doc = "Register `I2C_OAR2` writer"]
pub type W = crate::W<I2cOar2Spec>;
#[doc = "Field `OA2` reader - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
pub type Oa2R = crate::FieldReader;
#[doc = "Field `OA2` writer - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
pub type Oa2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA2MSK` reader - Own Address 2 masks - 000: No mask - 001: OA2\\[1\\] is masked and dont care. Only OA2\\[7:2\\] are compared. - 010: OA2\\[2:1\\] are masked and dont care. Only OA2\\[7:3\\] are compared. - 011: OA2\\[3:1\\] are masked and dont care. Only OA2\\[7:4\\] are compared. - 100: OA2\\[4:1\\] are masked and dont care. Only OA2\\[7:5\\] are compared. - 101: OA2\\[5:1\\] are masked and dont care. Only OA2\\[7:6\\] are compared. - 110: OA2\\[6:1\\] are masked and dont care. Only OA2\\[7\\] is compared. - 111: OA2\\[7:1\\] are masked and dont care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
pub type Oa2mskR = crate::FieldReader;
#[doc = "Field `OA2MSK` writer - Own Address 2 masks - 000: No mask - 001: OA2\\[1\\] is masked and dont care. Only OA2\\[7:2\\] are compared. - 010: OA2\\[2:1\\] are masked and dont care. Only OA2\\[7:3\\] are compared. - 011: OA2\\[3:1\\] are masked and dont care. Only OA2\\[7:4\\] are compared. - 100: OA2\\[4:1\\] are masked and dont care. Only OA2\\[7:5\\] are compared. - 101: OA2\\[5:1\\] are masked and dont care. Only OA2\\[7:6\\] are compared. - 110: OA2\\[6:1\\] are masked and dont care. Only OA2\\[7\\] is compared. - 111: OA2\\[7:1\\] are masked and dont care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
pub type Oa2mskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Own Address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oa2en {
    #[doc = "0: Own address 2 disabled. The received slave address OA2 is NACKed."]
    B0x0 = 0,
    #[doc = "1: Own address 2 enabled. The received slave address OA2 is ACKed."]
    B0x1 = 1,
}
impl From<Oa2en> for bool {
    #[inline(always)]
    fn from(variant: Oa2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type Oa2enR = crate::BitReader<Oa2en>;
impl Oa2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa2en {
        match self.bits {
            false => Oa2en::B0x0,
            true => Oa2en::B0x1,
        }
    }
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oa2en::B0x0
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oa2en::B0x1
    }
}
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type Oa2enW<'a, REG> = crate::BitWriter<'a, REG, Oa2en>;
impl<'a, REG> Oa2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2en::B0x0)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2en::B0x1)
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&self) -> Oa2R {
        Oa2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks - 000: No mask - 001: OA2\\[1\\] is masked and dont care. Only OA2\\[7:2\\] are compared. - 010: OA2\\[2:1\\] are masked and dont care. Only OA2\\[7:3\\] are compared. - 011: OA2\\[3:1\\] are masked and dont care. Only OA2\\[7:4\\] are compared. - 100: OA2\\[4:1\\] are masked and dont care. Only OA2\\[7:5\\] are compared. - 101: OA2\\[5:1\\] are masked and dont care. Only OA2\\[7:6\\] are compared. - 110: OA2\\[6:1\\] are masked and dont care. Only OA2\\[7\\] is compared. - 111: OA2\\[7:1\\] are masked and dont care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    #[inline(always)]
    pub fn oa2msk(&self) -> Oa2mskR {
        Oa2mskR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> Oa2enR {
        Oa2enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&mut self) -> Oa2W<'_, I2cOar2Spec> {
        Oa2W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks - 000: No mask - 001: OA2\\[1\\] is masked and dont care. Only OA2\\[7:2\\] are compared. - 010: OA2\\[2:1\\] are masked and dont care. Only OA2\\[7:3\\] are compared. - 011: OA2\\[3:1\\] are masked and dont care. Only OA2\\[7:4\\] are compared. - 100: OA2\\[4:1\\] are masked and dont care. Only OA2\\[7:5\\] are compared. - 101: OA2\\[5:1\\] are masked and dont care. Only OA2\\[7:6\\] are compared. - 110: OA2\\[6:1\\] are masked and dont care. Only OA2\\[7\\] is compared. - 111: OA2\\[7:1\\] are masked and dont care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> Oa2mskW<'_, I2cOar2Spec> {
        Oa2mskW::new(self, 8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> Oa2enW<'_, I2cOar2Spec> {
        Oa2enW::new(self, 15)
    }
}
#[doc = "I2C_OAR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cOar2Spec;
impl crate::RegisterSpec for I2cOar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_oar2::R`](R) reader structure"]
impl crate::Readable for I2cOar2Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_oar2::W`](W) writer structure"]
impl crate::Writable for I2cOar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_OAR2 to value 0"]
impl crate::Resettable for I2cOar2Spec {}
