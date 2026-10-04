#[doc = "Register `RNGPeriphID0` reader"]
pub type R = crate::R<RngperiphId0Spec>;
#[doc = "Field `PartNumber0` reader - These bits are read back as 0xE1"]
pub type PartNumber0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits are read back as 0xE1"]
    #[inline(always)]
    pub fn part_number0(&self) -> PartNumber0R {
        PartNumber0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RNGPeriphID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngperiphId0Spec;
impl crate::RegisterSpec for RngperiphId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngperiph_id0::R`](R) reader structure"]
impl crate::Readable for RngperiphId0Spec {}
#[doc = "`reset()` method sets RNGPeriphID0 to value 0xe1"]
impl crate::Resettable for RngperiphId0Spec {
    const RESET_VALUE: u32 = 0xe1;
}
