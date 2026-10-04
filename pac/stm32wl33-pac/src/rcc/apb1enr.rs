#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<Apb1enrSpec>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<Apb1enrSpec>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDIGEN` reader - AUXADC clock enable for Aux-ADC digital clock Set and enable by software. 0: clock disable 1: clock enable"]
pub type AdcdigenR = crate::BitReader;
#[doc = "Field `ADCDIGEN` writer - AUXADC clock enable for Aux-ADC digital clock Set and enable by software. 0: clock disable 1: clock enable"]
pub type AdcdigenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCANAEN` reader - ADC clock enable for Aux-ADC analog clock Set and enable by software. 0: clock disable 1: clock enable"]
pub type AdcanaenR = crate::BitReader;
#[doc = "Field `ADCANAEN` writer - ADC clock enable for Aux-ADC analog clock Set and enable by software. 0: clock disable 1: clock enable"]
pub type AdcanaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUARTEN` reader - LPUART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type LpuartenR = crate::BitReader;
#[doc = "Field `LPUARTEN` writer - LPUART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type LpuartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USARTEN` reader - USART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type UsartenR = crate::BitReader;
#[doc = "Field `USARTEN` writer - USART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type UsartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Spi3enR = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - AUXADC clock enable for Aux-ADC digital clock Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn adcdigen(&self) -> AdcdigenR {
        AdcdigenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC clock enable for Aux-ADC analog clock Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn adcanaen(&self) -> AdcanaenR {
        AdcanaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - LPUART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lpuarten(&self) -> LpuartenR {
        LpuartenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - USART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn usarten(&self) -> UsartenR {
        UsartenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI3 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<'_, Apb1enrSpec> {
        Spi1enW::new(self, 0)
    }
    #[doc = "Bit 4 - AUXADC clock enable for Aux-ADC digital clock Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn adcdigen(&mut self) -> AdcdigenW<'_, Apb1enrSpec> {
        AdcdigenW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC clock enable for Aux-ADC analog clock Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn adcanaen(&mut self) -> AdcanaenW<'_, Apb1enrSpec> {
        AdcanaenW::new(self, 5)
    }
    #[doc = "Bit 8 - LPUART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lpuarten(&mut self) -> LpuartenW<'_, Apb1enrSpec> {
        LpuartenW::new(self, 8)
    }
    #[doc = "Bit 10 - USART clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn usarten(&mut self) -> UsartenW<'_, Apb1enrSpec> {
        UsartenW::new(self, 10)
    }
    #[doc = "Bit 14 - SPI3 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> Spi3enW<'_, Apb1enrSpec> {
        Spi3enW::new(self, 14)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<'_, Apb1enrSpec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 23 - I2C2 clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<'_, Apb1enrSpec> {
        I2c2enW::new(self, 23)
    }
}
#[doc = "APB1ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enrSpec;
impl crate::RegisterSpec for Apb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr::R`](R) reader structure"]
impl crate::Readable for Apb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure"]
impl crate::Writable for Apb1enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for Apb1enrSpec {}
