#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `EN` reader - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRMODE` reader - PWRMODE\\[1:0\\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power"]
pub type PwrmodeR = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - PWRMODE\\[1:0\\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power"]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INMSEL` reader - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3"]
pub type InmselR = crate::FieldReader;
#[doc = "Field `INMSEL` writer - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3"]
pub type InmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPSEL` reader - INPSEL\\[1:0\\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2"]
pub type InpselR = crate::FieldReader;
#[doc = "Field `INPSEL` writer - INPSEL\\[1:0\\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2"]
pub type InpselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POLARITY` reader - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted"]
pub type PolarityR = crate::BitReader;
#[doc = "Field `POLARITY` writer - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - HYST\\[1:0\\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - HYST\\[1:0\\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLANKING` reader - BLANKING\\[2:0\\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved"]
pub type BlankingR = crate::FieldReader;
#[doc = "Field `BLANKING` writer - BLANKING\\[2:0\\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved"]
pub type BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRGEN` reader - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available."]
pub type BrgenR = crate::BitReader;
#[doc = "Field `BRGEN` writer - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available."]
pub type BrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable"]
pub type ScalenR = crate::BitReader;
#[doc = "Field `SCALEN` writer - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable"]
pub type ScalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALUE` reader - VALUE: Comparator output status bit This bit is read-only. It reflects the current comparator output taking into account POLARITY bit effect."]
pub type ValueR = crate::BitReader;
#[doc = "Field `LOCK` reader - LOCK: COMP_CSR register lock bit This bit is set by software and cleared by a hardware system reset. It locks the whole content of the comparator control register, COMP1_CSR\\[31:0\\]. 0: COMP1_CSR\\[31:0\\] are read/write 1: COMP1_CSR\\[31:0\\] are read-only"]
pub type LockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - PWRMODE\\[1:0\\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3"]
    #[inline(always)]
    pub fn inmsel(&self) -> InmselR {
        InmselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - INPSEL\\[1:0\\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2"]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 15 - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - HYST\\[1:0\\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - BLANKING\\[2:0\\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved"]
    #[inline(always)]
    pub fn blanking(&self) -> BlankingR {
        BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available."]
    #[inline(always)]
    pub fn brgen(&self) -> BrgenR {
        BrgenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable"]
    #[inline(always)]
    pub fn scalen(&self) -> ScalenR {
        ScalenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - VALUE: Comparator output status bit This bit is read-only. It reflects the current comparator output taking into account POLARITY bit effect."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK: COMP_CSR register lock bit This bit is set by software and cleared by a hardware system reset. It locks the whole content of the comparator control register, COMP1_CSR\\[31:0\\]. 0: COMP1_CSR\\[31:0\\] are read/write 1: COMP1_CSR\\[31:0\\] are read-only"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN: Comparator enable bit This bit is set and cleared by software (only if LOCK not set). It switches on Comparator. 0: Comparator switched OFF 1: Comparator switched ON"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PWRMODE\\[1:0\\]: Power Mode of the comparator These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the Comparator. 00:High speed 01 or 10:Medium speed 11:Ultra low power"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PwrmodeW<'_, CsrSpec> {
        PwrmodeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - INMSEL: Comparator input minus selection bits These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of comparator. 000: 1/4 VREFINT 001: 1/2 VREFINT 010: 3/4VREFINT 011: VREFINT 100: DAC OUT 101: PA13 110: PB0 111: PB3"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> InmselW<'_, CsrSpec> {
        InmselW::new(self, 4)
    }
    #[doc = "Bits 7:8 - INPSEL\\[1:0\\]: Comparator input plus selection bit This bit is set and cleared by software (only if LOCK not set). 00: PA14 01: PB1 1x: PB2"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<'_, CsrSpec> {
        InpselW::new(self, 7)
    }
    #[doc = "Bit 15 - POLARITY: Comparator polarity selection bit This bit is set and cleared by software (only if LOCK not set). It inverts Comparator polarity. 0: Comparator output value not inverted 1: Comparator output value inverted"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, CsrSpec> {
        PolarityW::new(self, 15)
    }
    #[doc = "Bits 16:17 - HYST\\[1:0\\]: Comparator hysteresis selection bits These bits are set and cleared by software (only if LOCK not set). They select the Hysteresis voltage of the comparator . 00: No hysteresis 01: Low hysteresis 10: Medium hysteresis 11: High hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, CsrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - BLANKING\\[2:0\\]: Comparator blanking source selection bits These bits select which timer output controls the comparator output blanking. 000: No blanking 001: TIM2 OC4 selected as blanking source 010: TIM16 OC1 selected as blanking source All other values: reserved"]
    #[inline(always)]
    pub fn blanking(&mut self) -> BlankingW<'_, CsrSpec> {
        BlankingW::new(self, 18)
    }
    #[doc = "Bit 22 - BRGEN: Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enable the bridge of the scaler. 0: Scaler resistor bridge disable 1: Scaler resistor bridge enable If SCALEN is set and BRGEN is reset, BG voltage reference is available but not 1/4BGAP, 1/2BGAP, 3/4 BGAP. BGAP value is sent instead of 1/4BGAP, 1/2BGAP, 3/4 BGAP. If SCALEN and BRGEN are set, 1/4 BGAP 1/2BGAP 3/4BGAP and BGAP voltage references are available."]
    #[inline(always)]
    pub fn brgen(&mut self) -> BrgenW<'_, CsrSpec> {
        BrgenW::new(self, 22)
    }
    #[doc = "Bit 23 - SCALEN: Voltage scaler enable bit This bit is set and cleared by software. This bit enable the outputs of the VREFINT divider available on the minus input of the Comparator 0: scaler disable 1: scaler enable"]
    #[inline(always)]
    pub fn scalen(&mut self) -> ScalenW<'_, CsrSpec> {
        ScalenW::new(self, 23)
    }
}
#[doc = "CSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
