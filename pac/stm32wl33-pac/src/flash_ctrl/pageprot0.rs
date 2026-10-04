#[doc = "Register `PAGEPROT0` reader"]
pub type R = crate::R<Pageprot0Spec>;
#[doc = "Register `PAGEPROT0` writer"]
pub type W = crate::W<Pageprot0Spec>;
#[doc = "Field `SEGSIZE0` reader - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize0R = crate::FieldReader;
#[doc = "Field `SEGSIZE0` writer - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEGOFFSET0` reader - First segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset0R = crate::FieldReader;
#[doc = "Field `SEGOFFSET0` writer - First segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEGSIZE1` reader - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize1R = crate::FieldReader;
#[doc = "Field `SEGSIZE1` writer - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
pub type Segsize1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEGOFFSET1` reader - Second segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset1R = crate::FieldReader;
#[doc = "Field `SEGOFFSET1` writer - Second segment, 7-bit page protection offset (first page number in protected segment)"]
pub type Segoffset1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize0(&self) -> Segsize0R {
        Segsize0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - First segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset0(&self) -> Segoffset0R {
        Segoffset0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize1(&self) -> Segsize1R {
        Segsize1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Second segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset1(&self) -> Segoffset1R {
        Segoffset1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize0(&mut self) -> Segsize0W<'_, Pageprot0Spec> {
        Segsize0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - First segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset0(&mut self) -> Segoffset0W<'_, Pageprot0Spec> {
        Segoffset0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)"]
    #[inline(always)]
    pub fn segsize1(&mut self) -> Segsize1W<'_, Pageprot0Spec> {
        Segsize1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Second segment, 7-bit page protection offset (first page number in protected segment)"]
    #[inline(always)]
    pub fn segoffset1(&mut self) -> Segoffset1W<'_, Pageprot0Spec> {
        Segoffset1W::new(self, 24)
    }
}
#[doc = "PAGEPROT0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pageprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pageprot0Spec;
impl crate::RegisterSpec for Pageprot0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pageprot0::R`](R) reader structure"]
impl crate::Readable for Pageprot0Spec {}
#[doc = "`write(|w| ..)` method takes [`pageprot0::W`](W) writer structure"]
impl crate::Writable for Pageprot0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAGEPROT0 to value 0"]
impl crate::Resettable for Pageprot0Spec {}
