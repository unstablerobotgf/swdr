#[doc = "Register `PA_REG` reader"]
pub type R = crate::R<PaRegSpec>;
#[doc = "Register `PA_REG` writer"]
pub type W = crate::W<PaRegSpec>;
#[doc = "Field `CFG_FILT` reader - FIR configuration:"]
pub type CfgFiltR = crate::FieldReader;
#[doc = "Field `CFG_FILT` writer - FIR configuration:"]
pub type CfgFiltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA_DEGEN_ON` reader - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve."]
pub type PaDegenOnR = crate::BitReader;
#[doc = "Field `PA_DEGEN_ON` writer - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve."]
pub type PaDegenOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIR configuration:"]
    #[inline(always)]
    pub fn cfg_filt(&self) -> CfgFiltR {
        CfgFiltR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve."]
    #[inline(always)]
    pub fn pa_degen_on(&self) -> PaDegenOnR {
        PaDegenOnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIR configuration:"]
    #[inline(always)]
    pub fn cfg_filt(&mut self) -> CfgFiltW<'_, PaRegSpec> {
        CfgFiltW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve."]
    #[inline(always)]
    pub fn pa_degen_on(&mut self) -> PaDegenOnW<'_, PaRegSpec> {
        PaDegenOnW::new(self, 3)
    }
}
#[doc = "PA_REG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaRegSpec;
impl crate::RegisterSpec for PaRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_reg::R`](R) reader structure"]
impl crate::Readable for PaRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_reg::W`](W) writer structure"]
impl crate::Writable for PaRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_REG to value 0"]
impl crate::Resettable for PaRegSpec {}
