#[doc = "Register `CCMR1` reader"]
pub type R = crate::R<Ccmr1Spec>;
#[doc = "Register `CCMR1` writer"]
pub type W = crate::W<Ccmr1Spec>;
#[doc = "Field `CC1S` reader - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
pub type Oc1feR = crate::BitReader;
#[doc = "Field `OC1FE` writer - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M_2_0` reader - OC1M\\[2:0\\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits."]
pub type Oc1m2_0R = crate::FieldReader;
#[doc = "Field `OC1M_2_0` writer - OC1M\\[2:0\\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits."]
pub type Oc1m2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal."]
pub type Oc1ceR = crate::BitReader;
#[doc = "Field `OC1CE` writer - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal."]
pub type Oc1ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M_3` reader - OC1M\\[3\\]: Output Compare 1 mode (bit 3)"]
pub type Oc1m3R = crate::BitReader;
#[doc = "Field `OC1M_3` writer - OC1M\\[3\\]: Output Compare 1 mode (bit 3)"]
pub type Oc1m3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC1M\\[2:0\\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits."]
    #[inline(always)]
    pub fn oc1m_2_0(&self) -> Oc1m2_0R {
        Oc1m2_0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal."]
    #[inline(always)]
    pub fn oc1ce(&self) -> Oc1ceR {
        Oc1ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - OC1M\\[3\\]: Output Compare 1 mode (bit 3)"]
    #[inline(always)]
    pub fn oc1m_3(&self) -> Oc1m3R {
        Oc1m3R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1Spec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bit 2 - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> Oc1feW<'_, Ccmr1Spec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3 - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed."]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<'_, Ccmr1Spec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC1M\\[2:0\\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits."]
    #[inline(always)]
    pub fn oc1m_2_0(&mut self) -> Oc1m2_0W<'_, Ccmr1Spec> {
        Oc1m2_0W::new(self, 4)
    }
    #[doc = "Bit 7 - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal."]
    #[inline(always)]
    pub fn oc1ce(&mut self) -> Oc1ceW<'_, Ccmr1Spec> {
        Oc1ceW::new(self, 7)
    }
    #[doc = "Bit 16 - OC1M\\[3\\]: Output Compare 1 mode (bit 3)"]
    #[inline(always)]
    pub fn oc1m_3(&mut self) -> Oc1m3W<'_, Ccmr1Spec> {
        Oc1m3W::new(self, 16)
    }
}
#[doc = "CCMR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1Spec;
impl crate::RegisterSpec for Ccmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1::R`](R) reader structure"]
impl crate::Readable for Ccmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1::W`](W) writer structure"]
impl crate::Writable for Ccmr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1 to value 0"]
impl crate::Resettable for Ccmr1Spec {}
