#[doc = "Register `WUFA` reader"]
pub type R = crate::R<WufaSpec>;
#[doc = "Register `WUFA` writer"]
pub type W = crate::W<WufaSpec>;
#[doc = "Field `WUFA` reader - WUFA\\[x\\] WakeUp Flag PA\\[x\\] This bit is set when a wakeup is detected on wakeup line PA\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
pub type WufaR = crate::FieldReader<u16>;
#[doc = "Field `WUFA` writer - WUFA\\[x\\] WakeUp Flag PA\\[x\\] This bit is set when a wakeup is detected on wakeup line PA\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
pub type WufaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WUFA\\[x\\] WakeUp Flag PA\\[x\\] This bit is set when a wakeup is detected on wakeup line PA\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
    #[inline(always)]
    pub fn wufa(&self) -> WufaR {
        WufaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WUFA\\[x\\] WakeUp Flag PA\\[x\\] This bit is set when a wakeup is detected on wakeup line PA\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
    #[inline(always)]
    pub fn wufa(&mut self) -> WufaW<'_, WufaSpec> {
        WufaW::new(self, 0)
    }
}
#[doc = "WUFA register\n\nYou can [`read`](crate::Reg::read) this register and get [`wufa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WufaSpec;
impl crate::RegisterSpec for WufaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wufa::R`](R) reader structure"]
impl crate::Readable for WufaSpec {}
#[doc = "`write(|w| ..)` method takes [`wufa::W`](W) writer structure"]
impl crate::Writable for WufaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUFA to value 0"]
impl crate::Resettable for WufaSpec {}
