#[doc = "Register `TEST_CONF` reader"]
pub type R = crate::R<TestConfSpec>;
#[doc = "Register `TEST_CONF` writer"]
pub type W = crate::W<TestConfSpec>;
#[doc = "Field `ADC_SWITCH_EN` reader - ADC_SWITCH_EN\\[15:0\\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\\[15:0\\]): Bit 0: VINM\\[0\\] to ADC negative input Bit 1: VINM\\[1\\] to ADC negative input Bit 2: VINM\\[2\\] to ADC negative input Bit 3: VINM\\[3\\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input."]
pub type AdcSwitchEnR = crate::FieldReader<u16>;
#[doc = "Field `ADC_SWITCH_EN` writer - ADC_SWITCH_EN\\[15:0\\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\\[15:0\\]): Bit 0: VINM\\[0\\] to ADC negative input Bit 1: VINM\\[1\\] to ADC negative input Bit 2: VINM\\[2\\] to ADC negative input Bit 3: VINM\\[3\\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input."]
pub type AdcSwitchEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SEL_VIN_TYPE` reader - SEL_VIN_TYPE\\[1:0\\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved"]
pub type SelVinTypeR = crate::FieldReader;
#[doc = "Field `SEL_VIN_TYPE` writer - SEL_VIN_TYPE\\[1:0\\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved"]
pub type SelVinTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_RUN` reader - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion."]
pub type AdcRunR = crate::BitReader;
#[doc = "Field `ADC_RUN` writer - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion."]
pub type AdcRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_ENABLE` reader - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)"]
pub type AdcEnableR = crate::BitReader;
#[doc = "Field `ADC_ENABLE` writer - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)"]
pub type AdcEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADC_SWITCH_EN\\[15:0\\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\\[15:0\\]): Bit 0: VINM\\[0\\] to ADC negative input Bit 1: VINM\\[1\\] to ADC negative input Bit 2: VINM\\[2\\] to ADC negative input Bit 3: VINM\\[3\\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input."]
    #[inline(always)]
    pub fn adc_switch_en(&self) -> AdcSwitchEnR {
        AdcSwitchEnR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 18:19 - SEL_VIN_TYPE\\[1:0\\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved"]
    #[inline(always)]
    pub fn sel_vin_type(&self) -> SelVinTypeR {
        SelVinTypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion."]
    #[inline(always)]
    pub fn adc_run(&self) -> AdcRunR {
        AdcRunR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)"]
    #[inline(always)]
    pub fn adc_enable(&self) -> AdcEnableR {
        AdcEnableR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC_SWITCH_EN\\[15:0\\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\\[15:0\\]): Bit 0: VINM\\[0\\] to ADC negative input Bit 1: VINM\\[1\\] to ADC negative input Bit 2: VINM\\[2\\] to ADC negative input Bit 3: VINM\\[3\\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input."]
    #[inline(always)]
    pub fn adc_switch_en(&mut self) -> AdcSwitchEnW<'_, TestConfSpec> {
        AdcSwitchEnW::new(self, 0)
    }
    #[doc = "Bits 18:19 - SEL_VIN_TYPE\\[1:0\\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved"]
    #[inline(always)]
    pub fn sel_vin_type(&mut self) -> SelVinTypeW<'_, TestConfSpec> {
        SelVinTypeW::new(self, 18)
    }
    #[doc = "Bit 21 - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion."]
    #[inline(always)]
    pub fn adc_run(&mut self) -> AdcRunW<'_, TestConfSpec> {
        AdcRunW::new(self, 21)
    }
    #[doc = "Bit 22 - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)"]
    #[inline(always)]
    pub fn adc_enable(&mut self) -> AdcEnableW<'_, TestConfSpec> {
        AdcEnableW::new(self, 22)
    }
}
#[doc = "TEST_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestConfSpec;
impl crate::RegisterSpec for TestConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_conf::R`](R) reader structure"]
impl crate::Readable for TestConfSpec {}
#[doc = "`write(|w| ..)` method takes [`test_conf::W`](W) writer structure"]
impl crate::Writable for TestConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST_CONF to value 0"]
impl crate::Resettable for TestConfSpec {}
