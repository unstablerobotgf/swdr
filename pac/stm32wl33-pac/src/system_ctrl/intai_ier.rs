#[doc = "Register `INTAI_IER` reader"]
pub type R = crate::R<IntaiIerSpec>;
#[doc = "Register `INTAI_IER` writer"]
pub type W = crate::W<IntaiIerSpec>;
#[doc = "Field `TX_IE` reader - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled"]
pub type TxIeR = crate::BitReader;
#[doc = "Field `TX_IE` writer - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled"]
pub type TxIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IE` reader - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled"]
pub type RxIeR = crate::BitReader;
#[doc = "Field `RX_IE` writer - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled"]
pub type RxIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_IE` reader - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled"]
pub type CompIeR = crate::BitReader;
#[doc = "Field `COMP_IE` writer - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled"]
pub type CompIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_BUSY_STATUS_IE` reader - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled"]
pub type RfipBusyStatusIeR = crate::BitReader;
#[doc = "Field `RFIP_BUSY_STATUS_IE` writer - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled"]
pub type RfipBusyStatusIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled"]
    #[inline(always)]
    pub fn tx_ie(&self) -> TxIeR {
        TxIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled"]
    #[inline(always)]
    pub fn rx_ie(&self) -> RxIeR {
        RxIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled"]
    #[inline(always)]
    pub fn comp_ie(&self) -> CompIeR {
        CompIeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled"]
    #[inline(always)]
    pub fn rfip_busy_status_ie(&self) -> RfipBusyStatusIeR {
        RfipBusyStatusIeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled"]
    #[inline(always)]
    pub fn tx_ie(&mut self) -> TxIeW<'_, IntaiIerSpec> {
        TxIeW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled"]
    #[inline(always)]
    pub fn rx_ie(&mut self) -> RxIeW<'_, IntaiIerSpec> {
        RxIeW::new(self, 1)
    }
    #[doc = "Bit 4 - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled"]
    #[inline(always)]
    pub fn comp_ie(&mut self) -> CompIeW<'_, IntaiIerSpec> {
        CompIeW::new(self, 4)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled"]
    #[inline(always)]
    pub fn rfip_busy_status_ie(&mut self) -> RfipBusyStatusIeW<'_, IntaiIerSpec> {
        RfipBusyStatusIeW::new(self, 5)
    }
}
#[doc = "INTAI_IER register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaiIerSpec;
impl crate::RegisterSpec for IntaiIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intai_ier::R`](R) reader structure"]
impl crate::Readable for IntaiIerSpec {}
#[doc = "`write(|w| ..)` method takes [`intai_ier::W`](W) writer structure"]
impl crate::Writable for IntaiIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTAI_IER to value 0"]
impl crate::Resettable for IntaiIerSpec {}
