#[doc = "Register `AGC_CONFIG` reader"]
pub type R = crate::R<AgcConfigSpec>;
#[doc = "Register `AGC_CONFIG` writer"]
pub type W = crate::W<AgcConfigSpec>;
#[doc = "Field `AGC_MODE` reader - Define the working mode of the AGC:"]
pub type AgcModeR = crate::FieldReader;
#[doc = "Field `AGC_MODE` writer - Define the working mode of the AGC:"]
pub type AgcModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AGC_HOLD_MODE` reader - The behavior when the AGC is ON and is working in HOLD mode"]
pub type AgcHoldModeR = crate::BitReader;
#[doc = "Field `AGC_HOLD_MODE` writer - The behavior when the AGC is ON and is working in HOLD mode"]
pub type AgcHoldModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_RESET_MODE` reader - The AGC reset behavior when the AGC is working in ON or HOLD mode"]
pub type AgcResetModeR = crate::BitReader;
#[doc = "Field `AGC_RESET_MODE` writer - The AGC reset behavior when the AGC is working in ON or HOLD mode"]
pub type AgcResetModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Define the working mode of the AGC:"]
    #[inline(always)]
    pub fn agc_mode(&self) -> AgcModeR {
        AgcModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - The behavior when the AGC is ON and is working in HOLD mode"]
    #[inline(always)]
    pub fn agc_hold_mode(&self) -> AgcHoldModeR {
        AgcHoldModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The AGC reset behavior when the AGC is working in ON or HOLD mode"]
    #[inline(always)]
    pub fn agc_reset_mode(&self) -> AgcResetModeR {
        AgcResetModeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Define the working mode of the AGC:"]
    #[inline(always)]
    pub fn agc_mode(&mut self) -> AgcModeW<'_, AgcConfigSpec> {
        AgcModeW::new(self, 0)
    }
    #[doc = "Bit 2 - The behavior when the AGC is ON and is working in HOLD mode"]
    #[inline(always)]
    pub fn agc_hold_mode(&mut self) -> AgcHoldModeW<'_, AgcConfigSpec> {
        AgcHoldModeW::new(self, 2)
    }
    #[doc = "Bit 3 - The AGC reset behavior when the AGC is working in ON or HOLD mode"]
    #[inline(always)]
    pub fn agc_reset_mode(&mut self) -> AgcResetModeW<'_, AgcConfigSpec> {
        AgcResetModeW::new(self, 3)
    }
}
#[doc = "AGC_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcConfigSpec;
impl crate::RegisterSpec for AgcConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_config::R`](R) reader structure"]
impl crate::Readable for AgcConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`agc_config::W`](W) writer structure"]
impl crate::Writable for AgcConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_CONFIG to value 0"]
impl crate::Resettable for AgcConfigSpec {}
