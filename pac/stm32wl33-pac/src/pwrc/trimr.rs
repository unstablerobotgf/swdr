#[doc = "Register `TRIMR` reader"]
pub type R = crate::R<TrimrSpec>;
#[doc = "Field `RFD_REG_TRIM` reader - RFD_REG_TRIM\\[2:0\\]: RF LDO Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the RF LDO trimming can be controlled by the dedicated ENGTRIM register. Default= '100'."]
pub type RfdRegTrimR = crate::FieldReader;
#[doc = "Field `SPARE` reader - "]
pub type SpareR = crate::BitReader;
#[doc = "Field `TRIM_MR` reader - TRIM_MR\\[3:0\\]: Main Regulator Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM.TRIMMREN is enabled the Main Regulator Voltage can be controlled by the dedicated ENGTRIM.TRIM_MR register. Default= '0000'."]
pub type TrimMrR = crate::FieldReader;
#[doc = "Field `SMPS_TRIM` reader - SMPS_TRIM\\[2:0\\]: SMPS Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '011'."]
pub type SmpsTrimR = crate::FieldReader;
#[doc = "Field `BOF_TRIM` reader - BOF_TRIM\\[2:0\\]: Bypass On the Fly Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '100'."]
pub type BofTrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - RFD_REG_TRIM\\[2:0\\]: RF LDO Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the RF LDO trimming can be controlled by the dedicated ENGTRIM register. Default= '100'."]
    #[inline(always)]
    pub fn rfd_reg_trim(&self) -> RfdRegTrimR {
        RfdRegTrimR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - TRIM_MR\\[3:0\\]: Main Regulator Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM.TRIMMREN is enabled the Main Regulator Voltage can be controlled by the dedicated ENGTRIM.TRIM_MR register. Default= '0000'."]
    #[inline(always)]
    pub fn trim_mr(&self) -> TrimMrR {
        TrimMrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - SMPS_TRIM\\[2:0\\]: SMPS Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '011'."]
    #[inline(always)]
    pub fn smps_trim(&self) -> SmpsTrimR {
        SmpsTrimR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - BOF_TRIM\\[2:0\\]: Bypass On the Fly Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '100'."]
    #[inline(always)]
    pub fn bof_trim(&self) -> BofTrimR {
        BofTrimR::new(((self.bits >> 11) & 7) as u8)
    }
}
#[doc = "TRIMR register\n\nYou can [`read`](crate::Reg::read) this register and get [`trimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrimrSpec;
impl crate::RegisterSpec for TrimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trimr::R`](R) reader structure"]
impl crate::Readable for TrimrSpec {}
#[doc = "`reset()` method sets TRIMR to value 0x2304"]
impl crate::Resettable for TrimrSpec {
    const RESET_VALUE: u32 = 0x2304;
}
