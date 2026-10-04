#[doc = "Register `AGC2_CTRL` reader"]
pub type R = crate::R<Agc2CtrlSpec>;
#[doc = "Register `AGC2_CTRL` writer"]
pub type W = crate::W<Agc2CtrlSpec>;
#[doc = "Field `AGC_MEAS_TIME` reader - Measure time."]
pub type AgcMeasTimeR = crate::FieldReader;
#[doc = "Field `AGC_MEAS_TIME` writer - Measure time."]
pub type AgcMeasTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AGC_START_MAX_ATTEN` reader - Start the AGC with maximum attenuation."]
pub type AgcStartMaxAttenR = crate::BitReader;
#[doc = "Field `AGC_START_MAX_ATTEN` writer - Start the AGC with maximum attenuation."]
pub type AgcStartMaxAttenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_FREEZE_ON_SYNC` reader - Enable the freeze on SYNC detection feature"]
pub type AgcFreezeOnSyncR = crate::BitReader;
#[doc = "Field `AGC_FREEZE_ON_SYNC` writer - Enable the freeze on SYNC detection feature"]
pub type AgcFreezeOnSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_FREEZE_ON_STEADY` reader - Enable the autofreeze feature"]
pub type AgcFreezeOnSteadyR = crate::BitReader;
#[doc = "Field `AGC_FREEZE_ON_STEADY` writer - Enable the autofreeze feature"]
pub type AgcFreezeOnSteadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_HIGH_ATTEN_MODE` reader - Enable the high attenuation mode."]
pub type AgcHighAttenModeR = crate::BitReader;
#[doc = "Field `AGC_HIGH_ATTEN_MODE` writer - Enable the high attenuation mode."]
pub type AgcHighAttenModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Measure time."]
    #[inline(always)]
    pub fn agc_meas_time(&self) -> AgcMeasTimeR {
        AgcMeasTimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Start the AGC with maximum attenuation."]
    #[inline(always)]
    pub fn agc_start_max_atten(&self) -> AgcStartMaxAttenR {
        AgcStartMaxAttenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the freeze on SYNC detection feature"]
    #[inline(always)]
    pub fn agc_freeze_on_sync(&self) -> AgcFreezeOnSyncR {
        AgcFreezeOnSyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the autofreeze feature"]
    #[inline(always)]
    pub fn agc_freeze_on_steady(&self) -> AgcFreezeOnSteadyR {
        AgcFreezeOnSteadyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the high attenuation mode."]
    #[inline(always)]
    pub fn agc_high_atten_mode(&self) -> AgcHighAttenModeR {
        AgcHighAttenModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Measure time."]
    #[inline(always)]
    pub fn agc_meas_time(&mut self) -> AgcMeasTimeW<'_, Agc2CtrlSpec> {
        AgcMeasTimeW::new(self, 0)
    }
    #[doc = "Bit 4 - Start the AGC with maximum attenuation."]
    #[inline(always)]
    pub fn agc_start_max_atten(&mut self) -> AgcStartMaxAttenW<'_, Agc2CtrlSpec> {
        AgcStartMaxAttenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable the freeze on SYNC detection feature"]
    #[inline(always)]
    pub fn agc_freeze_on_sync(&mut self) -> AgcFreezeOnSyncW<'_, Agc2CtrlSpec> {
        AgcFreezeOnSyncW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable the autofreeze feature"]
    #[inline(always)]
    pub fn agc_freeze_on_steady(&mut self) -> AgcFreezeOnSteadyW<'_, Agc2CtrlSpec> {
        AgcFreezeOnSteadyW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable the high attenuation mode."]
    #[inline(always)]
    pub fn agc_high_atten_mode(&mut self) -> AgcHighAttenModeW<'_, Agc2CtrlSpec> {
        AgcHighAttenModeW::new(self, 7)
    }
}
#[doc = "AGC2_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agc2CtrlSpec;
impl crate::RegisterSpec for Agc2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc2_ctrl::R`](R) reader structure"]
impl crate::Readable for Agc2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`agc2_ctrl::W`](W) writer structure"]
impl crate::Writable for Agc2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC2_CTRL to value 0xaf"]
impl crate::Resettable for Agc2CtrlSpec {
    const RESET_VALUE: u32 = 0xaf;
}
