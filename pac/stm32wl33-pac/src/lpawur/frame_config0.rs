#[doc = "Register `FRAME_CONFIG0` reader"]
pub type R = crate::R<FrameConfig0Spec>;
#[doc = "Register `FRAME_CONFIG0` writer"]
pub type W = crate::W<FrameConfig0Spec>;
#[doc = "Field `PREAMBLE_THRESHOLD_COUNT` reader - The number of transitions for preamble detection when receiving the manchester encoded preamble."]
pub type PreambleThresholdCountR = crate::FieldReader;
#[doc = "Field `PREAMBLE_THRESHOLD_COUNT` writer - The number of transitions for preamble detection when receiving the manchester encoded preamble."]
pub type PreambleThresholdCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNC_LENGTH` reader - Frame sync pattern length ( Manchester encoded )."]
pub type SyncLengthR = crate::BitReader;
#[doc = "Field `SYNC_LENGTH` writer - Frame sync pattern length ( Manchester encoded )."]
pub type SyncLengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_THRESHOLD_COUNT` reader - detection threshold when receivng the Frame sync ( Manchester encoded)."]
pub type SyncThresholdCountR = crate::FieldReader;
#[doc = "Field `SYNC_THRESHOLD_COUNT` writer - detection threshold when receivng the Frame sync ( Manchester encoded)."]
pub type SyncThresholdCountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PAYLOAD_LENGTH` reader - The number of data Bytes in the payload ( decoded )."]
pub type PayloadLengthR = crate::FieldReader;
#[doc = "Field `PAYLOAD_LENGTH` writer - The number of data Bytes in the payload ( decoded )."]
pub type PayloadLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLOW_CLK_CYCLE_PER_BIT_CNT` reader - The number of expected slow clock cycle per each manchester coded bit."]
pub type SlowClkCyclePerBitCntR = crate::FieldReader;
#[doc = "Field `SLOW_CLK_CYCLE_PER_BIT_CNT` writer - The number of expected slow clock cycle per each manchester coded bit."]
pub type SlowClkCyclePerBitCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - The number of transitions for preamble detection when receiving the manchester encoded preamble."]
    #[inline(always)]
    pub fn preamble_threshold_count(&self) -> PreambleThresholdCountR {
        PreambleThresholdCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Frame sync pattern length ( Manchester encoded )."]
    #[inline(always)]
    pub fn sync_length(&self) -> SyncLengthR {
        SyncLengthR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:15 - detection threshold when receivng the Frame sync ( Manchester encoded)."]
    #[inline(always)]
    pub fn sync_threshold_count(&self) -> SyncThresholdCountR {
        SyncThresholdCountR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - The number of data Bytes in the payload ( decoded )."]
    #[inline(always)]
    pub fn payload_length(&self) -> PayloadLengthR {
        PayloadLengthR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 21:25 - The number of expected slow clock cycle per each manchester coded bit."]
    #[inline(always)]
    pub fn slow_clk_cycle_per_bit_cnt(&self) -> SlowClkCyclePerBitCntR {
        SlowClkCyclePerBitCntR::new(((self.bits >> 21) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The number of transitions for preamble detection when receiving the manchester encoded preamble."]
    #[inline(always)]
    pub fn preamble_threshold_count(&mut self) -> PreambleThresholdCountW<'_, FrameConfig0Spec> {
        PreambleThresholdCountW::new(self, 0)
    }
    #[doc = "Bit 8 - Frame sync pattern length ( Manchester encoded )."]
    #[inline(always)]
    pub fn sync_length(&mut self) -> SyncLengthW<'_, FrameConfig0Spec> {
        SyncLengthW::new(self, 8)
    }
    #[doc = "Bits 10:15 - detection threshold when receivng the Frame sync ( Manchester encoded)."]
    #[inline(always)]
    pub fn sync_threshold_count(&mut self) -> SyncThresholdCountW<'_, FrameConfig0Spec> {
        SyncThresholdCountW::new(self, 10)
    }
    #[doc = "Bits 16:19 - The number of data Bytes in the payload ( decoded )."]
    #[inline(always)]
    pub fn payload_length(&mut self) -> PayloadLengthW<'_, FrameConfig0Spec> {
        PayloadLengthW::new(self, 16)
    }
    #[doc = "Bits 21:25 - The number of expected slow clock cycle per each manchester coded bit."]
    #[inline(always)]
    pub fn slow_clk_cycle_per_bit_cnt(&mut self) -> SlowClkCyclePerBitCntW<'_, FrameConfig0Spec> {
        SlowClkCyclePerBitCntW::new(self, 21)
    }
}
#[doc = "FRAME_CONFIG0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_config0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameConfig0Spec;
impl crate::RegisterSpec for FrameConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_config0::R`](R) reader structure"]
impl crate::Readable for FrameConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`frame_config0::W`](W) writer structure"]
impl crate::Writable for FrameConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_CONFIG0 to value 0x0207_4012"]
impl crate::Resettable for FrameConfig0Spec {
    const RESET_VALUE: u32 = 0x0207_4012;
}
