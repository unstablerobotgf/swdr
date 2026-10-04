#[doc = "Register `LCSC_ANATST_CFG` reader"]
pub type R = crate::R<LcscAnatstCfgSpec>;
#[doc = "Register `LCSC_ANATST_CFG` writer"]
pub type W = crate::W<LcscAnatstCfgSpec>;
#[doc = "Field `VCMBUFF_ENOUT_SEL` reader - Selection of the signal to be used to supply the DAC in the LCSC"]
pub type VcmbuffEnoutSelR = crate::BitReader;
#[doc = "Field `VCMBUFF_ENOUT_SEL` writer - Selection of the signal to be used to supply the DAC in the LCSC"]
pub type VcmbuffEnoutSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMBUFF_ENOUT` reader - VCMBUFFER output buffer enable pin"]
pub type VcmbuffEnoutR = crate::BitReader;
#[doc = "Field `VCMBUFF_ENOUT` writer - VCMBUFFER output buffer enable pin"]
pub type VcmbuffEnoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMBUFF_PWDN_SEL` reader - Selection of the signal to be used to supply the DAC in the LCSC"]
pub type VcmbuffPwdnSelR = crate::BitReader;
#[doc = "Field `VCMBUFF_PWDN_SEL` writer - Selection of the signal to be used to supply the DAC in the LCSC"]
pub type VcmbuffPwdnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMBUFF_PWDN` reader - VCMBUFF power-down pin"]
pub type VcmbuffPwdnR = crate::BitReader;
#[doc = "Field `VCMBUFF_PWDN` writer - VCMBUFF power-down pin"]
pub type VcmbuffPwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_PWDN_SEL` reader - Selection of the signal to be used to supply the COMP in the LCSC Analog part"]
pub type CompPwdnSelR = crate::BitReader;
#[doc = "Field `COMP_PWDN_SEL` writer - Selection of the signal to be used to supply the COMP in the LCSC Analog part"]
pub type CompPwdnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_PWDN` reader - COMP power-down pin"]
pub type CompPwdnR = crate::BitReader;
#[doc = "Field `COMP_PWDN` writer - COMP power-down pin"]
pub type CompPwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_PWDN_SEL` reader - Selection of the signal to be used to supply the DAC in the LCSC Analog part"]
pub type DacPwdnSelR = crate::BitReader;
#[doc = "Field `DAC_PWDN_SEL` writer - Selection of the signal to be used to supply the DAC in the LCSC Analog part"]
pub type DacPwdnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_PWDN` reader - DAC power-down pin"]
pub type DacPwdnR = crate::BitReader;
#[doc = "Field `DAC_PWDN` writer - DAC power-down pin"]
pub type DacPwdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selection of the signal to be used to supply the DAC in the LCSC"]
    #[inline(always)]
    pub fn vcmbuff_enout_sel(&self) -> VcmbuffEnoutSelR {
        VcmbuffEnoutSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCMBUFFER output buffer enable pin"]
    #[inline(always)]
    pub fn vcmbuff_enout(&self) -> VcmbuffEnoutR {
        VcmbuffEnoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selection of the signal to be used to supply the DAC in the LCSC"]
    #[inline(always)]
    pub fn vcmbuff_pwdn_sel(&self) -> VcmbuffPwdnSelR {
        VcmbuffPwdnSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VCMBUFF power-down pin"]
    #[inline(always)]
    pub fn vcmbuff_pwdn(&self) -> VcmbuffPwdnR {
        VcmbuffPwdnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selection of the signal to be used to supply the COMP in the LCSC Analog part"]
    #[inline(always)]
    pub fn comp_pwdn_sel(&self) -> CompPwdnSelR {
        CompPwdnSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMP power-down pin"]
    #[inline(always)]
    pub fn comp_pwdn(&self) -> CompPwdnR {
        CompPwdnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selection of the signal to be used to supply the DAC in the LCSC Analog part"]
    #[inline(always)]
    pub fn dac_pwdn_sel(&self) -> DacPwdnSelR {
        DacPwdnSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC power-down pin"]
    #[inline(always)]
    pub fn dac_pwdn(&self) -> DacPwdnR {
        DacPwdnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of the signal to be used to supply the DAC in the LCSC"]
    #[inline(always)]
    pub fn vcmbuff_enout_sel(&mut self) -> VcmbuffEnoutSelW<'_, LcscAnatstCfgSpec> {
        VcmbuffEnoutSelW::new(self, 0)
    }
    #[doc = "Bit 1 - VCMBUFFER output buffer enable pin"]
    #[inline(always)]
    pub fn vcmbuff_enout(&mut self) -> VcmbuffEnoutW<'_, LcscAnatstCfgSpec> {
        VcmbuffEnoutW::new(self, 1)
    }
    #[doc = "Bit 2 - Selection of the signal to be used to supply the DAC in the LCSC"]
    #[inline(always)]
    pub fn vcmbuff_pwdn_sel(&mut self) -> VcmbuffPwdnSelW<'_, LcscAnatstCfgSpec> {
        VcmbuffPwdnSelW::new(self, 2)
    }
    #[doc = "Bit 3 - VCMBUFF power-down pin"]
    #[inline(always)]
    pub fn vcmbuff_pwdn(&mut self) -> VcmbuffPwdnW<'_, LcscAnatstCfgSpec> {
        VcmbuffPwdnW::new(self, 3)
    }
    #[doc = "Bit 4 - Selection of the signal to be used to supply the COMP in the LCSC Analog part"]
    #[inline(always)]
    pub fn comp_pwdn_sel(&mut self) -> CompPwdnSelW<'_, LcscAnatstCfgSpec> {
        CompPwdnSelW::new(self, 4)
    }
    #[doc = "Bit 5 - COMP power-down pin"]
    #[inline(always)]
    pub fn comp_pwdn(&mut self) -> CompPwdnW<'_, LcscAnatstCfgSpec> {
        CompPwdnW::new(self, 5)
    }
    #[doc = "Bit 6 - Selection of the signal to be used to supply the DAC in the LCSC Analog part"]
    #[inline(always)]
    pub fn dac_pwdn_sel(&mut self) -> DacPwdnSelW<'_, LcscAnatstCfgSpec> {
        DacPwdnSelW::new(self, 6)
    }
    #[doc = "Bit 7 - DAC power-down pin"]
    #[inline(always)]
    pub fn dac_pwdn(&mut self) -> DacPwdnW<'_, LcscAnatstCfgSpec> {
        DacPwdnW::new(self, 7)
    }
}
#[doc = "LCSC ANA Test Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_anatst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_anatst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscAnatstCfgSpec;
impl crate::RegisterSpec for LcscAnatstCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_anatst_cfg::R`](R) reader structure"]
impl crate::Readable for LcscAnatstCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_anatst_cfg::W`](W) writer structure"]
impl crate::Writable for LcscAnatstCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_ANATST_CFG to value 0"]
impl crate::Resettable for LcscAnatstCfgSpec {}
