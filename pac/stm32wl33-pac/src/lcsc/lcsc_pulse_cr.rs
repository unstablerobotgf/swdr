#[doc = "Register `LCSC_PULSE_CR` reader"]
pub type R = crate::R<LcscPulseCrSpec>;
#[doc = "Register `LCSC_PULSE_CR` writer"]
pub type W = crate::W<LcscPulseCrSpec>;
#[doc = "Field `LCAB_PULSE_WIDTH` reader - Low Pulse Width for LCA and LCB"]
pub type LcabPulseWidthR = crate::FieldReader;
#[doc = "Field `LCAB_PULSE_WIDTH` writer - Low Pulse Width for LCA and LCB"]
pub type LcabPulseWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LCT_PULSE_WIDTH` reader - Low Pulse Width for LCT"]
pub type LctPulseWidthR = crate::FieldReader;
#[doc = "Field `LCT_PULSE_WIDTH` writer - Low Pulse Width for LCT"]
pub type LctPulseWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Low Pulse Width for LCA and LCB"]
    #[inline(always)]
    pub fn lcab_pulse_width(&self) -> LcabPulseWidthR {
        LcabPulseWidthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Low Pulse Width for LCT"]
    #[inline(always)]
    pub fn lct_pulse_width(&self) -> LctPulseWidthR {
        LctPulseWidthR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Low Pulse Width for LCA and LCB"]
    #[inline(always)]
    pub fn lcab_pulse_width(&mut self) -> LcabPulseWidthW<'_, LcscPulseCrSpec> {
        LcabPulseWidthW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Low Pulse Width for LCT"]
    #[inline(always)]
    pub fn lct_pulse_width(&mut self) -> LctPulseWidthW<'_, LcscPulseCrSpec> {
        LctPulseWidthW::new(self, 8)
    }
}
#[doc = "LCSC_PULSE_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_pulse_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_pulse_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscPulseCrSpec;
impl crate::RegisterSpec for LcscPulseCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_pulse_cr::R`](R) reader structure"]
impl crate::Readable for LcscPulseCrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_pulse_cr::W`](W) writer structure"]
impl crate::Writable for LcscPulseCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_PULSE_CR to value 0x70"]
impl crate::Resettable for LcscPulseCrSpec {
    const RESET_VALUE: u32 = 0x70;
}
