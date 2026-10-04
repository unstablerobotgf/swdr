#[doc = "Register `RTC_TSDR` reader"]
pub type R = crate::R<RtcTsdrSpec>;
#[doc = "Register `RTC_TSDR` writer"]
pub type W = crate::W<RtcTsdrSpec>;
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
#[doc = "Field `WDU` reader - Week day units"]
pub type WduR = crate::FieldReader;
#[doc = "Field `WDU` writer - Week day units"]
pub type WduW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&self) -> WduR {
        WduR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format."]
    #[inline(always)]
    pub fn du(&mut self) -> DuW<'_, RtcTsdrSpec> {
        DuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format."]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, RtcTsdrSpec> {
        DtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format."]
    #[inline(always)]
    pub fn mu(&mut self) -> MuW<'_, RtcTsdrSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format."]
    #[inline(always)]
    pub fn mt(&mut self) -> MtW<'_, RtcTsdrSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&mut self) -> WduW<'_, RtcTsdrSpec> {
        WduW::new(self, 13)
    }
}
#[doc = "RTC_TSDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tsdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTsdrSpec;
impl crate::RegisterSpec for RtcTsdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tsdr::R`](R) reader structure"]
impl crate::Readable for RtcTsdrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_tsdr::W`](W) writer structure"]
impl crate::Writable for RtcTsdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_TSDR to value 0"]
impl crate::Resettable for RtcTsdrSpec {}
