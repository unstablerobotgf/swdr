#[doc = "Register `VCO_CALIB_OUT` reader"]
pub type R = crate::R<VcoCalibOutSpec>;
#[doc = "Field `VCO_CALFREQ_OUT` reader - VCO frequency calibration value currently output by the VCO calibration block (and applied on the VCO when ON)"]
pub type VcoCalfreqOutR = crate::FieldReader;
#[doc = "Field `VCO_CALAMP_OUT` reader - VCO amplitude calibration value currently output by the VCO calibration block (and applied on the VCO when ON)"]
pub type VcoCalampOutR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:6 - VCO frequency calibration value currently output by the VCO calibration block (and applied on the VCO when ON)"]
    #[inline(always)]
    pub fn vco_calfreq_out(&self) -> VcoCalfreqOutR {
        VcoCalfreqOutR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:21 - VCO amplitude calibration value currently output by the VCO calibration block (and applied on the VCO when ON)"]
    #[inline(always)]
    pub fn vco_calamp_out(&self) -> VcoCalampOutR {
        VcoCalampOutR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "VCO_CALIB_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`vco_calib_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VcoCalibOutSpec;
impl crate::RegisterSpec for VcoCalibOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vco_calib_out::R`](R) reader structure"]
impl crate::Readable for VcoCalibOutSpec {}
#[doc = "`reset()` method sets VCO_CALIB_OUT to value 0xff40"]
impl crate::Resettable for VcoCalibOutSpec {
    const RESET_VALUE: u32 = 0xff40;
}
