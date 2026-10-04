#[doc = "Register `LCSC_CR0` reader"]
pub type R = crate::R<LcscCr0Spec>;
#[doc = "Register `LCSC_CR0` writer"]
pub type W = crate::W<LcscCr0Spec>;
#[doc = "Field `TMEAS` reader - Measurement Time"]
pub type TmeasR = crate::FieldReader<u16>;
#[doc = "Field `TMEAS` writer - Measurement Time"]
pub type TmeasW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TCAP` reader - Capture Time"]
pub type TcapR = crate::FieldReader;
#[doc = "Field `TCAP` writer - Capture Time"]
pub type TcapW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TICAP` reader - Inter Capture Time"]
pub type TicapR = crate::FieldReader;
#[doc = "Field `TICAP` writer - Inter Capture Time"]
pub type TicapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:13 - Measurement Time"]
    #[inline(always)]
    pub fn tmeas(&self) -> TmeasR {
        TmeasR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Capture Time"]
    #[inline(always)]
    pub fn tcap(&self) -> TcapR {
        TcapR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - Inter Capture Time"]
    #[inline(always)]
    pub fn ticap(&self) -> TicapR {
        TicapR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Measurement Time"]
    #[inline(always)]
    pub fn tmeas(&mut self) -> TmeasW<'_, LcscCr0Spec> {
        TmeasW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Capture Time"]
    #[inline(always)]
    pub fn tcap(&mut self) -> TcapW<'_, LcscCr0Spec> {
        TcapW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Inter Capture Time"]
    #[inline(always)]
    pub fn ticap(&mut self) -> TicapW<'_, LcscCr0Spec> {
        TicapW::new(self, 24)
    }
}
#[doc = "LCSC_CR0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscCr0Spec;
impl crate::RegisterSpec for LcscCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_cr0::R`](R) reader structure"]
impl crate::Readable for LcscCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_cr0::W`](W) writer structure"]
impl crate::Writable for LcscCr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_CR0 to value 0x000b_005c"]
impl crate::Resettable for LcscCr0Spec {
    const RESET_VALUE: u32 = 0x000b_005c;
}
