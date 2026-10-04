#[doc = "Register `PDCRA` reader"]
pub type R = crate::R<PdcraSpec>;
#[doc = "Register `PDCRA` writer"]
pub type W = crate::W<PdcraSpec>;
#[doc = "Field `PDA` reader - PDA\\[x\\]: Pull Down Port A Pull Down activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port A\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port A\\[i\\]"]
pub type PdaR = crate::FieldReader<u16>;
#[doc = "Field `PDA` writer - PDA\\[x\\]: Pull Down Port A Pull Down activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port A\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port A\\[i\\]"]
pub type PdaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PDA\\[x\\]: Pull Down Port A Pull Down activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port A\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port A\\[i\\]"]
    #[inline(always)]
    pub fn pda(&self) -> PdaR {
        PdaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDA\\[x\\]: Pull Down Port A Pull Down activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port A\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port A\\[i\\]"]
    #[inline(always)]
    pub fn pda(&mut self) -> PdaW<'_, PdcraSpec> {
        PdaW::new(self, 0)
    }
}
#[doc = "PDCRA register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcraSpec;
impl crate::RegisterSpec for PdcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcra::R`](R) reader structure"]
impl crate::Readable for PdcraSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcra::W`](W) writer structure"]
impl crate::Writable for PdcraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDCRA to value 0x08"]
impl crate::Resettable for PdcraSpec {
    const RESET_VALUE: u32 = 0x08;
}
