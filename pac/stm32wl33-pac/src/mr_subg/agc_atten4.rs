#[doc = "Register `AGC_ATTEN4` reader"]
pub type R = crate::R<AgcAtten4Spec>;
#[doc = "Register `AGC_ATTEN4` writer"]
pub type W = crate::W<AgcAtten4Spec>;
#[doc = "Field `ATTEN_AGCGAIN_4` reader - AGC attenuation control setting for step 4 for LNA+BOM point."]
pub type AttenAgcgain4R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_4` writer - AGC attenuation control setting for step 4 for LNA+BOM point."]
pub type AttenAgcgain4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_4` reader - AGC attenuation control setting for step 4 on PGA point."]
pub type PgaAgcgain4R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_4` writer - AGC attenuation control setting for step 4 on PGA point."]
pub type PgaAgcgain4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 4 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_4(&self) -> AttenAgcgain4R {
        AttenAgcgain4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 4 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_4(&self) -> PgaAgcgain4R {
        PgaAgcgain4R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 4 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_4(&mut self) -> AttenAgcgain4W<'_, AgcAtten4Spec> {
        AttenAgcgain4W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 4 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_4(&mut self) -> PgaAgcgain4W<'_, AgcAtten4Spec> {
        PgaAgcgain4W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten4Spec;
impl crate::RegisterSpec for AgcAtten4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten4::R`](R) reader structure"]
impl crate::Readable for AgcAtten4Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten4::W`](W) writer structure"]
impl crate::Writable for AgcAtten4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN4 to value 0x40"]
impl crate::Resettable for AgcAtten4Spec {
    const RESET_VALUE: u32 = 0x40;
}
