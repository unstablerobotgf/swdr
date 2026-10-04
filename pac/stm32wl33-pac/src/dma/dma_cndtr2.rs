#[doc = "Register `DMA_CNDTR2` reader"]
pub type R = crate::R<DmaCndtr2Spec>;
#[doc = "Register `DMA_CNDTR2` writer"]
pub type W = crate::W<DmaCndtr2Spec>;
#[doc = "Field `NDT` reader - NDT\\[15:0\\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - NDT\\[15:0\\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NDT\\[15:0\\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NDT\\[15:0\\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<'_, DmaCndtr2Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "DMA_CNDTRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cndtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCndtr2Spec;
impl crate::RegisterSpec for DmaCndtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cndtr2::R`](R) reader structure"]
impl crate::Readable for DmaCndtr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cndtr2::W`](W) writer structure"]
impl crate::Writable for DmaCndtr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CNDTR2 to value 0"]
impl crate::Resettable for DmaCndtr2Spec {}
