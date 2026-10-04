#[doc = "Register `SDWN_WUEN` reader"]
pub type R = crate::R<SdwnWuenSpec>;
#[doc = "Register `SDWN_WUEN` writer"]
pub type W = crate::W<SdwnWuenSpec>;
#[doc = "Field `WUEN` reader - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled"]
pub type WuenR = crate::BitReader;
#[doc = "Field `WUEN` writer - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled"]
pub type WuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled"]
    #[inline(always)]
    pub fn wuen(&self) -> WuenR {
        WuenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled"]
    #[inline(always)]
    pub fn wuen(&mut self) -> WuenW<'_, SdwnWuenSpec> {
        WuenW::new(self, 0)
    }
}
#[doc = "SDWN_WUEN register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdwn_wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdwnWuenSpec;
impl crate::RegisterSpec for SdwnWuenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdwn_wuen::R`](R) reader structure"]
impl crate::Readable for SdwnWuenSpec {}
#[doc = "`write(|w| ..)` method takes [`sdwn_wuen::W`](W) writer structure"]
impl crate::Writable for SdwnWuenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDWN_WUEN to value 0"]
impl crate::Resettable for SdwnWuenSpec {}
