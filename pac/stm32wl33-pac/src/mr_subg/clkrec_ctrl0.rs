#[doc = "Register `CLKREC_CTRL0` reader"]
pub type R = crate::R<ClkrecCtrl0Spec>;
#[doc = "Register `CLKREC_CTRL0` writer"]
pub type W = crate::W<ClkrecCtrl0Spec>;
#[doc = "Field `CLKREC_I_GAIN_FAST` reader - Integral fast gain for the clock recovery loop (PLL mode only)"]
pub type ClkrecIGainFastR = crate::FieldReader;
#[doc = "Field `CLKREC_I_GAIN_FAST` writer - Integral fast gain for the clock recovery loop (PLL mode only)"]
pub type ClkrecIGainFastW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKREC_P_GAIN_FAST` reader - Clock recovery fast loop gain (log2)"]
pub type ClkrecPGainFastR = crate::FieldReader;
#[doc = "Field `CLKREC_P_GAIN_FAST` writer - Clock recovery fast loop gain (log2)"]
pub type ClkrecPGainFastW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PSTFLT_LEN` reader - Control the length of the demodulator post-filter"]
pub type PstfltLenR = crate::BitReader;
#[doc = "Field `PSTFLT_LEN` writer - Control the length of the demodulator post-filter"]
pub type PstfltLenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Integral fast gain for the clock recovery loop (PLL mode only)"]
    #[inline(always)]
    pub fn clkrec_i_gain_fast(&self) -> ClkrecIGainFastR {
        ClkrecIGainFastR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Clock recovery fast loop gain (log2)"]
    #[inline(always)]
    pub fn clkrec_p_gain_fast(&self) -> ClkrecPGainFastR {
        ClkrecPGainFastR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Control the length of the demodulator post-filter"]
    #[inline(always)]
    pub fn pstflt_len(&self) -> PstfltLenR {
        PstfltLenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Integral fast gain for the clock recovery loop (PLL mode only)"]
    #[inline(always)]
    pub fn clkrec_i_gain_fast(&mut self) -> ClkrecIGainFastW<'_, ClkrecCtrl0Spec> {
        ClkrecIGainFastW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Clock recovery fast loop gain (log2)"]
    #[inline(always)]
    pub fn clkrec_p_gain_fast(&mut self) -> ClkrecPGainFastW<'_, ClkrecCtrl0Spec> {
        ClkrecPGainFastW::new(self, 4)
    }
    #[doc = "Bit 7 - Control the length of the demodulator post-filter"]
    #[inline(always)]
    pub fn pstflt_len(&mut self) -> PstfltLenW<'_, ClkrecCtrl0Spec> {
        PstfltLenW::new(self, 7)
    }
}
#[doc = "CLKREC_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkrec_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkrec_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkrecCtrl0Spec;
impl crate::RegisterSpec for ClkrecCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkrec_ctrl0::R`](R) reader structure"]
impl crate::Readable for ClkrecCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`clkrec_ctrl0::W`](W) writer structure"]
impl crate::Writable for ClkrecCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKREC_CTRL0 to value 0xb8"]
impl crate::Resettable for ClkrecCtrl0Spec {
    const RESET_VALUE: u32 = 0xb8;
}
