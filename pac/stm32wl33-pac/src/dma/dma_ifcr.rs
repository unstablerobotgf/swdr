#[doc = "Register `DMA_IFCR` writer"]
pub type W = crate::W<DmaIfcrSpec>;
#[doc = "Field `CGIF1` writer - CGIF1: Channel 1 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - CTCIF1: Channel 1 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - CHTIF1: Channel 1 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - CTEIF1: Channel 1 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - CGIF2: Channel 2 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - CTCIF2: Channel 2 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - CHTIF2: Channel 2 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - CTEIF2: Channel 2 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - CGIF3: Channel 3 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - CTCIF3: Channel 3 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - CHTIF3: Channel 3 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - CTEIF3: Channel 3 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` writer - CGIF4: Channel 4 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - CTCIF4: Channel 4 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - CHTIF4: Channel 4 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - CTEIF4: Channel 4 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` writer - CGIF5: Channel 5 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - CTCIF5: Channel 5 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - CHTIF5: Channel 5 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - CTEIF5: Channel 5 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF6` writer - CGIF6: Channel 6 global interrupt clear This bit is set and cleared by software. 0: No effect. 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - CTCIF6: Channel 6 transfer complete clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - CHTIF6: Channel 6 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - CTEIF6: Channel 6 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF7` writer - CGIF7: Channel 7 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - CTCIF7: Channel 7 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - CHTIF7: Channel 7 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - CTEIF7: Channel 7 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF8` writer - CGIF8: Channel 8 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
pub type Cgif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF8` writer - CTCIF8: Channel 8 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
pub type Ctcif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF8` writer - CHTIF8: Channel 8 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
pub type Chtif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF8` writer - CTEIF8: Channel 8 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
pub type Cteif8W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CGIF1: Channel 1 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif1(&mut self) -> Cgif1W<'_, DmaIfcrSpec> {
        Cgif1W::new(self, 0)
    }
    #[doc = "Bit 1 - CTCIF1: Channel 1 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> Ctcif1W<'_, DmaIfcrSpec> {
        Ctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - CHTIF1: Channel 1 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> Chtif1W<'_, DmaIfcrSpec> {
        Chtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - CTEIF1: Channel 1 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> Cteif1W<'_, DmaIfcrSpec> {
        Cteif1W::new(self, 3)
    }
    #[doc = "Bit 4 - CGIF2: Channel 2 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif2(&mut self) -> Cgif2W<'_, DmaIfcrSpec> {
        Cgif2W::new(self, 4)
    }
    #[doc = "Bit 5 - CTCIF2: Channel 2 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> Ctcif2W<'_, DmaIfcrSpec> {
        Ctcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - CHTIF2: Channel 2 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> Chtif2W<'_, DmaIfcrSpec> {
        Chtif2W::new(self, 6)
    }
    #[doc = "Bit 7 - CTEIF2: Channel 2 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> Cteif2W<'_, DmaIfcrSpec> {
        Cteif2W::new(self, 7)
    }
    #[doc = "Bit 8 - CGIF3: Channel 3 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif3(&mut self) -> Cgif3W<'_, DmaIfcrSpec> {
        Cgif3W::new(self, 8)
    }
    #[doc = "Bit 9 - CTCIF3: Channel 3 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> Ctcif3W<'_, DmaIfcrSpec> {
        Ctcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - CHTIF3: Channel 3 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> Chtif3W<'_, DmaIfcrSpec> {
        Chtif3W::new(self, 10)
    }
    #[doc = "Bit 11 - CTEIF3: Channel 3 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> Cteif3W<'_, DmaIfcrSpec> {
        Cteif3W::new(self, 11)
    }
    #[doc = "Bit 12 - CGIF4: Channel 4 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif4(&mut self) -> Cgif4W<'_, DmaIfcrSpec> {
        Cgif4W::new(self, 12)
    }
    #[doc = "Bit 13 - CTCIF4: Channel 4 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> Ctcif4W<'_, DmaIfcrSpec> {
        Ctcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - CHTIF4: Channel 4 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> Chtif4W<'_, DmaIfcrSpec> {
        Chtif4W::new(self, 14)
    }
    #[doc = "Bit 15 - CTEIF4: Channel 4 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> Cteif4W<'_, DmaIfcrSpec> {
        Cteif4W::new(self, 15)
    }
    #[doc = "Bit 16 - CGIF5: Channel 5 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif5(&mut self) -> Cgif5W<'_, DmaIfcrSpec> {
        Cgif5W::new(self, 16)
    }
    #[doc = "Bit 17 - CTCIF5: Channel 5 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> Ctcif5W<'_, DmaIfcrSpec> {
        Ctcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - CHTIF5: Channel 5 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> Chtif5W<'_, DmaIfcrSpec> {
        Chtif5W::new(self, 18)
    }
    #[doc = "Bit 19 - CTEIF5: Channel 5 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> Cteif5W<'_, DmaIfcrSpec> {
        Cteif5W::new(self, 19)
    }
    #[doc = "Bit 20 - CGIF6: Channel 6 global interrupt clear This bit is set and cleared by software. 0: No effect. 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif6(&mut self) -> Cgif6W<'_, DmaIfcrSpec> {
        Cgif6W::new(self, 20)
    }
    #[doc = "Bit 21 - CTCIF6: Channel 6 transfer complete clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> Ctcif6W<'_, DmaIfcrSpec> {
        Ctcif6W::new(self, 21)
    }
    #[doc = "Bit 22 - CHTIF6: Channel 6 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> Chtif6W<'_, DmaIfcrSpec> {
        Chtif6W::new(self, 22)
    }
    #[doc = "Bit 23 - CTEIF6: Channel 6 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> Cteif6W<'_, DmaIfcrSpec> {
        Cteif6W::new(self, 23)
    }
    #[doc = "Bit 24 - CGIF7: Channel 7 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif7(&mut self) -> Cgif7W<'_, DmaIfcrSpec> {
        Cgif7W::new(self, 24)
    }
    #[doc = "Bit 25 - CTCIF7: Channel 7 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> Ctcif7W<'_, DmaIfcrSpec> {
        Ctcif7W::new(self, 25)
    }
    #[doc = "Bit 26 - CHTIF7: Channel 7 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> Chtif7W<'_, DmaIfcrSpec> {
        Chtif7W::new(self, 26)
    }
    #[doc = "Bit 27 - CTEIF7: Channel 7 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> Cteif7W<'_, DmaIfcrSpec> {
        Cteif7W::new(self, 27)
    }
    #[doc = "Bit 28 - CGIF8: Channel 8 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register"]
    #[inline(always)]
    pub fn cgif8(&mut self) -> Cgif8W<'_, DmaIfcrSpec> {
        Cgif8W::new(self, 28)
    }
    #[doc = "Bit 29 - CTCIF8: Channel 8 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn ctcif8(&mut self) -> Ctcif8W<'_, DmaIfcrSpec> {
        Ctcif8W::new(self, 29)
    }
    #[doc = "Bit 30 - CHTIF8: Channel 8 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn chtif8(&mut self) -> Chtif8W<'_, DmaIfcrSpec> {
        Chtif8W::new(self, 30)
    }
    #[doc = "Bit 31 - CTEIF8: Channel 8 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register"]
    #[inline(always)]
    pub fn cteif8(&mut self) -> Cteif8W<'_, DmaIfcrSpec> {
        Cteif8W::new(self, 31)
    }
}
#[doc = "DMA_IFCR register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIfcrSpec;
impl crate::RegisterSpec for DmaIfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_ifcr::W`](W) writer structure"]
impl crate::Writable for DmaIfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IFCR to value 0"]
impl crate::Resettable for DmaIfcrSpec {}
