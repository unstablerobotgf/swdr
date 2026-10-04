#[doc = "Register `EXTSRR` reader"]
pub type R = crate::R<ExtsrrSpec>;
#[doc = "Register `EXTSRR` writer"]
pub type W = crate::W<ExtsrrSpec>;
#[doc = "Field `DEEPSTOPF` reader - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field - 0: System has not been in DEEPSTOP mode - 1: System has been in DEEPSTOP mode"]
pub type DeepstopfR = crate::BitReader;
#[doc = "Field `DEEPSTOPF` writer - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field - 0: System has not been in DEEPSTOP mode - 1: System has been in DEEPSTOP mode"]
pub type DeepstopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFPHASEF` reader - RFPHASEF RFPHASE Flag This bit is set by hardware after a S3LP wake-up event (S3LP activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP. - 0: RF IP does not require attention - 1: RF IP awake and requesting system attention"]
pub type RfphasefR = crate::BitReader;
#[doc = "Field `RFPHASEF` writer - RFPHASEF RFPHASE Flag This bit is set by hardware after a S3LP wake-up event (S3LP activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP. - 0: RF IP does not require attention - 1: RF IP awake and requesting system attention"]
pub type RfphasefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field - 0: System has not been in DEEPSTOP mode - 1: System has been in DEEPSTOP mode"]
    #[inline(always)]
    pub fn deepstopf(&self) -> DeepstopfR {
        DeepstopfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RFPHASEF RFPHASE Flag This bit is set by hardware after a S3LP wake-up event (S3LP activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP. - 0: RF IP does not require attention - 1: RF IP awake and requesting system attention"]
    #[inline(always)]
    pub fn rfphasef(&self) -> RfphasefR {
        RfphasefR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field - 0: System has not been in DEEPSTOP mode - 1: System has been in DEEPSTOP mode"]
    #[inline(always)]
    pub fn deepstopf(&mut self) -> DeepstopfW<'_, ExtsrrSpec> {
        DeepstopfW::new(self, 9)
    }
    #[doc = "Bit 10 - RFPHASEF RFPHASE Flag This bit is set by hardware after a S3LP wake-up event (S3LP activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP. - 0: RF IP does not require attention - 1: RF IP awake and requesting system attention"]
    #[inline(always)]
    pub fn rfphasef(&mut self) -> RfphasefW<'_, ExtsrrSpec> {
        RfphasefW::new(self, 10)
    }
}
#[doc = "EXTSRR register\n\nYou can [`read`](crate::Reg::read) this register and get [`extsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtsrrSpec;
impl crate::RegisterSpec for ExtsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extsrr::R`](R) reader structure"]
impl crate::Readable for ExtsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`extsrr::W`](W) writer structure"]
impl crate::Writable for ExtsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTSRR to value 0"]
impl crate::Resettable for ExtsrrSpec {}
