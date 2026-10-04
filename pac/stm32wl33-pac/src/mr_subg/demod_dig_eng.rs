#[doc = "Register `DEMOD_DIG_ENG` reader"]
pub type R = crate::R<DemodDigEngSpec>;
#[doc = "Register `DEMOD_DIG_ENG` writer"]
pub type W = crate::W<DemodDigEngSpec>;
#[doc = "Field `RX_BLANKING_LENGTH` reader - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:"]
pub type RxBlankingLengthR = crate::FieldReader;
#[doc = "Field `RX_BLANKING_LENGTH` writer - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:"]
pub type RxBlankingLengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:"]
    #[inline(always)]
    pub fn rx_blanking_length(&self) -> RxBlankingLengthR {
        RxBlankingLengthR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of data samples at RX start for which the signal at the output of the channel filter is kept forced to zero:"]
    #[inline(always)]
    pub fn rx_blanking_length(&mut self) -> RxBlankingLengthW<'_, DemodDigEngSpec> {
        RxBlankingLengthW::new(self, 0)
    }
}
#[doc = "DEMOD_DIG_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`demod_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demod_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DemodDigEngSpec;
impl crate::RegisterSpec for DemodDigEngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demod_dig_eng::R`](R) reader structure"]
impl crate::Readable for DemodDigEngSpec {}
#[doc = "`write(|w| ..)` method takes [`demod_dig_eng::W`](W) writer structure"]
impl crate::Writable for DemodDigEngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEMOD_DIG_ENG to value 0x03"]
impl crate::Resettable for DemodDigEngSpec {
    const RESET_VALUE: u32 = 0x03;
}
