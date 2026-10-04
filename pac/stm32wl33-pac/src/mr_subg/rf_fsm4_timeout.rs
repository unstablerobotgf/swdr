#[doc = "Register `RF_FSM4_TIMEOUT` reader"]
pub type R = crate::R<RfFsm4TimeoutSpec>;
#[doc = "Register `RF_FSM4_TIMEOUT` writer"]
pub type W = crate::W<RfFsm4TimeoutSpec>;
#[doc = "Field `EN_RX_TIMER` reader - Timeout for the analog RX chain setup (duration in EN_RX state)"]
pub type EnRxTimerR = crate::FieldReader;
#[doc = "Field `EN_RX_TIMER` writer - Timeout for the analog RX chain setup (duration in EN_RX state)"]
pub type EnRxTimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeout for the analog RX chain setup (duration in EN_RX state)"]
    #[inline(always)]
    pub fn en_rx_timer(&self) -> EnRxTimerR {
        EnRxTimerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout for the analog RX chain setup (duration in EN_RX state)"]
    #[inline(always)]
    pub fn en_rx_timer(&mut self) -> EnRxTimerW<'_, RfFsm4TimeoutSpec> {
        EnRxTimerW::new(self, 0)
    }
}
#[doc = "RF_FSM4_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm4_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm4_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfFsm4TimeoutSpec;
impl crate::RegisterSpec for RfFsm4TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_fsm4_timeout::R`](R) reader structure"]
impl crate::Readable for RfFsm4TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_fsm4_timeout::W`](W) writer structure"]
impl crate::Writable for RfFsm4TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_FSM4_TIMEOUT to value 0x0f"]
impl crate::Resettable for RfFsm4TimeoutSpec {
    const RESET_VALUE: u32 = 0x0f;
}
