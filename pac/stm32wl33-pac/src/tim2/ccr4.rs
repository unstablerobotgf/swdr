#[doc = "Register `CCR4` reader"]
pub type R = crate::R<Ccr4Spec>;
#[doc = "Register `CCR4` writer"]
pub type W = crate::W<Ccr4Spec>;
#[doc = "Field `CCR4` reader - CCR4\\[15:0\\]: Capture/Compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4)."]
pub type Ccr4R = crate::FieldReader<u16>;
#[doc = "Field `CCR4` writer - CCR4\\[15:0\\]: Capture/Compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4)."]
pub type Ccr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CCR4\\[15:0\\]: Capture/Compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4)."]
    #[inline(always)]
    pub fn ccr4(&self) -> Ccr4R {
        Ccr4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR4\\[15:0\\]: Capture/Compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4)."]
    #[inline(always)]
    pub fn ccr4(&mut self) -> Ccr4W<'_, Ccr4Spec> {
        Ccr4W::new(self, 0)
    }
}
#[doc = "CCR4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr4Spec;
impl crate::RegisterSpec for Ccr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4::R`](R) reader structure"]
impl crate::Readable for Ccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr4::W`](W) writer structure"]
impl crate::Writable for Ccr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for Ccr4Spec {}
