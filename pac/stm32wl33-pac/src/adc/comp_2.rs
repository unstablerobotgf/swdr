#[doc = "Register `COMP_2` reader"]
pub type R = crate::R<Comp2Spec>;
#[doc = "Register `COMP_2` writer"]
pub type W = crate::W<Comp2Spec>;
#[doc = "Field `GAIN2` reader - GAIN2\\[11:0\\]: second calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain2R = crate::FieldReader<u16>;
#[doc = "Field `GAIN2` writer - GAIN2\\[11:0\\]: second calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET2` reader - OFFSET2\\[7:0\\]: second calibration point"]
pub type Offset2R = crate::FieldReader;
#[doc = "Field `OFFSET2` writer - OFFSET2\\[7:0\\]: second calibration point"]
pub type Offset2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - GAIN2\\[11:0\\]: second calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain2(&self) -> Gain2R {
        Gain2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - OFFSET2\\[7:0\\]: second calibration point"]
    #[inline(always)]
    pub fn offset2(&self) -> Offset2R {
        Offset2R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - GAIN2\\[11:0\\]: second calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain2(&mut self) -> Gain2W<'_, Comp2Spec> {
        Gain2W::new(self, 0)
    }
    #[doc = "Bits 12:19 - OFFSET2\\[7:0\\]: second calibration point"]
    #[inline(always)]
    pub fn offset2(&mut self) -> Offset2W<'_, Comp2Spec> {
        Offset2W::new(self, 12)
    }
}
#[doc = "COMP_2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2Spec;
impl crate::RegisterSpec for Comp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_2::R`](R) reader structure"]
impl crate::Readable for Comp2Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_2::W`](W) writer structure"]
impl crate::Writable for Comp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_2 to value 0x0555"]
impl crate::Resettable for Comp2Spec {
    const RESET_VALUE: u32 = 0x0555;
}
