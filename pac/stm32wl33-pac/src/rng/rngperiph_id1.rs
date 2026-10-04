#[doc = "Register `RNGPeriphID1` reader"]
pub type R = crate::R<RngperiphId1Spec>;
#[doc = "Field `PartNumber1` reader - These bits are read back as 0x05"]
pub type PartNumber1R = crate::FieldReader;
#[doc = "Field `Designer0` reader - These bits are read back as 0x00"]
pub type Designer0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits are read back as 0x05"]
    #[inline(always)]
    pub fn part_number1(&self) -> PartNumber1R {
        PartNumber1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits are read back as 0x00"]
    #[inline(always)]
    pub fn designer0(&self) -> Designer0R {
        Designer0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "RNGPeriphID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngperiphId1Spec;
impl crate::RegisterSpec for RngperiphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngperiph_id1::R`](R) reader structure"]
impl crate::Readable for RngperiphId1Spec {}
#[doc = "`reset()` method sets RNGPeriphID1 to value 0x05"]
impl crate::Resettable for RngperiphId1Spec {
    const RESET_VALUE: u32 = 0x05;
}
