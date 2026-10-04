#[doc = "Register `COMP_1` reader"]
pub type R = crate::R<Comp1Spec>;
#[doc = "Register `COMP_1` writer"]
pub type W = crate::W<Comp1Spec>;
#[doc = "Field `GAIN1` reader - GAIN1\\[11:0\\]: first calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain1R = crate::FieldReader<u16>;
#[doc = "Field `GAIN1` writer - GAIN1\\[11:0\\]: first calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
pub type Gain1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET1` reader - OFFSET1\\[7:0\\]: first calibration point"]
pub type Offset1R = crate::FieldReader;
#[doc = "Field `OFFSET1` writer - OFFSET1\\[7:0\\]: first calibration point"]
pub type Offset1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - GAIN1\\[11:0\\]: first calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain1(&self) -> Gain1R {
        Gain1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - OFFSET1\\[7:0\\]: first calibration point"]
    #[inline(always)]
    pub fn offset1(&self) -> Offset1R {
        Offset1R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - GAIN1\\[11:0\\]: first calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]"]
    #[inline(always)]
    pub fn gain1(&mut self) -> Gain1W<'_, Comp1Spec> {
        Gain1W::new(self, 0)
    }
    #[doc = "Bits 12:19 - OFFSET1\\[7:0\\]: first calibration point"]
    #[inline(always)]
    pub fn offset1(&mut self) -> Offset1W<'_, Comp1Spec> {
        Offset1W::new(self, 12)
    }
}
#[doc = "COMP_1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1Spec;
impl crate::RegisterSpec for Comp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_1::R`](R) reader structure"]
impl crate::Readable for Comp1Spec {}
#[doc = "`write(|w| ..)` method takes [`comp_1::W`](W) writer structure"]
impl crate::Writable for Comp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_1 to value 0x0555"]
impl crate::Resettable for Comp1Spec {
    const RESET_VALUE: u32 = 0x0555;
}
