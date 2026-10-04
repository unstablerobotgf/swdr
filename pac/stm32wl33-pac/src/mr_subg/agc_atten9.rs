#[doc = "Register `AGC_ATTEN9` reader"]
pub type R = crate::R<AgcAtten9Spec>;
#[doc = "Register `AGC_ATTEN9` writer"]
pub type W = crate::W<AgcAtten9Spec>;
#[doc = "Field `ATTEN_AGCGAIN_9` reader - AGC attenuation control setting for step 9 for LNA+BOM point."]
pub type AttenAgcgain9R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_9` writer - AGC attenuation control setting for step 9 for LNA+BOM point."]
pub type AttenAgcgain9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_9` reader - AGC attenuation control setting for step 9 on PGA point."]
pub type PgaAgcgain9R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_9` writer - AGC attenuation control setting for step 9 on PGA point."]
pub type PgaAgcgain9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 9 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_9(&self) -> AttenAgcgain9R {
        AttenAgcgain9R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 9 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_9(&self) -> PgaAgcgain9R {
        PgaAgcgain9R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 9 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_9(&mut self) -> AttenAgcgain9W<'_, AgcAtten9Spec> {
        AttenAgcgain9W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 9 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_9(&mut self) -> PgaAgcgain9W<'_, AgcAtten9Spec> {
        PgaAgcgain9W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN9 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten9Spec;
impl crate::RegisterSpec for AgcAtten9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten9::R`](R) reader structure"]
impl crate::Readable for AgcAtten9Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten9::W`](W) writer structure"]
impl crate::Writable for AgcAtten9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN9 to value 0x5f"]
impl crate::Resettable for AgcAtten9Spec {
    const RESET_VALUE: u32 = 0x5f;
}
