#[doc = "Register `C6CR` reader"]
pub type R = crate::R<C6crSpec>;
#[doc = "Register `C6CR` writer"]
pub type W = crate::W<C6crSpec>;
#[doc = "Field `DMAREQ_ID` reader - DMAREQ_ID\\[4:0\\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DmareqIdR = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - DMAREQ_ID\\[4:0\\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DmareqIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMAREQ_ID\\[4:0\\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DmareqIdR {
        DmareqIdR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMAREQ_ID\\[4:0\\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DmareqIdW<'_, C6crSpec> {
        DmareqIdW::new(self, 0)
    }
}
#[doc = "CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6crSpec;
impl crate::RegisterSpec for C6crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6cr::R`](R) reader structure"]
impl crate::Readable for C6crSpec {}
#[doc = "`write(|w| ..)` method takes [`c6cr::W`](W) writer structure"]
impl crate::Writable for C6crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C6CR to value 0"]
impl crate::Resettable for C6crSpec {}
