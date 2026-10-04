#[doc = "Register `RF_FSM2_TIMEOUT` reader"]
pub type R = crate::R<RfFsm2TimeoutSpec>;
#[doc = "Register `RF_FSM2_TIMEOUT` writer"]
pub type W = crate::W<RfFsm2TimeoutSpec>;
#[doc = "Field `VCO_CALIB_LOCK_TIMER` reader - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)"]
pub type VcoCalibLockTimerR = crate::FieldReader;
#[doc = "Field `VCO_CALIB_LOCK_TIMER` writer - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)"]
pub type VcoCalibLockTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)"]
    #[inline(always)]
    pub fn vco_calib_lock_timer(&self) -> VcoCalibLockTimerR {
        VcoCalibLockTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)"]
    #[inline(always)]
    pub fn vco_calib_lock_timer(&mut self) -> VcoCalibLockTimerW<'_, RfFsm2TimeoutSpec> {
        VcoCalibLockTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM2_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm2_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm2_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm2TimeoutSpec;
impl crate::RegisterSpec for RfFsm2TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm2_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm2TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm2_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm2TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM2_TIMEOUT to value 0x50"]
impl crate::Resettable for RfFsm2TimeoutSpec {
    const RESET_VALUE: u32 = 0x50;
}
