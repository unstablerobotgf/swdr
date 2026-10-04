#[doc = "Register `DBGR` reader"]
pub type R = crate::R<DbgrSpec>;
#[doc = "Register `DBGR` writer"]
pub type W = crate::W<DbgrSpec>;
#[doc = "Field `DEEPSTOP2` reader - DEEPSTOP2 low power saving mode emulation enable this bit enable an emulated debug DEEPSTOP low power mode. If emulation is enabled, entering in DEEPSTOP mode, the v12i power domain still enters power saving mode, but its clock and power are maintained."]
pub type Deepstop2R = crate::BitReader;
#[doc = "Field `DEEPSTOP2` writer - DEEPSTOP2 low power saving mode emulation enable this bit enable an emulated debug DEEPSTOP low power mode. If emulation is enabled, entering in DEEPSTOP mode, the v12i power domain still enters power saving mode, but its clock and power are maintained."]
pub type Deepstop2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSFRDY` reader - SMPSFB Force ready check When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
pub type SmpsfrdyR = crate::BitReader;
#[doc = "Field `SMPSFRDY` writer - SMPSFB Force ready check When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
pub type SmpsfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KELVIN_TEST` reader - KELVIN_TEST\\[2:0\\]: Enable TEST mode Kelvin for LDO_RF (Write protected by IFR3 key) - 000: 0mA (open) (default 0x0) - 001 for 1mA - 010 for 3mA - 011 for 5mA - 100 for 8mA - 101 for 10mA else: 0mA (open) for other combinations."]
pub type KelvinTestR = crate::FieldReader;
#[doc = "Field `KELVIN_TEST` writer - KELVIN_TEST\\[2:0\\]: Enable TEST mode Kelvin for LDO_RF (Write protected by IFR3 key) - 000: 0mA (open) (default 0x0) - 001 for 1mA - 010 for 3mA - 011 for 5mA - 100 for 8mA - 101 for 10mA else: 0mA (open) for other combinations."]
pub type KelvinTestW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_PRECH` reader - DIS_PRECH\\[2:0\\]: disable precharge during deepstop (debug) allowed combination are: - 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) - 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)"]
pub type DisPrechR = crate::FieldReader;
#[doc = "Field `DIS_PRECH` writer - DIS_PRECH\\[2:0\\]: disable precharge during deepstop (debug) allowed combination are: - 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) - 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)"]
pub type DisPrechW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - DEEPSTOP2 low power saving mode emulation enable this bit enable an emulated debug DEEPSTOP low power mode. If emulation is enabled, entering in DEEPSTOP mode, the v12i power domain still enters power saving mode, but its clock and power are maintained."]
    #[inline(always)]
    pub fn deepstop2(&self) -> Deepstop2R {
        Deepstop2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - SMPSFB Force ready check When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
    #[inline(always)]
    pub fn smpsfrdy(&self) -> SmpsfrdyR {
        SmpsfrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - KELVIN_TEST\\[2:0\\]: Enable TEST mode Kelvin for LDO_RF (Write protected by IFR3 key) - 000: 0mA (open) (default 0x0) - 001 for 1mA - 010 for 3mA - 011 for 5mA - 100 for 8mA - 101 for 10mA else: 0mA (open) for other combinations."]
    #[inline(always)]
    pub fn kelvin_test(&self) -> KelvinTestR {
        KelvinTestR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:15 - DIS_PRECH\\[2:0\\]: disable precharge during deepstop (debug) allowed combination are: - 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) - 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)"]
    #[inline(always)]
    pub fn dis_prech(&self) -> DisPrechR {
        DisPrechR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DEEPSTOP2 low power saving mode emulation enable this bit enable an emulated debug DEEPSTOP low power mode. If emulation is enabled, entering in DEEPSTOP mode, the v12i power domain still enters power saving mode, but its clock and power are maintained."]
    #[inline(always)]
    pub fn deepstop2(&mut self) -> Deepstop2W<'_, DbgrSpec> {
        Deepstop2W::new(self, 0)
    }
    #[doc = "Bit 7 - SMPSFB Force ready check When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
    #[inline(always)]
    pub fn smpsfrdy(&mut self) -> SmpsfrdyW<'_, DbgrSpec> {
        SmpsfrdyW::new(self, 7)
    }
    #[doc = "Bits 8:10 - KELVIN_TEST\\[2:0\\]: Enable TEST mode Kelvin for LDO_RF (Write protected by IFR3 key) - 000: 0mA (open) (default 0x0) - 001 for 1mA - 010 for 3mA - 011 for 5mA - 100 for 8mA - 101 for 10mA else: 0mA (open) for other combinations."]
    #[inline(always)]
    pub fn kelvin_test(&mut self) -> KelvinTestW<'_, DbgrSpec> {
        KelvinTestW::new(self, 8)
    }
    #[doc = "Bits 13:15 - DIS_PRECH\\[2:0\\]: disable precharge during deepstop (debug) allowed combination are: - 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) - 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)"]
    #[inline(always)]
    pub fn dis_prech(&mut self) -> DisPrechW<'_, DbgrSpec> {
        DisPrechW::new(self, 13)
    }
}
#[doc = "DBGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgrSpec;
impl crate::RegisterSpec for DbgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgr::R`](R) reader structure"]
impl crate::Readable for DbgrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgr::W`](W) writer structure"]
impl crate::Writable for DbgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGR to value 0"]
impl crate::Resettable for DbgrSpec {}
