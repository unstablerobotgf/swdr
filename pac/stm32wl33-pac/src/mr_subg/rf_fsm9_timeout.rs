#[doc = "Register `RF_FSM9_TIMEOUT` reader"]
pub type R = crate::R<RfFsm9TimeoutSpec>;
#[doc = "Register `RF_FSM9_TIMEOUT` writer"]
pub type W = crate::W<RfFsm9TimeoutSpec>;
#[doc = "Field `END_RX_TIMER` reader - Timeout management for the RF regulator to stabilize after analog RX chain shut down"]
pub type EndRxTimerR = crate::FieldReader;
#[doc = "Field `END_RX_TIMER` writer - Timeout management for the RF regulator to stabilize after analog RX chain shut down"]
pub type EndRxTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after analog RX chain shut down"]
    #[inline(always)]
    pub fn end_rx_timer(&self) -> EndRxTimerR {
        EndRxTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after analog RX chain shut down"]
    #[inline(always)]
    pub fn end_rx_timer(&mut self) -> EndRxTimerW<'_, RfFsm9TimeoutSpec> {
        EndRxTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM9_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm9_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm9_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm9TimeoutSpec;
impl crate::RegisterSpec for RfFsm9TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm9_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm9TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm9_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm9TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM9_TIMEOUT to value 0x06"]
impl crate::Resettable for RfFsm9TimeoutSpec {
    const RESET_VALUE: u32 = 0x06;
}
