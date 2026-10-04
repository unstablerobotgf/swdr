#[doc = "Register `RTC_BKP0R` reader"]
pub type R = crate::R<RtcBkp0rSpec>;
#[doc = "Register `RTC_BKP0R` writer"]
pub type W = crate::W<RtcBkp0rSpec>;
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. They are powered-on by VDD12o so they are retained during DEEPSTOP mode. The application can write or read data to and from these registers. This register is reset on PORESETn only."]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. They are powered-on by VDD12o so they are retained during DEEPSTOP mode. The application can write or read data to and from these registers. This register is reset on PORESETn only."]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VDD12o so they are retained during DEEPSTOP mode. The application can write or read data to and from these registers. This register is reset on PORESETn only."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VDD12o so they are retained during DEEPSTOP mode. The application can write or read data to and from these registers. This register is reset on PORESETn only."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, RtcBkp0rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "RTC_BKPxR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcBkp0rSpec;
impl crate::RegisterSpec for RtcBkp0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_bkp0r::R`](R) reader structure"]
impl crate::Readable for RtcBkp0rSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_bkp0r::W`](W) writer structure"]
impl crate::Writable for RtcBkp0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_BKP0R to value 0"]
impl crate::Resettable for RtcBkp0rSpec {}
