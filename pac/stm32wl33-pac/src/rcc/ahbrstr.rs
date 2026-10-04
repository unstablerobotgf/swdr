#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `DMARST` reader - DMA and DMAMUX reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type DmarstR = crate::BitReader;
#[doc = "Field `DMARST` writer - DMA and DMAMUX reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type DmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOARST` reader - GPIOA reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - GPIOA reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - GPIOB reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - GPIOB reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGRST` reader - RNG reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type RngrstR = crate::BitReader;
#[doc = "Field `RNGRST` writer - RNG reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type RngrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESRST` reader - AES reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type AesrstR = crate::BitReader;
#[doc = "Field `AESRST` writer - AES reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type AesrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA and DMAMUX reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn dmarst(&self) -> DmarstR {
        DmarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOA reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOB reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn rngrst(&self) -> RngrstR {
        RngrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - AES reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn aesrst(&self) -> AesrstR {
        AesrstR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA and DMAMUX reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn dmarst(&mut self) -> DmarstW<'_, AhbrstrSpec> {
        DmarstW::new(self, 0)
    }
    #[doc = "Bit 2 - GPIOA reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GpioarstW<'_, AhbrstrSpec> {
        GpioarstW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOB reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GpiobrstW<'_, AhbrstrSpec> {
        GpiobrstW::new(self, 3)
    }
    #[doc = "Bit 12 - CRC reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<'_, AhbrstrSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 18 - RNG reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RngrstW<'_, AhbrstrSpec> {
        RngrstW::new(self, 18)
    }
    #[doc = "Bit 20 - AES reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn aesrst(&mut self) -> AesrstW<'_, AhbrstrSpec> {
        AesrstW::new(self, 20)
    }
}
#[doc = "AHBRSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstrSpec;
impl crate::RegisterSpec for AhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AhbrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AhbrstrSpec {}
