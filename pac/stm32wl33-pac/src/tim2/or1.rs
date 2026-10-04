#[doc = "Register `OR1` reader"]
pub type R = crate::R<Or1Spec>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<Or1Spec>;
#[doc = "Field `ETR_RMP` reader - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller."]
pub type EtrRmpR = crate::BitReader;
#[doc = "Field `ETR_RMP` writer - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller."]
pub type EtrRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OR1_1` reader - This field is not used in Blue51. Not available in IUM"]
pub type Or1_1R = crate::BitReader;
#[doc = "Field `OR1_1` writer - This field is not used in Blue51. Not available in IUM"]
pub type Or1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI4_RMP` reader - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT"]
pub type Ti4RmpR = crate::BitReader;
#[doc = "Field `TI4_RMP` writer - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT"]
pub type Ti4RmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller."]
    #[inline(always)]
    pub fn etr_rmp(&self) -> EtrRmpR {
        EtrRmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field is not used in Blue51. Not available in IUM"]
    #[inline(always)]
    pub fn or1_1(&self) -> Or1_1R {
        Or1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> Ti4RmpR {
        Ti4RmpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller."]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> EtrRmpW<'_, Or1Spec> {
        EtrRmpW::new(self, 0)
    }
    #[doc = "Bit 1 - This field is not used in Blue51. Not available in IUM"]
    #[inline(always)]
    pub fn or1_1(&mut self) -> Or1_1W<'_, Or1Spec> {
        Or1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT"]
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> Ti4RmpW<'_, Or1Spec> {
        Ti4RmpW::new(self, 2)
    }
}
#[doc = "OR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or1Spec;
impl crate::RegisterSpec for Or1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for Or1Spec {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for Or1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for Or1Spec {}
