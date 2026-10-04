#[doc = "Register `PAGEPROT1` reader"]
pub type R = crate::R<Pageprot1Spec>;
#[doc = "Register `PAGEPROT1` writer"]
pub type W = crate::W<Pageprot1Spec>;
#[doc = "Field `SEGSIZE2` reader - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize2R = crate::FieldReader;
#[doc = "Field `SEGSIZE2` writer - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEGOFFSET2` reader - Third segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset2R = crate::FieldReader;
#[doc = "Field `SEGOFFSET2` writer - Third segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEGSIZE3` reader - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize3R = crate::FieldReader;
#[doc = "Field `SEGSIZE3` writer - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEGOFFSET3` reader - Fourth segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset3R = crate::FieldReader;
#[doc = "Field `SEGOFFSET3` writer - Fourth segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize2(&self) -> Segsize2R {
        Segsize2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Third segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset2(&self) -> Segoffset2R {
        Segoffset2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize3(&self) -> Segsize3R {
        Segsize3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Fourth segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset3(&self) -> Segoffset3R {
        Segoffset3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize2(&mut self) -> Segsize2W<'_, Pageprot1Spec> {
        Segsize2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Third segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset2(&mut self) -> Segoffset2W<'_, Pageprot1Spec> {
        Segoffset2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize3(&mut self) -> Segsize3W<'_, Pageprot1Spec> {
        Segsize3W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Fourth segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset3(&mut self) -> Segoffset3W<'_, Pageprot1Spec> {
        Segoffset3W::new(self, 24)
    }
}
#[doc = "PAGEPROT1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pageprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pageprot1Spec;
impl crate::RegisterSpec for Pageprot1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pageprot1::R`](R) reader structure"]
impl crate::Readable for Pageprot1Spec {}
#[doc = "`write(|w| ..)` method takes [`pageprot1::W`](W) writer structure"]
impl crate::Writable for Pageprot1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAGEPROT1 to value 0"]
impl crate::Resettable for Pageprot1Spec {}
