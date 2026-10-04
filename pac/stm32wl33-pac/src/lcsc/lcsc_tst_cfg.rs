#[doc = "Register `LCSC_TST_CFG` reader"]
pub type R = crate::R<LcscTstCfgSpec>;
#[doc = "Register `LCSC_TST_CFG` writer"]
pub type W = crate::W<LcscTstCfgSpec>;
#[doc = "Field `TST_EN` reader - Test Enable"]
pub type TstEnR = crate::BitReader;
#[doc = "Field `TST_EN` writer - Test Enable"]
pub type TstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_CFG` reader - DTB output selection"]
pub type TstCfgR = crate::FieldReader;
#[doc = "Field `TST_CFG` writer - DTB output selection"]
pub type TstCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Test Enable"]
    #[inline(always)]
    pub fn tst_en(&self) -> TstEnR {
        TstEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DTB output selection"]
    #[inline(always)]
    pub fn tst_cfg(&self) -> TstCfgR {
        TstCfgR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test Enable"]
    #[inline(always)]
    pub fn tst_en(&mut self) -> TstEnW<'_, LcscTstCfgSpec> {
        TstEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - DTB output selection"]
    #[inline(always)]
    pub fn tst_cfg(&mut self) -> TstCfgW<'_, LcscTstCfgSpec> {
        TstCfgW::new(self, 1)
    }
}
#[doc = "LCSC Test Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_tst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_tst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscTstCfgSpec;
impl crate::RegisterSpec for LcscTstCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_tst_cfg::R`](R) reader structure"]
impl crate::Readable for LcscTstCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_tst_cfg::W`](W) writer structure"]
impl crate::Writable for LcscTstCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_TST_CFG to value 0"]
impl crate::Resettable for LcscTstCfgSpec {}
