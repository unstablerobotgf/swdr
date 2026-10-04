#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ADC_ON_OFF` reader - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC"]
pub type AdcOnOffR = crate::BitReader;
#[doc = "Field `ADC_ON_OFF` writer - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC"]
pub type AdcOnOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_CONV` writer - START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware."]
pub type StartConvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_OP_MODE` writer - STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit."]
pub type StopOpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_MODE` reader - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)"]
pub type TestModeR = crate::BitReader;
#[doc = "Field `TEST_MODE` writer - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)"]
pub type TestModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC"]
    #[inline(always)]
    pub fn adc_on_off(&self) -> AdcOnOffR {
        AdcOnOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)"]
    #[inline(always)]
    pub fn test_mode(&self) -> TestModeR {
        TestModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC"]
    #[inline(always)]
    pub fn adc_on_off(&mut self) -> AdcOnOffW<'_, CtrlSpec> {
        AdcOnOffW::new(self, 0)
    }
    #[doc = "Bit 1 - START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware."]
    #[inline(always)]
    pub fn start_conv(&mut self) -> StartConvW<'_, CtrlSpec> {
        StartConvW::new(self, 1)
    }
    #[doc = "Bit 2 - STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit."]
    #[inline(always)]
    pub fn stop_op_mode(&mut self) -> StopOpModeW<'_, CtrlSpec> {
        StopOpModeW::new(self, 2)
    }
    #[doc = "Bit 4 - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)"]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TestModeW<'_, CtrlSpec> {
        TestModeW::new(self, 4)
    }
}
#[doc = "CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
