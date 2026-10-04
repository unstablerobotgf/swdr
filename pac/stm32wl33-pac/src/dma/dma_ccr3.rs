#[doc = "Register `DMA_CCR3` reader"]
pub type R = crate::R<DmaCcr3Spec>;
#[doc = "Register `DMA_CCR3` writer"]
pub type W = crate::W<DmaCcr3Spec>;
#[doc = "Field `EN` reader - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled"]
pub type HtieR = crate::BitReader;
#[doc = "Field `HTIE` writer - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled"]
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIRC` reader - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled"]
pub type CircR = crate::BitReader;
#[doc = "Field `CIRC` writer - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled"]
pub type CircW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled"]
pub type PincR = crate::BitReader;
#[doc = "Field `PINC` writer - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled"]
pub type PincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled"]
pub type MincR = crate::BitReader;
#[doc = "Field `MINC` writer - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled"]
pub type MincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - PSIZE\\[1:0\\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
pub type PsizeR = crate::FieldReader;
#[doc = "Field `PSIZE` writer - PSIZE\\[1:0\\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - MSIZE\\[1:0\\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
pub type MsizeR = crate::FieldReader;
#[doc = "Field `MSIZE` writer - MSIZE\\[1:0\\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PL` reader - PL\\[1:0\\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high"]
pub type PlR = crate::FieldReader;
#[doc = "Field `PL` writer - PL\\[1:0\\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high"]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM2MEM` reader - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled"]
pub type Mem2memR = crate::BitReader;
#[doc = "Field `MEM2MEM` writer - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled"]
pub type Mem2memW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled"]
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled"]
    #[inline(always)]
    pub fn circ(&self) -> CircR {
        CircR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled"]
    #[inline(always)]
    pub fn pinc(&self) -> PincR {
        PincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled"]
    #[inline(always)]
    pub fn minc(&self) -> MincR {
        MincR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PSIZE\\[1:0\\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MSIZE\\[1:0\\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PL\\[1:0\\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high"]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled"]
    #[inline(always)]
    pub fn mem2mem(&self) -> Mem2memR {
        Mem2memR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN: Channel enable This bit is set and cleared by software. 0: Channel disabled 1: Channel enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, DmaCcr3Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - TCIE: Transfer complete interrupt enable This bit is set and cleared by software. 0: TC interrupt disabled 1: TC interrupt enabled"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, DmaCcr3Spec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - HTIE: Half transfer interrupt enable This bit is set and cleared by software. 0: HT interrupt disabled 1: HT interrupt enabled"]
    #[inline(always)]
    pub fn htie(&mut self) -> HtieW<'_, DmaCcr3Spec> {
        HtieW::new(self, 2)
    }
    #[doc = "Bit 3 - TEIE: Transfer error interrupt enable This bit is set and cleared by software. 0: TE interrupt disabled 1: TE interrupt enabled"]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<'_, DmaCcr3Spec> {
        TeieW::new(self, 3)
    }
    #[doc = "Bit 4 - DIR: Data transfer direction This bit is set and cleared by software. 0: Read from peripheral. 1: Read from memory"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, DmaCcr3Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - CIRC: Circular mode This bit is set and cleared by software. 0: Circular mode disabled 1: Circular mode enabled"]
    #[inline(always)]
    pub fn circ(&mut self) -> CircW<'_, DmaCcr3Spec> {
        CircW::new(self, 5)
    }
    #[doc = "Bit 6 - PINC: Peripheral increment mode This bit is set and cleared by software. 0: Peripheral increment mode disabled 1: Peripheral increment mode enabled"]
    #[inline(always)]
    pub fn pinc(&mut self) -> PincW<'_, DmaCcr3Spec> {
        PincW::new(self, 6)
    }
    #[doc = "Bit 7 - MINC: Memory increment mode This bit is set and cleared by software. 0: Memory increment mode disabled 1: Memory increment mode enabled"]
    #[inline(always)]
    pub fn minc(&mut self) -> MincW<'_, DmaCcr3Spec> {
        MincW::new(self, 7)
    }
    #[doc = "Bits 8:9 - PSIZE\\[1:0\\]: Peripheral size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
    #[inline(always)]
    pub fn psize(&mut self) -> PsizeW<'_, DmaCcr3Spec> {
        PsizeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - MSIZE\\[1:0\\]: Memory size These bits are set and cleared by software. 00: 8-bits 01: 16-bits 10: 32-bits"]
    #[inline(always)]
    pub fn msize(&mut self) -> MsizeW<'_, DmaCcr3Spec> {
        MsizeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PL\\[1:0\\]: Channel priority level These bits are set and cleared by software. 00: Low 01: Medium 10: High 11: Very high"]
    #[inline(always)]
    pub fn pl(&mut self) -> PlW<'_, DmaCcr3Spec> {
        PlW::new(self, 12)
    }
    #[doc = "Bit 14 - MEM2MEM: Memory to memory mode This bit is set and cleared by software. 0: Memory to memory mode disabled 1: Memory to memory mode enabled"]
    #[inline(always)]
    pub fn mem2mem(&mut self) -> Mem2memW<'_, DmaCcr3Spec> {
        Mem2memW::new(self, 14)
    }
}
#[doc = "DMA_CCRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCcr3Spec;
impl crate::RegisterSpec for DmaCcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ccr3::R`](R) reader structure"]
impl crate::Readable for DmaCcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_ccr3::W`](W) writer structure"]
impl crate::Writable for DmaCcr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CCR3 to value 0"]
impl crate::Resettable for DmaCcr3Spec {}
