#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `DBA` reader - DBA\\[4:0\\]: DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: 00000: TIMx_CR1, 00001: TIMx_CR2, 00010: Reserved, ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
pub type DbaR = crate::FieldReader;
#[doc = "Field `DBA` writer - DBA\\[4:0\\]: DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: 00000: TIMx_CR1, 00001: TIMx_CR2, 00010: Reserved, ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
pub type DbaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBL` reader - DBL\\[4:0\\]: DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). 00000: 1 transfer, 00001: 2 transfers, 00010: 3 transfers, ... 10001: 18 transfers."]
pub type DblR = crate::FieldReader;
#[doc = "Field `DBL` writer - DBL\\[4:0\\]: DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). 00000: 1 transfer, 00001: 2 transfers, 00010: 3 transfers, ... 10001: 18 transfers."]
pub type DblW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DBA\\[4:0\\]: DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: 00000: TIMx_CR1, 00001: TIMx_CR2, 00010: Reserved, ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
    #[inline(always)]
    pub fn dba(&self) -> DbaR {
        DbaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBL\\[4:0\\]: DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). 00000: 1 transfer, 00001: 2 transfers, 00010: 3 transfers, ... 10001: 18 transfers."]
    #[inline(always)]
    pub fn dbl(&self) -> DblR {
        DblR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DBA\\[4:0\\]: DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: 00000: TIMx_CR1, 00001: TIMx_CR2, 00010: Reserved, ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
    #[inline(always)]
    pub fn dba(&mut self) -> DbaW<'_, DcrSpec> {
        DbaW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DBL\\[4:0\\]: DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). 00000: 1 transfer, 00001: 2 transfers, 00010: 3 transfers, ... 10001: 18 transfers."]
    #[inline(always)]
    pub fn dbl(&mut self) -> DblW<'_, DcrSpec> {
        DblW::new(self, 8)
    }
}
#[doc = "DCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
