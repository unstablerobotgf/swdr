#[doc = "Register `RNGPCellID2` reader"]
pub type R = crate::R<RngpcellId2Spec>;
#[doc = "Field `RNGPCellID2` reader - These bits are read back as 0x05"]
pub type RngpcellId2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits are read back as 0x05"]
    #[inline(always)]
    pub fn rngpcell_id2(&self) -> RngpcellId2R {
        RngpcellId2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RNGPCellID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngpcellId2Spec;
impl crate::RegisterSpec for RngpcellId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpcell_id2::R`](R) reader structure"]
impl crate::Readable for RngpcellId2Spec {}
#[doc = "`reset()` method sets RNGPCellID2 to value 0x05"]
impl crate::Resettable for RngpcellId2Spec {
    const RESET_VALUE: u32 = 0x05;
}
