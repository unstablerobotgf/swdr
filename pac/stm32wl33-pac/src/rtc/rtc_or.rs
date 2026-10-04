#[doc = "Register `RTC_OR` reader"]
pub type R = crate::R<RtcOrSpec>;
#[doc = "Register `RTC_OR` writer"]
pub type W = crate::W<RtcOrSpec>;
#[doc = "RTC_ALARM on PA8 output type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alarmouttype {
    #[doc = "0: RTC_ALARM, when mapped on PA8, is open-drain output"]
    B0x0 = 0,
    #[doc = "1: RTC_ALARM, when mapped on PA8, is push-pull output"]
    B0x1 = 1,
}
impl From<Alarmouttype> for bool {
    #[inline(always)]
    fn from(variant: Alarmouttype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARMOUTTYPE` reader - RTC_ALARM on PA8 output type"]
pub type AlarmouttypeR = crate::BitReader<Alarmouttype>;
impl AlarmouttypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alarmouttype {
        match self.bits {
            false => Alarmouttype::B0x0,
            true => Alarmouttype::B0x1,
        }
    }
    #[doc = "RTC_ALARM, when mapped on PA8, is open-drain output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Alarmouttype::B0x0
    }
    #[doc = "RTC_ALARM, when mapped on PA8, is push-pull output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Alarmouttype::B0x1
    }
}
#[doc = "Field `ALARMOUTTYPE` writer - RTC_ALARM on PA8 output type"]
pub type AlarmouttypeW<'a, REG> = crate::BitWriter<'a, REG, Alarmouttype>;
impl<'a, REG> AlarmouttypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_ALARM, when mapped on PA8, is open-drain output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Alarmouttype::B0x0)
    }
    #[doc = "RTC_ALARM, when mapped on PA8, is push-pull output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Alarmouttype::B0x1)
    }
}
#[doc = "Field `RTC_OUT_RMP` reader - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8."]
pub type RtcOutRmpR = crate::BitReader;
#[doc = "Field `RTC_OUT_RMP` writer - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8."]
pub type RtcOutRmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC_ALARM on PA8 output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> AlarmouttypeR {
        AlarmouttypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8."]
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RtcOutRmpR {
        RtcOutRmpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_ALARM on PA8 output type"]
    #[inline(always)]
    pub fn alarmouttype(&mut self) -> AlarmouttypeW<'_, RtcOrSpec> {
        AlarmouttypeW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8."]
    #[inline(always)]
    pub fn rtc_out_rmp(&mut self) -> RtcOutRmpW<'_, RtcOrSpec> {
        RtcOutRmpW::new(self, 1)
    }
}
#[doc = "RTC_OR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcOrSpec;
impl crate::RegisterSpec for RtcOrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_or::R`](R) reader structure"]
impl crate::Readable for RtcOrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_or::W`](W) writer structure"]
impl crate::Writable for RtcOrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_OR to value 0"]
impl crate::Resettable for RtcOrSpec {}
