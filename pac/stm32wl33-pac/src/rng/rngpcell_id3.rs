#[doc = "Register `RNGPCellID3` reader"]
pub type R = crate::R<RngpcellId3Spec>;
#[doc = "Field `RNGPCellID3` reader - These bits are read back as 0xB1"]
pub type RngpcellId3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits are read back as 0xB1"]
    #[inline(always)]
    pub fn rngpcell_id3(&self) -> RngpcellId3R {
        RngpcellId3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RNGPCellID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngpcellId3Spec;
impl crate::RegisterSpec for RngpcellId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpcell_id3::R`](R) reader structure"]
impl crate::Readable for RngpcellId3Spec {}
#[doc = "`reset()` method sets RNGPCellID3 to value 0xb1"]
impl crate::Resettable for RngpcellId3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
