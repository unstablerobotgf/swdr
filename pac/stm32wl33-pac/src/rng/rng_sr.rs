#[doc = "Register `RNG_SR` reader"]
pub type R = crate::R<RngSrSpec>;
#[doc = "Register `RNG_SR` writer"]
pub type W = crate::W<RngSrSpec>;
#[doc = "Field `RNGRDY` reader - New Random Value Ready."]
pub type RngrdyR = crate::BitReader;
#[doc = "Field `REVCLK` reader - RNGCLK Clock Reveal bit."]
pub type RevclkR = crate::BitReader;
#[doc = "Field `FAULT` reader - Fault Reveal bit."]
pub type FaultR = crate::BitReader;
#[doc = "Field `FAULT` writer - Fault Reveal bit."]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - New Random Value Ready."]
    #[inline(always)]
    pub fn rngrdy(&self) -> RngrdyR {
        RngrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RNGCLK Clock Reveal bit."]
    #[inline(always)]
    pub fn revclk(&self) -> RevclkR {
        RevclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Reveal bit."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Fault Reveal bit."]
    #[inline(always)]
    pub fn fault(&mut self) -> FaultW<'_, RngSrSpec> {
        FaultW::new(self, 2)
    }
}
#[doc = "RNG_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngSrSpec;
impl crate::RegisterSpec for RngSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_sr::R`](R) reader structure"]
impl crate::Readable for RngSrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_sr::W`](W) writer structure"]
impl crate::Writable for RngSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_SR to value 0"]
impl crate::Resettable for RngSrSpec {}
