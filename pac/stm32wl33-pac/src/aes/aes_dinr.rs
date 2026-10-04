#[doc = "Register `AES_DINR` reader"]
pub type R = crate::R<AesDinrSpec>;
#[doc = "Register `AES_DINR` writer"]
pub type W = crate::W<AesDinrSpec>;
#[doc = "Field `DINR` reader - DINR\\[x+31:x\\]: One of four 32-bit words of a 128-bit input data block being written into the peripheral"]
pub type DinrR = crate::FieldReader<u32>;
#[doc = "Field `DINR` writer - DINR\\[x+31:x\\]: One of four 32-bit words of a 128-bit input data block being written into the peripheral"]
pub type DinrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DINR\\[x+31:x\\]: One of four 32-bit words of a 128-bit input data block being written into the peripheral"]
    #[inline(always)]
    pub fn dinr(&self) -> DinrR {
        DinrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DINR\\[x+31:x\\]: One of four 32-bit words of a 128-bit input data block being written into the peripheral"]
    #[inline(always)]
    pub fn dinr(&mut self) -> DinrW<'_, AesDinrSpec> {
        DinrW::new(self, 0)
    }
}
#[doc = "AES_DINR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_dinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_dinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDinrSpec;
impl crate::RegisterSpec for AesDinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_dinr::R`](R) reader structure"]
impl crate::Readable for AesDinrSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_dinr::W`](W) writer structure"]
impl crate::Writable for AesDinrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_DINR to value 0"]
impl crate::Resettable for AesDinrSpec {}
