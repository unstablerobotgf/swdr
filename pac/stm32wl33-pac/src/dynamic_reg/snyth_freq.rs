#[doc = "Register `SNYTH_FREQ` reader"]
pub type R = crate::R<SnythFreqSpec>;
#[doc = "Register `SNYTH_FREQ` writer"]
pub type W = crate::W<SnythFreqSpec>;
#[doc = "Field `SYNTH_FRAC` reader - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)"]
pub type SynthFracR = crate::FieldReader<u32>;
#[doc = "Field `SYNTH_FRAC` writer - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)"]
pub type SynthFracW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SYNTH_INT` reader - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)"]
pub type SynthIntR = crate::FieldReader;
#[doc = "Field `SYNTH_INT` writer - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)"]
pub type SynthIntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BS` reader - Synthesizer band selector, i."]
pub type BsR = crate::BitReader;
#[doc = "Field `BS` writer - Synthesizer band selector, i."]
pub type BsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)"]
    #[inline(always)]
    pub fn synth_frac(&self) -> SynthFracR {
        SynthFracR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:27 - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)"]
    #[inline(always)]
    pub fn synth_int(&self) -> SynthIntR {
        SynthIntR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Synthesizer band selector, i."]
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)"]
    #[inline(always)]
    pub fn synth_frac(&mut self) -> SynthFracW<'_, SnythFreqSpec> {
        SynthFracW::new(self, 0)
    }
    #[doc = "Bits 20:27 - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)"]
    #[inline(always)]
    pub fn synth_int(&mut self) -> SynthIntW<'_, SnythFreqSpec> {
        SynthIntW::new(self, 20)
    }
    #[doc = "Bit 30 - Synthesizer band selector, i."]
    #[inline(always)]
    pub fn bs(&mut self) -> BsW<'_, SnythFreqSpec> {
        BsW::new(self, 30)
    }
}
#[doc = "SNYTH_FREQ register\n\nYou can [`read`](crate::Reg::read) this register and get [`snyth_freq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snyth_freq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnythFreqSpec;
impl crate::RegisterSpec for SnythFreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snyth_freq::R`](R) reader structure"]
impl crate::Readable for SnythFreqSpec {}
#[doc = "`write(|w| ..)` method takes [`snyth_freq::W`](W) writer structure"]
impl crate::Writable for SnythFreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNYTH_FREQ to value 0x0485_1615"]
impl crate::Resettable for SnythFreqSpec {
    const RESET_VALUE: u32 = 0x0485_1615;
}
