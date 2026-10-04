#[doc = "Register `MOD1_CONFIG` reader"]
pub type R = crate::R<Mod1ConfigSpec>;
#[doc = "Register `MOD1_CONFIG` writer"]
pub type W = crate::W<Mod1ConfigSpec>;
#[doc = "Field `FDEV_M` reader - Mantissa of the frequency deviation (default: 28."]
pub type FdevMR = crate::FieldReader;
#[doc = "Field `FDEV_M` writer - Mantissa of the frequency deviation (default: 28."]
pub type FdevMW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FDEV_E` reader - Exponent of the frequency deviation (default: 28."]
pub type FdevER = crate::FieldReader;
#[doc = "Field `FDEV_E` writer - Exponent of the frequency deviation (default: 28."]
pub type FdevEW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHFLT_M` reader - Mantissa of the channel filter BW (default: 100 kHz)"]
pub type ChfltMR = crate::FieldReader;
#[doc = "Field `CHFLT_M` writer - Mantissa of the channel filter BW (default: 100 kHz)"]
pub type ChfltMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHFLT_E` reader - Exponent of the channel filter BW (default: 100 kHz)"]
pub type ChfltER = crate::FieldReader;
#[doc = "Field `CHFLT_E` writer - Exponent of the channel filter BW (default: 100 kHz)"]
pub type ChfltEW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Mantissa of the frequency deviation (default: 28."]
    #[inline(always)]
    pub fn fdev_m(&self) -> FdevMR {
        FdevMR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Exponent of the frequency deviation (default: 28."]
    #[inline(always)]
    pub fn fdev_e(&self) -> FdevER {
        FdevER::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Mantissa of the channel filter BW (default: 100 kHz)"]
    #[inline(always)]
    pub fn chflt_m(&self) -> ChfltMR {
        ChfltMR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Exponent of the channel filter BW (default: 100 kHz)"]
    #[inline(always)]
    pub fn chflt_e(&self) -> ChfltER {
        ChfltER::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mantissa of the frequency deviation (default: 28."]
    #[inline(always)]
    pub fn fdev_m(&mut self) -> FdevMW<'_, Mod1ConfigSpec> {
        FdevMW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Exponent of the frequency deviation (default: 28."]
    #[inline(always)]
    pub fn fdev_e(&mut self) -> FdevEW<'_, Mod1ConfigSpec> {
        FdevEW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Mantissa of the channel filter BW (default: 100 kHz)"]
    #[inline(always)]
    pub fn chflt_m(&mut self) -> ChfltMW<'_, Mod1ConfigSpec> {
        ChfltMW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Exponent of the channel filter BW (default: 100 kHz)"]
    #[inline(always)]
    pub fn chflt_e(&mut self) -> ChfltEW<'_, Mod1ConfigSpec> {
        ChfltEW::new(self, 20)
    }
}
#[doc = "MOD1_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mod1ConfigSpec;
impl crate::RegisterSpec for Mod1ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod1_config::R`](R) reader structure"]
impl crate::Readable for Mod1ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mod1_config::W`](W) writer structure"]
impl crate::Writable for Mod1ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOD1_CONFIG to value 0x0040_0435"]
impl crate::Resettable for Mod1ConfigSpec {
    const RESET_VALUE: u32 = 0x0040_0435;
}
