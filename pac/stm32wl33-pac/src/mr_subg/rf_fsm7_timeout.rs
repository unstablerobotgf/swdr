#[doc = "Register `RF_FSM7_TIMEOUT` reader"]
pub type R = crate::R<RfFsm7TimeoutSpec>;
#[doc = "Register `RF_FSM7_TIMEOUT` writer"]
pub type W = crate::W<RfFsm7TimeoutSpec>;
#[doc = "Field `EN_LNA_TIMER` reader - Timeout for the analog RX chain signals settlement once PGA precharge is shut down (duration in EN_LNA state)"]
pub type EnLnaTimerR = crate::FieldReader;
#[doc = "Field `EN_LNA_TIMER` writer - Timeout for the analog RX chain signals settlement once PGA precharge is shut down (duration in EN_LNA state)"]
pub type EnLnaTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the analog RX chain signals settlement once PGA precharge is shut down (duration in EN_LNA state)"]
    #[inline(always)]
    pub fn en_lna_timer(&self) -> EnLnaTimerR {
        EnLnaTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the analog RX chain signals settlement once PGA precharge is shut down (duration in EN_LNA state)"]
    #[inline(always)]
    pub fn en_lna_timer(&mut self) -> EnLnaTimerW<'_, RfFsm7TimeoutSpec> {
        EnLnaTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM7_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm7_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm7_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm7TimeoutSpec;
impl crate::RegisterSpec for RfFsm7TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm7_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm7TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm7_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm7TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM7_TIMEOUT to value 0x05"]
impl crate::Resettable for RfFsm7TimeoutSpec {
    const RESET_VALUE: u32 = 0x05;
}
