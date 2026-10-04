#[doc = "Register `SPI2S_I2SPR` reader"]
pub type R = crate::R<Spi2sI2sprSpec>;
#[doc = "Register `SPI2S_I2SPR` writer"]
pub type W = crate::W<Spi2sI2sprSpec>;
#[doc = "Field `I2SDIV` reader - I2S linear prescaler I2SDIV \\[7:0\\] = 0 or I2SDIV \\[7:0\\] = 1 are forbidden values."]
pub type I2sdivR = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler I2SDIV \\[7:0\\] = 0 or I2SDIV \\[7:0\\] = 1 are forbidden values."]
pub type I2sdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODD` reader - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1"]
pub type OddR = crate::BitReader;
#[doc = "Field `ODD` writer - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1"]
pub type OddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled"]
pub type MckoeR = crate::BitReader;
#[doc = "Field `MCKOE` writer - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled"]
pub type MckoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - I2S linear prescaler I2SDIV \\[7:0\\] = 0 or I2SDIV \\[7:0\\] = 1 are forbidden values."]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2sdivR {
        I2sdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1"]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled"]
    #[inline(always)]
    pub fn mckoe(&self) -> MckoeR {
        MckoeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S linear prescaler I2SDIV \\[7:0\\] = 0 or I2SDIV \\[7:0\\] = 1 are forbidden values."]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2sdivW<'_, Spi2sI2sprSpec> {
        I2sdivW::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1"]
    #[inline(always)]
    pub fn odd(&mut self) -> OddW<'_, Spi2sI2sprSpec> {
        OddW::new(self, 8)
    }
    #[doc = "Bit 9 - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MckoeW<'_, Spi2sI2sprSpec> {
        MckoeW::new(self, 9)
    }
}
#[doc = "SPI2S_I2SPR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2s_i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi2sI2sprSpec;
impl crate::RegisterSpec for Spi2sI2sprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi2s_i2spr::R`](R) reader structure"]
impl crate::Readable for Spi2sI2sprSpec {}
#[doc = "`write(|w| ..)` method takes [`spi2s_i2spr::W`](W) writer structure"]
impl crate::Writable for Spi2sI2sprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI2S_I2SPR to value 0x02"]
impl crate::Resettable for Spi2sI2sprSpec {
    const RESET_VALUE: u32 = 0x02;
}
