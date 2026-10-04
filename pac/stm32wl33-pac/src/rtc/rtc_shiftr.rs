#[doc = "Register `RTC_SHIFTR` reader"]
pub type R = crate::R<RtcShiftrSpec>;
#[doc = "Register `RTC_SHIFTR` writer"]
pub type W = crate::W<RtcShiftrSpec>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). The value which is written to SUBFS is added to the synchronous prescalers counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / ( PREDIV_S + 1 ) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by : Advance (seconds) = ( 1 - ( SUBFS / ( PREDIV_S + 1 ) ) ) ."]
pub type SubfsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADD1S` writer - Add one second 0: No effect 1: Add one second to the clock/calendar This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
pub type Add1sW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). The value which is written to SUBFS is added to the synchronous prescalers counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / ( PREDIV_S + 1 ) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by : Advance (seconds) = ( 1 - ( SUBFS / ( PREDIV_S + 1 ) ) ) ."]
    #[inline(always)]
    pub fn subfs(&mut self) -> SubfsW<'_, RtcShiftrSpec> {
        SubfsW::new(self, 0)
    }
    #[doc = "Bit 31 - Add one second 0: No effect 1: Add one second to the clock/calendar This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
    #[inline(always)]
    pub fn add1s(&mut self) -> Add1sW<'_, RtcShiftrSpec> {
        Add1sW::new(self, 31)
    }
}
#[doc = "RTC_SHIFTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_shiftr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_shiftr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcShiftrSpec;
impl crate::RegisterSpec for RtcShiftrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_shiftr::R`](R) reader structure"]
impl crate::Readable for RtcShiftrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_shiftr::W`](W) writer structure"]
impl crate::Writable for RtcShiftrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_SHIFTR to value 0"]
impl crate::Resettable for RtcShiftrSpec {}
