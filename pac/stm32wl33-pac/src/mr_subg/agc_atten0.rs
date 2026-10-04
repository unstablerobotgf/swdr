#[doc = "Register `AGC_ATTEN0` reader"]
pub type R = crate::R<AgcAtten0Spec>;
#[doc = "Register `AGC_ATTEN0` writer"]
pub type W = crate::W<AgcAtten0Spec>;
#[doc = "Field `ATTEN_AGCGAIN_0` reader - AGC attenuation control setting for step 0 for LNA+BOM point."]
pub type AttenAgcgain0R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_0` writer - AGC attenuation control setting for step 0 for LNA+BOM point."]
pub type AttenAgcgain0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_0` reader - AGC attenuation control setting for step 0 on PGA point."]
pub type PgaAgcgain0R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_0` writer - AGC attenuation control setting for step 0 on PGA point."]
pub type PgaAgcgain0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 0 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_0(&self) -> AttenAgcgain0R {
        AttenAgcgain0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 0 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_0(&self) -> PgaAgcgain0R {
        PgaAgcgain0R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 0 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_0(&mut self) -> AttenAgcgain0W<'_, AgcAtten0Spec> {
        AttenAgcgain0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 0 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_0(&mut self) -> PgaAgcgain0W<'_, AgcAtten0Spec> {
        PgaAgcgain0W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten0Spec;
impl crate::RegisterSpec for AgcAtten0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten0::R`](R) reader structure"]
impl crate::Readable for AgcAtten0Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten0::W`](W) writer structure"]
impl crate::Writable for AgcAtten0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN0 to value 0"]
impl crate::Resettable for AgcAtten0Spec {}
