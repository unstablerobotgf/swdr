#[doc = "Register `ADDRESS` reader"]
pub type R = crate::R<AddressSpec>;
#[doc = "Register `ADDRESS` writer"]
pub type W = crate::W<AddressSpec>;
#[doc = "Field `YADDR` reader - Flash column address offset to be used with some COMMAND"]
pub type YaddrR = crate::FieldReader;
#[doc = "Field `YADDR` writer - Flash column address offset to be used with some COMMAND"]
pub type YaddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XADDR` reader - Flash row address offset to be used with some COMMAND"]
pub type XaddrR = crate::FieldReader<u16>;
#[doc = "Field `XADDR` writer - Flash row address offset to be used with some COMMAND"]
pub type XaddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - Flash column address offset to be used with some COMMAND"]
    #[inline(always)]
    pub fn yaddr(&self) -> YaddrR {
        YaddrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:15 - Flash row address offset to be used with some COMMAND"]
    #[inline(always)]
    pub fn xaddr(&self) -> XaddrR {
        XaddrR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Flash column address offset to be used with some COMMAND"]
    #[inline(always)]
    pub fn yaddr(&mut self) -> YaddrW<'_, AddressSpec> {
        YaddrW::new(self, 0)
    }
    #[doc = "Bits 6:15 - Flash row address offset to be used with some COMMAND"]
    #[inline(always)]
    pub fn xaddr(&mut self) -> XaddrW<'_, AddressSpec> {
        XaddrW::new(self, 6)
    }
}
#[doc = "ADDRESS register\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressSpec;
impl crate::RegisterSpec for AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for AddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRESS to value 0"]
impl crate::Resettable for AddressSpec {}
