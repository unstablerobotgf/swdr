#[doc = "Register `AGC_ATTEN3` reader"]
pub type R = crate::R<AgcAtten3Spec>;
#[doc = "Register `AGC_ATTEN3` writer"]
pub type W = crate::W<AgcAtten3Spec>;
#[doc = "Field `ATTEN_AGCGAIN_3` reader - AGC attenuation control setting for step 3 for LNA+BOM point."]
pub type AttenAgcgain3R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_3` writer - AGC attenuation control setting for step 3 for LNA+BOM point."]
pub type AttenAgcgain3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_3` reader - AGC attenuation control setting for step 3 on PGA point."]
pub type PgaAgcgain3R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_3` writer - AGC attenuation control setting for step 3 on PGA point."]
pub type PgaAgcgain3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 3 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_3(&self) -> AttenAgcgain3R {
        AttenAgcgain3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 3 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_3(&self) -> PgaAgcgain3R {
        PgaAgcgain3R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 3 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_3(&mut self) -> AttenAgcgain3W<'_, AgcAtten3Spec> {
        AttenAgcgain3W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 3 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_3(&mut self) -> PgaAgcgain3W<'_, AgcAtten3Spec> {
        PgaAgcgain3W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten3Spec;
impl crate::RegisterSpec for AgcAtten3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten3::R`](R) reader structure"]
impl crate::Readable for AgcAtten3Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten3::W`](W) writer structure"]
impl crate::Writable for AgcAtten3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN3 to value 0x30"]
impl crate::Resettable for AgcAtten3Spec {
    const RESET_VALUE: u32 = 0x30;
}
