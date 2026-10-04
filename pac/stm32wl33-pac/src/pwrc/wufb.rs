#[doc = "Register `WUFB` reader"]
pub type R = crate::R<WufbSpec>;
#[doc = "Register `WUFB` writer"]
pub type W = crate::W<WufbSpec>;
#[doc = "Field `WUFB` reader - WUFB\\[x\\] WakeUp Flag PB\\[x\\] This bit is set when a wakeup is detected on wakeup line PB\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
pub type WufbR = crate::FieldReader<u16>;
#[doc = "Field `WUFB` writer - WUFB\\[x\\] WakeUp Flag PB\\[x\\] This bit is set when a wakeup is detected on wakeup line PB\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
pub type WufbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WUFB\\[x\\] WakeUp Flag PB\\[x\\] This bit is set when a wakeup is detected on wakeup line PB\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
    #[inline(always)]
    pub fn wufb(&self) -> WufbR {
        WufbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WUFB\\[x\\] WakeUp Flag PB\\[x\\] This bit is set when a wakeup is detected on wakeup line PB\\[x\\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:"]
    #[inline(always)]
    pub fn wufb(&mut self) -> WufbW<'_, WufbSpec> {
        WufbW::new(self, 0)
    }
}
#[doc = "WUFB register\n\nYou can [`read`](crate::Reg::read) this register and get [`wufb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WufbSpec;
impl crate::RegisterSpec for WufbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wufb::R`](R) reader structure"]
impl crate::Readable for WufbSpec {}
#[doc = "`write(|w| ..)` method takes [`wufb::W`](W) writer structure"]
impl crate::Writable for WufbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUFB to value 0"]
impl crate::Resettable for WufbSpec {}
