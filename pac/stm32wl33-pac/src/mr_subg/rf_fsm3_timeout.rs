#[doc = "Register `RF_FSM3_TIMEOUT` reader"]
pub type R = crate::R<RfFsm3TimeoutSpec>;
#[doc = "Register `RF_FSM3_TIMEOUT` writer"]
pub type W = crate::W<RfFsm3TimeoutSpec>;
#[doc = "Field `VCO_LOCK_TIMER` reader - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)"]
pub type VcoLockTimerR = crate::FieldReader;
#[doc = "Field `VCO_LOCK_TIMER` writer - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)"]
pub type VcoLockTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)"]
    #[inline(always)]
    pub fn vco_lock_timer(&self) -> VcoLockTimerR {
        VcoLockTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)"]
    #[inline(always)]
    pub fn vco_lock_timer(&mut self) -> VcoLockTimerW<'_, RfFsm3TimeoutSpec> {
        VcoLockTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM3_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm3_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm3_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm3TimeoutSpec;
impl crate::RegisterSpec for RfFsm3TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm3_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm3TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm3_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm3TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM3_TIMEOUT to value 0x28"]
impl crate::Resettable for RfFsm3TimeoutSpec {
    const RESET_VALUE: u32 = 0x28;
}
