#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "Field `SYNC` reader - Synchro word."]
pub type SyncR = crate::FieldReader<u32>;
#[doc = "Field `SYNC` writer - Synchro word."]
pub type SyncW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchro word."]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchro word."]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, SyncSpec> {
        SyncW::new(self, 0)
    }
}
#[doc = "SYNC register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC to value 0x2323_2323"]
impl crate::Resettable for SyncSpec {
    const RESET_VALUE: u32 = 0x2323_2323;
}
