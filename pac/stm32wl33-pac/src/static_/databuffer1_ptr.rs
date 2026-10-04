#[doc = "Register `DATABUFFER1_PTR` reader"]
pub type R = crate::R<Databuffer1PtrSpec>;
#[doc = "Register `DATABUFFER1_PTR` writer"]
pub type W = crate::W<Databuffer1PtrSpec>;
#[doc = "Field `DATABUFFER1_PTR` reader - Start address to be used by the Data Buffer1"]
pub type Databuffer1PtrR = crate::FieldReader<u32>;
#[doc = "Field `DATABUFFER1_PTR` writer - Start address to be used by the Data Buffer1"]
pub type Databuffer1PtrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start address to be used by the Data Buffer1"]
    #[inline(always)]
    pub fn databuffer1_ptr(&self) -> Databuffer1PtrR {
        Databuffer1PtrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start address to be used by the Data Buffer1"]
    #[inline(always)]
    pub fn databuffer1_ptr(&mut self) -> Databuffer1PtrW<'_, Databuffer1PtrSpec> {
        Databuffer1PtrW::new(self, 2)
    }
}
#[doc = "DATABUFFER1_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer1_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer1_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Databuffer1PtrSpec;
impl crate::RegisterSpec for Databuffer1PtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databuffer1_ptr::R`](R) reader structure"]
impl crate::Readable for Databuffer1PtrSpec {}
#[doc = "`write(|w| ..)` method takes [`databuffer1_ptr::W`](W) writer structure"]
impl crate::Writable for Databuffer1PtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATABUFFER1_PTR to value 0"]
impl crate::Resettable for Databuffer1PtrSpec {}
