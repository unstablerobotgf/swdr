#[doc = "Register `AF1` reader"]
pub type R = crate::R<Af1Spec>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<Af1Spec>;
#[doc = "Field `ETR_SEL` reader - ETRSEL\\[2:0\\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type EtrSelR = crate::FieldReader;
#[doc = "Field `ETR_SEL` writer - ETRSEL\\[2:0\\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type EtrSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETR_SEL_3` reader - ETRSEL\\[2:0\\]: External trigger source selection This field is not used in Blue51. Not available in IUM"]
pub type EtrSel3R = crate::BitReader;
#[doc = "Field `ETR_SEL_3` writer - ETRSEL\\[2:0\\]: External trigger source selection This field is not used in Blue51. Not available in IUM"]
pub type EtrSel3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 14:16 - ETRSEL\\[2:0\\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn etr_sel(&self) -> EtrSelR {
        EtrSelR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - ETRSEL\\[2:0\\]: External trigger source selection This field is not used in Blue51. Not available in IUM"]
    #[inline(always)]
    pub fn etr_sel_3(&self) -> EtrSel3R {
        EtrSel3R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 14:16 - ETRSEL\\[2:0\\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn etr_sel(&mut self) -> EtrSelW<'_, Af1Spec> {
        EtrSelW::new(self, 14)
    }
    #[doc = "Bit 17 - ETRSEL\\[2:0\\]: External trigger source selection This field is not used in Blue51. Not available in IUM"]
    #[inline(always)]
    pub fn etr_sel_3(&mut self) -> EtrSel3W<'_, Af1Spec> {
        EtrSel3W::new(self, 17)
    }
}
#[doc = "AF1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af1Spec;
impl crate::RegisterSpec for Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for Af1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for Af1Spec {}
