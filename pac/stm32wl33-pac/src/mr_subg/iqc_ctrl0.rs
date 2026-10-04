#[doc = "Register `IQC_CTRL0` reader"]
pub type R = crate::R<IqcCtrl0Spec>;
#[doc = "Register `IQC_CTRL0` writer"]
pub type W = crate::W<IqcCtrl0Spec>;
#[doc = "Field `FAST_GAIN` reader - Gain of the correction loop in fast mode."]
pub type FastGainR = crate::FieldReader;
#[doc = "Field `FAST_GAIN` writer - Gain of the correction loop in fast mode."]
pub type FastGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLOW_GAIN` reader - Gain of the correction loop in slow mode."]
pub type SlowGainR = crate::FieldReader;
#[doc = "Field `SLOW_GAIN` writer - Gain of the correction loop in slow mode."]
pub type SlowGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Gain of the correction loop in fast mode."]
    #[inline(always)]
    pub fn fast_gain(&self) -> FastGainR {
        FastGainR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Gain of the correction loop in slow mode."]
    #[inline(always)]
    pub fn slow_gain(&self) -> SlowGainR {
        SlowGainR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain of the correction loop in fast mode."]
    #[inline(always)]
    pub fn fast_gain(&mut self) -> FastGainW<'_, IqcCtrl0Spec> {
        FastGainW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Gain of the correction loop in slow mode."]
    #[inline(always)]
    pub fn slow_gain(&mut self) -> SlowGainW<'_, IqcCtrl0Spec> {
        SlowGainW::new(self, 4)
    }
}
#[doc = "IQC_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqcCtrl0Spec;
impl crate::RegisterSpec for IqcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqc_ctrl0::R`](R) reader structure"]
impl crate::Readable for IqcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`iqc_ctrl0::W`](W) writer structure"]
impl crate::Writable for IqcCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IQC_CTRL0 to value 0xe3"]
impl crate::Resettable for IqcCtrl0Spec {
    const RESET_VALUE: u32 = 0xe3;
}
