#[doc = "Register `JTAG_ID` reader"]
pub type R = crate::R<JtagIdSpec>;
#[doc = "Field `MANUF_ID` reader - Manufacturer ID"]
pub type ManufIdR = crate::FieldReader<u16>;
#[doc = "Field `PART_NUMBER` reader - Part number"]
pub type PartNumberR = crate::FieldReader<u16>;
#[doc = "Field `VERSION_NUMBER` reader - Version"]
pub type VersionNumberR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:11 - Manufacturer ID"]
    #[inline(always)]
    pub fn manuf_id(&self) -> ManufIdR {
        ManufIdR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - Part number"]
    #[inline(always)]
    pub fn part_number(&self) -> PartNumberR {
        PartNumberR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - Version"]
    #[inline(always)]
    pub fn version_number(&self) -> VersionNumberR {
        VersionNumberR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "JTAG_ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagIdSpec;
impl crate::RegisterSpec for JtagIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag_id::R`](R) reader structure"]
impl crate::Readable for JtagIdSpec {}
#[doc = "`reset()` method sets JTAG_ID to value 0x0202_7041"]
impl crate::Resettable for JtagIdSpec {
    const RESET_VALUE: u32 = 0x0202_7041;
}
