#[doc = "Register `DMA_CPAR4` reader"]
pub type R = crate::R<DmaCpar4Spec>;
#[doc = "Register `DMA_CPAR4` writer"]
pub type W = crate::W<DmaCpar4Spec>;
#[doc = "Field `PA` reader - PA\\[31:0\\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - PA\\[31:0\\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PA\\[31:0\\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PA\\[31:0\\]: Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a halfword address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, DmaCpar4Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "DMA_CPARx register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCpar4Spec;
impl crate::RegisterSpec for DmaCpar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cpar4::R`](R) reader structure"]
impl crate::Readable for DmaCpar4Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cpar4::W`](W) writer structure"]
impl crate::Writable for DmaCpar4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CPAR4 to value 0"]
impl crate::Resettable for DmaCpar4Spec {}
