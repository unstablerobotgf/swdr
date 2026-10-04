#[doc = "Register `DS_DATAOUT` reader"]
pub type R = crate::R<DsDataoutSpec>;
#[doc = "Field `DS_DATA` reader - DS_DATA\\[15:0\\]: contain the converted data at the output of the Down Sampler."]
pub type DsDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DS_DATA\\[15:0\\]: contain the converted data at the output of the Down Sampler."]
    #[inline(always)]
    pub fn ds_data(&self) -> DsDataR {
        DsDataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DS_DATAOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_dataout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsDataoutSpec;
impl crate::RegisterSpec for DsDataoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds_dataout::R`](R) reader structure"]
impl crate::Readable for DsDataoutSpec {}
#[doc = "`reset()` method sets DS_DATAOUT to value 0"]
impl crate::Resettable for DsDataoutSpec {}
