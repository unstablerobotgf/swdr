#[doc = "Register `RNG_VAL` reader"]
pub type R = crate::R<RngValSpec>;
#[doc = "Field `RANDOM_VALUE` reader - Random Value"]
pub type RandomValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Random Value"]
    #[inline(always)]
    pub fn random_value(&self) -> RandomValueR {
        RandomValueR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RNG_VAL register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngValSpec;
impl crate::RegisterSpec for RngValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_val::R`](R) reader structure"]
impl crate::Readable for RngValSpec {}
#[doc = "`reset()` method sets RNG_VAL to value 0"]
impl crate::Resettable for RngValSpec {}
