#[doc = "Register `WUPB` reader"]
pub type R = crate::R<WupbSpec>;
#[doc = "Register `WUPB` writer"]
pub type W = crate::W<WupbSpec>;
#[doc = "Field `WUPB` reader - WUPB\\[x\\] Wake-up Line PB\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
pub type WupbR = crate::FieldReader<u16>;
#[doc = "Field `WUPB` writer - WUPB\\[x\\] Wake-up Line PB\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
pub type WupbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WUPB\\[x\\] Wake-up Line PB\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn wupb(&self) -> WupbR {
        WupbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WUPB\\[x\\] Wake-up Line PB\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn wupb(&mut self) -> WupbW<'_, WupbSpec> {
        WupbW::new(self, 0)
    }
}
#[doc = "WUPB register\n\nYou can [`read`](crate::Reg::read) this register and get [`wupb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WupbSpec;
impl crate::RegisterSpec for WupbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wupb::R`](R) reader structure"]
impl crate::Readable for WupbSpec {}
#[doc = "`write(|w| ..)` method takes [`wupb::W`](W) writer structure"]
impl crate::Writable for WupbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUPB to value 0"]
impl crate::Resettable for WupbSpec {}
