#[doc = "Register `PUCRB` reader"]
pub type R = crate::R<PucrbSpec>;
#[doc = "Register `PUCRB` writer"]
pub type W = crate::W<PucrbSpec>;
#[doc = "Field `PUB` reader - PUB\\[x\\] : Pull Up Port B Pull up activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port B\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRB\\[x\\] is reset - 0: Pull-Up not activated on port B\\[i\\]"]
pub type PubR = crate::FieldReader<u16>;
#[doc = "Field `PUB` writer - PUB\\[x\\] : Pull Up Port B Pull up activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port B\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRB\\[x\\] is reset - 0: Pull-Up not activated on port B\\[i\\]"]
pub type PubW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PUB\\[x\\] : Pull Up Port B Pull up activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port B\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRB\\[x\\] is reset - 0: Pull-Up not activated on port B\\[i\\]"]
    #[inline(always)]
    pub fn pub_(&self) -> PubR {
        PubR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PUB\\[x\\] : Pull Up Port B Pull up activation on port B\\[i\\] pad when APC bit of PWRC CR1 is set - 1: Pull-Up activated on port B\\[i\\] when APC bit of PWRC CR1 bit is set and PWR_PDCRB\\[x\\] is reset - 0: Pull-Up not activated on port B\\[i\\]"]
    #[inline(always)]
    pub fn pub_(&mut self) -> PubW<'_, PucrbSpec> {
        PubW::new(self, 0)
    }
}
#[doc = "PUCRB register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PucrbSpec;
impl crate::RegisterSpec for PucrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrb::R`](R) reader structure"]
impl crate::Readable for PucrbSpec {}
#[doc = "`write(|w| ..)` method takes [`pucrb::W`](W) writer structure"]
impl crate::Writable for PucrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUCRB to value 0xffff"]
impl crate::Resettable for PucrbSpec {
    const RESET_VALUE: u32 = 0xffff;
}
