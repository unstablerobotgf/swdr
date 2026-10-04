#[doc = "Register `AGC_ATTEN1` reader"]
pub type R = crate::R<AgcAtten1Spec>;
#[doc = "Register `AGC_ATTEN1` writer"]
pub type W = crate::W<AgcAtten1Spec>;
#[doc = "Field `ATTEN_AGCGAIN_1` reader - AGC attenuation control setting for step 1 for LNA+BOM point."]
pub type AttenAgcgain1R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_1` writer - AGC attenuation control setting for step 1 for LNA+BOM point."]
pub type AttenAgcgain1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_1` reader - AGC attenuation control setting for step 1 on PGA point."]
pub type PgaAgcgain1R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_1` writer - AGC attenuation control setting for step 1 on PGA point."]
pub type PgaAgcgain1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 1 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_1(&self) -> AttenAgcgain1R {
        AttenAgcgain1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 1 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_1(&self) -> PgaAgcgain1R {
        PgaAgcgain1R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 1 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_1(&mut self) -> AttenAgcgain1W<'_, AgcAtten1Spec> {
        AttenAgcgain1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 1 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_1(&mut self) -> PgaAgcgain1W<'_, AgcAtten1Spec> {
        PgaAgcgain1W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten1Spec;
impl crate::RegisterSpec for AgcAtten1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten1::R`](R) reader structure"]
impl crate::Readable for AgcAtten1Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten1::W`](W) writer structure"]
impl crate::Writable for AgcAtten1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN1 to value 0x10"]
impl crate::Resettable for AgcAtten1Spec {
    const RESET_VALUE: u32 = 0x10;
}
