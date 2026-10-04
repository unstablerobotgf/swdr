#[doc = "Register `IQC_CTRL2` reader"]
pub type R = crate::R<IqcCtrl2Spec>;
#[doc = "Register `IQC_CTRL2` writer"]
pub type W = crate::W<IqcCtrl2Spec>;
#[doc = "Field `QPD_DECAY` reader - Decay coefficient for QPD:"]
pub type QpdDecayR = crate::FieldReader;
#[doc = "Field `QPD_DECAY` writer - Decay coefficient for QPD:"]
pub type QpdDecayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Decay coefficient for QPD:"]
    #[inline(always)]
    pub fn qpd_decay(&self) -> QpdDecayR {
        QpdDecayR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decay coefficient for QPD:"]
    #[inline(always)]
    pub fn qpd_decay(&mut self) -> QpdDecayW<'_, IqcCtrl2Spec> {
        QpdDecayW::new(self, 0)
    }
}
#[doc = "IQC_CTRL2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqcCtrl2Spec;
impl crate::RegisterSpec for IqcCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqc_ctrl2::R`](R) reader structure"]
impl crate::Readable for IqcCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`iqc_ctrl2::W`](W) writer structure"]
impl crate::Writable for IqcCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IQC_CTRL2 to value 0x08"]
impl crate::Resettable for IqcCtrl2Spec {
    const RESET_VALUE: u32 = 0x08;
}
