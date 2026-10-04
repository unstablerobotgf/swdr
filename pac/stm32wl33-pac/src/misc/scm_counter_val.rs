#[doc = "Register `SCM_COUNTER_VAL` reader"]
pub type R = crate::R<ScmCounterValSpec>;
#[doc = "Field `SCM_COUNTER_CURRVAL` reader - Slow Clock Measurement: number of 16 MHz clock cycles contained in 32 slow clock periods."]
pub type ScmCounterCurrvalR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Slow Clock Measurement: number of 16 MHz clock cycles contained in 32 slow clock periods."]
    #[inline(always)]
    pub fn scm_counter_currval(&self) -> ScmCounterCurrvalR {
        ScmCounterCurrvalR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "SCM_COUNTER_VAL register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm_counter_val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmCounterValSpec;
impl crate::RegisterSpec for ScmCounterValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scm_counter_val::R`](R) reader structure"]
impl crate::Readable for ScmCounterValSpec {}
#[doc = "`reset()` method sets SCM_COUNTER_VAL to value 0"]
impl crate::Resettable for ScmCounterValSpec {}
