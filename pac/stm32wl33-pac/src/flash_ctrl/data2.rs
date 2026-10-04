#[doc = "Register `DATA2` reader"]
pub type R = crate::R<Data2Spec>;
#[doc = "Register `DATA2` writer"]
pub type W = crate::W<Data2Spec>;
#[doc = "Field `DATA2` reader - Value to be used as DATA for any COMMAND of type WRITE"]
pub type Data2R = crate::FieldReader<u32>;
#[doc = "Field `DATA2` writer - Value to be used as DATA for any COMMAND of type WRITE"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value to be used as DATA for any COMMAND of type WRITE"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, Data2Spec> {
        Data2W::new(self, 0)
    }
}
#[doc = "DATA2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data2Spec;
impl crate::RegisterSpec for Data2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for Data2Spec {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for Data2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA2 to value 0xffff_ffff"]
impl crate::Resettable for Data2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
