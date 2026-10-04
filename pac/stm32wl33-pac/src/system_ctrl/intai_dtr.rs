#[doc = "Register `INTAI_DTR` reader"]
pub type R = crate::R<IntaiDtrSpec>;
#[doc = "Register `INTAI_DTR` writer"]
pub type W = crate::W<IntaiDtrSpec>;
#[doc = "Field `TX_DT` reader - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
pub type TxDtR = crate::BitReader;
#[doc = "Field `TX_DT` writer - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
pub type TxDtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DT` reader - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
pub type RxDtR = crate::BitReader;
#[doc = "Field `RX_DT` writer - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
pub type RxDtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_DT` reader - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level"]
pub type CompDtR = crate::BitReader;
#[doc = "Field `COMP_DT` writer - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level"]
pub type CompDtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_BUSY_STATUS_DT` reader - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level"]
pub type RfipBusyStatusDtR = crate::BitReader;
#[doc = "Field `RFIP_BUSY_STATUS_DT` writer - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level"]
pub type RfipBusyStatusDtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn tx_dt(&self) -> TxDtR {
        TxDtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn rx_dt(&self) -> RxDtR {
        RxDtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn comp_dt(&self) -> CompDtR {
        CompDtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn rfip_busy_status_dt(&self) -> RfipBusyStatusDtR {
        RfipBusyStatusDtR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn tx_dt(&mut self) -> TxDtW<'_, IntaiDtrSpec> {
        TxDtW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn rx_dt(&mut self) -> RxDtW<'_, IntaiDtrSpec> {
        RxDtW::new(self, 1)
    }
    #[doc = "Bit 4 - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn comp_dt(&mut self) -> CompDtW<'_, IntaiDtrSpec> {
        CompDtW::new(self, 4)
    }
    #[doc = "Bit 5 - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level"]
    #[inline(always)]
    pub fn rfip_busy_status_dt(&mut self) -> RfipBusyStatusDtW<'_, IntaiDtrSpec> {
        RfipBusyStatusDtW::new(self, 5)
    }
}
#[doc = "INTAI_DTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`intai_dtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_dtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaiDtrSpec;
impl crate::RegisterSpec for IntaiDtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intai_dtr::R`](R) reader structure"]
impl crate::Readable for IntaiDtrSpec {}
#[doc = "`write(|w| ..)` method takes [`intai_dtr::W`](W) writer structure"]
impl crate::Writable for IntaiDtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTAI_DTR to value 0"]
impl crate::Resettable for IntaiDtrSpec {}
