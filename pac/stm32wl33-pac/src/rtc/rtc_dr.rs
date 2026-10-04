#[doc = "Register `RTC_DR` reader"]
pub type R = crate::R<RtcDrSpec>;
#[doc = "Register `RTC_DR` writer"]
pub type W = crate::W<RtcDrSpec>;
#[doc = "Field `DU` reader - Date units in BCD format."]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units in BCD format."]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format."]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format."]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MU` reader - Month units in BCD format."]
pub type MuR = crate::FieldReader;
#[doc = "Field `MU` writer - Month units in BCD format."]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Month tens in BCD format."]
pub type MtR = crate::BitReader;
#[doc = "Field `MT` writer - Month tens in BCD format."]
pub type MtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDU` reader - Week day units 000: forbidden 001: Monday 010: Tuesday 011: Wednesday 100: Thursday 101: Friday 110: Saturday 111: Sunday"]
pub type WduR = crate::FieldReader;
#[doc = "Field `WDU` writer - Week day units 000: forbidden 001: Monday 010: Tuesday 011: Wednesday 100: Thursday 101: Friday 110: Saturday 111: Sunday"]
pub type WduW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `YU` reader - Year units in BCD format."]
pub type YuR = crate::FieldReader;
#[doc = "Field `YU` writer - Year units in BCD format."]
pub type YuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YT` reader - Year tens in BCD format."]
pub type YtR = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens in BCD format."]
pub type YtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format."]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format."]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format."]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units 000: forbidden 001: Monday 010: Tuesday 011: Wednesday 100: Thursday 101: Friday 110: Saturday 111: Sunday"]
    #[inline(always)]
    pub fn wdu(&self) -> WduR {
        WduR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD format."]
    #[inline(always)]
    pub fn yu(&self) -> YuR {
        YuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format."]
    #[inline(always)]
    pub fn yt(&self) -> YtR {
        YtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format."]
    #[inline(always)]
    pub fn du(&mut self) -> DuW<'_, RtcDrSpec> {
        DuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, RtcDrSpec> {
        DtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format."]
    #[inline(always)]
    pub fn mu(&mut self) -> MuW<'_, RtcDrSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format."]
    #[inline(always)]
    pub fn mt(&mut self) -> MtW<'_, RtcDrSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units 000: forbidden 001: Monday 010: Tuesday 011: Wednesday 100: Thursday 101: Friday 110: Saturday 111: Sunday"]
    #[inline(always)]
    pub fn wdu(&mut self) -> WduW<'_, RtcDrSpec> {
        WduW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units in BCD format."]
    #[inline(always)]
    pub fn yu(&mut self) -> YuW<'_, RtcDrSpec> {
        YuW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format."]
    #[inline(always)]
    pub fn yt(&mut self) -> YtW<'_, RtcDrSpec> {
        YtW::new(self, 20)
    }
}
#[doc = "RTC_DR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcDrSpec;
impl crate::RegisterSpec for RtcDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_dr::R`](R) reader structure"]
impl crate::Readable for RtcDrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_dr::W`](W) writer structure"]
impl crate::Writable for RtcDrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_DR to value 0x2101"]
impl crate::Resettable for RtcDrSpec {
    const RESET_VALUE: u32 = 0x2101;
}
