#[doc = "Register `SEC_SYNC` reader"]
pub type R = crate::R<SecSyncSpec>;
#[doc = "Register `SEC_SYNC` writer"]
pub type W = crate::W<SecSyncSpec>;
#[doc = "Field `SEC_SYNC` reader - Secondary Synchro word."]
pub type SecSyncR = crate::FieldReader<u32>;
#[doc = "Field `SEC_SYNC` writer - Secondary Synchro word."]
pub type SecSyncW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secondary Synchro word."]
    #[inline(always)]
    pub fn sec_sync(&self) -> SecSyncR {
        SecSyncR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secondary Synchro word."]
    #[inline(always)]
    pub fn sec_sync(&mut self) -> SecSyncW<'_, SecSyncSpec> {
        SecSyncW::new(self, 0)
    }
}
#[doc = "SEC_SYNC register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSyncSpec;
impl crate::RegisterSpec for SecSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_sync::R`](R) reader structure"]
impl crate::Readable for SecSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_sync::W`](W) writer structure"]
impl crate::Writable for SecSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_SYNC to value 0"]
impl crate::Resettable for SecSyncSpec {}
