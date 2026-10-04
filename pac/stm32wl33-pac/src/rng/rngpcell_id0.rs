#[doc = "Register `RNGPCellID0` reader"]
pub type R = crate::R<RngpcellId0Spec>;
#[doc = "Field `RNGPCellID0` reader - These bits are read back as 0x0D"]
pub type RngpcellId0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits are read back as 0x0D"]
    #[inline(always)]
    pub fn rngpcell_id0(&self) -> RngpcellId0R {
        RngpcellId0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RNGPCellID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngpcell_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngpcellId0Spec;
impl crate::RegisterSpec for RngpcellId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngpcell_id0::R`](R) reader structure"]
impl crate::Readable for RngpcellId0Spec {}
#[doc = "`reset()` method sets RNGPCellID0 to value 0x0d"]
impl crate::Resettable for RngpcellId0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
