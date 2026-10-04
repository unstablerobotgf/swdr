#[doc = "Register `UNLOCK012` reader"]
pub type R = crate::R<Unlock012Spec>;
#[doc = "Register `UNLOCK012` writer"]
pub type W = crate::W<Unlock012Spec>;
#[doc = "Field `UNLOCK012` reader - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors"]
pub type Unlock012R = crate::FieldReader<u32>;
#[doc = "Field `UNLOCK012` writer - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors"]
pub type Unlock012W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors"]
    #[inline(always)]
    pub fn unlock012(&self) -> Unlock012R {
        Unlock012R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - (NOT TO BE DOCUMENTED) Remove read-write protection from IFR0, IFR1, IFR2 sectors"]
    #[inline(always)]
    pub fn unlock012(&mut self) -> Unlock012W<'_, Unlock012Spec> {
        Unlock012W::new(self, 0)
    }
}
#[doc = "UNLOCK012 register\n\nYou can [`read`](crate::Reg::read) this register and get [`unlock012::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock012::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unlock012Spec;
impl crate::RegisterSpec for Unlock012Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unlock012::R`](R) reader structure"]
impl crate::Readable for Unlock012Spec {}
#[doc = "`write(|w| ..)` method takes [`unlock012::W`](W) writer structure"]
impl crate::Writable for Unlock012Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK012 to value 0xffff_ffff"]
impl crate::Resettable for Unlock012Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
