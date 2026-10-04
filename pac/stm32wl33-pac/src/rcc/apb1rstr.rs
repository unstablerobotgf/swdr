#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<Apb1rstrSpec>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<Apb1rstrSpec>;
#[doc = "Field `SPI1RST` reader - SPI1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset for Aux-ADC IP Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset for Aux-ADC IP Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUARTRST` reader - LPUART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type LpuartrstR = crate::BitReader;
#[doc = "Field `LPUARTRST` writer - LPUART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type LpuartrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USARTRST` reader - USART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type UsartrstR = crate::BitReader;
#[doc = "Field `USARTRST` writer - USART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type UsartrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type Spi3rstR = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ADC reset for Aux-ADC IP Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LPUART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn lpuartrst(&self) -> LpuartrstR {
        LpuartrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - USART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn usartrst(&self) -> UsartrstR {
        UsartrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI3 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<'_, Apb1rstrSpec> {
        Spi1rstW::new(self, 0)
    }
    #[doc = "Bit 4 - ADC reset for Aux-ADC IP Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<'_, Apb1rstrSpec> {
        AdcrstW::new(self, 4)
    }
    #[doc = "Bit 8 - LPUART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn lpuartrst(&mut self) -> LpuartrstW<'_, Apb1rstrSpec> {
        LpuartrstW::new(self, 8)
    }
    #[doc = "Bit 10 - USART reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn usartrst(&mut self) -> UsartrstW<'_, Apb1rstrSpec> {
        UsartrstW::new(self, 10)
    }
    #[doc = "Bit 14 - SPI3 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> Spi3rstW<'_, Apb1rstrSpec> {
        Spi3rstW::new(self, 14)
    }
    #[doc = "Bit 21 - I2C1 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2c1rstW<'_, Apb1rstrSpec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 23 - I2C2 reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2c2rstW<'_, Apb1rstrSpec> {
        I2c2rstW::new(self, 23)
    }
}
#[doc = "APB1RSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstrSpec;
impl crate::RegisterSpec for Apb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr::R`](R) reader structure"]
impl crate::Readable for Apb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure"]
impl crate::Writable for Apb1rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for Apb1rstrSpec {}
