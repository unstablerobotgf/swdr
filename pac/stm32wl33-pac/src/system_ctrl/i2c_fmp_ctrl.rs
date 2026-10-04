#[doc = "Register `I2C_FMP_CTRL` reader"]
pub type R = crate::R<I2cFmpCtrlSpec>;
#[doc = "Register `I2C_FMP_CTRL` writer"]
pub type W = crate::W<I2cFmpCtrlSpec>;
#[doc = "Field `I2C1_PA0_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed"]
pub type I2c1Pa0FmpR = crate::BitReader;
#[doc = "Field `I2C1_PA0_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed"]
pub type I2c1Pa0FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_PA1_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed"]
pub type I2c1Pa1FmpR = crate::BitReader;
#[doc = "Field `I2C1_PA1_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed"]
pub type I2c1Pa1FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_PB6_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed."]
pub type I2c1Pb6FmpR = crate::BitReader;
#[doc = "Field `I2C1_PB6_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed."]
pub type I2c1Pb6FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_PB7_FMP` reader - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed"]
pub type I2c1Pb7FmpR = crate::BitReader;
#[doc = "Field `I2C1_PB7_FMP` writer - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed"]
pub type I2c1Pb7FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_PB10_FMP` reader - I2C1_PB10_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB10 I/O. 0: PB10 pin operated in standard mode. 1: FM+ mode is enabled on PB10 pin, and speed control is bypassed."]
pub type I2c1Pb10FmpR = crate::BitReader;
#[doc = "Field `I2C1_PB10_FMP` writer - I2C1_PB10_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB10 I/O. 0: PB10 pin operated in standard mode. 1: FM+ mode is enabled on PB10 pin, and speed control is bypassed."]
pub type I2c1Pb10FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_PB11_FMP` reader - I2C1_PB11_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB11 I/O. 0: PB11 pin operated in standard mode. 1: FM+ mode is enabled on PB11 pin, and speed control is bypassed"]
pub type I2c1Pb11FmpR = crate::BitReader;
#[doc = "Field `I2C1_PB11_FMP` writer - I2C1_PB11_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB11 I/O. 0: PB11 pin operated in standard mode. 1: FM+ mode is enabled on PB11 pin, and speed control is bypassed"]
pub type I2c1Pb11FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_PA6_FMP` reader - I2C2_PA6_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA6 I/O. 0: PA6 pin operated in standard mode. 1: FM+ mode is enabled on PA6 pin, and speed control is bypassed."]
pub type I2c2Pa6FmpR = crate::BitReader;
#[doc = "Field `I2C2_PA6_FMP` writer - I2C2_PA6_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA6 I/O. 0: PA6 pin operated in standard mode. 1: FM+ mode is enabled on PA6 pin, and speed control is bypassed."]
pub type I2c2Pa6FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_PA7_FMP` reader - I2C2_PA7_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA7 I/O. 0: PA7 pin operated in standard mode. 1: FM+ mode is enabled on PA7 pin, and speed control is bypassed"]
pub type I2c2Pa7FmpR = crate::BitReader;
#[doc = "Field `I2C2_PA7_FMP` writer - I2C2_PA7_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA7 I/O. 0: PA7 pin operated in standard mode. 1: FM+ mode is enabled on PA7 pin, and speed control is bypassed"]
pub type I2c2Pa7FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_PA13_FMP` reader - I2C2_PA13_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA13 I/O. 0: PA13 pin operated in standard mode. 1: FM+ mode is enabled on PA13 pin, and speed control is bypassed."]
pub type I2c2Pa13FmpR = crate::BitReader;
#[doc = "Field `I2C2_PA13_FMP` writer - I2C2_PA13_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA13 I/O. 0: PA13 pin operated in standard mode. 1: FM+ mode is enabled on PA13 pin, and speed control is bypassed."]
pub type I2c2Pa13FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_PA14_FMP` reader - I2C2_PA14_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA14 I/O. 0: PA14 pin operated in standard mode. 1: FM+ mode is enabled on PA14 pin, and speed control is bypassed."]
pub type I2c2Pa14FmpR = crate::BitReader;
#[doc = "Field `I2C2_PA14_FMP` writer - I2C2_PA14_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA14 I/O. 0: PA14 pin operated in standard mode. 1: FM+ mode is enabled on PA14 pin, and speed control is bypassed."]
pub type I2c2Pa14FmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pa0_fmp(&self) -> I2c1Pa0FmpR {
        I2c1Pa0FmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pa1_fmp(&self) -> I2c1Pa1FmpR {
        I2c1Pa1FmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c1_pb6_fmp(&self) -> I2c1Pb6FmpR {
        I2c1Pb6FmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pb7_fmp(&self) -> I2c1Pb7FmpR {
        I2c1Pb7FmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C1_PB10_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB10 I/O. 0: PB10 pin operated in standard mode. 1: FM+ mode is enabled on PB10 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c1_pb10_fmp(&self) -> I2c1Pb10FmpR {
        I2c1Pb10FmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C1_PB11_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB11 I/O. 0: PB11 pin operated in standard mode. 1: FM+ mode is enabled on PB11 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pb11_fmp(&self) -> I2c1Pb11FmpR {
        I2c1Pb11FmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C2_PA6_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA6 I/O. 0: PA6 pin operated in standard mode. 1: FM+ mode is enabled on PA6 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c2_pa6_fmp(&self) -> I2c2Pa6FmpR {
        I2c2Pa6FmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C2_PA7_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA7 I/O. 0: PA7 pin operated in standard mode. 1: FM+ mode is enabled on PA7 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c2_pa7_fmp(&self) -> I2c2Pa7FmpR {
        I2c2Pa7FmpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C2_PA13_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA13 I/O. 0: PA13 pin operated in standard mode. 1: FM+ mode is enabled on PA13 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c2_pa13_fmp(&self) -> I2c2Pa13FmpR {
        I2c2Pa13FmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C2_PA14_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA14 I/O. 0: PA14 pin operated in standard mode. 1: FM+ mode is enabled on PA14 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c2_pa14_fmp(&self) -> I2c2Pa14FmpR {
        I2c2Pa14FmpR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pa0_fmp(&mut self) -> I2c1Pa0FmpW<'_, I2cFmpCtrlSpec> {
        I2c1Pa0FmpW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pa1_fmp(&mut self) -> I2c1Pa1FmpW<'_, I2cFmpCtrlSpec> {
        I2c1Pa1FmpW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c1_pb6_fmp(&mut self) -> I2c1Pb6FmpW<'_, I2cFmpCtrlSpec> {
        I2c1Pb6FmpW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pb7_fmp(&mut self) -> I2c1Pb7FmpW<'_, I2cFmpCtrlSpec> {
        I2c1Pb7FmpW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C1_PB10_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB10 I/O. 0: PB10 pin operated in standard mode. 1: FM+ mode is enabled on PB10 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c1_pb10_fmp(&mut self) -> I2c1Pb10FmpW<'_, I2cFmpCtrlSpec> {
        I2c1Pb10FmpW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C1_PB11_FMP: I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB11 I/O. 0: PB11 pin operated in standard mode. 1: FM+ mode is enabled on PB11 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c1_pb11_fmp(&mut self) -> I2c1Pb11FmpW<'_, I2cFmpCtrlSpec> {
        I2c1Pb11FmpW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C2_PA6_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA6 I/O. 0: PA6 pin operated in standard mode. 1: FM+ mode is enabled on PA6 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c2_pa6_fmp(&mut self) -> I2c2Pa6FmpW<'_, I2cFmpCtrlSpec> {
        I2c2Pa6FmpW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C2_PA7_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA7 I/O. 0: PA7 pin operated in standard mode. 1: FM+ mode is enabled on PA7 pin, and speed control is bypassed"]
    #[inline(always)]
    pub fn i2c2_pa7_fmp(&mut self) -> I2c2Pa7FmpW<'_, I2cFmpCtrlSpec> {
        I2c2Pa7FmpW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C2_PA13_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SCL on PA13 I/O. 0: PA13 pin operated in standard mode. 1: FM+ mode is enabled on PA13 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c2_pa13_fmp(&mut self) -> I2c2Pa13FmpW<'_, I2cFmpCtrlSpec> {
        I2c2Pa13FmpW::new(self, 8)
    }
    #[doc = "Bit 9 - I2C2_PA14_FMP: I2C2 Fast-Mode Plus driving capability for I2C2_SDA on PA14 I/O. 0: PA14 pin operated in standard mode. 1: FM+ mode is enabled on PA14 pin, and speed control is bypassed."]
    #[inline(always)]
    pub fn i2c2_pa14_fmp(&mut self) -> I2c2Pa14FmpW<'_, I2cFmpCtrlSpec> {
        I2c2Pa14FmpW::new(self, 9)
    }
}
#[doc = "I2C_FMP_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fmp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fmp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFmpCtrlSpec;
impl crate::RegisterSpec for I2cFmpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_fmp_ctrl::R`](R) reader structure"]
impl crate::Readable for I2cFmpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_fmp_ctrl::W`](W) writer structure"]
impl crate::Writable for I2cFmpCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FMP_CTRL to value 0"]
impl crate::Resettable for I2cFmpCtrlSpec {}
