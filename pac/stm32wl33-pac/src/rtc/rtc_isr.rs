#[doc = "Register `RTC_ISR` reader"]
pub type R = crate::R<RtcIsrSpec>;
#[doc = "Register `RTC_ISR` writer"]
pub type W = crate::W<RtcIsrSpec>;
#[doc = "Field `ALRAWF` reader - Alarm A write flag This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode. 0: Alarm A update not allowed 1: Alarm A update allowed."]
pub type AlrawfR = crate::BitReader;
#[doc = "Field `ALRAWF` writer - Alarm A write flag This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode. 0: Alarm A update not allowed 1: Alarm A update allowed."]
pub type AlrawfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag This bit is set by hardware when the wakeup timer values can be changed, after the WUTE bit has been set to 0 in RTC_CR. 0: Wakeup timer configuration update not allowed 1: Wakeup timer configuration update allowed."]
pub type WutwfR = crate::BitReader;
#[doc = "Field `WUTWF` writer - Wakeup timer write flag This bit is set by hardware when the wakeup timer values can be changed, after the WUTE bit has been set to 0 in RTC_CR. 0: Wakeup timer configuration update not allowed 1: Wakeup timer configuration update allowed."]
pub type WutwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHPF` reader - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
pub type ShpfR = crate::BitReader;
#[doc = "Field `SHPF` writer - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
pub type ShpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (power-on reset state). 0: Calendar has not been initialized 1: Calendar has been initialized"]
pub type InitsR = crate::BitReader;
#[doc = "Field `INITS` writer - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (power-on reset state). 0: Calendar has not been initialized 1: Calendar has been initialized"]
pub type InitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized."]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized."]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated. 0: Calendar registers update is not allowed 1: Calendar registers update is allowed."]
pub type InitfR = crate::BitReader;
#[doc = "Field `INITF` writer - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated. 0: Calendar registers update is not allowed 1: Calendar registers update is allowed."]
pub type InitfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT` reader - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAF` reader - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0."]
pub type AlrafR = crate::BitReader;
#[doc = "Field `ALRAF` writer - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0."]
pub type AlrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTF` reader - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
pub type WutfR = crate::BitReader;
#[doc = "Field `WUTF` writer - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
pub type WutfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0. If ITSF flag is set, TSF must be cleared together with ITSF by writing 0 in both bits."]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0. If ITSF flag is set, TSF must be cleared together with ITSF by writing 0 in both bits."]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOVF` reader - This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type TsovfR = crate::BitReader;
#[doc = "Field `TSOVF` writer - This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type TsovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1F` reader - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0"]
pub type Tamp1fR = crate::BitReader;
#[doc = "Field `TAMP1F` writer - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0"]
pub type Tamp1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0."]
pub type RecalpfR = crate::BitReader;
#[doc = "Field `RECALPF` writer - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0."]
pub type RecalpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSF` reader - Internal time-stamp flag This flag is set by hardware when a time-stamp on the internal event occurs. This flag is cleared by software by writing 0, and must be cleared together with TSF bit by writing 0 in both bits."]
pub type ItsfR = crate::BitReader;
#[doc = "Field `ITSF` writer - Internal time-stamp flag This flag is set by hardware when a time-stamp on the internal event occurs. This flag is cleared by software by writing 0, and must be cleared together with TSF bit by writing 0 in both bits."]
pub type ItsfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm A write flag This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode. 0: Alarm A update not allowed 1: Alarm A update allowed."]
    #[inline(always)]
    pub fn alrawf(&self) -> AlrawfR {
        AlrawfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag This bit is set by hardware when the wakeup timer values can be changed, after the WUTE bit has been set to 0 in RTC_CR. 0: Wakeup timer configuration update not allowed 1: Wakeup timer configuration update allowed."]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        WutwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
    #[inline(always)]
    pub fn shpf(&self) -> ShpfR {
        ShpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (power-on reset state). 0: Calendar has not been initialized 1: Calendar has been initialized"]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        InitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized."]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated. 0: Calendar registers update is not allowed 1: Calendar registers update is allowed."]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0."]
    #[inline(always)]
    pub fn alraf(&self) -> AlrafR {
        AlrafR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
    #[inline(always)]
    pub fn wutf(&self) -> WutfR {
        WutfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0. If ITSF flag is set, TSF must be cleared together with ITSF by writing 0 in both bits."]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn tsovf(&self) -> TsovfR {
        TsovfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0"]
    #[inline(always)]
    pub fn tamp1f(&self) -> Tamp1fR {
        Tamp1fR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0."]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        RecalpfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal time-stamp flag This flag is set by hardware when a time-stamp on the internal event occurs. This flag is cleared by software by writing 0, and must be cleared together with TSF bit by writing 0 in both bits."]
    #[inline(always)]
    pub fn itsf(&self) -> ItsfR {
        ItsfR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm A write flag This bit is set by hardware when Alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode. 0: Alarm A update not allowed 1: Alarm A update allowed."]
    #[inline(always)]
    pub fn alrawf(&mut self) -> AlrawfW<'_, RtcIsrSpec> {
        AlrawfW::new(self, 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag This bit is set by hardware when the wakeup timer values can be changed, after the WUTE bit has been set to 0 in RTC_CR. 0: Wakeup timer configuration update not allowed 1: Wakeup timer configuration update allowed."]
    #[inline(always)]
    pub fn wutwf(&mut self) -> WutwfW<'_, RtcIsrSpec> {
        WutwfW::new(self, 2)
    }
    #[doc = "Bit 3 - Shift operation pending 0: No shift operation is pending 1: A shift operation is pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
    #[inline(always)]
    pub fn shpf(&mut self) -> ShpfW<'_, RtcIsrSpec> {
        ShpfW::new(self, 3)
    }
    #[doc = "Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (power-on reset state). 0: Calendar has not been initialized 1: Calendar has been initialized"]
    #[inline(always)]
    pub fn inits(&mut self) -> InitsW<'_, RtcIsrSpec> {
        InitsW::new(self, 4)
    }
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSRx, RTC_TRx and RTC_DRx). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF=1), or when in bypass shadow regsiter mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. 0: Calendar shadow registers not yet synchronized 1: Calendar shadow registers synchronized."]
    #[inline(always)]
    pub fn rsf(&mut self) -> RsfW<'_, RtcIsrSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated. 0: Calendar registers update is not allowed 1: Calendar registers update is allowed."]
    #[inline(always)]
    pub fn initf(&mut self) -> InitfW<'_, RtcIsrSpec> {
        InitfW::new(self, 6)
    }
    #[doc = "Bit 7 - Initialization mode 0: Free running mode 1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, RtcIsrSpec> {
        InitW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR). This flag is cleared by software by writing 0."]
    #[inline(always)]
    pub fn alraf(&mut self) -> AlrafW<'_, RtcIsrSpec> {
        AlrafW::new(self, 8)
    }
    #[doc = "Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. This flag is cleared by software by writing 0. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
    #[inline(always)]
    pub fn wutf(&mut self) -> WutfW<'_, RtcIsrSpec> {
        WutfW::new(self, 10)
    }
    #[doc = "Bit 11 - This flag is set by hardware when a time-stamp event occurs. This flag is cleared by software by writing 0. If ITSF flag is set, TSF must be cleared together with ITSF by writing 0 in both bits."]
    #[inline(always)]
    pub fn tsf(&mut self) -> TsfW<'_, RtcIsrSpec> {
        TsfW::new(self, 11)
    }
    #[doc = "Bit 12 - This flag is set by hardware when a time-stamp event occurs while TSF is already set. This flag is cleared by software by writing 0. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn tsovf(&mut self) -> TsovfW<'_, RtcIsrSpec> {
        TsovfW::new(self, 12)
    }
    #[doc = "Bit 13 - RTC_TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the RTC_TAMP1 input. It is cleared by software writing 0"]
    #[inline(always)]
    pub fn tamp1f(&mut self) -> Tamp1fW<'_, RtcIsrSpec> {
        Tamp1fW::new(self, 13)
    }
    #[doc = "Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0."]
    #[inline(always)]
    pub fn recalpf(&mut self) -> RecalpfW<'_, RtcIsrSpec> {
        RecalpfW::new(self, 16)
    }
    #[doc = "Bit 17 - Internal time-stamp flag This flag is set by hardware when a time-stamp on the internal event occurs. This flag is cleared by software by writing 0, and must be cleared together with TSF bit by writing 0 in both bits."]
    #[inline(always)]
    pub fn itsf(&mut self) -> ItsfW<'_, RtcIsrSpec> {
        ItsfW::new(self, 17)
    }
}
#[doc = "RTC_ISR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcIsrSpec;
impl crate::RegisterSpec for RtcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_isr::R`](R) reader structure"]
impl crate::Readable for RtcIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_isr::W`](W) writer structure"]
impl crate::Writable for RtcIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ISR to value 0x07"]
impl crate::Resettable for RtcIsrSpec {
    const RESET_VALUE: u32 = 0x07;
}
