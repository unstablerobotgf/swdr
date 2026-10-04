#[doc = "Register `APB0RSTR` reader"]
pub type R = crate::R<Apb0rstrSpec>;
#[doc = "Register `APB0RSTR` writer"]
pub type W = crate::W<Apb0rstrSpec>;
#[doc = "Field `TIM2RST` reader - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset."]
pub type Tim2rstR = crate::BitReader;
#[doc = "Field `TIM2RST` writer - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset."]
pub type Tim2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset."]
pub type Tim16rstR = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset."]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGRST` reader - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset."]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset."]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCRST` reader - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset."]
pub type LcdcrstR = crate::BitReader;
#[doc = "Field `LCDCRST` writer - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset."]
pub type LcdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPRST` reader - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset."]
pub type ComprstR = crate::BitReader;
#[doc = "Field `COMPRST` writer - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset."]
pub type ComprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACRST` reader - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset."]
pub type DacrstR = crate::BitReader;
#[doc = "Field `DACRST` writer - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset."]
pub type DacrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCRST` reader - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset."]
pub type RtcrstR = crate::BitReader;
#[doc = "Field `RTCRST` writer - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset."]
pub type RtcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCSCRST` reader - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset."]
pub type LcscrstR = crate::BitReader;
#[doc = "Field `LCSCRST` writer - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset."]
pub type LcscrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGRST` reader - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset."]
pub type WdgrstR = crate::BitReader;
#[doc = "Field `WDGRST` writer - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset."]
pub type WdgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCURST` reader - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset."]
pub type DbgmcurstR = crate::BitReader;
#[doc = "Field `DBGMCURST` writer - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset."]
pub type DbgmcurstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset."]
    #[inline(always)]
    pub fn tim2rst(&self) -> Tim2rstR {
        Tim2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset."]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset."]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset."]
    #[inline(always)]
    pub fn lcdcrst(&self) -> LcdcrstR {
        LcdcrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset."]
    #[inline(always)]
    pub fn comprst(&self) -> ComprstR {
        ComprstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset."]
    #[inline(always)]
    pub fn dacrst(&self) -> DacrstR {
        DacrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset."]
    #[inline(always)]
    pub fn rtcrst(&self) -> RtcrstR {
        RtcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset."]
    #[inline(always)]
    pub fn lcscrst(&self) -> LcscrstR {
        LcscrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset."]
    #[inline(always)]
    pub fn wdgrst(&self) -> WdgrstR {
        WdgrstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset."]
    #[inline(always)]
    pub fn dbgmcurst(&self) -> DbgmcurstR {
        DbgmcurstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2RST: TIM2 reset. 0: TIM2 IP is not under reset. 1: TIM2 IP is under reset."]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> Tim2rstW<'_, Apb0rstrSpec> {
        Tim2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM16RST: TIM16 reset. 0: TIM16 IP is not under reset. 1: TIM16 IP is under reset."]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> Tim16rstW<'_, Apb0rstrSpec> {
        Tim16rstW::new(self, 1)
    }
    #[doc = "Bit 8 - SYSCFGRST: system controller reset. 0: system controller IP is not under reset. 1: system controller IP is under reset."]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<'_, Apb0rstrSpec> {
        SyscfgrstW::new(self, 8)
    }
    #[doc = "Bit 9 - LCDCRST: LCD controller reset. 0: LCD controller IP is not under reset. 1: LCD controller IP is under reset."]
    #[inline(always)]
    pub fn lcdcrst(&mut self) -> LcdcrstW<'_, Apb0rstrSpec> {
        LcdcrstW::new(self, 9)
    }
    #[doc = "Bit 10 - COMPRST: COMP reset. 0: COMP IP is not under reset. 1: COMP IP is under reset."]
    #[inline(always)]
    pub fn comprst(&mut self) -> ComprstW<'_, Apb0rstrSpec> {
        ComprstW::new(self, 10)
    }
    #[doc = "Bit 11 - DACRST: DAC reset. 0: DAC IP is not under reset. 1: DAC IP is under reset."]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DacrstW<'_, Apb0rstrSpec> {
        DacrstW::new(self, 11)
    }
    #[doc = "Bit 12 - RTCRST: RTC reset. 0: RTC IP is not under reset. 1: RTC IP is under reset."]
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RtcrstW<'_, Apb0rstrSpec> {
        RtcrstW::new(self, 12)
    }
    #[doc = "Bit 13 - LCSCRST: LCSC reset. 0: LCSC IP is not under reset. 1: LCSC IP is under reset."]
    #[inline(always)]
    pub fn lcscrst(&mut self) -> LcscrstW<'_, Apb0rstrSpec> {
        LcscrstW::new(self, 13)
    }
    #[doc = "Bit 14 - WDGRST: Watchdog reset. 0: Watchdog IP is not under reset. 1: Watchdog IP is under reset."]
    #[inline(always)]
    pub fn wdgrst(&mut self) -> WdgrstW<'_, Apb0rstrSpec> {
        WdgrstW::new(self, 14)
    }
    #[doc = "Bit 15 - DBGMCURST: DBGMCU reset. 0: DBGMCU IP is not under reset. 1: DBGMCU IP is under reset."]
    #[inline(always)]
    pub fn dbgmcurst(&mut self) -> DbgmcurstW<'_, Apb0rstrSpec> {
        DbgmcurstW::new(self, 15)
    }
}
#[doc = "APB0RSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb0rstrSpec;
impl crate::RegisterSpec for Apb0rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb0rstr::R`](R) reader structure"]
impl crate::Readable for Apb0rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb0rstr::W`](W) writer structure"]
impl crate::Writable for Apb0rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB0RSTR to value 0"]
impl crate::Resettable for Apb0rstrSpec {}
