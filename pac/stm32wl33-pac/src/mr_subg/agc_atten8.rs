#[doc = "Register `AGC_ATTEN8` reader"]
pub type R = crate::R<AgcAtten8Spec>;
#[doc = "Register `AGC_ATTEN8` writer"]
pub type W = crate::W<AgcAtten8Spec>;
#[doc = "Field `ATTEN_AGCGAIN_8` reader - AGC attenuation control setting for step 8 for LNA+BOM point."]
pub type AttenAgcgain8R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_8` writer - AGC attenuation control setting for step 8 for LNA+BOM point."]
pub type AttenAgcgain8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_8` reader - AGC attenuation control setting for step 8 on PGA point."]
pub type PgaAgcgain8R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_8` writer - AGC attenuation control setting for step 8 on PGA point."]
pub type PgaAgcgain8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 8 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_8(&self) -> AttenAgcgain8R {
        AttenAgcgain8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 8 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_8(&self) -> PgaAgcgain8R {
        PgaAgcgain8R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 8 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_8(&mut self) -> AttenAgcgain8W<'_, AgcAtten8Spec> {
        AttenAgcgain8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 8 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_8(&mut self) -> PgaAgcgain8W<'_, AgcAtten8Spec> {
        PgaAgcgain8W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN8 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten8Spec;
impl crate::RegisterSpec for AgcAtten8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten8::R`](R) reader structure"]
impl crate::Readable for AgcAtten8Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten8::W`](W) writer structure"]
impl crate::Writable for AgcAtten8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN8 to value 0x4f"]
impl crate::Resettable for AgcAtten8Spec {
    const RESET_VALUE: u32 = 0x4f;
}
