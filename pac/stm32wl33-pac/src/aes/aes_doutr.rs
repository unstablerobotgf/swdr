#[doc = "Register `AES_DOUTR` reader"]
pub type R = crate::R<AesDoutrSpec>;
#[doc = "Field `DOUTR` reader - DOUTR\\[x+31:x\\]: One of four 32-bit words of a 128-bit output data block being read from the"]
pub type DoutrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DOUTR\\[x+31:x\\]: One of four 32-bit words of a 128-bit output data block being read from the"]
    #[inline(always)]
    pub fn doutr(&self) -> DoutrR {
        DoutrR::new(self.bits)
    }
}
#[doc = "AES_DOUTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_doutr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDoutrSpec;
impl crate::RegisterSpec for AesDoutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_doutr::R`](R) reader structure"]
impl crate::Readable for AesDoutrSpec {}
#[doc = "`reset()` method sets AES_DOUTR to value 0"]
impl crate::Resettable for AesDoutrSpec {}
