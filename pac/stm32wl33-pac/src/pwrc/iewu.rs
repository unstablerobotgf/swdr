#[doc = "Register `IEWU` reader"]
pub type R = crate::R<IewuSpec>;
#[doc = "Register `IEWU` writer"]
pub type W = crate::W<IewuSpec>;
#[doc = "Field `EIWL0` reader - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl0R = crate::BitReader;
#[doc = "Field `EIWL0` writer - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWL1` reader - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl1R = crate::BitReader;
#[doc = "Field `EIWL1` writer - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWL2` reader - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl2R = crate::BitReader;
#[doc = "Field `EIWL2` writer - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWL3` reader - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl3R = crate::BitReader;
#[doc = "Field `EIWL3` writer - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWL4` reader - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl4R = crate::BitReader;
#[doc = "Field `EIWL4` writer - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
pub type Eiwl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWMRSUBG` reader - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled."]
pub type EwmrsubgR = crate::BitReader;
#[doc = "Field `EWMRSUBG` writer - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled."]
pub type EwmrsubgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWMRSUBGHCPU` reader - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled."]
pub type EwmrsubghcpuR = crate::BitReader;
#[doc = "Field `EWMRSUBGHCPU` writer - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled."]
pub type EwmrsubghcpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWLPAWUR` reader - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled."]
pub type EwlpawurR = crate::BitReader;
#[doc = "Field `EWLPAWUR` writer - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled."]
pub type EwlpawurW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl0(&self) -> Eiwl0R {
        Eiwl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl1(&self) -> Eiwl1R {
        Eiwl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl2(&self) -> Eiwl2R {
        Eiwl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl3(&self) -> Eiwl3R {
        Eiwl3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl4(&self) -> Eiwl4R {
        Eiwl4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled."]
    #[inline(always)]
    pub fn ewmrsubg(&self) -> EwmrsubgR {
        EwmrsubgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled."]
    #[inline(always)]
    pub fn ewmrsubghcpu(&self) -> EwmrsubghcpuR {
        EwmrsubghcpuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled."]
    #[inline(always)]
    pub fn ewlpawur(&self) -> EwlpawurR {
        EwlpawurR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWL0 Enable Internal WakeUp line LPUART When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl0(&mut self) -> Eiwl0W<'_, IewuSpec> {
        Eiwl0W::new(self, 0)
    }
    #[doc = "Bit 1 - EIWL1 Enable Internal WakeUp line RTC When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl1(&mut self) -> Eiwl1W<'_, IewuSpec> {
        Eiwl1W::new(self, 1)
    }
    #[doc = "Bit 2 - EIWL2 Enable Internal WakeUp line LCD When this bit is set the internal wakeup line is enabled and a rising edge will trigger a CPU wakeup event. - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl2(&mut self) -> Eiwl2W<'_, IewuSpec> {
        Eiwl2W::new(self, 2)
    }
    #[doc = "Bit 3 - EIWL3 Enable Internal Wakeup line COMP When this bit is set the COMP wakeup is enabled and an edge will trigger a COMP wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl3(&mut self) -> Eiwl3W<'_, IewuSpec> {
        Eiwl3W::new(self, 3)
    }
    #[doc = "Bit 4 - EIWL4 Enable Internal Wakeup line LCSC When this bit is set the LCSC wakeup is enabled and an edge will trigger a LCSC wakeup event - 0: wakeup disabled. - 1: wakeup enabled."]
    #[inline(always)]
    pub fn eiwl4(&mut self) -> Eiwl4W<'_, IewuSpec> {
        Eiwl4W::new(self, 4)
    }
    #[doc = "Bit 8 - EWMRSUB Wakeup MRSUBG Enable When this bit is set the MRSUBG wakeup is enabled and a rising edge will trigger a MRSUBG wakeup event - 0: MRSUBG wakeup disabled. - 1: MRSUBG wakeup enabled."]
    #[inline(always)]
    pub fn ewmrsubg(&mut self) -> EwmrsubgW<'_, IewuSpec> {
        EwmrsubgW::new(self, 8)
    }
    #[doc = "Bit 9 - EWMRSUBGHCPU Wakeup MRSUBG Host CPU Enable When this bit is set the MRSUBG HOST CPU wakeup is enabled and a rising edge will trigger a MRSUBG Host CPU wakeup event - 0: MRSUBG Host CPU wakeup disabled. - 1: MRSUBG Host CPU wakeup enabled."]
    #[inline(always)]
    pub fn ewmrsubghcpu(&mut self) -> EwmrsubghcpuW<'_, IewuSpec> {
        EwmrsubghcpuW::new(self, 9)
    }
    #[doc = "Bit 10 - EWLPAWUR: Wakeup Bubble Enable When this bit is set the Bubble wakeup is enabled and a rising edge will trigger a LPAWUR wakeup event - 0: LPAWUR wakeup disabled. - 1: LPAWUR wakeup enabled."]
    #[inline(always)]
    pub fn ewlpawur(&mut self) -> EwlpawurW<'_, IewuSpec> {
        EwlpawurW::new(self, 10)
    }
}
#[doc = "IEWU register\n\nYou can [`read`](crate::Reg::read) this register and get [`iewu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iewu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IewuSpec;
impl crate::RegisterSpec for IewuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iewu::R`](R) reader structure"]
impl crate::Readable for IewuSpec {}
#[doc = "`write(|w| ..)` method takes [`iewu::W`](W) writer structure"]
impl crate::Writable for IewuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEWU to value 0"]
impl crate::Resettable for IewuSpec {}
