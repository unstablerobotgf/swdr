#[doc = "Register `RNGPeriphID2` reader"]
pub type R = crate::R<RngperiphId2Spec>;
#[doc = "Field `Designer1` reader - These bits are read back as 0x08"]
pub type Designer1R = crate::FieldReader;
#[doc = "Field `Revision` reader - These bits are read back as 0x02"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits are read back as 0x08"]
    #[inline(always)]
    pub fn designer1(&self) -> Designer1R {
        Designer1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits are read back as 0x02"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "RNGPeriphID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngperiph_id2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngperiphId2Spec;
impl crate::RegisterSpec for RngperiphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngperiph_id2::R`](R) reader structure"]
impl crate::Readable for RngperiphId2Spec {}
#[doc = "`reset()` method sets RNGPeriphID2 to value 0x28"]
impl crate::Resettable for RngperiphId2Spec {
    const RESET_VALUE: u32 = 0x28;
}
