#[doc = "Register `CIER` reader"]
pub type R = crate::R<CierSpec>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CierSpec>;
#[doc = "Field `LSIRDYIE` reader - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization. 0: LSI ready interrupt disabled 1: LSI ready interrupt enabled"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization. 0: LSI ready interrupt disabled 1: LSI ready interrupt enabled"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization. 0: LSE ready interrupt disabled 1: LSE ready interrupt enabled"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization. 0: LSE ready interrupt disabled 1: LSE ready interrupt enabled"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization. 0: HSI ready interrupt disabled 1: HSI ready interrupt enabled"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization. 0: HSI ready interrupt disabled 1: HSI ready interrupt enabled"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization. 0: HSE ready interrupt disabled 1: HSE ready interrupt enabled"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization. 0: HSE ready interrupt disabled 1: HSE ready interrupt enabled"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIPLLRDYIE` reader - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE. 0: HSI PLL ready interrupt disabled 1: HSI PLL ready interrupt enabled"]
pub type HsipllrdyieR = crate::BitReader;
#[doc = "Field `HSIPLLRDYIE` writer - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE. 0: HSI PLL ready interrupt disabled 1: HSI PLL ready interrupt enabled"]
pub type HsipllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIPLLUNLOCKDETIE` reader - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
pub type HsipllunlockdetieR = crate::BitReader;
#[doc = "Field `HSIPLLUNLOCKDETIE` writer - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
pub type HsipllunlockdetieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCRSTIE` reader - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
pub type RtcrstieR = crate::BitReader;
#[doc = "Field `RTCRSTIE` writer - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
pub type RtcrstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGRSTIE` reader - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end. 0: interrupt disabled 1: interrupt enabled"]
pub type WdgrstieR = crate::BitReader;
#[doc = "Field `WDGRSTIE` writer - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end. 0: interrupt disabled 1: interrupt enabled"]
pub type WdgrstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPURSTIE` reader - LPURSTIE: LPUART reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LPUART reset end. 0: interrupt disabled 1: interrupt enabled"]
pub type LpurstieR = crate::BitReader;
#[doc = "Field `LPURSTIE` writer - LPURSTIE: LPUART reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LPUART reset end. 0: interrupt disabled 1: interrupt enabled"]
pub type LpurstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDRSTIE` reader - LCDRSTIE: LCD reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LCD reset end. 0: interrupt disabled 1: interrupt enabled"]
pub type LcdrstieR = crate::BitReader;
#[doc = "Field `LCDRSTIE` writer - LCDRSTIE: LCD reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LCD reset end. 0: interrupt disabled 1: interrupt enabled"]
pub type LcdrstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCSCRSTIE` reader - LCSCRSTIE: LCSC reset release interrupt enable. 0: LCSC reset release interrupt is disabled. 1: LCSC reset release interrupt is enabled."]
pub type LcscrstieR = crate::BitReader;
#[doc = "Field `LCSCRSTIE` writer - LCSCRSTIE: LCSC reset release interrupt enable. 0: LCSC reset release interrupt is disabled. 1: LCSC reset release interrupt is enabled."]
pub type LcscrstieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization. 0: LSI ready interrupt disabled 1: LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization. 0: LSE ready interrupt disabled 1: LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization. 0: HSI ready interrupt disabled 1: HSI ready interrupt enabled"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization. 0: HSE ready interrupt disabled 1: HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE. 0: HSI PLL ready interrupt disabled 1: HSI PLL ready interrupt enabled"]
    #[inline(always)]
    pub fn hsipllrdyie(&self) -> HsipllrdyieR {
        HsipllrdyieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
    #[inline(always)]
    pub fn hsipllunlockdetie(&self) -> HsipllunlockdetieR {
        HsipllunlockdetieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
    #[inline(always)]
    pub fn rtcrstie(&self) -> RtcrstieR {
        RtcrstieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end. 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn wdgrstie(&self) -> WdgrstieR {
        WdgrstieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPURSTIE: LPUART reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LPUART reset end. 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn lpurstie(&self) -> LpurstieR {
        LpurstieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCDRSTIE: LCD reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LCD reset end. 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn lcdrstie(&self) -> LcdrstieR {
        LcdrstieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - LCSCRSTIE: LCSC reset release interrupt enable. 0: LCSC reset release interrupt is disabled. 1: LCSC reset release interrupt is enabled."]
    #[inline(always)]
    pub fn lcscrstie(&self) -> LcscrstieR {
        LcscrstieR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization. 0: LSI ready interrupt disabled 1: LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<'_, CierSpec> {
        LsirdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization. 0: LSE ready interrupt disabled 1: LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<'_, CierSpec> {
        LserdyieW::new(self, 1)
    }
    #[doc = "Bit 3 - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization. 0: HSI ready interrupt disabled 1: HSI ready interrupt enabled"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<'_, CierSpec> {
        HsirdyieW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization. 0: HSE ready interrupt disabled 1: HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<'_, CierSpec> {
        HserdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE. 0: HSI PLL ready interrupt disabled 1: HSI PLL ready interrupt enabled"]
    #[inline(always)]
    pub fn hsipllrdyie(&mut self) -> HsipllrdyieW<'_, CierSpec> {
        HsipllrdyieW::new(self, 5)
    }
    #[doc = "Bit 6 - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
    #[inline(always)]
    pub fn hsipllunlockdetie(&mut self) -> HsipllunlockdetieW<'_, CierSpec> {
        HsipllunlockdetieW::new(self, 6)
    }
    #[doc = "Bit 7 - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end. 0: HSI PLL unlock detection interrupt disabled 1: HSI PLL unlock detection interrupt enabled"]
    #[inline(always)]
    pub fn rtcrstie(&mut self) -> RtcrstieW<'_, CierSpec> {
        RtcrstieW::new(self, 7)
    }
    #[doc = "Bit 8 - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end. 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn wdgrstie(&mut self) -> WdgrstieW<'_, CierSpec> {
        WdgrstieW::new(self, 8)
    }
    #[doc = "Bit 9 - LPURSTIE: LPUART reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LPUART reset end. 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn lpurstie(&mut self) -> LpurstieW<'_, CierSpec> {
        LpurstieW::new(self, 9)
    }
    #[doc = "Bit 10 - LCDRSTIE: LCD reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the LCD reset end. 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn lcdrstie(&mut self) -> LcdrstieW<'_, CierSpec> {
        LcdrstieW::new(self, 10)
    }
    #[doc = "Bit 13 - LCSCRSTIE: LCSC reset release interrupt enable. 0: LCSC reset release interrupt is disabled. 1: LCSC reset release interrupt is enabled."]
    #[inline(always)]
    pub fn lcscrstie(&mut self) -> LcscrstieW<'_, CierSpec> {
        LcscrstieW::new(self, 13)
    }
}
#[doc = "CIER register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CierSpec;
impl crate::RegisterSpec for CierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CierSpec {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CierSpec {}
