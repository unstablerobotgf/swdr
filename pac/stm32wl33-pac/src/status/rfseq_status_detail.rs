#[doc = "Register `RFSEQ_STATUS_DETAIL` reader"]
pub type R = crate::R<RfseqStatusDetailSpec>;
#[doc = "Register `RFSEQ_STATUS_DETAIL` writer"]
pub type W = crate::W<RfseqStatusDetailSpec>;
#[doc = "Field `DBM_FIFO_ERROR_F` reader - Data Buffer Manager internal FIFO overflow/underflow flag."]
pub type DbmFifoErrorFR = crate::BitReader;
#[doc = "Field `DBM_FIFO_ERROR_F` writer - Data Buffer Manager internal FIFO overflow/underflow flag."]
pub type DbmFifoErrorFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_LOCK_FAIL_F` reader - PLL lock fail status flag"]
pub type PllLockFailFR = crate::BitReader;
#[doc = "Field `PLL_LOCK_FAIL_F` writer - PLL lock fail status flag"]
pub type PllLockFailFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_UNLOCK_F` reader - PLL unlock event flag"]
pub type PllUnlockFR = crate::BitReader;
#[doc = "Field `PLL_UNLOCK_F` writer - PLL unlock event flag"]
pub type PllUnlockFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_CALFREQ_ERROR_F` reader - VCO frequency calibration error flag"]
pub type PllCalfreqErrorFR = crate::BitReader;
#[doc = "Field `PLL_CALFREQ_ERROR_F` writer - VCO frequency calibration error flag"]
pub type PllCalfreqErrorFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_CALAMP_ERROR_F` reader - VCO amplitude calibration error flag"]
pub type PllCalampErrorFR = crate::BitReader;
#[doc = "Field `PLL_CALAMP_ERROR_F` writer - VCO amplitude calibration error flag"]
pub type PllCalampErrorFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_ACTIONTIMEOUT_F` reader - The Sequencer has ended because the current SeqAction reached its ActionTimeout."]
pub type SeqActiontimeoutFR = crate::BitReader;
#[doc = "Field `SEQ_ACTIONTIMEOUT_F` writer - The Sequencer has ended because the current SeqAction reached its ActionTimeout."]
pub type SeqActiontimeoutFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_COMPLETE_F` reader - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)"]
pub type SeqCompleteFR = crate::BitReader;
#[doc = "Field `SEQ_COMPLETE_F` writer - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)"]
pub type SeqCompleteFW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Data Buffer Manager internal FIFO overflow/underflow flag."]
    #[inline(always)]
    pub fn dbm_fifo_error_f(&self) -> DbmFifoErrorFR {
        DbmFifoErrorFR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL lock fail status flag"]
    #[inline(always)]
    pub fn pll_lock_fail_f(&self) -> PllLockFailFR {
        PllLockFailFR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL unlock event flag"]
    #[inline(always)]
    pub fn pll_unlock_f(&self) -> PllUnlockFR {
        PllUnlockFR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VCO frequency calibration error flag"]
    #[inline(always)]
    pub fn pll_calfreq_error_f(&self) -> PllCalfreqErrorFR {
        PllCalfreqErrorFR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VCO amplitude calibration error flag"]
    #[inline(always)]
    pub fn pll_calamp_error_f(&self) -> PllCalampErrorFR {
        PllCalampErrorFR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - The Sequencer has ended because the current SeqAction reached its ActionTimeout."]
    #[inline(always)]
    pub fn seq_actiontimeout_f(&self) -> SeqActiontimeoutFR {
        SeqActiontimeoutFR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)"]
    #[inline(always)]
    pub fn seq_complete_f(&self) -> SeqCompleteFR {
        SeqCompleteFR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Data Buffer Manager internal FIFO overflow/underflow flag."]
    #[inline(always)]
    pub fn dbm_fifo_error_f(&mut self) -> DbmFifoErrorFW<'_, RfseqStatusDetailSpec> {
        DbmFifoErrorFW::new(self, 5)
    }
    #[doc = "Bit 8 - PLL lock fail status flag"]
    #[inline(always)]
    pub fn pll_lock_fail_f(&mut self) -> PllLockFailFW<'_, RfseqStatusDetailSpec> {
        PllLockFailFW::new(self, 8)
    }
    #[doc = "Bit 9 - PLL unlock event flag"]
    #[inline(always)]
    pub fn pll_unlock_f(&mut self) -> PllUnlockFW<'_, RfseqStatusDetailSpec> {
        PllUnlockFW::new(self, 9)
    }
    #[doc = "Bit 10 - VCO frequency calibration error flag"]
    #[inline(always)]
    pub fn pll_calfreq_error_f(&mut self) -> PllCalfreqErrorFW<'_, RfseqStatusDetailSpec> {
        PllCalfreqErrorFW::new(self, 10)
    }
    #[doc = "Bit 11 - VCO amplitude calibration error flag"]
    #[inline(always)]
    pub fn pll_calamp_error_f(&mut self) -> PllCalampErrorFW<'_, RfseqStatusDetailSpec> {
        PllCalampErrorFW::new(self, 11)
    }
    #[doc = "Bit 14 - The Sequencer has ended because the current SeqAction reached its ActionTimeout."]
    #[inline(always)]
    pub fn seq_actiontimeout_f(&mut self) -> SeqActiontimeoutFW<'_, RfseqStatusDetailSpec> {
        SeqActiontimeoutFW::new(self, 14)
    }
    #[doc = "Bit 15 - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)"]
    #[inline(always)]
    pub fn seq_complete_f(&mut self) -> SeqCompleteFW<'_, RfseqStatusDetailSpec> {
        SeqCompleteFW::new(self, 15)
    }
}
#[doc = "RFSEQ_STATUS_DETAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfseq_status_detail::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_status_detail::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfseqStatusDetailSpec;
impl crate::RegisterSpec for RfseqStatusDetailSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfseq_status_detail::R`](R) reader structure"]
impl crate::Readable for RfseqStatusDetailSpec {}
#[doc = "`write(|w| ..)` method takes [`rfseq_status_detail::W`](W) writer structure"]
impl crate::Writable for RfseqStatusDetailSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFSEQ_STATUS_DETAIL to value 0"]
impl crate::Resettable for RfseqStatusDetailSpec {}
