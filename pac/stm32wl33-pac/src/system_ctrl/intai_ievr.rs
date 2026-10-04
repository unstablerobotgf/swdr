#[doc = "Register `INTAI_IEVR` reader"]
pub type R = crate::R<IntaiIevrSpec>;
#[doc = "Register `INTAI_IEVR` writer"]
pub type W = crate::W<IntaiIevrSpec>;
#[doc = "Field `TX_IEV` reader - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type TxIevR = crate::BitReader;
#[doc = "Field `TX_IEV` writer - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type TxIevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IEV` reader - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type RxIevR = crate::BitReader;
#[doc = "Field `RX_IEV` writer - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type RxIevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_IEV` reader - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type CompIevR = crate::BitReader;
#[doc = "Field `COMP_IEV` writer - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type CompIevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_BUSY_STATUS_IEV` reader - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type RfipBusyStatusIevR = crate::BitReader;
#[doc = "Field `RFIP_BUSY_STATUS_IEV` writer - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
pub type RfipBusyStatusIevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn tx_iev(&self) -> TxIevR {
        TxIevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn rx_iev(&self) -> RxIevR {
        RxIevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn comp_iev(&self) -> CompIevR {
        CompIevR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn rfip_busy_status_iev(&self) -> RfipBusyStatusIevR {
        RfipBusyStatusIevR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn tx_iev(&mut self) -> TxIevW<'_, IntaiIevrSpec> {
        TxIevW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn rx_iev(&mut self) -> RxIevW<'_, IntaiIevrSpec> {
        RxIevW::new(self, 1)
    }
    #[doc = "Bit 4 - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn comp_iev(&mut self) -> CompIevW<'_, IntaiIevrSpec> {
        CompIevW::new(self, 4)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level"]
    #[inline(always)]
    pub fn rfip_busy_status_iev(&mut self) -> RfipBusyStatusIevW<'_, IntaiIevrSpec> {
        RfipBusyStatusIevW::new(self, 5)
    }
}
#[doc = "INTAI_IEVR register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_ievr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ievr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaiIevrSpec;
impl crate::RegisterSpec for IntaiIevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intai_ievr::R`](R) reader structure"]
impl crate::Readable for IntaiIevrSpec {}
#[doc = "`write(|w| ..)` method takes [`intai_ievr::W`](W) writer structure"]
impl crate::Writable for IntaiIevrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTAI_IEVR to value 0"]
impl crate::Resettable for IntaiIevrSpec {}
