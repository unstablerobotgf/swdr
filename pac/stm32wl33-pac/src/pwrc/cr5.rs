#[doc = "Register `CR5` reader"]
pub type R = crate::R<Cr5Spec>;
#[doc = "Register `CR5` writer"]
pub type W = crate::W<Cr5Spec>;
#[doc = "Field `SMPSLVL` reader - SMPSLVL\\[3:0\\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)"]
pub type SmpslvlR = crate::FieldReader;
#[doc = "Field `SMPSLVL` writer - SMPSLVL\\[3:0\\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)"]
pub type SmpslvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMPSBOMSEL` reader - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a"]
pub type SmpsbomselR = crate::FieldReader;
#[doc = "Field `SMPSBOMSEL` writer - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a"]
pub type SmpsbomselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPS_BOF_STATIC` reader - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)"]
pub type SmpsBofStaticR = crate::BitReader;
#[doc = "Field `SMPS_BOF_STATIC` writer - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)"]
pub type SmpsBofStaticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSMPS_BOF` reader - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;"]
pub type NosmpsBofR = crate::BitReader;
#[doc = "Field `NOSMPS_BOF` writer - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;"]
pub type NosmpsBofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSLPOPEN` reader - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating"]
pub type SmpslpopenR = crate::BitReader;
#[doc = "Field `SMPSLPOPEN` writer - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating"]
pub type SmpslpopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSFBYP` reader - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
pub type SmpsfbypR = crate::BitReader;
#[doc = "Field `SMPSFBYP` writer - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
pub type SmpsfbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSMPS` reader - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;"]
pub type NosmpsR = crate::BitReader;
#[doc = "Field `NOSMPS` writer - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;"]
pub type NosmpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPS_ENA_DCM` reader - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable"]
pub type SmpsEnaDcmR = crate::BitReader;
#[doc = "Field `SMPS_ENA_DCM` writer - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable"]
pub type SmpsEnaDcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDETR_DISABLE` reader - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled"]
pub type ClkdetrDisableR = crate::BitReader;
#[doc = "Field `CLKDETR_DISABLE` writer - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled"]
pub type ClkdetrDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPS_PRECH_CUR_SEL` reader - SMPS_PRECH_CUR_SEL\\[1:0\\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)"]
pub type SmpsPrechCurSelR = crate::FieldReader;
#[doc = "Field `SMPS_PRECH_CUR_SEL` writer - SMPS_PRECH_CUR_SEL\\[1:0\\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)"]
pub type SmpsPrechCurSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPS_BOF_DYN` reader - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)"]
pub type SmpsBofDynR = crate::BitReader;
#[doc = "Field `SMPS_BOF_DYN` writer - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)"]
pub type SmpsBofDynW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - SMPSLVL\\[3:0\\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)"]
    #[inline(always)]
    pub fn smpslvl(&self) -> SmpslvlR {
        SmpslvlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a"]
    #[inline(always)]
    pub fn smpsbomsel(&self) -> SmpsbomselR {
        SmpsbomselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)"]
    #[inline(always)]
    pub fn smps_bof_static(&self) -> SmpsBofStaticR {
        SmpsBofStaticR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;"]
    #[inline(always)]
    pub fn nosmps_bof(&self) -> NosmpsBofR {
        NosmpsBofR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating"]
    #[inline(always)]
    pub fn smpslpopen(&self) -> SmpslpopenR {
        SmpslpopenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
    #[inline(always)]
    pub fn smpsfbyp(&self) -> SmpsfbypR {
        SmpsfbypR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;"]
    #[inline(always)]
    pub fn nosmps(&self) -> NosmpsR {
        NosmpsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable"]
    #[inline(always)]
    pub fn smps_ena_dcm(&self) -> SmpsEnaDcmR {
        SmpsEnaDcmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled"]
    #[inline(always)]
    pub fn clkdetr_disable(&self) -> ClkdetrDisableR {
        ClkdetrDisableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - SMPS_PRECH_CUR_SEL\\[1:0\\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)"]
    #[inline(always)]
    pub fn smps_prech_cur_sel(&self) -> SmpsPrechCurSelR {
        SmpsPrechCurSelR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)"]
    #[inline(always)]
    pub fn smps_bof_dyn(&self) -> SmpsBofDynR {
        SmpsBofDynR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SMPSLVL\\[3:0\\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)"]
    #[inline(always)]
    pub fn smpslvl(&mut self) -> SmpslvlW<'_, Cr5Spec> {
        SmpslvlW::new(self, 0)
    }
    #[doc = "Bits 4:5 - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a"]
    #[inline(always)]
    pub fn smpsbomsel(&mut self) -> SmpsbomselW<'_, Cr5Spec> {
        SmpsbomselW::new(self, 4)
    }
    #[doc = "Bit 6 - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)"]
    #[inline(always)]
    pub fn smps_bof_static(&mut self) -> SmpsBofStaticW<'_, Cr5Spec> {
        SmpsBofStaticW::new(self, 6)
    }
    #[doc = "Bit 7 - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;"]
    #[inline(always)]
    pub fn nosmps_bof(&mut self) -> NosmpsBofW<'_, Cr5Spec> {
        NosmpsBofW::new(self, 7)
    }
    #[doc = "Bit 8 - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating"]
    #[inline(always)]
    pub fn smpslpopen(&mut self) -> SmpslpopenW<'_, Cr5Spec> {
        SmpslpopenW::new(self, 8)
    }
    #[doc = "Bit 9 - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)"]
    #[inline(always)]
    pub fn smpsfbyp(&mut self) -> SmpsfbypW<'_, Cr5Spec> {
        SmpsfbypW::new(self, 9)
    }
    #[doc = "Bit 10 - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;"]
    #[inline(always)]
    pub fn nosmps(&mut self) -> NosmpsW<'_, Cr5Spec> {
        NosmpsW::new(self, 10)
    }
    #[doc = "Bit 11 - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable"]
    #[inline(always)]
    pub fn smps_ena_dcm(&mut self) -> SmpsEnaDcmW<'_, Cr5Spec> {
        SmpsEnaDcmW::new(self, 11)
    }
    #[doc = "Bit 12 - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled"]
    #[inline(always)]
    pub fn clkdetr_disable(&mut self) -> ClkdetrDisableW<'_, Cr5Spec> {
        ClkdetrDisableW::new(self, 12)
    }
    #[doc = "Bits 13:14 - SMPS_PRECH_CUR_SEL\\[1:0\\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)"]
    #[inline(always)]
    pub fn smps_prech_cur_sel(&mut self) -> SmpsPrechCurSelW<'_, Cr5Spec> {
        SmpsPrechCurSelW::new(self, 13)
    }
    #[doc = "Bit 15 - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)"]
    #[inline(always)]
    pub fn smps_bof_dyn(&mut self) -> SmpsBofDynW<'_, Cr5Spec> {
        SmpsBofDynW::new(self, 15)
    }
}
#[doc = "CR5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr5Spec;
impl crate::RegisterSpec for Cr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr5::R`](R) reader structure"]
impl crate::Readable for Cr5Spec {}
#[doc = "`write(|w| ..)` method takes [`cr5::W`](W) writer structure"]
impl crate::Writable for Cr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR5 to value 0x6014"]
impl crate::Resettable for Cr5Spec {
    const RESET_VALUE: u32 = 0x6014;
}
