#[doc = "Register `CCMR2` reader"]
pub type R = crate::R<Ccmr2Spec>;
#[doc = "Register `CCMR2` writer"]
pub type W = crate::W<Ccmr2Spec>;
#[doc = "Field `CC3S` reader - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - OC3FE: Output compare 3 fast enable"]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - OC3FE: Output compare 3 fast enable"]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - OC3PE: Output compare 3 preload enable"]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - OC3PE: Output compare 3 preload enable"]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M_2_0` reader - OC3M: Output compare 3 mode"]
pub type Oc3m2_0R = crate::FieldReader;
#[doc = "Field `OC3M_2_0` writer - OC3M: Output compare 3 mode"]
pub type Oc3m2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - OC3CE: Output compare 3 clear enable"]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - OC3CE: Output compare 3 clear enable"]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4S` reader - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - OC4FE: Output Compare 4 fast enable"]
pub type Oc4feR = crate::BitReader;
#[doc = "Field `OC4FE` writer - OC4FE: Output Compare 4 fast enable"]
pub type Oc4feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - OC4PE: Output Compare 4 preload enable"]
pub type Oc4peR = crate::BitReader;
#[doc = "Field `OC4PE` writer - OC4PE: Output Compare 4 preload enable"]
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M_2_0` reader - OC4M\\[2:0\\]: Output Compare 4 mode"]
pub type Oc4m2_0R = crate::FieldReader;
#[doc = "Field `OC4M_2_0` writer - OC4M\\[2:0\\]: Output Compare 4 mode"]
pub type Oc4m2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - OC4CE: Output Compare 4 clear enable"]
pub type Oc4ceR = crate::BitReader;
#[doc = "Field `OC4CE` writer - OC4CE: Output Compare 4 clear enable"]
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M_3` reader - OC3M\\[3\\]: Output Compare 3 mode (bit 3)"]
pub type Oc3m3R = crate::BitReader;
#[doc = "Field `OC3M_3` writer - OC3M\\[3\\]: Output Compare 3 mode (bit 3)"]
pub type Oc3m3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M_3` reader - OC4M\\[3\\]: Output Compare 4 mode (bit 3)"]
pub type Oc4m3R = crate::BitReader;
#[doc = "Field `OC4M_3` writer - OC4M\\[3\\]: Output Compare 4 mode (bit 3)"]
pub type Oc4m3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC3FE: Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC3PE: Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC3M: Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m_2_0(&self) -> Oc3m2_0R {
        Oc3m2_0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC3CE: Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OC4FE: Output Compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&self) -> Oc4feR {
        Oc4feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC4PE: Output Compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC4M\\[2:0\\]: Output Compare 4 mode"]
    #[inline(always)]
    pub fn oc4m_2_0(&self) -> Oc4m2_0R {
        Oc4m2_0R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC4CE: Output Compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OC3M\\[3\\]: Output Compare 3 mode (bit 3)"]
    #[inline(always)]
    pub fn oc3m_3(&self) -> Oc3m3R {
        Oc3m3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC4M\\[3\\]: Output Compare 4 mode (bit 3)"]
    #[inline(always)]
    pub fn oc4m_3(&self) -> Oc4m3R {
        Oc4m3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3S: Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2Spec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bit 2 - OC3FE: Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> Oc3feW<'_, Ccmr2Spec> {
        Oc3feW::new(self, 2)
    }
    #[doc = "Bit 3 - OC3PE: Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<'_, Ccmr2Spec> {
        Oc3peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC3M: Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m_2_0(&mut self) -> Oc3m2_0W<'_, Ccmr2Spec> {
        Oc3m2_0W::new(self, 4)
    }
    #[doc = "Bit 7 - OC3CE: Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<'_, Ccmr2Spec> {
        Oc3ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<'_, Ccmr2Spec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bit 10 - OC4FE: Output Compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> Oc4feW<'_, Ccmr2Spec> {
        Oc4feW::new(self, 10)
    }
    #[doc = "Bit 11 - OC4PE: Output Compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> Oc4peW<'_, Ccmr2Spec> {
        Oc4peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC4M\\[2:0\\]: Output Compare 4 mode"]
    #[inline(always)]
    pub fn oc4m_2_0(&mut self) -> Oc4m2_0W<'_, Ccmr2Spec> {
        Oc4m2_0W::new(self, 12)
    }
    #[doc = "Bit 15 - OC4CE: Output Compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&mut self) -> Oc4ceW<'_, Ccmr2Spec> {
        Oc4ceW::new(self, 15)
    }
    #[doc = "Bit 16 - OC3M\\[3\\]: Output Compare 3 mode (bit 3)"]
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> Oc3m3W<'_, Ccmr2Spec> {
        Oc3m3W::new(self, 16)
    }
    #[doc = "Bit 24 - OC4M\\[3\\]: Output Compare 4 mode (bit 3)"]
    #[inline(always)]
    pub fn oc4m_3(&mut self) -> Oc4m3W<'_, Ccmr2Spec> {
        Oc4m3W::new(self, 24)
    }
}
#[doc = "CCMR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2Spec;
impl crate::RegisterSpec for Ccmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2::R`](R) reader structure"]
impl crate::Readable for Ccmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2::W`](W) writer structure"]
impl crate::Writable for Ccmr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2 to value 0"]
impl crate::Resettable for Ccmr2Spec {}
