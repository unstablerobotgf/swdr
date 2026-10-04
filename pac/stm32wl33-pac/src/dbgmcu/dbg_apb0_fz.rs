#[doc = "Register `DBG_APB0_FZ` reader"]
pub type R = crate::R<DbgApb0FzSpec>;
#[doc = "Register `DBG_APB0_FZ` writer"]
pub type W = crate::W<DbgApb0FzSpec>;
#[doc = "Field `DBG_TIM2_STOP` reader - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode."]
pub type DbgTim2StopR = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode."]
pub type DbgTim2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode."]
pub type DbgTim16StopR = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode."]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode."]
pub type DbgRtcStopR = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode."]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode."]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode."]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DbgTim2StopR {
        DbgTim2StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in the CPU debug - 0: Normal operation. TIM2 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM2 is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DbgTim2StopW<'_, DbgApb0FzSpec> {
        DbgTim2StopW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM16 stop in the CPU debug - 0: Normal operation. TIM16 continues to operate while the CPU is in debug mode - 1: Stop in debug. TIM16 is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<'_, DbgApb0FzSpec> {
        DbgTim16StopW::new(self, 1)
    }
    #[doc = "Bit 12 - RTC stop in CPU debug - 0: Normal operation. RTC continues to operate while the CPU is in debug mode - 1: Stop in debug. RTC is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<'_, DbgApb0FzSpec> {
        DbgRtcStopW::new(self, 12)
    }
    #[doc = "Bit 14 - IWDG stop in the CPU debug - 0: Normal operation. IWDG continues to operate while the CPU is in debug mode - 1: Stop in debug. IWDG is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<'_, DbgApb0FzSpec> {
        DbgIwdgStopW::new(self, 14)
    }
}
#[doc = "DBG_APB0_FZ register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_apb0_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb0_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgApb0FzSpec;
impl crate::RegisterSpec for DbgApb0FzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb0_fz::R`](R) reader structure"]
impl crate::Readable for DbgApb0FzSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb0_fz::W`](W) writer structure"]
impl crate::Writable for DbgApb0FzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_APB0_FZ to value 0"]
impl crate::Resettable for DbgApb0FzSpec {}
