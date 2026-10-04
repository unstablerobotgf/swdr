#[doc = "Register `RF_FSM5_TIMEOUT` reader"]
pub type R = crate::R<RfFsm5TimeoutSpec>;
#[doc = "Register `RF_FSM5_TIMEOUT` writer"]
pub type W = crate::W<RfFsm5TimeoutSpec>;
#[doc = "Field `EN_PA_TIMER` reader - Timeout for the analog PA (DAC) setup (duration in EN_PA state)"]
pub type EnPaTimerR = crate::FieldReader;
#[doc = "Field `EN_PA_TIMER` writer - Timeout for the analog PA (DAC) setup (duration in EN_PA state)"]
pub type EnPaTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the analog PA (DAC) setup (duration in EN_PA state)"]
    #[inline(always)]
    pub fn en_pa_timer(&self) -> EnPaTimerR {
        EnPaTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the analog PA (DAC) setup (duration in EN_PA state)"]
    #[inline(always)]
    pub fn en_pa_timer(&mut self) -> EnPaTimerW<'_, RfFsm5TimeoutSpec> {
        EnPaTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM5_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm5_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm5_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm5TimeoutSpec;
impl crate::RegisterSpec for RfFsm5TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm5_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm5TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm5_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm5TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM5_TIMEOUT to value 0x19"]
impl crate::Resettable for RfFsm5TimeoutSpec {
    const RESET_VALUE: u32 = 0x19;
}
