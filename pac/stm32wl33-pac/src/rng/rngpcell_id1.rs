#[doc = "Register `RNGPCellID1` reader"]
pub type R = crate::R<RngpcellId1Spec>;
#[doc = "Field `RNGPCellID1` reader - These bits are read back as 0xF0"]
pub type RngpcellId1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits are read back as 0xF0"]
    #[inline(always)]
    pub fn rngpcell_id1(&self) -> RngpcellId1R {
        RngpcellId1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RNGPCellID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngpcellId1Spec;
impl crate::RegisterSpec for RngpcellId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpcell_id1::R`](R) reader structure"]
impl crate::Readable for RngpcellId1Spec {}
#[doc = "`reset()` method sets RNGPCellID1 to value 0xf0"]
impl crate::Resettable for RngpcellId1Spec {
    const RESET_VALUE: u32 = 0xf0;
}
