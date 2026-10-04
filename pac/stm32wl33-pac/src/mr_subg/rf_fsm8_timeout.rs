#[doc = "Register `RF_FSM8_TIMEOUT` reader"]
pub type R = crate::R<RfFsm8TimeoutSpec>;
#[doc = "Register `RF_FSM8_TIMEOUT` writer"]
pub type W = crate::W<RfFsm8TimeoutSpec>;
#[doc = "Field `SYNTH_PDWN_TIMER` reader - Timeout management for the RF regulator to stabilize after PLL shut down"]
pub type SynthPdwnTimerR = crate::FieldReader;
#[doc = "Field `SYNTH_PDWN_TIMER` writer - Timeout management for the RF regulator to stabilize after PLL shut down"]
pub type SynthPdwnTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after PLL shut down"]
    #[inline(always)]
    pub fn synth_pdwn_timer(&self) -> SynthPdwnTimerR {
        SynthPdwnTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after PLL shut down"]
    #[inline(always)]
    pub fn synth_pdwn_timer(&mut self) -> SynthPdwnTimerW<'_, RfFsm8TimeoutSpec> {
        SynthPdwnTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM8_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm8_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm8_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm8TimeoutSpec;
impl crate::RegisterSpec for RfFsm8TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm8_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm8TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm8_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm8TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM8_TIMEOUT to value 0x0a"]
impl crate::Resettable for RfFsm8TimeoutSpec {
    const RESET_VALUE: u32 = 0x0a;
}
