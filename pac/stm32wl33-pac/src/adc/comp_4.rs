#[doc = "Register `COMP_4` reader"]
pub type R = crate::R<Comp4Spec>;
#[doc = "Register `COMP_4` writer"]
pub type W = crate::W<Comp4Spec>;
#[doc = "Field `GAIN4` reader - GAIN4\\[11:0\\]: fourth calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain4R = crate::FieldReader<u16>;
#[doc = "Field `GAIN4` writer - GAIN4\\[11:0\\]: fourth calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET4` reader - OFFSET4\\[7:0\\]: fourth calibration point"]
pub type Offset4R = crate::FieldReader;
#[doc = "Field `OFFSET4` writer - OFFSET4\\[7:0\\]: fourth calibration point"]
pub type Offset4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - GAIN4\\[11:0\\]: fourth calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain4(&self) -> Gain4R {
        Gain4R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - OFFSET4\\[7:0\\]: fourth calibration point"]
    #[inline(always)]
    pub fn offset4(&self) -> Offset4R {
        Offset4R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - GAIN4\\[11:0\\]: fourth calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain4(&mut self) -> Gain4W<'_, Comp4Spec> {
        Gain4W::new(self, 0)
    }
    #[doc = "Bits 12:19 - OFFSET4\\[7:0\\]: fourth calibration point"]
    #[inline(always)]
    pub fn offset4(&mut self) -> Offset4W<'_, Comp4Spec> {
        Offset4W::new(self, 12)
    }
}
#[doc = "COMP_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp4Spec;
impl crate::RegisterSpec for Comp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_4::R`](R) reader structure"]
impl crate::Readable for Comp4Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_4::W`](W) writer structure"]
impl crate::Writable for Comp4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_4 to value 0x0555"]
impl crate::Resettable for Comp4Spec {
    const RESET_VALUE: u32 = 0x0555;
}
