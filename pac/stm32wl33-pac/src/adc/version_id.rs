#[doc = "Register `VERSION_ID` reader"]
pub type R = crate::R<VersionIdSpec>;
#[doc = "Field `VERSION_ID` reader - VERSION_ID\\[7:0\\]: version of the embedded IP."]
pub type VersionIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VERSION_ID\\[7:0\\]: version of the embedded IP."]
    #[inline(always)]
    pub fn version_id(&self) -> VersionIdR {
        VersionIdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "VERSION_ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`version_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionIdSpec;
impl crate::RegisterSpec for VersionIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version_id::R`](R) reader structure"]
impl crate::Readable for VersionIdSpec {}
#[doc = "`reset()` method sets VERSION_ID to value 0x21"]
impl crate::Resettable for VersionIdSpec {
    const RESET_VALUE: u32 = 0x21;
}
