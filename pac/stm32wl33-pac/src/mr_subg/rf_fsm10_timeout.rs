#[doc = "Register `RF_FSM10_TIMEOUT` reader"]
pub type R = crate::R<RfFsm10TimeoutSpec>;
#[doc = "Register `RF_FSM10_TIMEOUT` writer"]
pub type W = crate::W<RfFsm10TimeoutSpec>;
#[doc = "Field `END_TX_TIMER` reader - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block"]
pub type EndTxTimerR = crate::FieldReader;
#[doc = "Field `END_TX_TIMER` writer - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block"]
pub type EndTxTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block"]
    #[inline(always)]
    pub fn end_tx_timer(&self) -> EndTxTimerR {
        EndTxTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block"]
    #[inline(always)]
    pub fn end_tx_timer(&mut self) -> EndTxTimerW<'_, RfFsm10TimeoutSpec> {
        EndTxTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM10_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm10_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm10_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm10TimeoutSpec;
impl crate::RegisterSpec for RfFsm10TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm10_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm10TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm10_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm10TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM10_TIMEOUT to value 0x06"]
impl crate::Resettable for RfFsm10TimeoutSpec {
    const RESET_VALUE: u32 = 0x06;
}
