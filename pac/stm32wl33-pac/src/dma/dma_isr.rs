#[doc = "Register `DMA_ISR` reader"]
pub type R = crate::R<DmaIsrSpec>;
#[doc = "Field `GIF1` reader - GIF1: Channel 1 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 1 1: A TE, HT or TC event occurred on channel 1"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `TCIF1` reader - TCIF1: Channel 1 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 1 1: A transfer complete (TC) event occurred on channel 1"]
pub type Tcif1R = crate::BitReader;
#[doc = "Field `HTIF1` reader - HTIF1: Channel 1 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 1 1: A half transfer (HT) event occurred on channel 1"]
pub type Htif1R = crate::BitReader;
#[doc = "Field `TE1F1` reader - TEIF1: Channel 1 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 1 1: A transfer error (TE) occurred on channel 1"]
pub type Te1f1R = crate::BitReader;
#[doc = "Field `GIF2` reader - GIF2: Channel 2 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 2 1: A TE, HT or TC event occurred on channel 2"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `TCIF2` reader - TCIF2: Channel 2 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 2 1: A transfer complete (TC) event occurred on channel 2"]
pub type Tcif2R = crate::BitReader;
#[doc = "Field `HTIF2` reader - HTIF2: Channel 2 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 2 1: A half transfer (HT) event occurred on channel 2"]
pub type Htif2R = crate::BitReader;
#[doc = "Field `TE1F2` reader - TEIF2: Channel 2 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 2 1: A transfer error (TE) occurred on channel 2"]
pub type Te1f2R = crate::BitReader;
#[doc = "Field `GIF3` reader - GIF3: Channel 3 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 3 1: A TE, HT or TC event occurred on channel 3"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `TCIF3` reader - TCIF3: Channel 3 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 3 1: A transfer complete (TC) event occurred on channel 3"]
pub type Tcif3R = crate::BitReader;
#[doc = "Field `HTIF3` reader - HTIF3: Channel 3 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 3 1: A half transfer (HT) event occurred on channel 3"]
pub type Htif3R = crate::BitReader;
#[doc = "Field `TE1F3` reader - TEIF3: Channel 3 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 3 1: A transfer error (TE) occurred on channel 3"]
pub type Te1f3R = crate::BitReader;
#[doc = "Field `GIF4` reader - GIF4: Channel 4 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 4 1: A TE, HT or TC event occurred on channel 4"]
pub type Gif4R = crate::BitReader;
#[doc = "Field `TCIF4` reader - TCIF4: Channel 4 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 4 1: A transfer complete (TC) event occurred on channel 4"]
pub type Tcif4R = crate::BitReader;
#[doc = "Field `HTIF4` reader - HTIF4: Channel 4 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 4 1: A half transfer (HT) event occurred on channel 4"]
pub type Htif4R = crate::BitReader;
#[doc = "Field `TE1F4` reader - TEIF4: Channel 4 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 4 1: A transfer error (TE) occurred on channel 4"]
pub type Te1f4R = crate::BitReader;
#[doc = "Field `GIF5` reader - GIF5: Channel 5 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 5 1: A TE, HT or TC event occurred on channel 5"]
pub type Gif5R = crate::BitReader;
#[doc = "Field `TCIF5` reader - TCIF5: Channel 5 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 5 1: A transfer complete (TC) event occurred on channel 5"]
pub type Tcif5R = crate::BitReader;
#[doc = "Field `HTIF5` reader - HTIF5: Channel 5 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 5 1: A half transfer (HT) event occurred on channel 5"]
pub type Htif5R = crate::BitReader;
#[doc = "Field `TE1F5` reader - TEIF5: Channel 5 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 5 1: A transfer error (TE) occurred on channel 5"]
pub type Te1f5R = crate::BitReader;
#[doc = "Field `GIF6` reader - GIF6: Channel 6 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 6 1: A TE, HT or TC event occurred on channel 6"]
pub type Gif6R = crate::BitReader;
#[doc = "Field `TCIF6` reader - TCIF6: Channel 6 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 6 1: A transfer complete (TC) event occurred on channel 6"]
pub type Tcif6R = crate::BitReader;
#[doc = "Field `HTIF6` reader - HTIF6: Channel 6 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 6 1: A half transfer (HT) event occurred on channel 6"]
pub type Htif6R = crate::BitReader;
#[doc = "Field `TE1F6` reader - TEIF6: Channel 6 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 6 1: A transfer error (TE) occurred on channel 6"]
pub type Te1f6R = crate::BitReader;
#[doc = "Field `GIF7` reader - GIF7: Channel 7 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 7 1: A TE, HT or TC event occurred on channel 7"]
pub type Gif7R = crate::BitReader;
#[doc = "Field `TCIF7` reader - TCIF7: Channel 7 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 7 1: A transfer complete (TC) event occurred on channel 7"]
pub type Tcif7R = crate::BitReader;
#[doc = "Field `HTIF7` reader - HTIF7: Channel 7 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 7 1: A half transfer (HT) event occurred on channel 7"]
pub type Htif7R = crate::BitReader;
#[doc = "Field `TE1F7` reader - TEIF7: Channel 7 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 7 1: A transfer error (TE) occurred on channel 7"]
pub type Te1f7R = crate::BitReader;
#[doc = "Field `GIF8` reader - GIF8: Channel 8 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 8 1: A TE, HT or TC event occurred on channel 8"]
pub type Gif8R = crate::BitReader;
#[doc = "Field `TCIF8` reader - TCIF8: Channel 8 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 8 1: A transfer complete (TC) event occurred on channel 8"]
pub type Tcif8R = crate::BitReader;
#[doc = "Field `HTIF8` reader - HTIF8: Channel 8 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 8 1: A half transfer (HT) event occurred on channel 8"]
pub type Htif8R = crate::BitReader;
#[doc = "Field `TE1F8` reader - TEIF8: Channel 8 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 8 1: A transfer error (TE) occurred on channel 8"]
pub type Te1f8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GIF1: Channel 1 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 1 1: A TE, HT or TC event occurred on channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCIF1: Channel 1 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 1 1: A transfer complete (TC) event occurred on channel 1"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIF1: Channel 1 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 1 1: A half transfer (HT) event occurred on channel 1"]
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TEIF1: Channel 1 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 1 1: A transfer error (TE) occurred on channel 1"]
    #[inline(always)]
    pub fn te1f1(&self) -> Te1f1R {
        Te1f1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GIF2: Channel 2 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 2 1: A TE, HT or TC event occurred on channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCIF2: Channel 2 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 2 1: A transfer complete (TC) event occurred on channel 2"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HTIF2: Channel 2 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 2 1: A half transfer (HT) event occurred on channel 2"]
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TEIF2: Channel 2 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 2 1: A transfer error (TE) occurred on channel 2"]
    #[inline(always)]
    pub fn te1f2(&self) -> Te1f2R {
        Te1f2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GIF3: Channel 3 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 3 1: A TE, HT or TC event occurred on channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCIF3: Channel 3 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 3 1: A transfer complete (TC) event occurred on channel 3"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HTIF3: Channel 3 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 3 1: A half transfer (HT) event occurred on channel 3"]
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TEIF3: Channel 3 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 3 1: A transfer error (TE) occurred on channel 3"]
    #[inline(always)]
    pub fn te1f3(&self) -> Te1f3R {
        Te1f3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GIF4: Channel 4 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 4 1: A TE, HT or TC event occurred on channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCIF4: Channel 4 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 4 1: A transfer complete (TC) event occurred on channel 4"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HTIF4: Channel 4 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 4 1: A half transfer (HT) event occurred on channel 4"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TEIF4: Channel 4 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 4 1: A transfer error (TE) occurred on channel 4"]
    #[inline(always)]
    pub fn te1f4(&self) -> Te1f4R {
        Te1f4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GIF5: Channel 5 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 5 1: A TE, HT or TC event occurred on channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCIF5: Channel 5 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 5 1: A transfer complete (TC) event occurred on channel 5"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HTIF5: Channel 5 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 5 1: A half transfer (HT) event occurred on channel 5"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEIF5: Channel 5 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 5 1: A transfer error (TE) occurred on channel 5"]
    #[inline(always)]
    pub fn te1f5(&self) -> Te1f5R {
        Te1f5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GIF6: Channel 6 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 6 1: A TE, HT or TC event occurred on channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCIF6: Channel 6 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 6 1: A transfer complete (TC) event occurred on channel 6"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTIF6: Channel 6 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 6 1: A half transfer (HT) event occurred on channel 6"]
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        Htif6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEIF6: Channel 6 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 6 1: A transfer error (TE) occurred on channel 6"]
    #[inline(always)]
    pub fn te1f6(&self) -> Te1f6R {
        Te1f6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GIF7: Channel 7 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 7 1: A TE, HT or TC event occurred on channel 7"]
    #[inline(always)]
    pub fn gif7(&self) -> Gif7R {
        Gif7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCIF7: Channel 7 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 7 1: A transfer complete (TC) event occurred on channel 7"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HTIF7: Channel 7 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 7 1: A half transfer (HT) event occurred on channel 7"]
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        Htif7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TEIF7: Channel 7 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 7 1: A transfer error (TE) occurred on channel 7"]
    #[inline(always)]
    pub fn te1f7(&self) -> Te1f7R {
        Te1f7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GIF8: Channel 8 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 8 1: A TE, HT or TC event occurred on channel 8"]
    #[inline(always)]
    pub fn gif8(&self) -> Gif8R {
        Gif8R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TCIF8: Channel 8 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 8 1: A transfer complete (TC) event occurred on channel 8"]
    #[inline(always)]
    pub fn tcif8(&self) -> Tcif8R {
        Tcif8R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HTIF8: Channel 8 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 8 1: A half transfer (HT) event occurred on channel 8"]
    #[inline(always)]
    pub fn htif8(&self) -> Htif8R {
        Htif8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEIF8: Channel 8 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 8 1: A transfer error (TE) occurred on channel 8"]
    #[inline(always)]
    pub fn te1f8(&self) -> Te1f8R {
        Te1f8R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMA_ISR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIsrSpec;
impl crate::RegisterSpec for DmaIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_isr::R`](R) reader structure"]
impl crate::Readable for DmaIsrSpec {}
#[doc = "`reset()` method sets DMA_ISR to value 0"]
impl crate::Resettable for DmaIsrSpec {}
