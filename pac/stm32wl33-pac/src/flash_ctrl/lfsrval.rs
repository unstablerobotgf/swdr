#[doc = "Register `LFSRVAL` reader"]
pub type R = crate::R<LfsrvalSpec>;
#[doc = "Field `LFSRVAL` reader - Flash read data CRC signature"]
pub type LfsrvalR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Flash read data CRC signature"]
    #[inline(always)]
    pub fn lfsrval(&self) -> LfsrvalR {
        LfsrvalR::new(self.bits)
    }
}
#[doc = "LFSRVAL register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsrval::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfsrvalSpec;
impl crate::RegisterSpec for LfsrvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsrval::R`](R) reader structure"]
impl crate::Readable for LfsrvalSpec {}
#[doc = "`reset()` method sets LFSRVAL to value 0xffff_ffff"]
impl crate::Resettable for LfsrvalSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
