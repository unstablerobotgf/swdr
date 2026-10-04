#[doc = "Register `INTAI_ISCR` reader"]
pub type R = crate::R<IntaiIscrSpec>;
#[doc = "Register `INTAI_ISCR` writer"]
pub type W = crate::W<IntaiIscrSpec>;
#[doc = "Field `TX_ISC` reader - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred"]
pub type TxIscR = crate::BitReader;
#[doc = "Field `TX_ISC` writer - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred"]
pub type TxIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ISC` reader - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred"]
pub type RxIscR = crate::BitReader;
#[doc = "Field `RX_ISC` writer - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred"]
pub type RxIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ISEDGE` reader - TX_ISEDGE: interrupt edge status on TX_SEQUENCE signal: 0: falling edge on TX_SEQUENCE detected. 1: rising edge on TX_SEQUENCE detected."]
pub type TxIsedgeR = crate::BitReader;
#[doc = "Field `RX_ISEDGE` reader - RX_ISEDGE: interrupt edge status on RX_SEQUENCE signal: 0: falling edge on RX_SEQUENCE detected. 1: rising edge on RX_SEQUENCE detected."]
pub type RxIsedgeR = crate::BitReader;
#[doc = "Field `COMP_ISC` reader - COMP_ISC: interrupt status on COMP_OUT (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on COMP_OUT detected. 1: activity on COMP_OUT occurred"]
pub type CompIscR = crate::BitReader;
#[doc = "Field `COMP_ISC` writer - COMP_ISC: interrupt status on COMP_OUT (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on COMP_OUT detected. 1: activity on COMP_OUT occurred"]
pub type CompIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_BUSY_STATUS_ISC` reader - RFIP_BUSY_STATUS_ISC: interrupt status on RFIP_BUSY_STATUS (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on RFIP_BUSY_STATUS detected. 1: activity on RFIP_BUSY_STATUS occurred"]
pub type RfipBusyStatusIscR = crate::BitReader;
#[doc = "Field `RFIP_BUSY_STATUS_ISC` writer - RFIP_BUSY_STATUS_ISC: interrupt status on RFIP_BUSY_STATUS (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on RFIP_BUSY_STATUS detected. 1: activity on RFIP_BUSY_STATUS occurred"]
pub type RfipBusyStatusIscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred"]
    #[inline(always)]
    pub fn tx_isc(&self) -> TxIscR {
        TxIscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred"]
    #[inline(always)]
    pub fn rx_isc(&self) -> RxIscR {
        RxIscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX_ISEDGE: interrupt edge status on TX_SEQUENCE signal: 0: falling edge on TX_SEQUENCE detected. 1: rising edge on TX_SEQUENCE detected."]
    #[inline(always)]
    pub fn tx_isedge(&self) -> TxIsedgeR {
        TxIsedgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX_ISEDGE: interrupt edge status on RX_SEQUENCE signal: 0: falling edge on RX_SEQUENCE detected. 1: rising edge on RX_SEQUENCE detected."]
    #[inline(always)]
    pub fn rx_isedge(&self) -> RxIsedgeR {
        RxIsedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP_ISC: interrupt status on COMP_OUT (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on COMP_OUT detected. 1: activity on COMP_OUT occurred"]
    #[inline(always)]
    pub fn comp_isc(&self) -> CompIscR {
        CompIscR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_ISC: interrupt status on RFIP_BUSY_STATUS (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on RFIP_BUSY_STATUS detected. 1: activity on RFIP_BUSY_STATUS occurred"]
    #[inline(always)]
    pub fn rfip_busy_status_isc(&self) -> RfipBusyStatusIscR {
        RfipBusyStatusIscR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred"]
    #[inline(always)]
    pub fn tx_isc(&mut self) -> TxIscW<'_, IntaiIscrSpec> {
        TxIscW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred"]
    #[inline(always)]
    pub fn rx_isc(&mut self) -> RxIscW<'_, IntaiIscrSpec> {
        RxIscW::new(self, 1)
    }
    #[doc = "Bit 4 - COMP_ISC: interrupt status on COMP_OUT (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on COMP_OUT detected. 1: activity on COMP_OUT occurred"]
    #[inline(always)]
    pub fn comp_isc(&mut self) -> CompIscW<'_, IntaiIscrSpec> {
        CompIscW::new(self, 4)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_ISC: interrupt status on RFIP_BUSY_STATUS (can be a rising or a falling edge depending on INTAI_IEVR and INTAI_IBER): 0: no activity on RFIP_BUSY_STATUS detected. 1: activity on RFIP_BUSY_STATUS occurred"]
    #[inline(always)]
    pub fn rfip_busy_status_isc(&mut self) -> RfipBusyStatusIscW<'_, IntaiIscrSpec> {
        RfipBusyStatusIscW::new(self, 5)
    }
}
#[doc = "INTAI_ISCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_iscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaiIscrSpec;
impl crate::RegisterSpec for IntaiIscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intai_iscr::R`](R) reader structure"]
impl crate::Readable for IntaiIscrSpec {}
#[doc = "`write(|w| ..)` method takes [`intai_iscr::W`](W) writer structure"]
impl crate::Writable for IntaiIscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTAI_ISCR to value 0"]
impl crate::Resettable for IntaiIscrSpec {}
