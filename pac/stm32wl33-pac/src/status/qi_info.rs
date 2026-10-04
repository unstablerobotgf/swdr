#[doc = "Register `QI_INFO` reader"]
pub type R = crate::R<QiInfoSpec>;
#[doc = "Field `PQI_INFO` reader - Preamble Quality Indicator (PQI) value of the received packet."]
pub type PqiInfoR = crate::FieldReader;
#[doc = "Field `SQI_INFO` reader - SYNC Quality Indicator (SQI) value of the received packet."]
pub type SqiInfoR = crate::FieldReader;
#[doc = "Field `SQI_SEC` reader - Indicate if measured SQI refers to SYNC word or secondary SYNC word"]
pub type SqiSecR = crate::BitReader;
#[doc = "Field `AFC_CORRECTION` reader - AFC value frozen at sync reception."]
pub type AfcCorrectionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble Quality Indicator (PQI) value of the received packet."]
    #[inline(always)]
    pub fn pqi_info(&self) -> PqiInfoR {
        PqiInfoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - SYNC Quality Indicator (SQI) value of the received packet."]
    #[inline(always)]
    pub fn sqi_info(&self) -> SqiInfoR {
        SqiInfoR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Indicate if measured SQI refers to SYNC word or secondary SYNC word"]
    #[inline(always)]
    pub fn sqi_sec(&self) -> SqiSecR {
        SqiSecR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - AFC value frozen at sync reception."]
    #[inline(always)]
    pub fn afc_correction(&self) -> AfcCorrectionR {
        AfcCorrectionR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "QI_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`qi_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QiInfoSpec;
impl crate::RegisterSpec for QiInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qi_info::R`](R) reader structure"]
impl crate::Readable for QiInfoSpec {}
#[doc = "`reset()` method sets QI_INFO to value 0"]
impl crate::Resettable for QiInfoSpec {}
