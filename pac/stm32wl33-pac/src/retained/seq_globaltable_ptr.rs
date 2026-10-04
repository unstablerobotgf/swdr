#[doc = "Register `SEQ_GLOBALTABLE_PTR` reader"]
pub type R = crate::R<SeqGlobaltablePtrSpec>;
#[doc = "Register `SEQ_GLOBALTABLE_PTR` writer"]
pub type W = crate::W<SeqGlobaltablePtrSpec>;
#[doc = "Field `SEQ_GLOBALTABLE_PTR` reader - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point."]
pub type SeqGlobaltablePtrR = crate::FieldReader<u16>;
#[doc = "Field `SEQ_GLOBALTABLE_PTR` writer - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point."]
pub type SeqGlobaltablePtrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point."]
    #[inline(always)]
    pub fn seq_globaltable_ptr(&self) -> SeqGlobaltablePtrR {
        SeqGlobaltablePtrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Contain the offset versus the SoC RAM base address of the GlobalConfiguration RAM table entry point."]
    #[inline(always)]
    pub fn seq_globaltable_ptr(&mut self) -> SeqGlobaltablePtrW<'_, SeqGlobaltablePtrSpec> {
        SeqGlobaltablePtrW::new(self, 0)
    }
}
#[doc = "SEQ_GLOBALTABLE_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_globaltable_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_globaltable_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqGlobaltablePtrSpec;
impl crate::RegisterSpec for SeqGlobaltablePtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_globaltable_ptr::R`](R) reader structure"]
impl crate::Readable for SeqGlobaltablePtrSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_globaltable_ptr::W`](W) writer structure"]
impl crate::Writable for SeqGlobaltablePtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQ_GLOBALTABLE_PTR to value 0"]
impl crate::Resettable for SeqGlobaltablePtrSpec {}
