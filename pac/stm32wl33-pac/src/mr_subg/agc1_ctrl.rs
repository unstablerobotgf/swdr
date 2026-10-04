#[doc = "Register `AGC1_CTRL` reader"]
pub type R = crate::R<Agc1CtrlSpec>;
#[doc = "Register `AGC1_CTRL` writer"]
pub type W = crate::W<Agc1CtrlSpec>;
#[doc = "Field `AGC_MIN_THR` reader - Minimum signal threshold."]
pub type AgcMinThrR = crate::FieldReader;
#[doc = "Field `AGC_MIN_THR` writer - Minimum signal threshold."]
pub type AgcMinThrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AGC_MAX_THR` reader - Maximum signal threshold."]
pub type AgcMaxThrR = crate::FieldReader;
#[doc = "Field `AGC_MAX_THR` writer - Maximum signal threshold."]
pub type AgcMaxThrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Minimum signal threshold."]
    #[inline(always)]
    pub fn agc_min_thr(&self) -> AgcMinThrR {
        AgcMinThrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Maximum signal threshold."]
    #[inline(always)]
    pub fn agc_max_thr(&self) -> AgcMaxThrR {
        AgcMaxThrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Minimum signal threshold."]
    #[inline(always)]
    pub fn agc_min_thr(&mut self) -> AgcMinThrW<'_, Agc1CtrlSpec> {
        AgcMinThrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Maximum signal threshold."]
    #[inline(always)]
    pub fn agc_max_thr(&mut self) -> AgcMaxThrW<'_, Agc1CtrlSpec> {
        AgcMaxThrW::new(self, 4)
    }
}
#[doc = "AGC1_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agc1CtrlSpec;
impl crate::RegisterSpec for Agc1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc1_ctrl::R`](R) reader structure"]
impl crate::Readable for Agc1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`agc1_ctrl::W`](W) writer structure"]
impl crate::Writable for Agc1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC1_CTRL to value 0x62"]
impl crate::Resettable for Agc1CtrlSpec {
    const RESET_VALUE: u32 = 0x62;
}
