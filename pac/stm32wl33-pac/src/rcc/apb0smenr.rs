#[doc = "Register `APB0SMENR` reader"]
pub type R = crate::R<Apb0smenrSpec>;
#[doc = "Register `APB0SMENR` writer"]
pub type W = crate::W<Apb0smenrSpec>;
#[doc = "Field `TIM2SMEN` reader - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)"]
pub type Tim2smenR = crate::BitReader;
#[doc = "Field `TIM2SMEN` writer - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)"]
pub type Tim2smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)"]
pub type Tim16smenR = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)"]
pub type Tim16smenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)"]
pub type SyscfgsmenR = crate::BitReader;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)"]
pub type SyscfgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSMEN` reader - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)"]
pub type LcdcsmenR = crate::BitReader;
#[doc = "Field `LCDCSMEN` writer - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)"]
pub type LcdcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPSMEN` reader - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)"]
pub type CompsmenR = crate::BitReader;
#[doc = "Field `COMPSMEN` writer - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)"]
pub type CompsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSMEN` reader - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)"]
pub type DacsmenR = crate::BitReader;
#[doc = "Field `DACSMEN` writer - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)"]
pub type DacsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSMEN` reader - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)"]
pub type RtcsmenR = crate::BitReader;
#[doc = "Field `RTCSMEN` writer - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)"]
pub type RtcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCSCSMEN` reader - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)"]
pub type LcscsmenR = crate::BitReader;
#[doc = "Field `LCSCSMEN` writer - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)"]
pub type LcscsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGSMEN` reader - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)"]
pub type WdgsmenR = crate::BitReader;
#[doc = "Field `WDGSMEN` writer - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)"]
pub type WdgsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCUSMEN` reader - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)"]
pub type DbgmcusmenR = crate::BitReader;
#[doc = "Field `DBGMCUSMEN` writer - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)"]
pub type DbgmcusmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)"]
    #[inline(always)]
    pub fn tim2smen(&self) -> Tim2smenR {
        Tim2smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)"]
    #[inline(always)]
    pub fn tim16smen(&self) -> Tim16smenR {
        Tim16smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SyscfgsmenR {
        SyscfgsmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)"]
    #[inline(always)]
    pub fn lcdcsmen(&self) -> LcdcsmenR {
        LcdcsmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)"]
    #[inline(always)]
    pub fn compsmen(&self) -> CompsmenR {
        CompsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)"]
    #[inline(always)]
    pub fn dacsmen(&self) -> DacsmenR {
        DacsmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)"]
    #[inline(always)]
    pub fn rtcsmen(&self) -> RtcsmenR {
        RtcsmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)"]
    #[inline(always)]
    pub fn lcscsmen(&self) -> LcscsmenR {
        LcscsmenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)"]
    #[inline(always)]
    pub fn wdgsmen(&self) -> WdgsmenR {
        WdgsmenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)"]
    #[inline(always)]
    pub fn dbgmcusmen(&self) -> DbgmcusmenR {
        DbgmcusmenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)"]
    #[inline(always)]
    pub fn tim2smen(&mut self) -> Tim2smenW<'_, Apb0smenrSpec> {
        Tim2smenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)"]
    #[inline(always)]
    pub fn tim16smen(&mut self) -> Tim16smenW<'_, Apb0smenrSpec> {
        Tim16smenW::new(self, 1)
    }
    #[doc = "Bit 8 - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)"]
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SyscfgsmenW<'_, Apb0smenrSpec> {
        SyscfgsmenW::new(self, 8)
    }
    #[doc = "Bit 9 - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)"]
    #[inline(always)]
    pub fn lcdcsmen(&mut self) -> LcdcsmenW<'_, Apb0smenrSpec> {
        LcdcsmenW::new(self, 9)
    }
    #[doc = "Bit 10 - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)"]
    #[inline(always)]
    pub fn compsmen(&mut self) -> CompsmenW<'_, Apb0smenrSpec> {
        CompsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)"]
    #[inline(always)]
    pub fn dacsmen(&mut self) -> DacsmenW<'_, Apb0smenrSpec> {
        DacsmenW::new(self, 11)
    }
    #[doc = "Bit 12 - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)"]
    #[inline(always)]
    pub fn rtcsmen(&mut self) -> RtcsmenW<'_, Apb0smenrSpec> {
        RtcsmenW::new(self, 12)
    }
    #[doc = "Bit 13 - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)"]
    #[inline(always)]
    pub fn lcscsmen(&mut self) -> LcscsmenW<'_, Apb0smenrSpec> {
        LcscsmenW::new(self, 13)
    }
    #[doc = "Bit 14 - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)"]
    #[inline(always)]
    pub fn wdgsmen(&mut self) -> WdgsmenW<'_, Apb0smenrSpec> {
        WdgsmenW::new(self, 14)
    }
    #[doc = "Bit 15 - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)"]
    #[inline(always)]
    pub fn dbgmcusmen(&mut self) -> DbgmcusmenW<'_, Apb0smenrSpec> {
        DbgmcusmenW::new(self, 15)
    }
}
#[doc = "APB0SMENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb0smenrSpec;
impl crate::RegisterSpec for Apb0smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb0smenr::R`](R) reader structure"]
impl crate::Readable for Apb0smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb0smenr::W`](W) writer structure"]
impl crate::Writable for Apb0smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB0SMENR to value 0xff03"]
impl crate::Resettable for Apb0smenrSpec {
    const RESET_VALUE: u32 = 0xff03;
}
