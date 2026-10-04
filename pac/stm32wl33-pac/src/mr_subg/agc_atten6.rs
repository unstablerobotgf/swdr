#[doc = "Register `AGC_ATTEN6` reader"]
pub type R = crate::R<AgcAtten6Spec>;
#[doc = "Register `AGC_ATTEN6` writer"]
pub type W = crate::W<AgcAtten6Spec>;
#[doc = "Field `ATTEN_AGCGAIN_6` reader - AGC attenuation control setting for step 6 for LNA+BOM point."]
pub type AttenAgcgain6R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_6` writer - AGC attenuation control setting for step 6 for LNA+BOM point."]
pub type AttenAgcgain6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_6` reader - AGC attenuation control setting for step 6 on PGA point."]
pub type PgaAgcgain6R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_6` writer - AGC attenuation control setting for step 6 on PGA point."]
pub type PgaAgcgain6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 6 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_6(&self) -> AttenAgcgain6R {
        AttenAgcgain6R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 6 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_6(&self) -> PgaAgcgain6R {
        PgaAgcgain6R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 6 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_6(&mut self) -> AttenAgcgain6W<'_, AgcAtten6Spec> {
        AttenAgcgain6W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 6 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_6(&mut self) -> PgaAgcgain6W<'_, AgcAtten6Spec> {
        PgaAgcgain6W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten6Spec;
impl crate::RegisterSpec for AgcAtten6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten6::R`](R) reader structure"]
impl crate::Readable for AgcAtten6Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten6::W`](W) writer structure"]
impl crate::Writable for AgcAtten6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN6 to value 0x43"]
impl crate::Resettable for AgcAtten6Spec {
    const RESET_VALUE: u32 = 0x43;
}
