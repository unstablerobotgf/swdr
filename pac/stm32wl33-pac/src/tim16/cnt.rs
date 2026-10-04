#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNT` reader - CNT\\[15:0\\]: Counter value"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - CNT\\[15:0\\]: Counter value"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UIF_CPY` reader - UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0."]
pub type UifCpyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - CNT\\[15:0\\]: Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0."]
    #[inline(always)]
    pub fn uif_cpy(&self) -> UifCpyR {
        UifCpyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNT\\[15:0\\]: Counter value"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, CntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "CNT register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {}
