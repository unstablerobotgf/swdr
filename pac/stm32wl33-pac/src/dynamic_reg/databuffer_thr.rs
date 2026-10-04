#[doc = "Register `DATABUFFER_THR` reader"]
pub type R = crate::R<DatabufferThrSpec>;
#[doc = "Register `DATABUFFER_THR` writer"]
pub type W = crate::W<DatabufferThrSpec>;
#[doc = "Field `RX_ALMOST_FULL_THR` reader - Almost Full threshold for RX Data Buffers"]
pub type RxAlmostFullThrR = crate::FieldReader<u16>;
#[doc = "Field `RX_ALMOST_FULL_THR` writer - Almost Full threshold for RX Data Buffers"]
pub type RxAlmostFullThrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX_ALMOST_EMPTY_THR` reader - Almost Empty threshold for TX Data Buffers."]
pub type TxAlmostEmptyThrR = crate::FieldReader<u16>;
#[doc = "Field `TX_ALMOST_EMPTY_THR` writer - Almost Empty threshold for TX Data Buffers."]
pub type TxAlmostEmptyThrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Almost Full threshold for RX Data Buffers"]
    #[inline(always)]
    pub fn rx_almost_full_thr(&self) -> RxAlmostFullThrR {
        RxAlmostFullThrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Almost Empty threshold for TX Data Buffers."]
    #[inline(always)]
    pub fn tx_almost_empty_thr(&self) -> TxAlmostEmptyThrR {
        TxAlmostEmptyThrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Almost Full threshold for RX Data Buffers"]
    #[inline(always)]
    pub fn rx_almost_full_thr(&mut self) -> RxAlmostFullThrW<'_, DatabufferThrSpec> {
        RxAlmostFullThrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Almost Empty threshold for TX Data Buffers."]
    #[inline(always)]
    pub fn tx_almost_empty_thr(&mut self) -> TxAlmostEmptyThrW<'_, DatabufferThrSpec> {
        TxAlmostEmptyThrW::new(self, 16)
    }
}
#[doc = "DATABUFFER_THR register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufferThrSpec;
impl crate::RegisterSpec for DatabufferThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databuffer_thr::R`](R) reader structure"]
impl crate::Readable for DatabufferThrSpec {}
#[doc = "`write(|w| ..)` method takes [`databuffer_thr::W`](W) writer structure"]
impl crate::Writable for DatabufferThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATABUFFER_THR to value 0"]
impl crate::Resettable for DatabufferThrSpec {}
