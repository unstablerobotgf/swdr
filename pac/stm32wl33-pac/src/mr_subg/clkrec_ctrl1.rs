#[doc = "Register `CLKREC_CTRL1` reader"]
pub type R = crate::R<ClkrecCtrl1Spec>;
#[doc = "Register `CLKREC_CTRL1` writer"]
pub type W = crate::W<ClkrecCtrl1Spec>;
#[doc = "Field `CLKREC_I_GAIN_SLOW` reader - Integral slow gain for the clock recovery loop (PLL mode only)"]
pub type ClkrecIGainSlowR = crate::FieldReader;
#[doc = "Field `CLKREC_I_GAIN_SLOW` writer - Integral slow gain for the clock recovery loop (PLL mode only)"]
pub type ClkrecIGainSlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKREC_P_GAIN_SLOW` reader - Clock recovery slow loop gain (log2)"]
pub type ClkrecPGainSlowR = crate::FieldReader;
#[doc = "Field `CLKREC_P_GAIN_SLOW` writer - Clock recovery slow loop gain (log2)"]
pub type ClkrecPGainSlowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKREC_ALGO_SEL` reader - Symbol timing recovery algorithm selection"]
pub type ClkrecAlgoSelR = crate::BitReader;
#[doc = "Field `CLKREC_ALGO_SEL` writer - Symbol timing recovery algorithm selection"]
pub type ClkrecAlgoSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Integral slow gain for the clock recovery loop (PLL mode only)"]
    #[inline(always)]
    pub fn clkrec_i_gain_slow(&self) -> ClkrecIGainSlowR {
        ClkrecIGainSlowR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Clock recovery slow loop gain (log2)"]
    #[inline(always)]
    pub fn clkrec_p_gain_slow(&self) -> ClkrecPGainSlowR {
        ClkrecPGainSlowR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Symbol timing recovery algorithm selection"]
    #[inline(always)]
    pub fn clkrec_algo_sel(&self) -> ClkrecAlgoSelR {
        ClkrecAlgoSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Integral slow gain for the clock recovery loop (PLL mode only)"]
    #[inline(always)]
    pub fn clkrec_i_gain_slow(&mut self) -> ClkrecIGainSlowW<'_, ClkrecCtrl1Spec> {
        ClkrecIGainSlowW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Clock recovery slow loop gain (log2)"]
    #[inline(always)]
    pub fn clkrec_p_gain_slow(&mut self) -> ClkrecPGainSlowW<'_, ClkrecCtrl1Spec> {
        ClkrecPGainSlowW::new(self, 4)
    }
    #[doc = "Bit 7 - Symbol timing recovery algorithm selection"]
    #[inline(always)]
    pub fn clkrec_algo_sel(&mut self) -> ClkrecAlgoSelW<'_, ClkrecCtrl1Spec> {
        ClkrecAlgoSelW::new(self, 7)
    }
}
#[doc = "CLKREC_CTRL1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkrecCtrl1Spec;
impl crate::RegisterSpec for ClkrecCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkrec_ctrl1::R`](R) reader structure"]
impl crate::Readable for ClkrecCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkrec_ctrl1::W`](W) writer structure"]
impl crate::Writable for ClkrecCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKREC_CTRL1 to value 0x5c"]
impl crate::Resettable for ClkrecCtrl1Spec {
    const RESET_VALUE: u32 = 0x5c;
}
