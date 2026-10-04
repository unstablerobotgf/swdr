#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `ID0` reader - ID0: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id0R = crate::BitReader;
#[doc = "Field `ID1` reader - ID1: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id1R = crate::BitReader;
#[doc = "Field `ID2` reader - ID2: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id2R = crate::BitReader;
#[doc = "Field `ID3` reader - ID3: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id3R = crate::BitReader;
#[doc = "Field `ID4` reader - ID4: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id4R = crate::BitReader;
#[doc = "Field `ID5` reader - ID5: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id5R = crate::BitReader;
#[doc = "Field `ID6` reader - ID6: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id6R = crate::BitReader;
#[doc = "Field `ID7` reader - ID7: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id7R = crate::BitReader;
#[doc = "Field `ID8` reader - ID8: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id8R = crate::BitReader;
#[doc = "Field `ID9` reader - ID9: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id9R = crate::BitReader;
#[doc = "Field `ID10` reader - ID10: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id10R = crate::BitReader;
#[doc = "Field `ID11` reader - ID11: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id11R = crate::BitReader;
#[doc = "Field `ID12` reader - ID12: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id12R = crate::BitReader;
#[doc = "Field `ID13` reader - ID13: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id13R = crate::BitReader;
#[doc = "Field `ID14` reader - ID14: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id14R = crate::BitReader;
#[doc = "Field `ID15` reader - ID15: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
pub type Id15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ID0: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id0(&self) -> Id0R {
        Id0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ID1: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id1(&self) -> Id1R {
        Id1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ID2: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id2(&self) -> Id2R {
        Id2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ID3: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id3(&self) -> Id3R {
        Id3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ID4: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id4(&self) -> Id4R {
        Id4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ID5: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id5(&self) -> Id5R {
        Id5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ID6: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id6(&self) -> Id6R {
        Id6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ID7: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id7(&self) -> Id7R {
        Id7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID8: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id8(&self) -> Id8R {
        Id8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ID9: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id9(&self) -> Id9R {
        Id9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ID10: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id10(&self) -> Id10R {
        Id10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ID11: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id11(&self) -> Id11R {
        Id11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ID12: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id12(&self) -> Id12R {
        Id12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ID13: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id13(&self) -> Id13R {
        Id13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ID14: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id14(&self) -> Id14R {
        Id14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ID15: Port A input data bit.These bits are read-only. They contain the input value of the corresponding I/O port"]
    #[inline(always)]
    pub fn id15(&self) -> Id15R {
        Id15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "IDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
