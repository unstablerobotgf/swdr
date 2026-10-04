#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Field `SMPSBYPR` reader - SMPSBYPR: SMPS Force Bypass Control Replica This bit mirrors the actual BYPASS_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
pub type SmpsbyprR = crate::BitReader;
#[doc = "Field `SMPSENR` reader - SMPSENR: SMPS Enable Control Replica This bit mirrors the actual ENABLE_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
pub type SmpsenrR = crate::BitReader;
#[doc = "Field `SMPSRDY` reader - SMPSRDY: SMPS Ready Status This bit provides the information whether SMPS is ready. - 0: SMPS regulator is not ready - 1: SMPS regulator is ready."]
pub type SmpsrdyR = crate::BitReader;
#[doc = "Field `IOBOOTVAL2` reader - Bit3: PB15 input value on VDD33 latched at POR Bit2: PB14 input value on VDD33 latched at POR Bit1: PB13 input value on VDD33 latched at POR Bit0: PB12 input value on VDD33 latched at POR"]
pub type Iobootval2R = crate::FieldReader;
#[doc = "Field `REGLPS` reader - REGLPS: Regulator Low Power Started This bit provides the information whether low power regulator is ready. - 0: LP regulator is not ready. - 1: LP regulator is ready."]
pub type ReglpsR = crate::BitReader;
#[doc = "Field `REGMS` reader - REGMS: Main regulator ready status. - 0: The Main regulator is not ready. - 1: The Main regulator is ready."]
pub type RegmsR = crate::BitReader;
#[doc = "Field `PVDO` reader - PVDO: Power Voltage Detector Output When the Power Voltage Detector is enabled (CR2.PVDE) this bit is set when the system supply (VDDIO) is lower than the selected PVD threshold (CR2.PVDLS)"]
pub type PvdoR = crate::BitReader;
#[doc = "Field `IOBOOTVAL` reader - Bit3: PA11 input value on VDD33 latched at POR Bit2: PA10 input value on VDD33 latched at POR Bit1: PA9 input value on VDD33 latched at POR Bit0: PA8 input value on VDD33 latched at POR"]
pub type IobootvalR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SMPSBYPR: SMPS Force Bypass Control Replica This bit mirrors the actual BYPASS_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
    #[inline(always)]
    pub fn smpsbypr(&self) -> SmpsbyprR {
        SmpsbyprR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMPSENR: SMPS Enable Control Replica This bit mirrors the actual ENABLE_3V3 control signal driven to the SMPS regulator, dependant on the real working state."]
    #[inline(always)]
    pub fn smpsenr(&self) -> SmpsenrR {
        SmpsenrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMPSRDY: SMPS Ready Status This bit provides the information whether SMPS is ready. - 0: SMPS regulator is not ready - 1: SMPS regulator is ready."]
    #[inline(always)]
    pub fn smpsrdy(&self) -> SmpsrdyR {
        SmpsrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bit3: PB15 input value on VDD33 latched at POR Bit2: PB14 input value on VDD33 latched at POR Bit1: PB13 input value on VDD33 latched at POR Bit0: PB12 input value on VDD33 latched at POR"]
    #[inline(always)]
    pub fn iobootval2(&self) -> Iobootval2R {
        Iobootval2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - REGLPS: Regulator Low Power Started This bit provides the information whether low power regulator is ready. - 0: LP regulator is not ready. - 1: LP regulator is ready."]
    #[inline(always)]
    pub fn reglps(&self) -> ReglpsR {
        ReglpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REGMS: Main regulator ready status. - 0: The Main regulator is not ready. - 1: The Main regulator is ready."]
    #[inline(always)]
    pub fn regms(&self) -> RegmsR {
        RegmsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - PVDO: Power Voltage Detector Output When the Power Voltage Detector is enabled (CR2.PVDE) this bit is set when the system supply (VDDIO) is lower than the selected PVD threshold (CR2.PVDLS)"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Bit3: PA11 input value on VDD33 latched at POR Bit2: PA10 input value on VDD33 latched at POR Bit1: PA9 input value on VDD33 latched at POR Bit0: PA8 input value on VDD33 latched at POR"]
    #[inline(always)]
    pub fn iobootval(&self) -> IobootvalR {
        IobootvalR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "SR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`reset()` method sets SR2 to value 0xf3f6"]
impl crate::Resettable for Sr2Spec {
    const RESET_VALUE: u32 = 0xf3f6;
}
