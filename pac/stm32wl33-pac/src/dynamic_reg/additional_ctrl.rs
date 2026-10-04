#[doc = "Register `ADDITIONAL_CTRL` reader"]
pub type R = crate::R<AdditionalCtrlSpec>;
#[doc = "Register `ADDITIONAL_CTRL` writer"]
pub type W = crate::W<AdditionalCtrlSpec>;
#[doc = "Field `CH_NUM` reader - Channel number."]
pub type ChNumR = crate::FieldReader;
#[doc = "Field `CH_NUM` writer - Channel number."]
pub type ChNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH_SPACING` reader - Channel spacing."]
pub type ChSpacingR = crate::FieldReader;
#[doc = "Field `PA_FC` reader - Power control bandwidth selection according data rate"]
pub type PaFcR = crate::FieldReader;
#[doc = "Field `PA_FC` writer - Power control bandwidth selection according data rate"]
pub type PaFcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIME_CAPTURESEL` reader - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\\[31:0\\] register"]
pub type TimeCaptureselR = crate::FieldReader;
#[doc = "Field `TIME_CAPTURESEL` writer - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\\[31:0\\] register"]
pub type TimeCaptureselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AS_ENABLE` reader - Enable the antenna switching feature."]
pub type AsEnableR = crate::BitReader;
#[doc = "Field `AS_ENABLE` writer - Enable the antenna switching feature."]
pub type AsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Channel number."]
    #[inline(always)]
    pub fn ch_num(&self) -> ChNumR {
        ChNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Channel spacing."]
    #[inline(always)]
    pub fn ch_spacing(&self) -> ChSpacingR {
        ChSpacingR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Power control bandwidth selection according data rate"]
    #[inline(always)]
    pub fn pa_fc(&self) -> PaFcR {
        PaFcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\\[31:0\\] register"]
    #[inline(always)]
    pub fn time_capturesel(&self) -> TimeCaptureselR {
        TimeCaptureselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 31 - Enable the antenna switching feature."]
    #[inline(always)]
    pub fn as_enable(&self) -> AsEnableR {
        AsEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel number."]
    #[inline(always)]
    pub fn ch_num(&mut self) -> ChNumW<'_, AdditionalCtrlSpec> {
        ChNumW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Power control bandwidth selection according data rate"]
    #[inline(always)]
    pub fn pa_fc(&mut self) -> PaFcW<'_, AdditionalCtrlSpec> {
        PaFcW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\\[31:0\\] register"]
    #[inline(always)]
    pub fn time_capturesel(&mut self) -> TimeCaptureselW<'_, AdditionalCtrlSpec> {
        TimeCaptureselW::new(self, 20)
    }
    #[doc = "Bit 31 - Enable the antenna switching feature."]
    #[inline(always)]
    pub fn as_enable(&mut self) -> AsEnableW<'_, AdditionalCtrlSpec> {
        AsEnableW::new(self, 31)
    }
}
#[doc = "ADDITIONAL_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`additional_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`additional_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdditionalCtrlSpec;
impl crate::RegisterSpec for AdditionalCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`additional_ctrl::R`](R) reader structure"]
impl crate::Readable for AdditionalCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`additional_ctrl::W`](W) writer structure"]
impl crate::Writable for AdditionalCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDITIONAL_CTRL to value 0x0003_8800"]
impl crate::Resettable for AdditionalCtrlSpec {
    const RESET_VALUE: u32 = 0x0003_8800;
}
