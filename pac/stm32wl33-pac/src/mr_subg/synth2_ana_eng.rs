#[doc = "Register `SYNTH2_ANA_ENG` reader"]
pub type R = crate::R<Synth2AnaEngSpec>;
#[doc = "Register `SYNTH2_ANA_ENG` writer"]
pub type W = crate::W<Synth2AnaEngSpec>;
#[doc = "Field `RFD_PLL_VCO_ALC_AMP` reader - Select the level of max VCO amplitude in amplitude level control loop."]
pub type RfdPllVcoAlcAmpR = crate::FieldReader;
#[doc = "Field `RFD_PLL_VCO_ALC_AMP` writer - Select the level of max VCO amplitude in amplitude level control loop."]
pub type RfdPllVcoAlcAmpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RFD_PLL_LD_WIN_ACC` reader - Select the PLL lock detector window selection:"]
pub type RfdPllLdWinAccR = crate::BitReader;
#[doc = "Field `RFD_PLL_LD_WIN_ACC` writer - Select the PLL lock detector window selection:"]
pub type RfdPllLdWinAccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Select the level of max VCO amplitude in amplitude level control loop."]
    #[inline(always)]
    pub fn rfd_pll_vco_alc_amp(&self) -> RfdPllVcoAlcAmpR {
        RfdPllVcoAlcAmpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Select the PLL lock detector window selection:"]
    #[inline(always)]
    pub fn rfd_pll_ld_win_acc(&self) -> RfdPllLdWinAccR {
        RfdPllLdWinAccR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select the level of max VCO amplitude in amplitude level control loop."]
    #[inline(always)]
    pub fn rfd_pll_vco_alc_amp(&mut self) -> RfdPllVcoAlcAmpW<'_, Synth2AnaEngSpec> {
        RfdPllVcoAlcAmpW::new(self, 0)
    }
    #[doc = "Bit 3 - Select the PLL lock detector window selection:"]
    #[inline(always)]
    pub fn rfd_pll_ld_win_acc(&mut self) -> RfdPllLdWinAccW<'_, Synth2AnaEngSpec> {
        RfdPllLdWinAccW::new(self, 3)
    }
}
#[doc = "SYNTH2_ANA_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`synth2_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synth2_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Synth2AnaEngSpec;
impl crate::RegisterSpec for Synth2AnaEngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synth2_ana_eng::R`](R) reader structure"]
impl crate::Readable for Synth2AnaEngSpec {}
#[doc = "`write(|w| ..)` method takes [`synth2_ana_eng::W`](W) writer structure"]
impl crate::Writable for Synth2AnaEngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNTH2_ANA_ENG to value 0x4c"]
impl crate::Resettable for Synth2AnaEngSpec {
    const RESET_VALUE: u32 = 0x4c;
}
