#[doc = "Register `RTC_CR` reader"]
pub type R = crate::R<RtcCrSpec>;
#[doc = "Register `RTC_CR` writer"]
pub type W = crate::W<RtcCrSpec>;
#[doc = "Field `WUCKSEL` reader - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value"]
pub type WuckselR = crate::FieldReader;
#[doc = "Field `WUCKSEL` writer - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value"]
pub type WuckselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDGE` reader - Time-stamp event active edge 0: RTC_TS input rising edge generates a time-stamp event 1: RTC_TS input falling edge generates a time-stamp event TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TsedgeR = crate::BitReader;
#[doc = "Field `TSEDGE` writer - Time-stamp event active edge 0: RTC_TS input rising edge generates a time-stamp event 1: RTC_TS input falling edge generates a time-stamp event TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TsedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
pub type BypshadR = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
pub type BypshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmt {
    #[doc = "0: 24 hour/day format"]
    B0x0 = 0,
    #[doc = "1: AM/PM hour format"]
    B0x1 = 1,
}
impl From<Fmt> for bool {
    #[inline(always)]
    fn from(variant: Fmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMT` reader - Hour format"]
pub type FmtR = crate::BitReader<Fmt>;
impl FmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmt {
        match self.bits {
            false => Fmt::B0x0,
            true => Fmt::B0x1,
        }
    }
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmt::B0x0
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmt::B0x1
    }
}
#[doc = "Field `FMT` writer - Hour format"]
pub type FmtW<'a, REG> = crate::BitWriter<'a, REG, Fmt>;
impl<'a, REG> FmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmt::B0x0)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmt::B0x1)
    }
}
#[doc = "Field `ALRAE` reader - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled"]
pub type AlraeR = crate::BitReader;
#[doc = "Field `ALRAE` writer - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled"]
pub type AlraeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled"]
pub type WuteR = crate::BitReader;
#[doc = "Field `WUTE` writer - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled"]
pub type WuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Timestamp enable 0: Timestamp disable 1: Timestamp enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Timestamp enable 0: Timestamp disable 1: Timestamp enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled"]
pub type AlraieR = crate::BitReader;
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled"]
pub type AlraieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled"]
pub type WutieR = crate::BitReader;
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled"]
pub type WutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Time-stamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsie {
    #[doc = "0: Time-stamp Interrupt disable"]
    B0x0 = 0,
    #[doc = "1: Time-stamp Interrupt enable"]
    B0x1 = 1,
}
impl From<Tsie> for bool {
    #[inline(always)]
    fn from(variant: Tsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TsieR = crate::BitReader<Tsie>;
impl TsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsie {
        match self.bits {
            false => Tsie::B0x0,
            true => Tsie::B0x1,
        }
    }
    #[doc = "Time-stamp Interrupt disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsie::B0x0
    }
    #[doc = "Time-stamp Interrupt enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsie::B0x1
    }
}
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG, Tsie>;
impl<'a, REG> TsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time-stamp Interrupt disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsie::B0x0)
    }
    #[doc = "Time-stamp Interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsie::B0x1)
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0. 0: No effect 1: Adds 1 hour to the current time. This can be used for summer time change"]
pub type Add1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0. 0: No effect 1: Subtracts 1 hour to the current time. This can be used for winter time change."]
pub type Sub1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255)."]
pub type CoselR = crate::BitReader;
#[doc = "Field `COSEL` writer - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255)."]
pub type CoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\])."]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\])."]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled"]
pub type OselR = crate::FieldReader;
#[doc = "Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled"]
pub type OselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COE` reader - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled"]
pub type CoeR = crate::BitReader;
#[doc = "Field `COE` writer - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled"]
pub type CoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSE` reader - Timestamp on internal event enable 0: Internal event timestamp disable 1: Internal event timestamp enable"]
pub type ItseR = crate::BitReader;
#[doc = "Field `ITSE` writer - Timestamp on internal event enable 0: Internal event timestamp disable 1: Internal event timestamp enable"]
pub type ItseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value"]
    #[inline(always)]
    pub fn wucksel(&self) -> WuckselR {
        WuckselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Time-stamp event active edge 0: RTC_TS input rising edge generates a time-stamp event 1: RTC_TS input falling edge generates a time-stamp event TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&self) -> TsedgeR {
        TsedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn bypshad(&self) -> BypshadR {
        BypshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled"]
    #[inline(always)]
    pub fn alrae(&self) -> AlraeR {
        AlraeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled"]
    #[inline(always)]
    pub fn wute(&self) -> WuteR {
        WuteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timestamp enable 0: Timestamp disable 1: Timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn alraie(&self) -> AlraieR {
        AlraieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn wutie(&self) -> WutieR {
        WutieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255)."]
    #[inline(always)]
    pub fn cosel(&self) -> CoselR {
        CoselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\])."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled"]
    #[inline(always)]
    pub fn osel(&self) -> OselR {
        OselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled"]
    #[inline(always)]
    pub fn coe(&self) -> CoeR {
        CoeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timestamp on internal event enable 0: Internal event timestamp disable 1: Internal event timestamp enable"]
    #[inline(always)]
    pub fn itse(&self) -> ItseR {
        ItseR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup clock selection 000: RTC/16 clock is selected 001: RTC/8 clock is selected 010: RTC/4 clock is selected 011: RTC/2 clock is selected 10x: ck_spre (usually 1 Hz) clock is selected 11x: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value"]
    #[inline(always)]
    pub fn wucksel(&mut self) -> WuckselW<'_, RtcCrSpec> {
        WuckselW::new(self, 0)
    }
    #[doc = "Bit 3 - Time-stamp event active edge 0: RTC_TS input rising edge generates a time-stamp event 1: RTC_TS input falling edge generates a time-stamp event TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TsedgeW<'_, RtcCrSpec> {
        TsedgeW::new(self, 3)
    }
    #[doc = "Bit 5 - Bypass the shadow registers 0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles. 1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BypshadW<'_, RtcCrSpec> {
        BypshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FmtW<'_, RtcCrSpec> {
        FmtW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable 0: Alarm A disabled 1: Alarm A enabled"]
    #[inline(always)]
    pub fn alrae(&mut self) -> AlraeW<'_, RtcCrSpec> {
        AlraeW::new(self, 8)
    }
    #[doc = "Bit 10 - Wakeup timer enable 0: Wakeup timer disabled 1: Wakeup timer enabled"]
    #[inline(always)]
    pub fn wute(&mut self) -> WuteW<'_, RtcCrSpec> {
        WuteW::new(self, 10)
    }
    #[doc = "Bit 11 - Timestamp enable 0: Timestamp disable 1: Timestamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<'_, RtcCrSpec> {
        TseW::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable 0: Alarm A interrupt disabled 1: Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn alraie(&mut self) -> AlraieW<'_, RtcCrSpec> {
        AlraieW::new(self, 12)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable 0: Wakeup timer interrupt disabled 1: Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WutieW<'_, RtcCrSpec> {
        WutieW::new(self, 14)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TsieW<'_, RtcCrSpec> {
        TsieW::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0. 0: No effect 1: Adds 1 hour to the current time. This can be used for summer time change"]
    #[inline(always)]
    pub fn add1h(&mut self) -> Add1hW<'_, RtcCrSpec> {
        Add1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0. 0: No effect 1: Subtracts 1 hour to the current time. This can be used for winter time change."]
    #[inline(always)]
    pub fn sub1h(&mut self) -> Sub1hW<'_, RtcCrSpec> {
        Sub1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, RtcCrSpec> {
        BkpW::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. 0: Calibration output is 512 Hz 1: Calibration output is 1 Hz These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255)."]
    #[inline(always)]
    pub fn cosel(&mut self) -> CoselW<'_, RtcCrSpec> {
        CoselW::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output 0: The pin is high when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\]) 1: The pin is low when ALRAF/WUTF is asserted (depending on OSEL\\[1:0\\])."]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, RtcCrSpec> {
        PolW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output 00: Output disabled 01: Alarm A output enabled 10: Reserved 11: Wakeup output enabled"]
    #[inline(always)]
    pub fn osel(&mut self) -> OselW<'_, RtcCrSpec> {
        OselW::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the RTC_CALIB output 0: Calibration output disabled 1: Calibration output enabled"]
    #[inline(always)]
    pub fn coe(&mut self) -> CoeW<'_, RtcCrSpec> {
        CoeW::new(self, 23)
    }
    #[doc = "Bit 24 - Timestamp on internal event enable 0: Internal event timestamp disable 1: Internal event timestamp enable"]
    #[inline(always)]
    pub fn itse(&mut self) -> ItseW<'_, RtcCrSpec> {
        ItseW::new(self, 24)
    }
}
#[doc = "RTC_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCrSpec;
impl crate::RegisterSpec for RtcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_cr::R`](R) reader structure"]
impl crate::Readable for RtcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cr::W`](W) writer structure"]
impl crate::Writable for RtcCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CR to value 0"]
impl crate::Resettable for RtcCrSpec {}
