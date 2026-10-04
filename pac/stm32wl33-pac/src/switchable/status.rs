#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BIT_SYNC_DETECTED_F` reader - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type BitSyncDetectedFR = crate::BitReader;
#[doc = "Field `BIT_SYNC_DETECTED_F` writer - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type BitSyncDetectedFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_SYNC_COMPLETE_F` reader - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type FrameSyncCompleteFR = crate::BitReader;
#[doc = "Field `FRAME_SYNC_COMPLETE_F` writer - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
pub type FrameSyncCompleteFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_COMPLETE_F` reader - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
pub type FrameCompleteFR = crate::BitReader;
#[doc = "Field `FRAME_COMPLETE_F` writer - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
pub type FrameCompleteFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_VALID_F` reader - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
pub type FrameValidFR = crate::BitReader;
#[doc = "Field `FRAME_VALID_F` writer - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
pub type FrameValidFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_F` reader - - 11 : CRC error"]
pub type ErrorFR = crate::FieldReader;
#[doc = "Field `ERROR_F` writer - - 11 : CRC error"]
pub type ErrorFW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn bit_sync_detected_f(&self) -> BitSyncDetectedFR {
        BitSyncDetectedFR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn frame_sync_complete_f(&self) -> FrameSyncCompleteFR {
        FrameSyncCompleteFR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
    #[inline(always)]
    pub fn frame_complete_f(&self) -> FrameCompleteFR {
        FrameCompleteFR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
    #[inline(always)]
    pub fn frame_valid_f(&self) -> FrameValidFR {
        FrameValidFR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 30:31 - - 11 : CRC error"]
    #[inline(always)]
    pub fn error_f(&self) -> ErrorFR {
        ErrorFR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn bit_sync_detected_f(&mut self) -> BitSyncDetectedFW<'_, StatusSpec> {
        BitSyncDetectedFW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid."]
    #[inline(always)]
    pub fn frame_sync_complete_f(&mut self) -> FrameSyncCompleteFW<'_, StatusSpec> {
        FrameSyncCompleteFW::new(self, 1)
    }
    #[doc = "Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid."]
    #[inline(always)]
    pub fn frame_complete_f(&mut self) -> FrameCompleteFW<'_, StatusSpec> {
        FrameCompleteFW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC)."]
    #[inline(always)]
    pub fn frame_valid_f(&mut self) -> FrameValidFW<'_, StatusSpec> {
        FrameValidFW::new(self, 3)
    }
    #[doc = "Bits 30:31 - - 11 : CRC error"]
    #[inline(always)]
    pub fn error_f(&mut self) -> ErrorFW<'_, StatusSpec> {
        ErrorFW::new(self, 30)
    }
}
#[doc = "STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
