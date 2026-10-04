#[doc = "Register `RNG_CR` reader"]
pub type R = crate::R<RngCrSpec>;
#[doc = "Register `RNG_CR` writer"]
pub type W = crate::W<RngCrSpec>;
#[doc = "Field `RNG_DIS` reader - RNG Disable bit."]
pub type RngDisR = crate::BitReader;
#[doc = "Field `RNG_DIS` writer - RNG Disable bit."]
pub type RngDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_CLK` reader - RNG Test Clock bit."]
pub type TstClkR = crate::BitReader;
#[doc = "Field `TST_CLK` writer - RNG Test Clock bit."]
pub type TstClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - RNG Disable bit."]
    #[inline(always)]
    pub fn rng_dis(&self) -> RngDisR {
        RngDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNG Test Clock bit."]
    #[inline(always)]
    pub fn tst_clk(&self) -> TstClkR {
        TstClkR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RNG Disable bit."]
    #[inline(always)]
    pub fn rng_dis(&mut self) -> RngDisW<'_, RngCrSpec> {
        RngDisW::new(self, 2)
    }
    #[doc = "Bit 3 - RNG Test Clock bit."]
    #[inline(always)]
    pub fn tst_clk(&mut self) -> TstClkW<'_, RngCrSpec> {
        TstClkW::new(self, 3)
    }
}
#[doc = "RNG_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngCrSpec;
impl crate::RegisterSpec for RngCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_cr::R`](R) reader structure"]
impl crate::Readable for RngCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_cr::W`](W) writer structure"]
impl crate::Writable for RngCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_CR to value 0"]
impl crate::Resettable for RngCrSpec {}
