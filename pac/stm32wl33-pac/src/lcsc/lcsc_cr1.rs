#[doc = "Register `LCSC_CR1` reader"]
pub type R = crate::R<LcscCr1Spec>;
#[doc = "Register `LCSC_CR1` writer"]
pub type W = crate::W<LcscCr1Spec>;
#[doc = "Field `LCAB_DAMP_THRES` reader - LCAB_DAMP_THRES\\[7:0\\]: Damping threshold for LCA and LCB"]
pub type LcabDampThresR = crate::FieldReader;
#[doc = "Field `LCAB_DAMP_THRES` writer - LCAB_DAMP_THRES\\[7:0\\]: Damping threshold for LCA and LCB"]
pub type LcabDampThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TREC_VCM` reader - VCMBUFF Recovery Time"]
pub type TrecVcmR = crate::FieldReader<u16>;
#[doc = "Field `TREC_VCM` writer - VCMBUFF Recovery Time"]
pub type TrecVcmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TSTART_VCM` reader - VCMBUFF Starting Time"]
pub type TstartVcmR = crate::FieldReader<u16>;
#[doc = "Field `TSTART_VCM` writer - VCMBUFF Starting Time"]
pub type TstartVcmW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - LCAB_DAMP_THRES\\[7:0\\]: Damping threshold for LCA and LCB"]
    #[inline(always)]
    pub fn lcab_damp_thres(&self) -> LcabDampThresR {
        LcabDampThresR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:18 - VCMBUFF Recovery Time"]
    #[inline(always)]
    pub fn trec_vcm(&self) -> TrecVcmR {
        TrecVcmR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:30 - VCMBUFF Starting Time"]
    #[inline(always)]
    pub fn tstart_vcm(&self) -> TstartVcmR {
        TstartVcmR::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCAB_DAMP_THRES\\[7:0\\]: Damping threshold for LCA and LCB"]
    #[inline(always)]
    pub fn lcab_damp_thres(&mut self) -> LcabDampThresW<'_, LcscCr1Spec> {
        LcabDampThresW::new(self, 0)
    }
    #[doc = "Bits 10:18 - VCMBUFF Recovery Time"]
    #[inline(always)]
    pub fn trec_vcm(&mut self) -> TrecVcmW<'_, LcscCr1Spec> {
        TrecVcmW::new(self, 10)
    }
    #[doc = "Bits 20:30 - VCMBUFF Starting Time"]
    #[inline(always)]
    pub fn tstart_vcm(&mut self) -> TstartVcmW<'_, LcscCr1Spec> {
        TstartVcmW::new(self, 20)
    }
}
#[doc = "LCSC_CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscCr1Spec;
impl crate::RegisterSpec for LcscCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_cr1::R`](R) reader structure"]
impl crate::Readable for LcscCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_cr1::W`](W) writer structure"]
impl crate::Writable for LcscCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_CR1 to value 0x3c01_0c80"]
impl crate::Resettable for LcscCr1Spec {
    const RESET_VALUE: u32 = 0x3c01_0c80;
}
