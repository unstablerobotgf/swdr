#[doc = "Register `WUPA` reader"]
pub type R = crate::R<WupaSpec>;
#[doc = "Register `WUPA` writer"]
pub type W = crate::W<WupaSpec>;
#[doc = "Field `WUPA` reader - WUPA\\[x\\] Wake-up Line PA\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
pub type WupaR = crate::FieldReader<u16>;
#[doc = "Field `WUPA` writer - WUPA\\[x\\] Wake-up Line PA\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
pub type WupaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WUPA\\[x\\] Wake-up Line PA\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn wupa(&self) -> WupaR {
        WupaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WUPA\\[x\\] Wake-up Line PA\\[x\\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\\[x\\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn wupa(&mut self) -> WupaW<'_, WupaSpec> {
        WupaW::new(self, 0)
    }
}
#[doc = "WUPA register\n\nYou can [`read`](crate::Reg::read) this register and get [`wupa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WupaSpec;
impl crate::RegisterSpec for WupaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wupa::R`](R) reader structure"]
impl crate::Readable for WupaSpec {}
#[doc = "`write(|w| ..)` method takes [`wupa::W`](W) writer structure"]
impl crate::Writable for WupaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUPA to value 0"]
impl crate::Resettable for WupaSpec {}
