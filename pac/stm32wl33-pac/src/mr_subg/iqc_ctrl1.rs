#[doc = "Register `IQC_CTRL1` reader"]
pub type R = crate::R<IqcCtrl1Spec>;
#[doc = "Register `IQC_CTRL1` writer"]
pub type W = crate::W<IqcCtrl1Spec>;
#[doc = "Field `QPD_ATTACK` reader - Attack coefficient for QPD:"]
pub type QpdAttackR = crate::FieldReader;
#[doc = "Field `QPD_ATTACK` writer - Attack coefficient for QPD:"]
pub type QpdAttackW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Attack coefficient for QPD:"]
    #[inline(always)]
    pub fn qpd_attack(&self) -> QpdAttackR {
        QpdAttackR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attack coefficient for QPD:"]
    #[inline(always)]
    pub fn qpd_attack(&mut self) -> QpdAttackW<'_, IqcCtrl1Spec> {
        QpdAttackW::new(self, 0)
    }
}
#[doc = "IQC_CTRL1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqcCtrl1Spec;
impl crate::RegisterSpec for IqcCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqc_ctrl1::R`](R) reader structure"]
impl crate::Readable for IqcCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`iqc_ctrl1::W`](W) writer structure"]
impl crate::Writable for IqcCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IQC_CTRL1 to value 0x08"]
impl crate::Resettable for IqcCtrl1Spec {
    const RESET_VALUE: u32 = 0x08;
}
