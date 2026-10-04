#[doc = "Register `RTC_TSTR` reader"]
pub type R = crate::R<RtcTstrSpec>;
#[doc = "Register `RTC_TSTR` writer"]
pub type W = crate::W<RtcTstrSpec>;
#[doc = "Field `SU` reader - Second units in BCD format."]
pub type SuR = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format."]
pub type SuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format."]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format."]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MNU` reader - Minute units in BCD format."]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format."]
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format."]
pub type MntR = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format."]
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HU` reader - Hour units in BCD format."]
pub type HuR = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format."]
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format."]
pub type HtR = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format."]
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM notation 0: AM or 24-hour format 1: PM"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation 0: AM or 24-hour format 1: PM"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation 0: AM or 24-hour format 1: PM"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn su(&mut self) -> SuW<'_, RtcTstrSpec> {
        SuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn st(&mut self) -> StW<'_, RtcTstrSpec> {
        StW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&mut self) -> MnuW<'_, RtcTstrSpec> {
        MnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&mut self) -> MntW<'_, RtcTstrSpec> {
        MntW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hu(&mut self) -> HuW<'_, RtcTstrSpec> {
        HuW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn ht(&mut self) -> HtW<'_, RtcTstrSpec> {
        HtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation 0: AM or 24-hour format 1: PM"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, RtcTstrSpec> {
        PmW::new(self, 22)
    }
}
#[doc = "RTC_TSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTstrSpec;
impl crate::RegisterSpec for RtcTstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tstr::R`](R) reader structure"]
impl crate::Readable for RtcTstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_tstr::W`](W) writer structure"]
impl crate::Writable for RtcTstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_TSTR to value 0"]
impl crate::Resettable for RtcTstrSpec {}
