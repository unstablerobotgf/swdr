#[doc = "Register `DATABUFFER_SIZE` reader"]
pub type R = crate::R<DatabufferSizeSpec>;
#[doc = "Register `DATABUFFER_SIZE` writer"]
pub type W = crate::W<DatabufferSizeSpec>;
#[doc = "Field `DATABUFFER_SIZE` reader - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit."]
pub type DatabufferSizeR = crate::FieldReader<u16>;
#[doc = "Field `DATABUFFER_SIZE` writer - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit."]
pub type DatabufferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit."]
    #[inline(always)]
    pub fn databuffer_size(&self) -> DatabufferSizeR {
        DatabufferSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit."]
    #[inline(always)]
    pub fn databuffer_size(&mut self) -> DatabufferSizeW<'_, DatabufferSizeSpec> {
        DatabufferSizeW::new(self, 0)
    }
}
#[doc = "DATABUFFER_SIZE register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufferSizeSpec;
impl crate::RegisterSpec for DatabufferSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databuffer_size::R`](R) reader structure"]
impl crate::Readable for DatabufferSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`databuffer_size::W`](W) writer structure"]
impl crate::Writable for DatabufferSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATABUFFER_SIZE to value 0"]
impl crate::Resettable for DatabufferSizeSpec {}
