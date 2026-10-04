#[doc = "Register `AGC3_CTRL` reader"]
pub type R = crate::R<Agc3CtrlSpec>;
#[doc = "Register `AGC3_CTRL` writer"]
pub type W = crate::W<Agc3CtrlSpec>;
#[doc = "Field `AGC_MIN_ATTEN` reader - Minimum AGC attenuation."]
pub type AgcMinAttenR = crate::FieldReader;
#[doc = "Field `AGC_MIN_ATTEN` writer - Minimum AGC attenuation."]
pub type AgcMinAttenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AGC_MAX_ATTEN` reader - Maximum AGC attenuation."]
pub type AgcMaxAttenR = crate::FieldReader;
#[doc = "Field `AGC_MAX_ATTEN` writer - Maximum AGC attenuation."]
pub type AgcMaxAttenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Minimum AGC attenuation."]
    #[inline(always)]
    pub fn agc_min_atten(&self) -> AgcMinAttenR {
        AgcMinAttenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Maximum AGC attenuation."]
    #[inline(always)]
    pub fn agc_max_atten(&self) -> AgcMaxAttenR {
        AgcMaxAttenR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Minimum AGC attenuation."]
    #[inline(always)]
    pub fn agc_min_atten(&mut self) -> AgcMinAttenW<'_, Agc3CtrlSpec> {
        AgcMinAttenW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Maximum AGC attenuation."]
    #[inline(always)]
    pub fn agc_max_atten(&mut self) -> AgcMaxAttenW<'_, Agc3CtrlSpec> {
        AgcMaxAttenW::new(self, 4)
    }
}
#[doc = "AGC3_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agc3CtrlSpec;
impl crate::RegisterSpec for Agc3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc3_ctrl::R`](R) reader structure"]
impl crate::Readable for Agc3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`agc3_ctrl::W`](W) writer structure"]
impl crate::Writable for Agc3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC3_CTRL to value 0x90"]
impl crate::Resettable for Agc3CtrlSpec {
    const RESET_VALUE: u32 = 0x90;
}
