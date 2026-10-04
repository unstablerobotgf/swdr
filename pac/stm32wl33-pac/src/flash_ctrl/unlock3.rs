#[doc = "Register `UNLOCK3` reader"]
pub type R = crate::R<Unlock3Spec>;
#[doc = "Register `UNLOCK3` writer"]
pub type W = crate::W<Unlock3Spec>;
#[doc = "Field `UNLOCK3` reader - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector"]
pub type Unlock3R = crate::FieldReader<u32>;
#[doc = "Field `UNLOCK3` writer - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector"]
pub type Unlock3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector"]
    #[inline(always)]
    pub fn unlock3(&self) -> Unlock3R {
        Unlock3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR3 sector"]
    #[inline(always)]
    pub fn unlock3(&mut self) -> Unlock3W<'_, Unlock3Spec> {
        Unlock3W::new(self, 0)
    }
}
#[doc = "UNLOCK3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`unlock3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unlock3Spec;
impl crate::RegisterSpec for Unlock3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unlock3::R`](R) reader structure"]
impl crate::Readable for Unlock3Spec {}
#[doc = "`write(|w| ..)` method takes [`unlock3::W`](W) writer structure"]
impl crate::Writable for Unlock3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK3 to value 0xffff_ffff"]
impl crate::Resettable for Unlock3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
