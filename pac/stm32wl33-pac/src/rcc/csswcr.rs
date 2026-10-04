#[doc = "Register `CSSWCR` reader"]
pub type R = crate::R<CsswcrSpec>;
#[doc = "Register `CSSWCR` writer"]
pub type W = crate::W<CsswcrSpec>;
#[doc = "Field `LSISWTRIMEN` reader - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled"]
pub type LsiswtrimenR = crate::BitReader;
#[doc = "Field `LSISWTRIMEN` writer - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled"]
pub type LsiswtrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSISWBW` reader - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn"]
pub type LsiswbwR = crate::FieldReader;
#[doc = "Field `LSISWBW` writer - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn"]
pub type LsiswbwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LSEDRV` reader - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability"]
pub type LsedrvR = crate::FieldReader;
#[doc = "Field `LSEDRV` writer - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability"]
pub type LsedrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSISWTRIMEN` reader - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled"]
pub type HsiswtrimenR = crate::BitReader;
#[doc = "Field `HSISWTRIMEN` writer - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled"]
pub type HsiswtrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSITRIMSW` reader - High Speed Internal clock trimming value to set by SW."]
pub type HsitrimswR = crate::FieldReader;
#[doc = "Field `HSITRIMSW` writer - High Speed Internal clock trimming value to set by SW."]
pub type HsitrimswW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled"]
    #[inline(always)]
    pub fn lsiswtrimen(&self) -> LsiswtrimenR {
        LsiswtrimenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn"]
    #[inline(always)]
    pub fn lsiswbw(&self) -> LsiswbwR {
        LsiswbwR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LsedrvR {
        LsedrvR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 23 - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled"]
    #[inline(always)]
    pub fn hsiswtrimen(&self) -> HsiswtrimenR {
        HsiswtrimenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - High Speed Internal clock trimming value to set by SW."]
    #[inline(always)]
    pub fn hsitrimsw(&self) -> HsitrimswR {
        HsitrimswR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled"]
    #[inline(always)]
    pub fn lsiswtrimen(&mut self) -> LsiswtrimenW<'_, CsswcrSpec> {
        LsiswtrimenW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn"]
    #[inline(always)]
    pub fn lsiswbw(&mut self) -> LsiswbwW<'_, CsswcrSpec> {
        LsiswbwW::new(self, 1)
    }
    #[doc = "Bits 5:6 - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LsedrvW<'_, CsswcrSpec> {
        LsedrvW::new(self, 5)
    }
    #[doc = "Bit 23 - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled"]
    #[inline(always)]
    pub fn hsiswtrimen(&mut self) -> HsiswtrimenW<'_, CsswcrSpec> {
        HsiswtrimenW::new(self, 23)
    }
    #[doc = "Bits 24:29 - High Speed Internal clock trimming value to set by SW."]
    #[inline(always)]
    pub fn hsitrimsw(&mut self) -> HsitrimswW<'_, CsswcrSpec> {
        HsitrimswW::new(self, 24)
    }
}
#[doc = "CSSWCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsswcrSpec;
impl crate::RegisterSpec for CsswcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csswcr::R`](R) reader structure"]
impl crate::Readable for CsswcrSpec {}
#[doc = "`write(|w| ..)` method takes [`csswcr::W`](W) writer structure"]
impl crate::Writable for CsswcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSSWCR to value 0"]
impl crate::Resettable for CsswcrSpec {}
