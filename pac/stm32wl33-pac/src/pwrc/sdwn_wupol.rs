#[doc = "Register `SDWN_WUPOL` reader"]
pub type R = crate::R<SdwnWupolSpec>;
#[doc = "Register `SDWN_WUPOL` writer"]
pub type W = crate::W<SdwnWupolSpec>;
#[doc = "Field `WUPOL` reader - WUPOL PB0 I/O WakeUp from shutdown Polarity This bit defines the polarity used for wakeup from shutdown detection on PB0 pin. It is cleared by a PORESETn. - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
pub type WupolR = crate::BitReader;
#[doc = "Field `WUPOL` writer - WUPOL PB0 I/O WakeUp from shutdown Polarity This bit defines the polarity used for wakeup from shutdown detection on PB0 pin. It is cleared by a PORESETn. - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
pub type WupolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WUPOL PB0 I/O WakeUp from shutdown Polarity This bit defines the polarity used for wakeup from shutdown detection on PB0 pin. It is cleared by a PORESETn. - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn wupol(&self) -> WupolR {
        WupolR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WUPOL PB0 I/O WakeUp from shutdown Polarity This bit defines the polarity used for wakeup from shutdown detection on PB0 pin. It is cleared by a PORESETn. - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn wupol(&mut self) -> WupolW<'_, SdwnWupolSpec> {
        WupolW::new(self, 0)
    }
}
#[doc = "SDWN_WUPOL register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdwn_wupol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wupol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdwnWupolSpec;
impl crate::RegisterSpec for SdwnWupolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdwn_wupol::R`](R) reader structure"]
impl crate::Readable for SdwnWupolSpec {}
#[doc = "`write(|w| ..)` method takes [`sdwn_wupol::W`](W) writer structure"]
impl crate::Writable for SdwnWupolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDWN_WUPOL to value 0"]
impl crate::Resettable for SdwnWupolSpec {}
