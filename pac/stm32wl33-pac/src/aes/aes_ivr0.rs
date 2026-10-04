#[doc = "Register `AES_IVR0` reader"]
pub type R = crate::R<AesIvr0Spec>;
#[doc = "Register `AES_IVR0` writer"]
pub type W = crate::W<AesIvr0Spec>;
#[doc = "Field `IVI` reader - IVI \\[((32*x)+31):((32*x)+0)\\]: Initialization vector register (LSB IVR\\[((32*x)+31):((32*x)+0)\\])"]
pub type IviR = crate::FieldReader<u32>;
#[doc = "Field `IVI` writer - IVI \\[((32*x)+31):((32*x)+0)\\]: Initialization vector register (LSB IVR\\[((32*x)+31):((32*x)+0)\\])"]
pub type IviW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IVI \\[((32*x)+31):((32*x)+0)\\]: Initialization vector register (LSB IVR\\[((32*x)+31):((32*x)+0)\\])"]
    #[inline(always)]
    pub fn ivi(&self) -> IviR {
        IviR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IVI \\[((32*x)+31):((32*x)+0)\\]: Initialization vector register (LSB IVR\\[((32*x)+31):((32*x)+0)\\])"]
    #[inline(always)]
    pub fn ivi(&mut self) -> IviW<'_, AesIvr0Spec> {
        IviW::new(self, 0)
    }
}
#[doc = "AES_IVRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ivr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIvr0Spec;
impl crate::RegisterSpec for AesIvr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ivr0::R`](R) reader structure"]
impl crate::Readable for AesIvr0Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_ivr0::W`](W) writer structure"]
impl crate::Writable for AesIvr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_IVR0 to value 0"]
impl crate::Resettable for AesIvr0Spec {}
