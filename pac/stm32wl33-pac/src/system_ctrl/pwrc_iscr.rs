#[doc = "Register `PWRC_ISCR` reader"]
pub type R = crate::R<PwrcIscrSpec>;
#[doc = "Register `PWRC_ISCR` writer"]
pub type W = crate::W<PwrcIscrSpec>;
#[doc = "Field `BORH_ISC` reader - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
pub type BorhIscR = crate::BitReader;
#[doc = "Field `BORH_ISC` writer - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
pub type BorhIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD_ISC` reader - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
pub type PvdIscR = crate::BitReader;
#[doc = "Field `PVD_ISC` writer - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
pub type PvdIscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP_ISC` reader - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system."]
pub type WkupIscR = crate::BitReader;
#[doc = "Field `WKUP_ISC` writer - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system."]
pub type WkupIscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
    #[inline(always)]
    pub fn borh_isc(&self) -> BorhIscR {
        BorhIscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
    #[inline(always)]
    pub fn pvd_isc(&self) -> PvdIscR {
        PvdIscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system."]
    #[inline(always)]
    pub fn wkup_isc(&self) -> WkupIscR {
        WkupIscR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
    #[inline(always)]
    pub fn borh_isc(&mut self) -> BorhIscW<'_, PwrcIscrSpec> {
        BorhIscW::new(self, 0)
    }
    #[doc = "Bit 1 - PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
    #[inline(always)]
    pub fn pvd_isc(&mut self) -> PvdIscW<'_, PwrcIscrSpec> {
        PvdIscW::new(self, 1)
    }
    #[doc = "Bit 2 - WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system."]
    #[inline(always)]
    pub fn wkup_isc(&mut self) -> WkupIscW<'_, PwrcIscrSpec> {
        WkupIscW::new(self, 2)
    }
}
#[doc = "PWRC_ISCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc_iscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_iscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcIscrSpec;
impl crate::RegisterSpec for PwrcIscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrc_iscr::R`](R) reader structure"]
impl crate::Readable for PwrcIscrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrc_iscr::W`](W) writer structure"]
impl crate::Writable for PwrcIscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC_ISCR to value 0"]
impl crate::Resettable for PwrcIscrSpec {}
