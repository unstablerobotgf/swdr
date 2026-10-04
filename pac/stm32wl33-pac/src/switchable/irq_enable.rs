#[doc = "Register `IRQ_ENABLE` reader"]
pub type R = crate::R<IrqEnableSpec>;
#[doc = "Register `IRQ_ENABLE` writer"]
pub type W = crate::W<IrqEnableSpec>;
#[doc = "Field `BIT_SYNC_DETECTED_E` reader - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type BitSyncDetectedER = crate::BitReader;
#[doc = "Field `BIT_SYNC_DETECTED_E` writer - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type BitSyncDetectedEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_SYNC_COMPLETE_E` reader - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type FrameSyncCompleteER = crate::BitReader;
#[doc = "Field `FRAME_SYNC_COMPLETE_E` writer - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type FrameSyncCompleteEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_COMPLETE_E` reader - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
pub type FrameCompleteER = crate::BitReader;
#[doc = "Field `FRAME_COMPLETE_E` writer - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
pub type FrameCompleteEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_VALID_E` reader - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
pub type FrameValidER = crate::BitReader;
#[doc = "Field `FRAME_VALID_E` writer - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
pub type FrameValidEW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn bit_sync_detected_e(&self) -> BitSyncDetectedER {
        BitSyncDetectedER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn frame_sync_complete_e(&self) -> FrameSyncCompleteER {
        FrameSyncCompleteER::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
    #[inline(always)]
    pub fn frame_complete_e(&self) -> FrameCompleteER {
        FrameCompleteER::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
    #[inline(always)]
    pub fn frame_valid_e(&self) -> FrameValidER {
        FrameValidER::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn bit_sync_detected_e(&mut self) -> BitSyncDetectedEW<'_, IrqEnableSpec> {
        BitSyncDetectedEW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn frame_sync_complete_e(&mut self) -> FrameSyncCompleteEW<'_, IrqEnableSpec> {
        FrameSyncCompleteEW::new(self, 1)
    }
    #[doc = "Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
    #[inline(always)]
    pub fn frame_complete_e(&mut self) -> FrameCompleteEW<'_, IrqEnableSpec> {
        FrameCompleteEW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
    #[inline(always)]
    pub fn frame_valid_e(&mut self) -> FrameValidEW<'_, IrqEnableSpec> {
        FrameValidEW::new(self, 3)
    }
}
#[doc = "IRQ_ENABLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnableSpec;
impl crate::RegisterSpec for IrqEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enable::R`](R) reader structure"]
impl crate::Readable for IrqEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure"]
impl crate::Writable for IrqEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENABLE to value 0"]
impl crate::Resettable for IrqEnableSpec {}
