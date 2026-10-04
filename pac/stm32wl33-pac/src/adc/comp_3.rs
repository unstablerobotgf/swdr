#[doc = "Register `COMP_3` reader"]
pub type R = crate::R<Comp3Spec>;
#[doc = "Register `COMP_3` writer"]
pub type W = crate::W<Comp3Spec>;
#[doc = "Field `GAIN3` reader - GAIN3\\[11:0\\]: third calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain3R = crate::FieldReader<u16>;
#[doc = "Field `GAIN3` writer - GAIN3\\[11:0\\]: third calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET3` reader - OFFSET3\\[7:0\\]: third calibration point"]
pub type Offset3R = crate::FieldReader;
#[doc = "Field `OFFSET3` writer - OFFSET3\\[7:0\\]: third calibration point"]
pub type Offset3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - GAIN3\\[11:0\\]: third calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain3(&self) -> Gain3R {
        Gain3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - OFFSET3\\[7:0\\]: third calibration point"]
    #[inline(always)]
    pub fn offset3(&self) -> Offset3R {
        Offset3R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - GAIN3\\[11:0\\]: third calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain3(&mut self) -> Gain3W<'_, Comp3Spec> {
        Gain3W::new(self, 0)
    }
    #[doc = "Bits 12:19 - OFFSET3\\[7:0\\]: third calibration point"]
    #[inline(always)]
    pub fn offset3(&mut self) -> Offset3W<'_, Comp3Spec> {
        Offset3W::new(self, 12)
    }
}
#[doc = "COMP_3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp3Spec;
impl crate::RegisterSpec for Comp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_3::R`](R) reader structure"]
impl crate::Readable for Comp3Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_3::W`](W) writer structure"]
impl crate::Writable for Comp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_3 to value 0x0555"]
impl crate::Resettable for Comp3Spec {
    const RESET_VALUE: u32 = 0x0555;
}
