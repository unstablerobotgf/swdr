#[doc = "Register `DMA_CMAR3` reader"]
pub type R = crate::R<DmaCmar3Spec>;
#[doc = "Register `DMA_CMAR3` writer"]
pub type W = crate::W<DmaCmar3Spec>;
#[doc = "Field `MA` reader - MA\\[31:0\\]: Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - MA\\[31:0\\]: Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MA\\[31:0\\]: Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MA\\[31:0\\]: Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<'_, DmaCmar3Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA_CMARx register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cmar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCmar3Spec;
impl crate::RegisterSpec for DmaCmar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cmar3::R`](R) reader structure"]
impl crate::Readable for DmaCmar3Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cmar3::W`](W) writer structure"]
impl crate::Writable for DmaCmar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CMAR3 to value 0"]
impl crate::Resettable for DmaCmar3Spec {}
