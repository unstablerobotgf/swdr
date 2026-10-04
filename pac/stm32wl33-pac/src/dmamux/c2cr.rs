#[doc = "Register `C2CR` reader"]
pub type R = crate::R<C2crSpec>;
#[doc = "Register `C2CR` writer"]
pub type W = crate::W<C2crSpec>;
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
    pub fn dmareq_id(&mut self) -> DmareqIdW<'_, C2crSpec> {
        DmareqIdW::new(self, 0)
    }
}
#[doc = "CxCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2crSpec;
impl crate::RegisterSpec for C2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr::R`](R) reader structure"]
impl crate::Readable for C2crSpec {}
#[doc = "`write(|w| ..)` method takes [`c2cr::W`](W) writer structure"]
impl crate::Writable for C2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2CR to value 0"]
impl crate::Resettable for C2crSpec {}
