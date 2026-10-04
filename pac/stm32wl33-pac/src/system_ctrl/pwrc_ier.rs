#[doc = "Register `PWRC_IER` reader"]
pub type R = crate::R<PwrcIerSpec>;
#[doc = "Register `PWRC_IER` writer"]
pub type W = crate::W<PwrcIerSpec>;
#[doc = "Field `BORH_IE` reader - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled."]
pub type BorhIeR = crate::BitReader;
#[doc = "Field `BORH_IE` writer - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled."]
pub type BorhIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD_IE` reader - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled."]
pub type PvdIeR = crate::BitReader;
#[doc = "Field `PVD_IE` writer - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled."]
pub type PvdIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP_IE` reader - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled."]
pub type WkupIeR = crate::BitReader;
#[doc = "Field `WKUP_IE` writer - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled."]
pub type WkupIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled."]
    #[inline(always)]
    pub fn borh_ie(&self) -> BorhIeR {
        BorhIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled."]
    #[inline(always)]
    pub fn pvd_ie(&self) -> PvdIeR {
        PvdIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled."]
    #[inline(always)]
    pub fn wkup_ie(&self) -> WkupIeR {
        WkupIeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled."]
    #[inline(always)]
    pub fn borh_ie(&mut self) -> BorhIeW<'_, PwrcIerSpec> {
        BorhIeW::new(self, 0)
    }
    #[doc = "Bit 1 - PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled."]
    #[inline(always)]
    pub fn pvd_ie(&mut self) -> PvdIeW<'_, PwrcIerSpec> {
        PvdIeW::new(self, 1)
    }
    #[doc = "Bit 2 - WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled."]
    #[inline(always)]
    pub fn wkup_ie(&mut self) -> WkupIeW<'_, PwrcIerSpec> {
        WkupIeW::new(self, 2)
    }
}
#[doc = "PWRC_IER register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcIerSpec;
impl crate::RegisterSpec for PwrcIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrc_ier::R`](R) reader structure"]
impl crate::Readable for PwrcIerSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrc_ier::W`](W) writer structure"]
impl crate::Writable for PwrcIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC_IER to value 0"]
impl crate::Resettable for PwrcIerSpec {}
