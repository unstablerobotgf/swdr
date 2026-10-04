#[doc = "Register `CCMR2_in` reader"]
pub type R = crate::R<Ccmr2InSpec>;
#[doc = "Register `CCMR2_in` writer"]
pub type W = crate::W<Ccmr2InSpec>;
#[doc = "Field `CC3S` reader - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3PSC` reader - IC3PSC: Input capture 3 prescaler"]
pub type Ic3pscR = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - IC3PSC: Input capture 3 prescaler"]
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - IC3F: Input capture 3 filter"]
pub type Ic3fR = crate::FieldReader;
#[doc = "Field `IC3F` writer - IC3F: Input capture 3 filter"]
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4S` reader - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4PSC` reader - IC4PSC: Input capture 4 prescaler"]
pub type Ic4pscR = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - IC4PSC: Input capture 4 prescaler"]
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - IC4F: Input capture 4 filter"]
pub type Ic4fR = crate::FieldReader;
#[doc = "Field `IC4F` writer - IC4F: Input capture 4 filter"]
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IC3PSC: Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - IC3F: Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IC4PSC: Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - IC4F: Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2InSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - IC3PSC: Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> Ic3pscW<'_, Ccmr2InSpec> {
        Ic3pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - IC3F: Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> Ic3fW<'_, Ccmr2InSpec> {
        Ic3fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<'_, Ccmr2InSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - IC4PSC: Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> Ic4pscW<'_, Ccmr2InSpec> {
        Ic4pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - IC4F: Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> Ic4fW<'_, Ccmr2InSpec> {
        Ic4fW::new(self, 12)
    }
}
#[doc = "CCMR2_in register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2InSpec;
impl crate::RegisterSpec for Ccmr2InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_in::R`](R) reader structure"]
impl crate::Readable for Ccmr2InSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_in::W`](W) writer structure"]
impl crate::Writable for Ccmr2InSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2_in to value 0"]
impl crate::Resettable for Ccmr2InSpec {}
