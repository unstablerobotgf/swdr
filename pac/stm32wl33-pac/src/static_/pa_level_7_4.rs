#[doc = "Register `PA_LEVEL_7_4` reader"]
pub type R = crate::R<PaLevel7_4Spec>;
#[doc = "Register `PA_LEVEL_7_4` writer"]
pub type W = crate::W<PaLevel7_4Spec>;
#[doc = "Field `PA_LEVEL4` reader - Output power level for fifth step"]
pub type PaLevel4R = crate::FieldReader;
#[doc = "Field `PA_LEVEL4` writer - Output power level for fifth step"]
pub type PaLevel4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_LEVEL5` reader - Output power level for sixth step"]
pub type PaLevel5R = crate::FieldReader;
#[doc = "Field `PA_LEVEL5` writer - Output power level for sixth step"]
pub type PaLevel5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_LEVEL6` reader - Output power level for seventh step"]
pub type PaLevel6R = crate::FieldReader;
#[doc = "Field `PA_LEVEL6` writer - Output power level for seventh step"]
pub type PaLevel6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_LEVEL7` reader - Output power level for eighth step"]
pub type PaLevel7R = crate::FieldReader;
#[doc = "Field `PA_LEVEL7` writer - Output power level for eighth step"]
pub type PaLevel7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Output power level for fifth step"]
    #[inline(always)]
    pub fn pa_level4(&self) -> PaLevel4R {
        PaLevel4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output power level for sixth step"]
    #[inline(always)]
    pub fn pa_level5(&self) -> PaLevel5R {
        PaLevel5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output power level for seventh step"]
    #[inline(always)]
    pub fn pa_level6(&self) -> PaLevel6R {
        PaLevel6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Output power level for eighth step"]
    #[inline(always)]
    pub fn pa_level7(&self) -> PaLevel7R {
        PaLevel7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Output power level for fifth step"]
    #[inline(always)]
    pub fn pa_level4(&mut self) -> PaLevel4W<'_, PaLevel7_4Spec> {
        PaLevel4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Output power level for sixth step"]
    #[inline(always)]
    pub fn pa_level5(&mut self) -> PaLevel5W<'_, PaLevel7_4Spec> {
        PaLevel5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Output power level for seventh step"]
    #[inline(always)]
    pub fn pa_level6(&mut self) -> PaLevel6W<'_, PaLevel7_4Spec> {
        PaLevel6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Output power level for eighth step"]
    #[inline(always)]
    pub fn pa_level7(&mut self) -> PaLevel7W<'_, PaLevel7_4Spec> {
        PaLevel7W::new(self, 24)
    }
}
#[doc = "PA_LEVEL_7_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_level_7_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_7_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaLevel7_4Spec;
impl crate::RegisterSpec for PaLevel7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_level_7_4::R`](R) reader structure"]
impl crate::Readable for PaLevel7_4Spec {}
#[doc = "`write(|w| ..)` method takes [`pa_level_7_4::W`](W) writer structure"]
impl crate::Writable for PaLevel7_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_LEVEL_7_4 to value 0x5147_3b2f"]
impl crate::Resettable for PaLevel7_4Spec {
    const RESET_VALUE: u32 = 0x5147_3b2f;
}
