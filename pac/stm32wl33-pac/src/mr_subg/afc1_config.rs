#[doc = "Register `AFC1_CONFIG` reader"]
pub type R = crate::R<Afc1ConfigSpec>;
#[doc = "Register `AFC1_CONFIG` writer"]
pub type W = crate::W<Afc1ConfigSpec>;
#[doc = "Field `AFC_FAST_PERIOD` reader - Length of the AFC fast period (in number of samples unit)"]
pub type AfcFastPeriodR = crate::FieldReader;
#[doc = "Field `AFC_FAST_PERIOD` writer - Length of the AFC fast period (in number of samples unit)"]
pub type AfcFastPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Length of the AFC fast period (in number of samples unit)"]
    #[inline(always)]
    pub fn afc_fast_period(&self) -> AfcFastPeriodR {
        AfcFastPeriodR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Length of the AFC fast period (in number of samples unit)"]
    #[inline(always)]
    pub fn afc_fast_period(&mut self) -> AfcFastPeriodW<'_, Afc1ConfigSpec> {
        AfcFastPeriodW::new(self, 0)
    }
}
#[doc = "AFC1_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afc1ConfigSpec;
impl crate::RegisterSpec for Afc1ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afc1_config::R`](R) reader structure"]
impl crate::Readable for Afc1ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`afc1_config::W`](W) writer structure"]
impl crate::Writable for Afc1ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFC1_CONFIG to value 0x18"]
impl crate::Resettable for Afc1ConfigSpec {
    const RESET_VALUE: u32 = 0x18;
}
