#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UIF` reader - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)"]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC1IF` writer - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)"]
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description"]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC2IF` writer - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description"]
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description"]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `CC3IF` writer - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description"]
pub type Cc3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description"]
pub type Cc4ifR = crate::BitReader;
#[doc = "Field `CC4IF` writer - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description"]
pub type Cc4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending."]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending."]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description"]
pub type Cc2ofR = crate::BitReader;
#[doc = "Field `CC2OF` writer - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description"]
pub type Cc2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub type Cc3ofR = crate::BitReader;
#[doc = "Field `CC3OF` writer - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description"]
pub type Cc3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub type Cc4ofR = crate::BitReader;
#[doc = "Field `CC4OF` writer - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description"]
pub type Cc4ofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending."]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&self) -> Cc4ofR {
        Cc4ofR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIF: Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred. 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<'_, SrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IF: Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value, with some exception in center-aligned mode (refer to the CMS bits in the TIMx_CR1 register description). It is cleared by software. 0: No match. 1: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the contents of TIMx_CCR1 are greater than the contents of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in upcounting and up/down-counting modes) or underflow (in downcounting mode) If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the TIMx_CCR1 register. 0: No input capture occurred 1: The counter value has been captured in TIMx_CCR1 register (An edge has been detected on IC1 which matches the selected polarity)"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<'_, SrSpec> {
        Cc1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - CC2IF: Capture/Compare 2 interrupt flag refer to CC1IF description"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<'_, SrSpec> {
        Cc2ifW::new(self, 2)
    }
    #[doc = "Bit 3 - CC3IF: Capture/Compare 3 interrupt flag refer to CC1IF description"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<'_, SrSpec> {
        Cc3ifW::new(self, 3)
    }
    #[doc = "Bit 4 - CC4IF: Capture/Compare 4 interrupt flag refer to CC1IF description"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> Cc4ifW<'_, SrSpec> {
        Cc4ifW::new(self, 4)
    }
    #[doc = "Bit 6 - TIF: Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.. 0: No trigger event occurred. 1: Trigger interrupt pending."]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, SrSpec> {
        TifW::new(self, 6)
    }
    #[doc = "Bit 9 - CC1OF: Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected 1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<'_, SrSpec> {
        Cc1ofW::new(self, 9)
    }
    #[doc = "Bit 10 - CC2OF: Capture/Compare 2 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> Cc2ofW<'_, SrSpec> {
        Cc2ofW::new(self, 10)
    }
    #[doc = "Bit 11 - CC3OF: Capture/Compare 3 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> Cc3ofW<'_, SrSpec> {
        Cc3ofW::new(self, 11)
    }
    #[doc = "Bit 12 - CC4OF: Capture/Compare 4 overcapture flag refer to CC1OF description"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> Cc4ofW<'_, SrSpec> {
        Cc4ofW::new(self, 12)
    }
}
#[doc = "SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
