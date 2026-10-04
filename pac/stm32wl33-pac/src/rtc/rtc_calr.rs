#[doc = "Register `RTC_CALR` reader"]
pub type R = crate::R<RtcCalrSpec>;
#[doc = "Register `RTC_CALR` writer"]
pub type W = crate::W<RtcCalrSpec>;
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP."]
pub type CalmR = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP."]
pub type CalmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1 , the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\] is stucked at 0 when CALW16=1."]
pub type Calw16R = crate::BitReader;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1 , the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\] is stucked at 0 when CALW16=1."]
pub type Calw16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1 , the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stucked at '00' when CALW8=1."]
pub type Calw8R = crate::BitReader;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1 , the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stucked at '00' when CALW8=1."]
pub type Calw8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm 0: No RTCCLK pulses are added. 1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency incresed by 488.5 ppm). This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM."]
pub type CalpR = crate::BitReader;
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm 0: No RTCCLK pulses are added. 1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency incresed by 488.5 ppm). This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM."]
pub type CalpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP."]
    #[inline(always)]
    pub fn calm(&self) -> CalmR {
        CalmR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1 , the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\] is stucked at 0 when CALW16=1."]
    #[inline(always)]
    pub fn calw16(&self) -> Calw16R {
        Calw16R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1 , the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stucked at '00' when CALW8=1."]
    #[inline(always)]
    pub fn calw8(&self) -> Calw8R {
        Calw8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm 0: No RTCCLK pulses are added. 1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency incresed by 488.5 ppm). This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM."]
    #[inline(always)]
    pub fn calp(&self) -> CalpR {
        CalpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP."]
    #[inline(always)]
    pub fn calm(&mut self) -> CalmW<'_, RtcCalrSpec> {
        CalmW::new(self, 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1 , the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\] is stucked at 0 when CALW16=1."]
    #[inline(always)]
    pub fn calw16(&mut self) -> Calw16W<'_, RtcCalrSpec> {
        Calw16W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1 , the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stucked at '00' when CALW8=1."]
    #[inline(always)]
    pub fn calw8(&mut self) -> Calw8W<'_, RtcCalrSpec> {
        Calw8W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm 0: No RTCCLK pulses are added. 1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency incresed by 488.5 ppm). This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM."]
    #[inline(always)]
    pub fn calp(&mut self) -> CalpW<'_, RtcCalrSpec> {
        CalpW::new(self, 15)
    }
}
#[doc = "RTC_CALR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCalrSpec;
impl crate::RegisterSpec for RtcCalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_calr::R`](R) reader structure"]
impl crate::Readable for RtcCalrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_calr::W`](W) writer structure"]
impl crate::Writable for RtcCalrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CALR to value 0"]
impl crate::Resettable for RtcCalrSpec {}
