#[doc = "Register `DOR` reader"]
pub type R = crate::R<DorSpec>;
#[doc = "Field `DACDOR` reader - DACDOR\\[5:0\\]: DAC channel data output These bits are read-only, they contain data output for DAC channel."]
pub type DacdorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - DACDOR\\[5:0\\]: DAC channel data output These bits are read-only, they contain data output for DAC channel."]
    #[inline(always)]
    pub fn dacdor(&self) -> DacdorR {
        DacdorR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "DOR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DorSpec;
impl crate::RegisterSpec for DorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor::R`](R) reader structure"]
impl crate::Readable for DorSpec {}
#[doc = "`reset()` method sets DOR to value 0"]
impl crate::Resettable for DorSpec {}
