#[doc = "Register `AES_KEYR0` reader"]
pub type R = crate::R<AesKeyr0Spec>;
#[doc = "Register `AES_KEYR0` writer"]
pub type W = crate::W<AesKeyr0Spec>;
#[doc = "Field `KEY` reader - KEY \\[((32*x)+31):((32*x)+0)\\]: Cryptographic key, bits \\[((32*x)+31):((32*x)+0)\\]"]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - KEY \\[((32*x)+31):((32*x)+0)\\]: Cryptographic key, bits \\[((32*x)+31):((32*x)+0)\\]"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - KEY \\[((32*x)+31):((32*x)+0)\\]: Cryptographic key, bits \\[((32*x)+31):((32*x)+0)\\]"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - KEY \\[((32*x)+31):((32*x)+0)\\]: Cryptographic key, bits \\[((32*x)+31):((32*x)+0)\\]"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, AesKeyr0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "AES_KEYRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_keyr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKeyr0Spec;
impl crate::RegisterSpec for AesKeyr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_keyr0::R`](R) reader structure"]
impl crate::Readable for AesKeyr0Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_keyr0::W`](W) writer structure"]
impl crate::Writable for AesKeyr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_KEYR0 to value 0"]
impl crate::Resettable for AesKeyr0Spec {}
