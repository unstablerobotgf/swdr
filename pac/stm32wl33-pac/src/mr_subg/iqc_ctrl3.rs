#[doc = "Register `IQC_CTRL3` reader"]
pub type R = crate::R<IqcCtrl3Spec>;
#[doc = "Register `IQC_CTRL3` writer"]
pub type W = crate::W<IqcCtrl3Spec>;
#[doc = "Field `FAST_TIME` reader - Duration of the fast mode."]
pub type FastTimeR = crate::FieldReader;
#[doc = "Field `FAST_TIME` writer - Duration of the fast mode."]
pub type FastTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Duration of the fast mode."]
    #[inline(always)]
    pub fn fast_time(&self) -> FastTimeR {
        FastTimeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Duration of the fast mode."]
    #[inline(always)]
    pub fn fast_time(&mut self) -> FastTimeW<'_, IqcCtrl3Spec> {
        FastTimeW::new(self, 0)
    }
}
#[doc = "IQC_CTRL3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqcCtrl3Spec;
impl crate::RegisterSpec for IqcCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqc_ctrl3::R`](R) reader structure"]
impl crate::Readable for IqcCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`iqc_ctrl3::W`](W) writer structure"]
impl crate::Writable for IqcCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IQC_CTRL3 to value 0x07"]
impl crate::Resettable for IqcCtrl3Spec {
    const RESET_VALUE: u32 = 0x07;
}
