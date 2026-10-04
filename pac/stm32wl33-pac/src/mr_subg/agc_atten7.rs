#[doc = "Register `AGC_ATTEN7` reader"]
pub type R = crate::R<AgcAtten7Spec>;
#[doc = "Register `AGC_ATTEN7` writer"]
pub type W = crate::W<AgcAtten7Spec>;
#[doc = "Field `ATTEN_AGCGAIN_7` reader - AGC attenuation control setting for step 7 for LNA+BOM point."]
pub type AttenAgcgain7R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_7` writer - AGC attenuation control setting for step 7 for LNA+BOM point."]
pub type AttenAgcgain7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_7` reader - AGC attenuation control setting for step 7 on PGA point."]
pub type PgaAgcgain7R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_7` writer - AGC attenuation control setting for step 7 on PGA point."]
pub type PgaAgcgain7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 7 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_7(&self) -> AttenAgcgain7R {
        AttenAgcgain7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 7 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_7(&self) -> PgaAgcgain7R {
        PgaAgcgain7R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 7 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_7(&mut self) -> AttenAgcgain7W<'_, AgcAtten7Spec> {
        AttenAgcgain7W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 7 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_7(&mut self) -> PgaAgcgain7W<'_, AgcAtten7Spec> {
        PgaAgcgain7W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten7Spec;
impl crate::RegisterSpec for AgcAtten7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten7::R`](R) reader structure"]
impl crate::Readable for AgcAtten7Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten7::W`](W) writer structure"]
impl crate::Writable for AgcAtten7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN7 to value 0x47"]
impl crate::Resettable for AgcAtten7Spec {
    const RESET_VALUE: u32 = 0x47;
}
