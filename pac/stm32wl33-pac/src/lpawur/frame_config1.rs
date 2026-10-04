#[doc = "Register `FRAME_CONFIG1` reader"]
pub type R = crate::R<FrameConfig1Spec>;
#[doc = "Register `FRAME_CONFIG1` writer"]
pub type W = crate::W<FrameConfig1Spec>;
#[doc = "Field `KI` reader - ki gain value for the timing recovery loop."]
pub type KiR = crate::FieldReader;
#[doc = "Field `KI` writer - ki gain value for the timing recovery loop."]
pub type KiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KP` reader - kp gain value for the timing recovery loop."]
pub type KpR = crate::FieldReader;
#[doc = "Field `KP` writer - kp gain value for the timing recovery loop."]
pub type KpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRAME_SYNC_COUNTER_TIMEOUT` reader - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised."]
pub type FrameSyncCounterTimeoutR = crate::FieldReader;
#[doc = "Field `FRAME_SYNC_COUNTER_TIMEOUT` writer - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised."]
pub type FrameSyncCounterTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PREAMBLE_ENABLE` reader - Preamble detection enable"]
pub type PreambleEnableR = crate::BitReader;
#[doc = "Field `PREAMBLE_ENABLE` writer - Preamble detection enable"]
pub type PreambleEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TREC_LOOP_ALGO_SEL` reader - Timing recovery loop algorithm selection:"]
pub type TrecLoopAlgoSelR = crate::BitReader;
#[doc = "Field `TREC_LOOP_ALGO_SEL` writer - Timing recovery loop algorithm selection:"]
pub type TrecLoopAlgoSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - ki gain value for the timing recovery loop."]
    #[inline(always)]
    pub fn ki(&self) -> KiR {
        KiR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - kp gain value for the timing recovery loop."]
    #[inline(always)]
    pub fn kp(&self) -> KpR {
        KpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised."]
    #[inline(always)]
    pub fn frame_sync_counter_timeout(&self) -> FrameSyncCounterTimeoutR {
        FrameSyncCounterTimeoutR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Preamble detection enable"]
    #[inline(always)]
    pub fn preamble_enable(&self) -> PreambleEnableR {
        PreambleEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timing recovery loop algorithm selection:"]
    #[inline(always)]
    pub fn trec_loop_algo_sel(&self) -> TrecLoopAlgoSelR {
        TrecLoopAlgoSelR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ki gain value for the timing recovery loop."]
    #[inline(always)]
    pub fn ki(&mut self) -> KiW<'_, FrameConfig1Spec> {
        KiW::new(self, 0)
    }
    #[doc = "Bits 4:7 - kp gain value for the timing recovery loop."]
    #[inline(always)]
    pub fn kp(&mut self) -> KpW<'_, FrameConfig1Spec> {
        KpW::new(self, 4)
    }
    #[doc = "Bits 8:15 - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised."]
    #[inline(always)]
    pub fn frame_sync_counter_timeout(&mut self) -> FrameSyncCounterTimeoutW<'_, FrameConfig1Spec> {
        FrameSyncCounterTimeoutW::new(self, 8)
    }
    #[doc = "Bit 17 - Preamble detection enable"]
    #[inline(always)]
    pub fn preamble_enable(&mut self) -> PreambleEnableW<'_, FrameConfig1Spec> {
        PreambleEnableW::new(self, 17)
    }
    #[doc = "Bit 18 - Timing recovery loop algorithm selection:"]
    #[inline(always)]
    pub fn trec_loop_algo_sel(&mut self) -> TrecLoopAlgoSelW<'_, FrameConfig1Spec> {
        TrecLoopAlgoSelW::new(self, 18)
    }
}
#[doc = "FRAME_CONFIG1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameConfig1Spec;
impl crate::RegisterSpec for FrameConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_config1::R`](R) reader structure"]
impl crate::Readable for FrameConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`frame_config1::W`](W) writer structure"]
impl crate::Writable for FrameConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_CONFIG1 to value 0x0002_4669"]
impl crate::Resettable for FrameConfig1Spec {
    const RESET_VALUE: u32 = 0x0002_4669;
}
