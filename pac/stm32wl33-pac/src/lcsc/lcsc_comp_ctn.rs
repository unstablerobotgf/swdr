#[doc = "Register `LCSC_COMP_CTN` reader"]
pub type R = crate::R<LcscCompCtnSpec>;
#[doc = "Field `CMP_LCA_CNT` reader - LCA Comparator last damping count"]
pub type CmpLcaCntR = crate::FieldReader;
#[doc = "Field `CMP_LCB_CNT` reader - LCB Comparator last damping count"]
pub type CmpLcbCntR = crate::FieldReader;
#[doc = "Field `CMP_LCT_CNT` reader - LCT Comparator last damping count"]
pub type CmpLctCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - LCA Comparator last damping count"]
    #[inline(always)]
    pub fn cmp_lca_cnt(&self) -> CmpLcaCntR {
        CmpLcaCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:17 - LCB Comparator last damping count"]
    #[inline(always)]
    pub fn cmp_lcb_cnt(&self) -> CmpLcbCntR {
        CmpLcbCntR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - LCT Comparator last damping count"]
    #[inline(always)]
    pub fn cmp_lct_cnt(&self) -> CmpLctCntR {
        CmpLctCntR::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[doc = "LCSC_COMP_CTN register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_comp_ctn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscCompCtnSpec;
impl crate::RegisterSpec for LcscCompCtnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_comp_ctn::R`](R) reader structure"]
impl crate::Readable for LcscCompCtnSpec {}
#[doc = "`reset()` method sets LCSC_COMP_CTN to value 0"]
impl crate::Resettable for LcscCompCtnSpec {}
