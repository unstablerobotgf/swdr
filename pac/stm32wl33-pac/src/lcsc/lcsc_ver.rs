#[doc = "Register `LCSC_VER` reader"]
pub type R = crate::R<LcscVerSpec>;
#[doc = "Field `REV` reader - Revision of the RFIP to be used for metal fixes)"]
pub type RevR = crate::FieldReader;
#[doc = "Field `VER` reader - Version of the RFIP (to be used for cut upgrades)"]
pub type VerR = crate::FieldReader;
#[doc = "Field `PROD` reader - Used for major upgrades (new protocols support / new features)"]
pub type ProdR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Revision of the RFIP to be used for metal fixes)"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Version of the RFIP (to be used for cut upgrades)"]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Used for major upgrades (new protocols support / new features)"]
    #[inline(always)]
    pub fn prod(&self) -> ProdR {
        ProdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "LCSC_VER register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscVerSpec;
impl crate::RegisterSpec for LcscVerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_ver::R`](R) reader structure"]
impl crate::Readable for LcscVerSpec {}
#[doc = "`reset()` method sets LCSC_VER to value 0x1000"]
impl crate::Resettable for LcscVerSpec {
    const RESET_VALUE: u32 = 0x1000;
}
