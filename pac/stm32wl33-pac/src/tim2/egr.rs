#[doc = "Register `EGR` reader"]
pub type R = crate::R<EgrSpec>;
#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected)."]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - CC1G: Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - CC2G: Capture/Compare 2 generation refer to CC1G description"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - CC3G: Capture/Compare 3 generation refer to CC1G description"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - CC4G: Capture/Compare 4 generation refer to CC1G description"]
pub type Cc4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt can occur if enabled."]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - CC2G: Capture/Compare 2 generation refer to CC1G description"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<'_, EgrSpec> {
        Cc2gW::new(self, 2)
    }
    #[doc = "Bit 3 - CC3G: Capture/Compare 3 generation refer to CC1G description"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<'_, EgrSpec> {
        Cc3gW::new(self, 3)
    }
    #[doc = "Bit 4 - CC4G: Capture/Compare 4 generation refer to CC1G description"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> Cc4gW<'_, EgrSpec> {
        Cc4gW::new(self, 4)
    }
    #[doc = "Bit 6 - TG: Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in TIMx_SR register. Related interrupt can occur if enabled."]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, EgrSpec> {
        TgW::new(self, 6)
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
