#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CifrSpec>;
#[doc = "Register `CIFR` writer"]
pub type W = crate::W<CifrSpec>;
#[doc = "Field `LSIRDYIF` reader - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator"]
pub type LsirdyifR = crate::BitReader;
#[doc = "Field `LSIRDYIF` writer - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator"]
pub type LsirdyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIF` reader - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator"]
pub type LserdyifR = crate::BitReader;
#[doc = "Field `LSERDYIF` writer - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator"]
pub type LserdyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIF` reader - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator"]
pub type HsirdyifR = crate::BitReader;
#[doc = "Field `HSIRDYIF` writer - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator"]
pub type HsirdyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIF` reader - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator"]
pub type HserdyifR = crate::BitReader;
#[doc = "Field `HSERDYIF` writer - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator"]
pub type HserdyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIPLLRDYIF` reader - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator"]
pub type HsipllrdyifR = crate::BitReader;
#[doc = "Field `HSIPLLRDYIF` writer - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator"]
pub type HsipllrdyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIPLLUNLOCKDETIF` reader - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag."]
pub type HsipllunlockdetifR = crate::BitReader;
#[doc = "Field `HSIPLLUNLOCKDETIF` writer - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag."]
pub type HsipllunlockdetifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCRSTIF` reader - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type RtcrstifR = crate::BitReader;
#[doc = "Field `RTCRSTIF` writer - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type RtcrstifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGRSTIF` reader - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type WdgrstifR = crate::BitReader;
#[doc = "Field `WDGRSTIF` writer - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type WdgrstifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPURSTIF` reader - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type LpurstifR = crate::BitReader;
#[doc = "Field `LPURSTIF` writer - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type LpurstifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDRSTIF` reader - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type LcdrstifR = crate::BitReader;
#[doc = "Field `LCDRSTIF` writer - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type LcdrstifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCSCRSTIF` reader - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type LcscrstifR = crate::BitReader;
#[doc = "Field `LCSCRSTIF` writer - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
pub type LcscrstifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator"]
    #[inline(always)]
    pub fn lsirdyif(&self) -> LsirdyifR {
        LsirdyifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator"]
    #[inline(always)]
    pub fn lserdyif(&self) -> LserdyifR {
        LserdyifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator"]
    #[inline(always)]
    pub fn hsirdyif(&self) -> HsirdyifR {
        HsirdyifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator"]
    #[inline(always)]
    pub fn hserdyif(&self) -> HserdyifR {
        HserdyifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator"]
    #[inline(always)]
    pub fn hsipllrdyif(&self) -> HsipllrdyifR {
        HsipllrdyifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag."]
    #[inline(always)]
    pub fn hsipllunlockdetif(&self) -> HsipllunlockdetifR {
        HsipllunlockdetifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn rtcrstif(&self) -> RtcrstifR {
        RtcrstifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn wdgrstif(&self) -> WdgrstifR {
        WdgrstifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn lpurstif(&self) -> LpurstifR {
        LpurstifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn lcdrstif(&self) -> LcdrstifR {
        LcdrstifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn lcscrstif(&self) -> LcscrstifR {
        LcscrstifR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI Ready Interrupt flag Set by hardware when LSI clock becomes stable. 0: No clock ready interrupt caused by the internal RC 32 KHz oscillator 1: Clock ready interrupt caused by the internal RC 32 kHz oscillator"]
    #[inline(always)]
    pub fn lsirdyif(&mut self) -> LsirdyifW<'_, CifrSpec> {
        LsirdyifW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt Flag. Set by hardware when LSE clock becomes stable. 0: No clock ready interrupt caused by the LSE oscillator 1: Clock ready interrupt caused by the LSE oscillator"]
    #[inline(always)]
    pub fn lserdyif(&mut self) -> LserdyifW<'_, CifrSpec> {
        LserdyifW::new(self, 1)
    }
    #[doc = "Bit 3 - HSI Ready Interrupt Flag. Set by hardware when HSI becomes stable. 0: No clock ready interrupt caused by the HSI oscillator 1: Clock ready interrupt caused by the HSI oscillator"]
    #[inline(always)]
    pub fn hsirdyif(&mut self) -> HsirdyifW<'_, CifrSpec> {
        HsirdyifW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE Ready Interrupt Flag. Set by hardware when HSE becomes stable. 0: No clock ready interrupt caused by the HSE oscillator 1: Clock ready interrupt caused by the HSE oscillator"]
    #[inline(always)]
    pub fn hserdyif(&mut self) -> HserdyifW<'_, CifrSpec> {
        HserdyifW::new(self, 4)
    }
    #[doc = "Bit 5 - HSI PLL Ready Interrupt Flag. Set by hardware when HSI PLL 64MHz becomes stable. 0: No clock ready interrupt caused by the HSI PLL64 MHz oscillator 1: Clock ready interrupt caused by the HSI PLL64 MHz oscillator"]
    #[inline(always)]
    pub fn hsipllrdyif(&mut self) -> HsipllrdyifW<'_, CifrSpec> {
        HsipllrdyifW::new(self, 5)
    }
    #[doc = "Bit 6 - HSIPLLUNLOCKDETIF: HSI PLL unlock detection Interrupt Flag."]
    #[inline(always)]
    pub fn hsipllunlockdetif(&mut self) -> HsipllunlockdetifW<'_, CifrSpec> {
        HsipllunlockdetifW::new(self, 6)
    }
    #[doc = "Bit 7 - RTC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn rtcrstif(&mut self) -> RtcrstifW<'_, CifrSpec> {
        RtcrstifW::new(self, 7)
    }
    #[doc = "Bit 8 - WDG reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn wdgrstif(&mut self) -> WdgrstifW<'_, CifrSpec> {
        WdgrstifW::new(self, 8)
    }
    #[doc = "Bit 9 - LPUART reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn lpurstif(&mut self) -> LpurstifW<'_, CifrSpec> {
        LpurstifW::new(self, 9)
    }
    #[doc = "Bit 10 - LCD reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn lcdrstif(&mut self) -> LcdrstifW<'_, CifrSpec> {
        LcdrstifW::new(self, 10)
    }
    #[doc = "Bit 13 - LCSC reset end Interrupt Flag. Raised when reset is released on 32kHz clock"]
    #[inline(always)]
    pub fn lcscrstif(&mut self) -> LcscrstifW<'_, CifrSpec> {
        LcscrstifW::new(self, 13)
    }
}
#[doc = "CIFR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CifrSpec;
impl crate::RegisterSpec for CifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CifrSpec {}
#[doc = "`write(|w| ..)` method takes [`cifr::W`](W) writer structure"]
impl crate::Writable for CifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIFR to value 0x08"]
impl crate::Resettable for CifrSpec {
    const RESET_VALUE: u32 = 0x08;
}
