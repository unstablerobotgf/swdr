#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `DMAEN` reader - DMA and DMAMUX enable Set and enable by software. 0: does not enable 1: enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA and DMAMUX enable Set and enable by software. 0: does not enable 1: enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOAEN` reader - GPIOA enable. It must be enabled by default"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - GPIOA enable. It must be enabled by default"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - GPIOB enable. It must be enabled by default"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - GPIOB enable. It must be enabled by default"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC enable Set and enable by software. 0: does not enable 1: enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC enable Set and enable by software. 0: does not enable 1: enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - RNG clock enable Set and enable by software. 0: does not enable 1: enable"]
pub type RngenR = crate::BitReader;
#[doc = "Field `RNGEN` writer - RNG clock enable Set and enable by software. 0: does not enable 1: enable"]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEN` reader - AESEN: AES clock enable. 0: AES IP is clock gated. 1: AES IP is clocked."]
pub type AesenR = crate::BitReader;
#[doc = "Field `AESEN` writer - AESEN: AES clock enable. 0: AES IP is clock gated. 1: AES IP is clocked."]
pub type AesenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA and DMAMUX enable Set and enable by software. 0: does not enable 1: enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOA enable. It must be enabled by default"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOB enable. It must be enabled by default"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC enable Set and enable by software. 0: does not enable 1: enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable Set and enable by software. 0: does not enable 1: enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - AESEN: AES clock enable. 0: AES IP is clock gated. 1: AES IP is clocked."]
    #[inline(always)]
    pub fn aesen(&self) -> AesenR {
        AesenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA and DMAMUX enable Set and enable by software. 0: does not enable 1: enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, AhbenrSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 2 - GPIOA enable. It must be enabled by default"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GpioaenW<'_, AhbenrSpec> {
        GpioaenW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOB enable. It must be enabled by default"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GpiobenW<'_, AhbenrSpec> {
        GpiobenW::new(self, 3)
    }
    #[doc = "Bit 12 - CRC enable Set and enable by software. 0: does not enable 1: enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, AhbenrSpec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 18 - RNG clock enable Set and enable by software. 0: does not enable 1: enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RngenW<'_, AhbenrSpec> {
        RngenW::new(self, 18)
    }
    #[doc = "Bit 20 - AESEN: AES clock enable. 0: AES IP is clock gated. 1: AES IP is clocked."]
    #[inline(always)]
    pub fn aesen(&mut self) -> AesenW<'_, AhbenrSpec> {
        AesenW::new(self, 20)
    }
}
#[doc = "AHBENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBENR to value 0x0c"]
impl crate::Resettable for AhbenrSpec {
    const RESET_VALUE: u32 = 0x0c;
}
