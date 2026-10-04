#[doc = "Register `EWUB` reader"]
pub type R = crate::R<EwubSpec>;
#[doc = "Register `EWUB` writer"]
pub type W = crate::W<EwubSpec>;
#[doc = "Field `EWUB` reader - EWUB\\[x\\] Enable WakeUp line PB\\[x\\] When this bit is set the PB\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PB\\[x\\] will trigger a CPU wakeup event depending on CR9.WUPB\\[x\\] bit."]
pub type EwubR = crate::FieldReader<u16>;
#[doc = "Field `EWUB` writer - EWUB\\[x\\] Enable WakeUp line PB\\[x\\] When this bit is set the PB\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PB\\[x\\] will trigger a CPU wakeup event depending on CR9.WUPB\\[x\\] bit."]
pub type EwubW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - EWUB\\[x\\] Enable WakeUp line PB\\[x\\] When this bit is set the PB\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PB\\[x\\] will trigger a CPU wakeup event depending on CR9.WUPB\\[x\\] bit."]
    #[inline(always)]
    pub fn ewub(&self) -> EwubR {
        EwubR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EWUB\\[x\\] Enable WakeUp line PB\\[x\\] When this bit is set the PB\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PB\\[x\\] will trigger a CPU wakeup event depending on CR9.WUPB\\[x\\] bit."]
    #[inline(always)]
    pub fn ewub(&mut self) -> EwubW<'_, EwubSpec> {
        EwubW::new(self, 0)
    }
}
#[doc = "EWUB register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwubSpec;
impl crate::RegisterSpec for EwubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewub::R`](R) reader structure"]
impl crate::Readable for EwubSpec {}
#[doc = "`write(|w| ..)` method takes [`ewub::W`](W) writer structure"]
impl crate::Writable for EwubSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EWUB to value 0"]
impl crate::Resettable for EwubSpec {}
