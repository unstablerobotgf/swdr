#[doc = "Register `SDWN_WUF` reader"]
pub type R = crate::R<SdwnWufSpec>;
#[doc = "Register `SDWN_WUF` writer"]
pub type W = crate::W<SdwnWufSpec>;
#[doc = "Field `WUF` reader - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred"]
pub type WufR = crate::BitReader;
#[doc = "Field `WUF` writer - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred"]
pub type WufW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WufW<'_, SdwnWufSpec> {
        WufW::new(self, 0)
    }
}
#[doc = "SDWN_WUF register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdwn_wuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdwnWufSpec;
impl crate::RegisterSpec for SdwnWufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdwn_wuf::R`](R) reader structure"]
impl crate::Readable for SdwnWufSpec {}
#[doc = "`write(|w| ..)` method takes [`sdwn_wuf::W`](W) writer structure"]
impl crate::Writable for SdwnWufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDWN_WUF to value 0"]
impl crate::Resettable for SdwnWufSpec {}
