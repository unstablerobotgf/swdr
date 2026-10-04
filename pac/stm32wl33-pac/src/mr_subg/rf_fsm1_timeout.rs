#[doc = "Register `RF_FSM1_TIMEOUT` reader"]
pub type R = crate::R<RfFsm1TimeoutSpec>;
#[doc = "Register `RF_FSM1_TIMEOUT` writer"]
pub type W = crate::W<RfFsm1TimeoutSpec>;
#[doc = "Field `SYNTH_SETUP_TIMER` reader - Timeout management for the RF regulator to stabilize after RF PLL power on"]
pub type SynthSetupTimerR = crate::FieldReader;
#[doc = "Field `SYNTH_SETUP_TIMER` writer - Timeout management for the RF regulator to stabilize after RF PLL power on"]
pub type SynthSetupTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after RF PLL power on"]
    #[inline(always)]
    pub fn synth_setup_timer(&self) -> SynthSetupTimerR {
        SynthSetupTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after RF PLL power on"]
    #[inline(always)]
    pub fn synth_setup_timer(&mut self) -> SynthSetupTimerW<'_, RfFsm1TimeoutSpec> {
        SynthSetupTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM1_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm1_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm1_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm1TimeoutSpec;
impl crate::RegisterSpec for RfFsm1TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm1_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm1TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm1_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm1TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM1_TIMEOUT to value 0x06"]
impl crate::Resettable for RfFsm1TimeoutSpec {
    const RESET_VALUE: u32 = 0x06;
}
