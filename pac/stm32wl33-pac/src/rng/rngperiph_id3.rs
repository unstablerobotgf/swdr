#[doc = "Register `RNGPeriphID3` reader"]
pub type R = crate::R<RngperiphId3Spec>;
#[doc = "Field `Configuration` reader - These bits are read back as 0x00"]
pub type ConfigurationR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits are read back as 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> ConfigurationR {
        ConfigurationR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RNGPeriphID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngperiphId3Spec;
impl crate::RegisterSpec for RngperiphId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngperiph_id3::R`](R) reader structure"]
impl crate::Readable for RngperiphId3Spec {}
#[doc = "`reset()` method sets RNGPeriphID3 to value 0"]
impl crate::Resettable for RngperiphId3Spec {}
