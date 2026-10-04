#[doc = "Register `LCSC_CR2` reader"]
pub type R = crate::R<LcscCr2Spec>;
#[doc = "Register `LCSC_CR2` writer"]
pub type W = crate::W<LcscCr2Spec>;
#[doc = "Field `TAMP_PSC` reader - Tamper measurement interval."]
pub type TampPscR = crate::FieldReader;
#[doc = "Field `TAMP_PSC` writer - Tamper measurement interval."]
pub type TampPscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCT_DAMP_THRES` reader - Damping threshold for LCT"]
pub type LctDampThresR = crate::FieldReader;
#[doc = "Field `LCT_DAMP_THRES` writer - Damping threshold for LCT"]
pub type LctDampThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Tamper measurement interval."]
    #[inline(always)]
    pub fn tamp_psc(&self) -> TampPscR {
        TampPscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Damping threshold for LCT"]
    #[inline(always)]
    pub fn lct_damp_thres(&self) -> LctDampThresR {
        LctDampThresR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tamper measurement interval."]
    #[inline(always)]
    pub fn tamp_psc(&mut self) -> TampPscW<'_, LcscCr2Spec> {
        TampPscW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Damping threshold for LCT"]
    #[inline(always)]
    pub fn lct_damp_thres(&mut self) -> LctDampThresW<'_, LcscCr2Spec> {
        LctDampThresW::new(self, 8)
    }
}
#[doc = "LCSC_CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscCr2Spec;
impl crate::RegisterSpec for LcscCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_cr2::R`](R) reader structure"]
impl crate::Readable for LcscCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_cr2::W`](W) writer structure"]
impl crate::Writable for LcscCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_CR2 to value 0x8000"]
impl crate::Resettable for LcscCr2Spec {
    const RESET_VALUE: u32 = 0x8000;
}
