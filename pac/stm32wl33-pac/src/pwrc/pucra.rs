#[doc = "Register `PUCRA` reader"]
pub type R = crate::R<PucraSpec>;
#[doc = "Register `PUCRA` writer"]
pub type W = crate::W<PucraSpec>;
#[doc = "Field `PUA` reader - PUA\\[x\\] : Pull Up Port A Pull up activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port A\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRA\\[x\\] is reset - 0: Pull-Up not activated on port A\\[i\\]"]
pub type PuaR = crate::FieldReader<u16>;
#[doc = "Field `PUA` writer - PUA\\[x\\] : Pull Up Port A Pull up activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port A\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRA\\[x\\] is reset - 0: Pull-Up not activated on port A\\[i\\]"]
pub type PuaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PUA\\[x\\] : Pull Up Port A Pull up activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port A\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRA\\[x\\] is reset - 0: Pull-Up not activated on port A\\[i\\]"]
    #[inline(always)]
    pub fn pua(&self) -> PuaR {
        PuaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PUA\\[x\\] : Pull Up Port A Pull up activation on port A\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port A\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRA\\[x\\] is reset - 0: Pull-Up not activated on port A\\[i\\]"]
    #[inline(always)]
    pub fn pua(&mut self) -> PuaW<'_, PucraSpec> {
        PuaW::new(self, 0)
    }
}
#[doc = "PUCRA register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PucraSpec;
impl crate::RegisterSpec for PucraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucra::R`](R) reader structure"]
impl crate::Readable for PucraSpec {}
#[doc = "`write(|w| ..)` method takes [`pucra::W`](W) writer structure"]
impl crate::Writable for PucraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUCRA to value 0xfff7"]
impl crate::Resettable for PucraSpec {
    const RESET_VALUE: u32 = 0xfff7;
}
