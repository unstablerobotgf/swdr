#[doc = "Register `PA_LEVEL_3_0` reader"]
pub type R = crate::R<PaLevel3_0Spec>;
#[doc = "Register `PA_LEVEL_3_0` writer"]
pub type W = crate::W<PaLevel3_0Spec>;
#[doc = "Field `PA_LEVEL0` reader - Output power level for first step"]
pub type PaLevel0R = crate::FieldReader;
#[doc = "Field `PA_LEVEL0` writer - Output power level for first step"]
pub type PaLevel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_LEVEL1` reader - Output power level for second step"]
pub type PaLevel1R = crate::FieldReader;
#[doc = "Field `PA_LEVEL1` writer - Output power level for second step"]
pub type PaLevel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_LEVEL2` reader - Output power level for third step"]
pub type PaLevel2R = crate::FieldReader;
#[doc = "Field `PA_LEVEL2` writer - Output power level for third step"]
pub type PaLevel2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_LEVEL3` reader - Output power level for fourth step"]
pub type PaLevel3R = crate::FieldReader;
#[doc = "Field `PA_LEVEL3` writer - Output power level for fourth step"]
pub type PaLevel3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Output power level for first step"]
    #[inline(always)]
    pub fn pa_level0(&self) -> PaLevel0R {
        PaLevel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output power level for second step"]
    #[inline(always)]
    pub fn pa_level1(&self) -> PaLevel1R {
        PaLevel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output power level for third step"]
    #[inline(always)]
    pub fn pa_level2(&self) -> PaLevel2R {
        PaLevel2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output power level for fourth step"]
    #[inline(always)]
    pub fn pa_level3(&self) -> PaLevel3R {
        PaLevel3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output power level for first step"]
    #[inline(always)]
    pub fn pa_level0(&mut self) -> PaLevel0W<'_, PaLevel3_0Spec> {
        PaLevel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Output power level for second step"]
    #[inline(always)]
    pub fn pa_level1(&mut self) -> PaLevel1W<'_, PaLevel3_0Spec> {
        PaLevel1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Output power level for third step"]
    #[inline(always)]
    pub fn pa_level2(&mut self) -> PaLevel2W<'_, PaLevel3_0Spec> {
        PaLevel2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Output power level for fourth step"]
    #[inline(always)]
    pub fn pa_level3(&mut self) -> PaLevel3W<'_, PaLevel3_0Spec> {
        PaLevel3W::new(self, 24)
    }
}
#[doc = "PA_LEVEL_3_0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_level_3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaLevel3_0Spec;
impl crate::RegisterSpec for PaLevel3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_level_3_0::R`](R) reader structure"]
impl crate::Readable for PaLevel3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`pa_level_3_0::W`](W) writer structure"]
impl crate::Writable for PaLevel3_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_LEVEL_3_0 to value 0x230b_0100"]
impl crate::Resettable for PaLevel3_0Spec {
    const RESET_VALUE: u32 = 0x230b_0100;
}
