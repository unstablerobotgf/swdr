#[doc = "Register `RNG_ITIP` reader"]
pub type R = crate::R<RngItipSpec>;
#[doc = "Register `RNG_ITIP` writer"]
pub type W = crate::W<RngItipSpec>;
#[doc = "Field `ITIP` reader - Integration-test input register"]
pub type ItipR = crate::BitReader;
#[doc = "Field `ITIP` writer - Integration-test input register"]
pub type ItipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration-test input register"]
    #[inline(always)]
    pub fn itip(&self) -> ItipR {
        ItipR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration-test input register"]
    #[inline(always)]
    pub fn itip(&mut self) -> ItipW<'_, RngItipSpec> {
        ItipW::new(self, 0)
    }
}
#[doc = "RNG_ITIP register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_itip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_itip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngItipSpec;
impl crate::RegisterSpec for RngItipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_itip::R`](R) reader structure"]
impl crate::Readable for RngItipSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_itip::W`](W) writer structure"]
impl crate::Writable for RngItipSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_ITIP to value 0"]
impl crate::Resettable for RngItipSpec {}
