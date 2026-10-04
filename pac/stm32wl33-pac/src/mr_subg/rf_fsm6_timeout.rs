#[doc = "Register `RF_FSM6_TIMEOUT` reader"]
pub type R = crate::R<RfFsm6TimeoutSpec>;
#[doc = "Register `RF_FSM6_TIMEOUT` writer"]
pub type W = crate::W<RfFsm6TimeoutSpec>;
#[doc = "Field `PA_DWN_ANA_TIMER` reader - Timeout for the analog PA (DAC) ramp down (duration in PA_DWN_ANA state)"]
pub type PaDwnAnaTimerR = crate::FieldReader;
#[doc = "Field `PA_DWN_ANA_TIMER` writer - Timeout for the analog PA (DAC) ramp down (duration in PA_DWN_ANA state)"]
pub type PaDwnAnaTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the analog PA (DAC) ramp down (duration in PA_DWN_ANA state)"]
    #[inline(always)]
    pub fn pa_dwn_ana_timer(&self) -> PaDwnAnaTimerR {
        PaDwnAnaTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the analog PA (DAC) ramp down (duration in PA_DWN_ANA state)"]
    #[inline(always)]
    pub fn pa_dwn_ana_timer(&mut self) -> PaDwnAnaTimerW<'_, RfFsm6TimeoutSpec> {
        PaDwnAnaTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM6_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm6_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm6_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm6TimeoutSpec;
impl crate::RegisterSpec for RfFsm6TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm6_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm6TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm6_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm6TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM6_TIMEOUT to value 0x19"]
impl crate::Resettable for RfFsm6TimeoutSpec {
    const RESET_VALUE: u32 = 0x19;
}
