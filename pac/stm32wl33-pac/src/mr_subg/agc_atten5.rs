#[doc = "Register `AGC_ATTEN5` reader"]
pub type R = crate::R<AgcAtten5Spec>;
#[doc = "Register `AGC_ATTEN5` writer"]
pub type W = crate::W<AgcAtten5Spec>;
#[doc = "Field `ATTEN_AGCGAIN_5` reader - AGC attenuation control setting for step 5 for LNA+BOM point."]
pub type AttenAgcgain5R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_5` writer - AGC attenuation control setting for step 5 for LNA+BOM point."]
pub type AttenAgcgain5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_5` reader - AGC attenuation control setting for step 5 on PGA point."]
pub type PgaAgcgain5R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_5` writer - AGC attenuation control setting for step 5 on PGA point."]
pub type PgaAgcgain5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 5 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_5(&self) -> AttenAgcgain5R {
        AttenAgcgain5R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 5 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_5(&self) -> PgaAgcgain5R {
        PgaAgcgain5R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 5 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_5(&mut self) -> AttenAgcgain5W<'_, AgcAtten5Spec> {
        AttenAgcgain5W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 5 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_5(&mut self) -> PgaAgcgain5W<'_, AgcAtten5Spec> {
        PgaAgcgain5W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten5Spec;
impl crate::RegisterSpec for AgcAtten5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten5::R`](R) reader structure"]
impl crate::Readable for AgcAtten5Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten5::W`](W) writer structure"]
impl crate::Writable for AgcAtten5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN5 to value 0x41"]
impl crate::Resettable for AgcAtten5Spec {
    const RESET_VALUE: u32 = 0x41;
}
