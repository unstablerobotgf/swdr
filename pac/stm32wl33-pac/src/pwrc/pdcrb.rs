#[doc = "Register `PDCRB` reader"]
pub type R = crate::R<PdcrbSpec>;
#[doc = "Register `PDCRB` writer"]
pub type W = crate::W<PdcrbSpec>;
#[doc = "Field `PDB` reader - PDB\\[x\\]: Pull Down Port B Pull Down activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port B\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port B\\[i\\]"]
pub type PdbR = crate::FieldReader<u16>;
#[doc = "Field `PDB` writer - PDB\\[x\\]: Pull Down Port B Pull Down activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port B\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port B\\[i\\]"]
pub type PdbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PDB\\[x\\]: Pull Down Port B Pull Down activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port B\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port B\\[i\\]"]
    #[inline(always)]
    pub fn pdb(&self) -> PdbR {
        PdbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB\\[x\\]: Pull Down Port B Pull Down activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Down activated on Port B\\[i\\] when APC bit of PWRC CR1 bit is set - 0: Pull-Down not activated on Port B\\[i\\]"]
    #[inline(always)]
    pub fn pdb(&mut self) -> PdbW<'_, PdcrbSpec> {
        PdbW::new(self, 0)
    }
}
#[doc = "PDCRB register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcrbSpec;
impl crate::RegisterSpec for PdcrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrb::R`](R) reader structure"]
impl crate::Readable for PdcrbSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcrb::W`](W) writer structure"]
impl crate::Writable for PdcrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDCRB to value 0"]
impl crate::Resettable for PdcrbSpec {}
