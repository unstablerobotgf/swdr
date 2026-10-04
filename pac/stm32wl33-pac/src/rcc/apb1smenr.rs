#[doc = "Register `APB1SMENR` reader"]
pub type R = crate::R<Apb1smenrSpec>;
#[doc = "Register `APB1SMENR` writer"]
pub type W = crate::W<Apb1smenrSpec>;
#[doc = "Field `SPI1SMEN` reader - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)"]
pub type Spi1smenR = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)"]
pub type Spi1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDIGSMEN` reader - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)"]
pub type AdcdigsmenR = crate::BitReader;
#[doc = "Field `ADCDIGSMEN` writer - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)"]
pub type AdcdigsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUARTSMEN` reader - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)"]
pub type LpuartsmenR = crate::BitReader;
#[doc = "Field `LPUARTSMEN` writer - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)"]
pub type LpuartsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USARTSMEN` reader - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)"]
pub type UsartsmenR = crate::BitReader;
#[doc = "Field `USARTSMEN` writer - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)"]
pub type UsartsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3SMEN` reader - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)"]
pub type Spi3smenR = crate::BitReader;
#[doc = "Field `SPI3SMEN` writer - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)"]
pub type Spi3smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)"]
pub type I2c1smenR = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)"]
pub type I2c1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)"]
pub type I2c2smenR = crate::BitReader;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)"]
pub type I2c2smenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)"]
    #[inline(always)]
    pub fn spi1smen(&self) -> Spi1smenR {
        Spi1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)"]
    #[inline(always)]
    pub fn adcdigsmen(&self) -> AdcdigsmenR {
        AdcdigsmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)"]
    #[inline(always)]
    pub fn lpuartsmen(&self) -> LpuartsmenR {
        LpuartsmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)"]
    #[inline(always)]
    pub fn usartsmen(&self) -> UsartsmenR {
        UsartsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)"]
    #[inline(always)]
    pub fn spi3smen(&self) -> Spi3smenR {
        Spi3smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2c1smenR {
        I2c1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2c2smenR {
        I2c2smenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> Spi1smenW<'_, Apb1smenrSpec> {
        Spi1smenW::new(self, 0)
    }
    #[doc = "Bit 4 - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)"]
    #[inline(always)]
    pub fn adcdigsmen(&mut self) -> AdcdigsmenW<'_, Apb1smenrSpec> {
        AdcdigsmenW::new(self, 4)
    }
    #[doc = "Bit 8 - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)"]
    #[inline(always)]
    pub fn lpuartsmen(&mut self) -> LpuartsmenW<'_, Apb1smenrSpec> {
        LpuartsmenW::new(self, 8)
    }
    #[doc = "Bit 10 - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)"]
    #[inline(always)]
    pub fn usartsmen(&mut self) -> UsartsmenW<'_, Apb1smenrSpec> {
        UsartsmenW::new(self, 10)
    }
    #[doc = "Bit 14 - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)"]
    #[inline(always)]
    pub fn spi3smen(&mut self) -> Spi3smenW<'_, Apb1smenrSpec> {
        Spi3smenW::new(self, 14)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)"]
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2c1smenW<'_, Apb1smenrSpec> {
        I2c1smenW::new(self, 21)
    }
    #[doc = "Bit 23 - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)"]
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2c2smenW<'_, Apb1smenrSpec> {
        I2c2smenW::new(self, 23)
    }
}
#[doc = "APB1SMENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1smenrSpec;
impl crate::RegisterSpec for Apb1smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr::R`](R) reader structure"]
impl crate::Readable for Apb1smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr::W`](W) writer structure"]
impl crate::Writable for Apb1smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1SMENR to value 0x00a0_4511"]
impl crate::Resettable for Apb1smenrSpec {
    const RESET_VALUE: u32 = 0x00a0_4511;
}
