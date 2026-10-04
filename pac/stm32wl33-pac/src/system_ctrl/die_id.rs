#[doc = "Register `DIE_ID` reader"]
pub type R = crate::R<DieIdSpec>;
#[doc = "Field `REVISION` reader - Cut revision (metal fix)"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `VERSION` reader - Cut version"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `PRODUCT` reader - Product version. May be used to discriminate several version of a same digital BLE LPH device embedding different analog versions"]
pub type ProductR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Cut revision (metal fix)"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Cut version"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Product version. May be used to discriminate several version of a same digital BLE LPH device embedding different analog versions"]
    #[inline(always)]
    pub fn product(&self) -> ProductR {
        ProductR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "DIE_ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`die_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieIdSpec;
impl crate::RegisterSpec for DieIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`die_id::R`](R) reader structure"]
impl crate::Readable for DieIdSpec {}
#[doc = "`reset()` method sets DIE_ID to value 0x0120"]
impl crate::Resettable for DieIdSpec {
    const RESET_VALUE: u32 = 0x0120;
}
