#[doc = "Register `SCM_MIN_MAX` reader"]
pub type R = crate::R<ScmMinMaxSpec>;
#[doc = "Register `SCM_MIN_MAX` writer"]
pub type W = crate::W<ScmMinMaxSpec>;
#[doc = "Field `SCM_COUNTER_MINVAL` reader - Slow Clock Measurement: minimum SCM_COUNTER value seen since the counter is ON and since last clear request."]
pub type ScmCounterMinvalR = crate::FieldReader<u16>;
#[doc = "Field `SCM_COUNTER_MAXVAL` reader - Slow Clock Measurement: maximum SCM_COUNTER value seen since the counter is ON and since last clear request."]
pub type ScmCounterMaxvalR = crate::FieldReader<u16>;
#[doc = "Field `CLEAR_MIN_MAX` writer - Write 1' to clear the SCM_COUNTER_MINVAL and SCM_COUNTER_MAXVAL bit fields."]
pub type ClearMinMaxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Slow Clock Measurement: minimum SCM_COUNTER value seen since the counter is ON and since last clear request."]
    #[inline(always)]
    pub fn scm_counter_minval(&self) -> ScmCounterMinvalR {
        ScmCounterMinvalR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Slow Clock Measurement: maximum SCM_COUNTER value seen since the counter is ON and since last clear request."]
    #[inline(always)]
    pub fn scm_counter_maxval(&self) -> ScmCounterMaxvalR {
        ScmCounterMaxvalR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Write 1' to clear the SCM_COUNTER_MINVAL and SCM_COUNTER_MAXVAL bit fields."]
    #[inline(always)]
    pub fn clear_min_max(&mut self) -> ClearMinMaxW<'_, ScmMinMaxSpec> {
        ClearMinMaxW::new(self, 31)
    }
}
#[doc = "SCM_MIN_MAX register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm_min_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm_min_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmMinMaxSpec;
impl crate::RegisterSpec for ScmMinMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scm_min_max::R`](R) reader structure"]
impl crate::Readable for ScmMinMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`scm_min_max::W`](W) writer structure"]
impl crate::Writable for ScmMinMaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCM_MIN_MAX to value 0x7fff"]
impl crate::Resettable for ScmMinMaxSpec {
    const RESET_VALUE: u32 = 0x7fff;
}
