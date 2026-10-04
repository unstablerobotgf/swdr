#[doc = "Register `INTAI_IBER` reader"]
pub type R = crate::R<IntaiIberSpec>;
#[doc = "Register `INTAI_IBER` writer"]
pub type W = crate::W<IntaiIberSpec>;
#[doc = "Field `TX_IBE` reader - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type TxIbeR = crate::BitReader;
#[doc = "Field `TX_IBE` writer - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type TxIbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IBE` reader - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type RxIbeR = crate::BitReader;
#[doc = "Field `RX_IBE` writer - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type RxIbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_IBE` reader - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type CompIbeR = crate::BitReader;
#[doc = "Field `COMP_IBE` writer - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type CompIbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_BUSY_STATUS_IBE` reader - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type RfipBusyStatusIbeR = crate::BitReader;
#[doc = "Field `RFIP_BUSY_STATUS_IBE` writer - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges"]
pub type RfipBusyStatusIbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn tx_ibe(&self) -> TxIbeR {
        TxIbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn rx_ibe(&self) -> RxIbeR {
        RxIbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn comp_ibe(&self) -> CompIbeR {
        CompIbeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn rfip_busy_status_ibe(&self) -> RfipBusyStatusIbeR {
        RfipBusyStatusIbeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn tx_ibe(&mut self) -> TxIbeW<'_, IntaiIberSpec> {
        TxIbeW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn rx_ibe(&mut self) -> RxIbeW<'_, IntaiIberSpec> {
        RxIbeW::new(self, 1)
    }
    #[doc = "Bit 4 - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn comp_ibe(&mut self) -> CompIbeW<'_, IntaiIberSpec> {
        CompIbeW::new(self, 4)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges"]
    #[inline(always)]
    pub fn rfip_busy_status_ibe(&mut self) -> RfipBusyStatusIbeW<'_, IntaiIberSpec> {
        RfipBusyStatusIbeW::new(self, 5)
    }
}
#[doc = "INTAI_IBER register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_iber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaiIberSpec;
impl crate::RegisterSpec for IntaiIberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intai_iber::R`](R) reader structure"]
impl crate::Readable for IntaiIberSpec {}
#[doc = "`write(|w| ..)` method takes [`intai_iber::W`](W) writer structure"]
impl crate::Writable for IntaiIberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTAI_IBER to value 0"]
impl crate::Resettable for IntaiIberSpec {}
