#[doc = "Register `CCMR1_in` reader"]
pub type R = crate::R<Ccmr1InSpec>;
#[doc = "Register `CCMR1_in` writer"]
pub type W = crate::W<Ccmr1InSpec>;
#[doc = "Field `CC1S` reader - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - IC1PSC: Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register). 00: no prescaler, capture is done each time an edge is detected on the capture input. 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
pub type Ic1pscR = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - IC1PSC: Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register). 00: no prescaler, capture is done each time an edge is detected on the capture input. 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
pub type Ic1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - Bits 7:4 IC1F\\[3:0\\]: Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N= 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
pub type Ic1fR = crate::FieldReader;
#[doc = "Field `IC1F` writer - Bits 7:4 IC1F\\[3:0\\]: Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N= 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IC1PSC: Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register). 00: no prescaler, capture is done each time an edge is detected on the capture input. 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Bits 7:4 IC1F\\[3:0\\]: Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N= 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1InSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - IC1PSC: Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register). 00: no prescaler, capture is done each time an edge is detected on the capture input. 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> Ic1pscW<'_, Ccmr1InSpec> {
        Ic1pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Bits 7:4 IC1F\\[3:0\\]: Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N= 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> Ic1fW<'_, Ccmr1InSpec> {
        Ic1fW::new(self, 4)
    }
}
#[doc = "CCMR1_in register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1InSpec;
impl crate::RegisterSpec for Ccmr1InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_in::R`](R) reader structure"]
impl crate::Readable for Ccmr1InSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_in::W`](W) writer structure"]
impl crate::Writable for Ccmr1InSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1_in to value 0"]
impl crate::Resettable for Ccmr1InSpec {}
