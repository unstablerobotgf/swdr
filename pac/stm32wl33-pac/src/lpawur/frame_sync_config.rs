#[doc = "Register `FRAME_SYNC_CONFIG` reader"]
pub type R = crate::R<FrameSyncConfigSpec>;
#[doc = "Register `FRAME_SYNC_CONFIG` writer"]
pub type W = crate::W<FrameSyncConfigSpec>;
#[doc = "Field `FRAME_SYNC_PATTERN_L` reader - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)"]
pub type FrameSyncPatternLR = crate::FieldReader<u16>;
#[doc = "Field `FRAME_SYNC_PATTERN_L` writer - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)"]
pub type FrameSyncPatternLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FRAME_SYNC_PATTERN_H` reader - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )"]
pub type FrameSyncPatternHR = crate::FieldReader<u16>;
#[doc = "Field `FRAME_SYNC_PATTERN_H` writer - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )"]
pub type FrameSyncPatternHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)"]
    #[inline(always)]
    pub fn frame_sync_pattern_l(&self) -> FrameSyncPatternLR {
        FrameSyncPatternLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )"]
    #[inline(always)]
    pub fn frame_sync_pattern_h(&self) -> FrameSyncPatternHR {
        FrameSyncPatternHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)"]
    #[inline(always)]
    pub fn frame_sync_pattern_l(&mut self) -> FrameSyncPatternLW<'_, FrameSyncConfigSpec> {
        FrameSyncPatternLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )"]
    #[inline(always)]
    pub fn frame_sync_pattern_h(&mut self) -> FrameSyncPatternHW<'_, FrameSyncConfigSpec> {
        FrameSyncPatternHW::new(self, 16)
    }
}
#[doc = "FRAME_SYNC_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_sync_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_sync_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSyncConfigSpec;
impl crate::RegisterSpec for FrameSyncConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_sync_config::R`](R) reader structure"]
impl crate::Readable for FrameSyncConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`frame_sync_config::W`](W) writer structure"]
impl crate::Writable for FrameSyncConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_SYNC_CONFIG to value 0x9696"]
impl crate::Resettable for FrameSyncConfigSpec {
    const RESET_VALUE: u32 = 0x9696;
}
