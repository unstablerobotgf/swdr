#[doc = "Register `ENGTRIM` reader"]
pub type R = crate::R<EngtrimSpec>;
#[doc = "Register `ENGTRIM` writer"]
pub type W = crate::W<EngtrimSpec>;
#[doc = "Field `TRIMRFDREGEN` reader - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type TrimrfdregenR = crate::BitReader;
#[doc = "Field `TRIMRFDREGEN` writer - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type TrimrfdregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_RFDREG` reader - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register."]
pub type TrimRfdregR = crate::FieldReader;
#[doc = "Field `TRIM_RFDREG` writer - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register."]
pub type TrimRfdregW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPARE` reader - "]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - "]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMMREN` reader - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type TrimmrenR = crate::BitReader;
#[doc = "Field `TRIMMREN` writer - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type TrimmrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_MR` reader - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register."]
pub type TrimMrR = crate::FieldReader;
#[doc = "Field `TRIM_MR` writer - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register."]
pub type TrimMrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMPSTRIMEN` reader - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type SmpstrimenR = crate::BitReader;
#[doc = "Field `SMPSTRIMEN` writer - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type SmpstrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPS_TRIM` reader - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register."]
pub type SmpsTrimR = crate::FieldReader;
#[doc = "Field `SMPS_TRIM` writer - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register."]
pub type SmpsTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn trimrfdregen(&self) -> TrimrfdregenR {
        TrimrfdregenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register."]
    #[inline(always)]
    pub fn trim_rfdreg(&self) -> TrimRfdregR {
        TrimRfdregR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn trimmren(&self) -> TrimmrenR {
        TrimmrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register."]
    #[inline(always)]
    pub fn trim_mr(&self) -> TrimMrR {
        TrimMrR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn smpstrimen(&self) -> SmpstrimenR {
        SmpstrimenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register."]
    #[inline(always)]
    pub fn smps_trim(&self) -> SmpsTrimR {
        SmpsTrimR::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn trimrfdregen(&mut self) -> TrimrfdregenW<'_, EngtrimSpec> {
        TrimrfdregenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register."]
    #[inline(always)]
    pub fn trim_rfdreg(&mut self) -> TrimRfdregW<'_, EngtrimSpec> {
        TrimRfdregW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spare(&mut self) -> SpareW<'_, EngtrimSpec> {
        SpareW::new(self, 4)
    }
    #[doc = "Bit 5 - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn trimmren(&mut self) -> TrimmrenW<'_, EngtrimSpec> {
        TrimmrenW::new(self, 5)
    }
    #[doc = "Bits 6:9 - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register."]
    #[inline(always)]
    pub fn trim_mr(&mut self) -> TrimMrW<'_, EngtrimSpec> {
        TrimMrW::new(self, 6)
    }
    #[doc = "Bit 10 - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn smpstrimen(&mut self) -> SmpstrimenW<'_, EngtrimSpec> {
        SmpstrimenW::new(self, 10)
    }
    #[doc = "Bits 11:13 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register."]
    #[inline(always)]
    pub fn smps_trim(&mut self) -> SmpsTrimW<'_, EngtrimSpec> {
        SmpsTrimW::new(self, 11)
    }
}
#[doc = "ENGTRIM register\n\nYou can [`read`](crate::Reg::read) this register and get [`engtrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EngtrimSpec;
impl crate::RegisterSpec for EngtrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`engtrim::R`](R) reader structure"]
impl crate::Readable for EngtrimSpec {}
#[doc = "`write(|w| ..)` method takes [`engtrim::W`](W) writer structure"]
impl crate::Writable for EngtrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENGTRIM to value 0"]
impl crate::Resettable for EngtrimSpec {}
