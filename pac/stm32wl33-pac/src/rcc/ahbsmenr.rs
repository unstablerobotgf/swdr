#[doc = "Register `AHBSMENR` reader"]
pub type R = crate::R<AhbsmenrSpec>;
#[doc = "Register `AHBSMENR` writer"]
pub type W = crate::W<AhbsmenrSpec>;
#[doc = "Field `DMASMEN` reader - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)"]
pub type DmasmenR = crate::BitReader;
#[doc = "Field `DMASMEN` writer - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)"]
pub type DmasmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD"]
pub type FlashsmenR = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD"]
pub type FlashsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOASMEN` reader - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)"]
pub type GpioasmenR = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)"]
pub type GpioasmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)"]
pub type GpiobsmenR = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)"]
pub type GpiobsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM0SMEN` reader - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode"]
pub type Sram0smenR = crate::BitReader;
#[doc = "Field `SRAM0SMEN` writer - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode"]
pub type Sram0smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode"]
pub type Sram1smenR = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode"]
pub type Sram1smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)"]
pub type CrcsmenR = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)"]
pub type CrcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)"]
pub type RngsmenR = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)"]
pub type RngsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSMEN` reader - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)"]
pub type AessmenR = crate::BitReader;
#[doc = "Field `AESSMEN` writer - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)"]
pub type AessmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)"]
    #[inline(always)]
    pub fn dmasmen(&self) -> DmasmenR {
        DmasmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FlashsmenR {
        FlashsmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GpioasmenR {
        GpioasmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GpiobsmenR {
        GpiobsmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn sram0smen(&self) -> Sram0smenR {
        Sram0smenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn sram1smen(&self) -> Sram1smenR {
        Sram1smenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CrcsmenR {
        CrcsmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RngsmenR {
        RngsmenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)"]
    #[inline(always)]
    pub fn aessmen(&self) -> AessmenR {
        AessmenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)"]
    #[inline(always)]
    pub fn dmasmen(&mut self) -> DmasmenW<'_, AhbsmenrSpec> {
        DmasmenW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD"]
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FlashsmenW<'_, AhbsmenrSpec> {
        FlashsmenW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)"]
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GpioasmenW<'_, AhbsmenrSpec> {
        GpioasmenW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)"]
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GpiobsmenW<'_, AhbsmenrSpec> {
        GpiobsmenW::new(self, 3)
    }
    #[doc = "Bit 9 - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn sram0smen(&mut self) -> Sram0smenW<'_, AhbsmenrSpec> {
        Sram0smenW::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn sram1smen(&mut self) -> Sram1smenW<'_, AhbsmenrSpec> {
        Sram1smenW::new(self, 10)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)"]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CrcsmenW<'_, AhbsmenrSpec> {
        CrcsmenW::new(self, 12)
    }
    #[doc = "Bit 18 - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)"]
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RngsmenW<'_, AhbsmenrSpec> {
        RngsmenW::new(self, 18)
    }
    #[doc = "Bit 20 - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)"]
    #[inline(always)]
    pub fn aessmen(&mut self) -> AessmenW<'_, AhbsmenrSpec> {
        AessmenW::new(self, 20)
    }
}
#[doc = "AHBSMENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbsmenrSpec;
impl crate::RegisterSpec for AhbsmenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbsmenr::R`](R) reader structure"]
impl crate::Readable for AhbsmenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure"]
impl crate::Writable for AhbsmenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBSMENR to value 0x0014_160f"]
impl crate::Resettable for AhbsmenrSpec {
    const RESET_VALUE: u32 = 0x0014_160f;
}
