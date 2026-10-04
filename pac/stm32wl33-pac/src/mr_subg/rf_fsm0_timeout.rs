#[doc = "Register `RF_FSM0_TIMEOUT` reader"]
pub type R = crate::R<RfFsm0TimeoutSpec>;
#[doc = "Register `RF_FSM0_TIMEOUT` writer"]
pub type W = crate::W<RfFsm0TimeoutSpec>;
#[doc = "Field `ENA_RFREG_TIMER` reader - Timeout for the RF regulator startup (duration in ENA_RF_REG state)"]
pub type EnaRfregTimerR = crate::FieldReader;
#[doc = "Field `ENA_RFREG_TIMER` writer - Timeout for the RF regulator startup (duration in ENA_RF_REG state)"]
pub type EnaRfregTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the RF regulator startup (duration in ENA_RF_REG state)"]
    #[inline(always)]
    pub fn ena_rfreg_timer(&self) -> EnaRfregTimerR {
        EnaRfregTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the RF regulator startup (duration in ENA_RF_REG state)"]
    #[inline(always)]
    pub fn ena_rfreg_timer(&mut self) -> EnaRfregTimerW<'_, RfFsm0TimeoutSpec> {
        EnaRfregTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM0_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm0_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm0_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm0TimeoutSpec;
impl crate::RegisterSpec for RfFsm0TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm0_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm0TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm0_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm0TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM0_TIMEOUT to value 0"]
impl crate::Resettable for RfFsm0TimeoutSpec {}
