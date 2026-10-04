#[doc = "Register `RF_INFO_OUT` reader"]
pub type R = crate::R<RfInfoOutSpec>;
#[doc = "Field `FQCY_BAND_ID` reader - FQCY_BAND_ID\\[3:0\\]: Indicates the version of the RFSUBG IP embedded in the device"]
pub type FqcyBandIdR = crate::FieldReader;
#[doc = "Field `RFSUBG_ID` reader - Indicate the version of the analog RFSUBG IP embedded in the device"]
pub type RfsubgIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - FQCY_BAND_ID\\[3:0\\]: Indicates the version of the RFSUBG IP embedded in the device"]
    #[inline(always)]
    pub fn fqcy_band_id(&self) -> FqcyBandIdR {
        FqcyBandIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicate the version of the analog RFSUBG IP embedded in the device"]
    #[inline(always)]
    pub fn rfsubg_id(&self) -> RfsubgIdR {
        RfsubgIdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "RF_INFO_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_info_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfInfoOutSpec;
impl crate::RegisterSpec for RfInfoOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_info_out::R`](R) reader structure"]
impl crate::Readable for RfInfoOutSpec {}
#[doc = "`reset()` method sets RF_INFO_OUT to value 0x40"]
impl crate::Resettable for RfInfoOutSpec {
    const RESET_VALUE: u32 = 0x40;
}
