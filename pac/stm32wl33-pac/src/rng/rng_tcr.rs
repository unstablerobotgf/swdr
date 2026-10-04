#[doc = "Register `RNG_TCR` reader"]
pub type R = crate::R<RngTcrSpec>;
#[doc = "Register `RNG_TCR` writer"]
pub type W = crate::W<RngTcrSpec>;
#[doc = "Field `TCR` reader - Test-control register"]
pub type TcrR = crate::BitReader;
#[doc = "Field `TCR` writer - Test-control register"]
pub type TcrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Test-control register"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test-control register"]
    #[inline(always)]
    pub fn tcr(&mut self) -> TcrW<'_, RngTcrSpec> {
        TcrW::new(self, 0)
    }
}
#[doc = "RNG_TCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngTcrSpec;
impl crate::RegisterSpec for RngTcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_tcr::R`](R) reader structure"]
impl crate::Readable for RngTcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_tcr::W`](W) writer structure"]
impl crate::Writable for RngTcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_TCR to value 0"]
impl crate::Resettable for RngTcrSpec {}
