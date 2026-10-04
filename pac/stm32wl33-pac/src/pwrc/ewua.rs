#[doc = "Register `EWUA` reader"]
pub type R = crate::R<EwuaSpec>;
#[doc = "Register `EWUA` writer"]
pub type W = crate::W<EwuaSpec>;
#[doc = "Field `EWUA` reader - EWUA\\[x\\] Enable WakeUp line PA\\[x\\] When this bit is set the PA\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PA\\[x\\] will trigger a CPU wakeup event depending on CR7.WUPA\\[x\\] bit."]
pub type EwuaR = crate::FieldReader<u16>;
#[doc = "Field `EWUA` writer - EWUA\\[x\\] Enable WakeUp line PA\\[x\\] When this bit is set the PA\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PA\\[x\\] will trigger a CPU wakeup event depending on CR7.WUPA\\[x\\] bit."]
pub type EwuaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - EWUA\\[x\\] Enable WakeUp line PA\\[x\\] When this bit is set the PA\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PA\\[x\\] will trigger a CPU wakeup event depending on CR7.WUPA\\[x\\] bit."]
    #[inline(always)]
    pub fn ewua(&self) -> EwuaR {
        EwuaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EWUA\\[x\\] Enable WakeUp line PA\\[x\\] When this bit is set the PA\\[x\\] wakeup line is enabled and a rising or falling edge on wakeup line PA\\[x\\] will trigger a CPU wakeup event depending on CR7.WUPA\\[x\\] bit."]
    #[inline(always)]
    pub fn ewua(&mut self) -> EwuaW<'_, EwuaSpec> {
        EwuaW::new(self, 0)
    }
}
#[doc = "EWUA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewua::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewua::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwuaSpec;
impl crate::RegisterSpec for EwuaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewua::R`](R) reader structure"]
impl crate::Readable for EwuaSpec {}
#[doc = "`write(|w| ..)` method takes [`ewua::W`](W) writer structure"]
impl crate::Writable for EwuaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EWUA to value 0"]
impl crate::Resettable for EwuaSpec {}
