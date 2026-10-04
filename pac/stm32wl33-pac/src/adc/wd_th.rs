#[doc = "Register `WD_TH` reader"]
pub type R = crate::R<WdThSpec>;
#[doc = "Register `WD_TH` writer"]
pub type W = crate::W<WdThSpec>;
#[doc = "Field `WD_LT` reader - WD_LT\\[11:0\\]: analog watchdog low level threshold."]
pub type WdLtR = crate::FieldReader<u16>;
#[doc = "Field `WD_LT` writer - WD_LT\\[11:0\\]: analog watchdog low level threshold."]
pub type WdLtW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WD_HT` reader - WD_HT\\[11:0\\]: analog watchdog high level threshold."]
pub type WdHtR = crate::FieldReader<u16>;
#[doc = "Field `WD_HT` writer - WD_HT\\[11:0\\]: analog watchdog high level threshold."]
pub type WdHtW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - WD_LT\\[11:0\\]: analog watchdog low level threshold."]
    #[inline(always)]
    pub fn wd_lt(&self) -> WdLtR {
        WdLtR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WD_HT\\[11:0\\]: analog watchdog high level threshold."]
    #[inline(always)]
    pub fn wd_ht(&self) -> WdHtR {
        WdHtR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WD_LT\\[11:0\\]: analog watchdog low level threshold."]
    #[inline(always)]
    pub fn wd_lt(&mut self) -> WdLtW<'_, WdThSpec> {
        WdLtW::new(self, 0)
    }
    #[doc = "Bits 16:27 - WD_HT\\[11:0\\]: analog watchdog high level threshold."]
    #[inline(always)]
    pub fn wd_ht(&mut self) -> WdHtW<'_, WdThSpec> {
        WdHtW::new(self, 16)
    }
}
#[doc = "WD_TH register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdThSpec;
impl crate::RegisterSpec for WdThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd_th::R`](R) reader structure"]
impl crate::Readable for WdThSpec {}
#[doc = "`write(|w| ..)` method takes [`wd_th::W`](W) writer structure"]
impl crate::Writable for WdThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD_TH to value 0x0fff_0000"]
impl crate::Resettable for WdThSpec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
