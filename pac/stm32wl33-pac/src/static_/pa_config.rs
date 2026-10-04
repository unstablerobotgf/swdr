#[doc = "Register `PA_CONFIG` reader"]
pub type R = crate::R<PaConfigSpec>;
#[doc = "Register `PA_CONFIG` writer"]
pub type W = crate::W<PaConfigSpec>;
#[doc = "Field `PA_RAMP_STEP_WIDTH` reader - Step width (unit: 1/8 of bit period)."]
pub type PaRampStepWidthR = crate::FieldReader;
#[doc = "Field `PA_RAMP_STEP_WIDTH` writer - Step width (unit: 1/8 of bit period)."]
pub type PaRampStepWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA_LEVEL_MAX_INDEX` reader - Final level for power ramping (i."]
pub type PaLevelMaxIndexR = crate::FieldReader;
#[doc = "Field `PA_LEVEL_MAX_INDEX` writer - Final level for power ramping (i."]
pub type PaLevelMaxIndexW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PA_INTERP_EN` reader - Enable power level interpolator."]
pub type PaInterpEnR = crate::BitReader;
#[doc = "Field `PA_INTERP_EN` writer - Enable power level interpolator."]
pub type PaInterpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASK_OOK_EN` reader - Enable the generation of the internal TXDATA signal provided to the FIR."]
pub type AskOokEnR = crate::BitReader;
#[doc = "Field `ASK_OOK_EN` writer - Enable the generation of the internal TXDATA signal provided to the FIR."]
pub type AskOokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_DRV_MODE` reader - Select the PA topology"]
pub type PaDrvModeR = crate::FieldReader;
#[doc = "Field `PA_DRV_MODE` writer - Select the PA topology"]
pub type PaDrvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA_MODE` reader - Configure the Power Amplifier (PA) mode"]
pub type PaModeR = crate::FieldReader;
#[doc = "Field `PA_MODE` writer - Configure the Power Amplifier (PA) mode"]
pub type PaModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LIN_NLOG` reader - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator"]
pub type LinNlogR = crate::BitReader;
#[doc = "Field `LIN_NLOG` writer - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator"]
pub type LinNlogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_RAMP_ENABLE` reader - Enable the power ramping"]
pub type PaRampEnableR = crate::BitReader;
#[doc = "Field `PA_RAMP_ENABLE` writer - Enable the power ramping"]
pub type PaRampEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Step width (unit: 1/8 of bit period)."]
    #[inline(always)]
    pub fn pa_ramp_step_width(&self) -> PaRampStepWidthR {
        PaRampStepWidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Final level for power ramping (i."]
    #[inline(always)]
    pub fn pa_level_max_index(&self) -> PaLevelMaxIndexR {
        PaLevelMaxIndexR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 6 - Enable power level interpolator."]
    #[inline(always)]
    pub fn pa_interp_en(&self) -> PaInterpEnR {
        PaInterpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the generation of the internal TXDATA signal provided to the FIR."]
    #[inline(always)]
    pub fn ask_ook_en(&self) -> AskOokEnR {
        AskOokEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Select the PA topology"]
    #[inline(always)]
    pub fn pa_drv_mode(&self) -> PaDrvModeR {
        PaDrvModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configure the Power Amplifier (PA) mode"]
    #[inline(always)]
    pub fn pa_mode(&self) -> PaModeR {
        PaModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator"]
    #[inline(always)]
    pub fn lin_nlog(&self) -> LinNlogR {
        LinNlogR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable the power ramping"]
    #[inline(always)]
    pub fn pa_ramp_enable(&self) -> PaRampEnableR {
        PaRampEnableR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Step width (unit: 1/8 of bit period)."]
    #[inline(always)]
    pub fn pa_ramp_step_width(&mut self) -> PaRampStepWidthW<'_, PaConfigSpec> {
        PaRampStepWidthW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Final level for power ramping (i."]
    #[inline(always)]
    pub fn pa_level_max_index(&mut self) -> PaLevelMaxIndexW<'_, PaConfigSpec> {
        PaLevelMaxIndexW::new(self, 2)
    }
    #[doc = "Bit 6 - Enable power level interpolator."]
    #[inline(always)]
    pub fn pa_interp_en(&mut self) -> PaInterpEnW<'_, PaConfigSpec> {
        PaInterpEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable the generation of the internal TXDATA signal provided to the FIR."]
    #[inline(always)]
    pub fn ask_ook_en(&mut self) -> AskOokEnW<'_, PaConfigSpec> {
        AskOokEnW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Select the PA topology"]
    #[inline(always)]
    pub fn pa_drv_mode(&mut self) -> PaDrvModeW<'_, PaConfigSpec> {
        PaDrvModeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Configure the Power Amplifier (PA) mode"]
    #[inline(always)]
    pub fn pa_mode(&mut self) -> PaModeW<'_, PaConfigSpec> {
        PaModeW::new(self, 10)
    }
    #[doc = "Bit 13 - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator"]
    #[inline(always)]
    pub fn lin_nlog(&mut self) -> LinNlogW<'_, PaConfigSpec> {
        LinNlogW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable the power ramping"]
    #[inline(always)]
    pub fn pa_ramp_enable(&mut self) -> PaRampEnableW<'_, PaConfigSpec> {
        PaRampEnableW::new(self, 14)
    }
}
#[doc = "PA_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaConfigSpec;
impl crate::RegisterSpec for PaConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_config::R`](R) reader structure"]
impl crate::Readable for PaConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_config::W`](W) writer structure"]
impl crate::Writable for PaConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_CONFIG to value 0x015c"]
impl crate::Resettable for PaConfigSpec {
    const RESET_VALUE: u32 = 0x015c;
}
