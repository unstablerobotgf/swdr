#[doc = "Register `CCER` reader"]
pub type R = crate::R<CcerSpec>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CcerSpec>;
#[doc = "Field `CC1E` reader - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled"]
pub type Cc1eR = crate::BitReader;
#[doc = "Field `CC1E` writer - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled"]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1pR = crate::BitReader;
#[doc = "Field `CC1P` writer - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1np {
    #[doc = "0: OC1N active high."]
    B0x0 = 0,
    #[doc = "1: OC1N active low."]
    B0x1 = 1,
}
impl From<Cc1np> for bool {
    #[inline(always)]
    fn from(variant: Cc1np) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1NP` reader - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output)."]
pub type Cc1npR = crate::BitReader<Cc1np>;
impl Cc1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1np {
        match self.bits {
            false => Cc1np::B0x0,
            true => Cc1np::B0x1,
        }
    }
    #[doc = "OC1N active high."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1np::B0x0
    }
    #[doc = "OC1N active low."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1np::B0x1
    }
}
#[doc = "Field `CC1NP` writer - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output)."]
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG, Cc1np>;
impl<'a, REG> Cc1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N active high."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1np::B0x0)
    }
    #[doc = "OC1N active low."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1np::B0x1)
    }
}
#[doc = "Field `CC2E` reader - CC2E: Capture/Compare 2 output enable refer to CC1E description"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - CC2E: Capture/Compare 2 output enable refer to CC1E description"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - CC2P: Capture/Compare 2 output polarity refer to CC1P description"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - CC2P: Capture/Compare 2 output polarity refer to CC1P description"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - CC3E: Capture/Compare 3 output enable refer to CC1E description"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - CC3E: Capture/Compare 3 output enable refer to CC1E description"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - CC3P: Capture/Compare 3 output polarity refer to CC1P description"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - CC3P: Capture/Compare 3 output polarity refer to CC1P description"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
pub type Cc3npR = crate::BitReader;
#[doc = "Field `CC3NP` writer - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - CC4E: Capture/Compare 4 output enable refer to CC1E description"]
pub type Cc4eR = crate::BitReader;
#[doc = "Field `CC4E` writer - CC4E: Capture/Compare 4 output enable refer to CC1E description"]
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - CC4P: Capture/Compare 4 output polarity refer to CC1P description"]
pub type Cc4pR = crate::BitReader;
#[doc = "Field `CC4P` writer - CC4P: Capture/Compare 4 output polarity refer to CC1P description"]
pub type Cc4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
pub type Cc4npR = crate::BitReader;
#[doc = "Field `CC4NP` writer - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
pub type Cc4npW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled"]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output)."]
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC2E: Capture/Compare 2 output enable refer to CC1E description"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC2P: Capture/Compare 2 output polarity refer to CC1P description"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CC3E: Capture/Compare 3 output enable refer to CC1E description"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC3P: Capture/Compare 3 output polarity refer to CC1P description"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4E: Capture/Compare 4 output enable refer to CC1E description"]
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CC4P: Capture/Compare 4 output polarity refer to CC1P description"]
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
    #[inline(always)]
    pub fn cc4np(&self) -> Cc4npR {
        Cc4npR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> Cc1eW<'_, CcerSpec> {
        Cc1eW::new(self, 0)
    }
    #[doc = "Bit 1 - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<'_, CcerSpec> {
        Cc1pW::new(self, 1)
    }
    #[doc = "Bit 3 - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output)."]
    #[inline(always)]
    pub fn cc1np(&mut self) -> Cc1npW<'_, CcerSpec> {
        Cc1npW::new(self, 3)
    }
    #[doc = "Bit 4 - CC2E: Capture/Compare 2 output enable refer to CC1E description"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<'_, CcerSpec> {
        Cc2eW::new(self, 4)
    }
    #[doc = "Bit 5 - CC2P: Capture/Compare 2 output polarity refer to CC1P description"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<'_, CcerSpec> {
        Cc2pW::new(self, 5)
    }
    #[doc = "Bit 7 - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> Cc2npW<'_, CcerSpec> {
        Cc2npW::new(self, 7)
    }
    #[doc = "Bit 8 - CC3E: Capture/Compare 3 output enable refer to CC1E description"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> Cc3eW<'_, CcerSpec> {
        Cc3eW::new(self, 8)
    }
    #[doc = "Bit 9 - CC3P: Capture/Compare 3 output polarity refer to CC1P description"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> Cc3pW<'_, CcerSpec> {
        Cc3pW::new(self, 9)
    }
    #[doc = "Bit 11 - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> Cc3npW<'_, CcerSpec> {
        Cc3npW::new(self, 11)
    }
    #[doc = "Bit 12 - CC4E: Capture/Compare 4 output enable refer to CC1E description"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> Cc4eW<'_, CcerSpec> {
        Cc4eW::new(self, 12)
    }
    #[doc = "Bit 13 - CC4P: Capture/Compare 4 output polarity refer to CC1P description"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> Cc4pW<'_, CcerSpec> {
        Cc4pW::new(self, 13)
    }
    #[doc = "Bit 15 - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description"]
    #[inline(always)]
    pub fn cc4np(&mut self) -> Cc4npW<'_, CcerSpec> {
        Cc4npW::new(self, 15)
    }
}
#[doc = "CCER register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerSpec;
impl crate::RegisterSpec for CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CcerSpec {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CcerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CcerSpec {}
