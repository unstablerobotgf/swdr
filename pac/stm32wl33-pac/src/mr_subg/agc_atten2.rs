#[doc = "Register `AGC_ATTEN2` reader"]
pub type R = crate::R<AgcAtten2Spec>;
#[doc = "Register `AGC_ATTEN2` writer"]
pub type W = crate::W<AgcAtten2Spec>;
#[doc = "Field `ATTEN_AGCGAIN_2` reader - AGC attenuation control setting for step 2 for LNA+BOM point."]
pub type AttenAgcgain2R = crate::FieldReader;
#[doc = "Field `ATTEN_AGCGAIN_2` writer - AGC attenuation control setting for step 2 for LNA+BOM point."]
pub type AttenAgcgain2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_AGCGAIN_2` reader - AGC attenuation control setting for step 2 on PGA point."]
pub type PgaAgcgain2R = crate::FieldReader;
#[doc = "Field `PGA_AGCGAIN_2` writer - AGC attenuation control setting for step 2 on PGA point."]
pub type PgaAgcgain2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 2 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_2(&self) -> AttenAgcgain2R {
        AttenAgcgain2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 2 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_2(&self) -> PgaAgcgain2R {
        PgaAgcgain2R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AGC attenuation control setting for step 2 for LNA+BOM point."]
    #[inline(always)]
    pub fn atten_agcgain_2(&mut self) -> AttenAgcgain2W<'_, AgcAtten2Spec> {
        AttenAgcgain2W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AGC attenuation control setting for step 2 on PGA point."]
    #[inline(always)]
    pub fn pga_agcgain_2(&mut self) -> PgaAgcgain2W<'_, AgcAtten2Spec> {
        PgaAgcgain2W::new(self, 4)
    }
}
#[doc = "AGC_ATTEN2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_atten2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_atten2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAtten2Spec;
impl crate::RegisterSpec for AgcAtten2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_atten2::R`](R) reader structure"]
impl crate::Readable for AgcAtten2Spec {}
#[doc = "`write(|w| ..)` method takes [`agc_atten2::W`](W) writer structure"]
impl crate::Writable for AgcAtten2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ATTEN2 to value 0x20"]
impl crate::Resettable for AgcAtten2Spec {
    const RESET_VALUE: u32 = 0x20;
}
