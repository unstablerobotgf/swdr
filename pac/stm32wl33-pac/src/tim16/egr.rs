#[doc = "Register `EGR` reader"]
pub type R = crate::R<EgrSpec>;
#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected)."]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - CC1G: Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` writer - COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output."]
pub type ComgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` writer - BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
pub type BgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected)."]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - CC1G: Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<'_, EgrSpec> {
        Cc1gW::new(self, 1)
    }
    #[doc = "Bit 5 - COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn comg(&mut self) -> ComgW<'_, EgrSpec> {
        ComgW::new(self, 5)
    }
    #[doc = "Bit 6 - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, EgrSpec> {
        TgW::new(self, 6)
    }
    #[doc = "Bit 7 - BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn bg(&mut self) -> BgW<'_, EgrSpec> {
        BgW::new(self, 7)
    }
}
#[doc = "EGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`egr::R`](R) reader structure"]
impl crate::Readable for EgrSpec {}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EgrSpec {}
