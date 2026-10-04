#[doc = "Register `DATABUFFER0_PTR` reader"]
pub type R = crate::R<Databuffer0PtrSpec>;
#[doc = "Register `DATABUFFER0_PTR` writer"]
pub type W = crate::W<Databuffer0PtrSpec>;
#[doc = "Field `DATABUFFER0_PTR` reader - Start address to be used by the Data Buffer0"]
pub type Databuffer0PtrR = crate::FieldReader<u32>;
#[doc = "Field `DATABUFFER0_PTR` writer - Start address to be used by the Data Buffer0"]
pub type Databuffer0PtrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start address to be used by the Data Buffer0"]
    #[inline(always)]
    pub fn databuffer0_ptr(&self) -> Databuffer0PtrR {
        Databuffer0PtrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start address to be used by the Data Buffer0"]
    #[inline(always)]
    pub fn databuffer0_ptr(&mut self) -> Databuffer0PtrW<'_, Databuffer0PtrSpec> {
        Databuffer0PtrW::new(self, 2)
    }
}
#[doc = "DATABUFFER0_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer0_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer0_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Databuffer0PtrSpec;
impl crate::RegisterSpec for Databuffer0PtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databuffer0_ptr::R`](R) reader structure"]
impl crate::Readable for Databuffer0PtrSpec {}
#[doc = "`write(|w| ..)` method takes [`databuffer0_ptr::W`](W) writer structure"]
impl crate::Writable for Databuffer0PtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATABUFFER0_PTR to value 0"]
impl crate::Resettable for Databuffer0PtrSpec {}
