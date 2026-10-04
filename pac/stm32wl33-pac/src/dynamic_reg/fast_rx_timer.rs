#[doc = "Register `FAST_RX_TIMER` reader"]
pub type R = crate::R<FastRxTimerSpec>;
#[doc = "Register `FAST_RX_TIMER` writer"]
pub type W = crate::W<FastRxTimerSpec>;
#[doc = "Field `FAST_RX_TIMEOUT` reader - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)"]
pub type FastRxTimeoutR = crate::FieldReader;
#[doc = "Field `FAST_RX_TIMEOUT` writer - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)"]
pub type FastRxTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FAST_CS_TERM_EN` reader - Enable the Fast RX Termination feature"]
pub type FastCsTermEnR = crate::BitReader;
#[doc = "Field `FAST_CS_TERM_EN` writer - Enable the Fast RX Termination feature"]
pub type FastCsTermEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)"]
    #[inline(always)]
    pub fn fast_rx_timeout(&self) -> FastRxTimeoutR {
        FastRxTimeoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable the Fast RX Termination feature"]
    #[inline(always)]
    pub fn fast_cs_term_en(&self) -> FastCsTermEnR {
        FastCsTermEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)"]
    #[inline(always)]
    pub fn fast_rx_timeout(&mut self) -> FastRxTimeoutW<'_, FastRxTimerSpec> {
        FastRxTimeoutW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable the Fast RX Termination feature"]
    #[inline(always)]
    pub fn fast_cs_term_en(&mut self) -> FastCsTermEnW<'_, FastRxTimerSpec> {
        FastCsTermEnW::new(self, 8)
    }
}
#[doc = "FAST_RX_TIMER register\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_rx_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_rx_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastRxTimerSpec;
impl crate::RegisterSpec for FastRxTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_rx_timer::R`](R) reader structure"]
impl crate::Readable for FastRxTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`fast_rx_timer::W`](W) writer structure"]
impl crate::Writable for FastRxTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FAST_RX_TIMER to value 0"]
impl crate::Resettable for FastRxTimerSpec {}
