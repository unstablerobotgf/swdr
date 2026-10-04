#[doc = "Register `LCSC_STAT` reader"]
pub type R = crate::R<LcscStatSpec>;
#[doc = "Register `LCSC_STAT` writer"]
pub type W = crate::W<LcscStatSpec>;
#[doc = "Field `MIN_LCAB_CNT` reader - The Minimum of CMP_LCA_CNT, CMP_LCB_CNT reached during the"]
pub type MinLcabCntR = crate::FieldReader;
#[doc = "Field `MAX_LCAB_CNT` reader - The Maximum of CMP_LCA_CNT, CMP_LCB_CNT reached during"]
pub type MaxLcabCntR = crate::FieldReader;
#[doc = "Field `MIN_LCAB_CNT_BOUND` reader - The Minimum bound of CMP_LCA_COUNT,"]
pub type MinLcabCntBoundR = crate::FieldReader;
#[doc = "Field `MIN_LCAB_CNT_BOUND` writer - The Minimum bound of CMP_LCA_COUNT,"]
pub type MinLcabCntBoundW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAX_LCAB_CNT_BOUND` reader - The Maximum bound of CMP_LCA_COUNT,"]
pub type MaxLcabCntBoundR = crate::FieldReader;
#[doc = "Field `MAX_LCAB_CNT_BOUND` writer - The Maximum bound of CMP_LCA_COUNT,"]
pub type MaxLcabCntBoundW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The Minimum of CMP_LCA_CNT, CMP_LCB_CNT reached during the"]
    #[inline(always)]
    pub fn min_lcab_cnt(&self) -> MinLcabCntR {
        MinLcabCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The Maximum of CMP_LCA_CNT, CMP_LCB_CNT reached during"]
    #[inline(always)]
    pub fn max_lcab_cnt(&self) -> MaxLcabCntR {
        MaxLcabCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The Minimum bound of CMP_LCA_COUNT,"]
    #[inline(always)]
    pub fn min_lcab_cnt_bound(&self) -> MinLcabCntBoundR {
        MinLcabCntBoundR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The Maximum bound of CMP_LCA_COUNT,"]
    #[inline(always)]
    pub fn max_lcab_cnt_bound(&self) -> MaxLcabCntBoundR {
        MaxLcabCntBoundR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - The Minimum bound of CMP_LCA_COUNT,"]
    #[inline(always)]
    pub fn min_lcab_cnt_bound(&mut self) -> MinLcabCntBoundW<'_, LcscStatSpec> {
        MinLcabCntBoundW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The Maximum bound of CMP_LCA_COUNT,"]
    #[inline(always)]
    pub fn max_lcab_cnt_bound(&mut self) -> MaxLcabCntBoundW<'_, LcscStatSpec> {
        MaxLcabCntBoundW::new(self, 24)
    }
}
#[doc = "LCSC_STAT register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscStatSpec;
impl crate::RegisterSpec for LcscStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_stat::R`](R) reader structure"]
impl crate::Readable for LcscStatSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_stat::W`](W) writer structure"]
impl crate::Writable for LcscStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_STAT to value 0xff00_00ff"]
impl crate::Resettable for LcscStatSpec {
    const RESET_VALUE: u32 = 0xff00_00ff;
}
