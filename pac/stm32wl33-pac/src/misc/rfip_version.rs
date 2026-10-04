#[doc = "Register `RFIP_VERSION` reader"]
pub type R = crate::R<RfipVersionSpec>;
#[doc = "Field `REVISION` reader - Revision of the MR_SubG (to be used for metal fixes)"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `VERSION` reader - Version of the MR_SubG (to be used for cut upgrades)"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `PRODUCT` reader - Used for major upgrades (new protocols support / new features)"]
pub type ProductR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Revision of the MR_SubG (to be used for metal fixes)"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Version of the MR_SubG (to be used for cut upgrades)"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Used for major upgrades (new protocols support / new features)"]
    #[inline(always)]
    pub fn product(&self) -> ProductR {
        ProductR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "RFIP_VERSION register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfipVersionSpec;
impl crate::RegisterSpec for RfipVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfip_version::R`](R) reader structure"]
impl crate::Readable for RfipVersionSpec {}
#[doc = "`reset()` method sets RFIP_VERSION to value 0x1200"]
impl crate::Resettable for RfipVersionSpec {
    const RESET_VALUE: u32 = 0x1200;
}
