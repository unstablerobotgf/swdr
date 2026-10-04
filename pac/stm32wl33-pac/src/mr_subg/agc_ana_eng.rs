#[doc = "Register `AGC_ANA_ENG` reader"]
pub type R = crate::R<AgcAnaEngSpec>;
#[doc = "Register `AGC_ANA_ENG` writer"]
pub type W = crate::W<AgcAnaEngSpec>;
#[doc = "Field `FORCE_AGC_GAINS` reader - Select the mode for AGC analog part:"]
pub type ForceAgcGainsR = crate::BitReader;
#[doc = "Field `FORCE_AGC_GAINS` writer - Select the mode for AGC analog part:"]
pub type ForceAgcGainsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFD_RX_ATTEN_AGCGAIN` reader - Attenuation at LNA level by step of 6dB with thermometric code:"]
pub type RfdRxAttenAgcgainR = crate::FieldReader;
#[doc = "Field `RFD_RX_ATTEN_AGCGAIN` writer - Attenuation at LNA level by step of 6dB with thermometric code:"]
pub type RfdRxAttenAgcgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RFD_RX_PGA_AGCGAIN` reader - Attenuation at PGA level by step of 6dB with binary code:"]
pub type RfdRxPgaAgcgainR = crate::FieldReader;
#[doc = "Field `RFD_RX_PGA_AGCGAIN` writer - Attenuation at PGA level by step of 6dB with binary code:"]
pub type RfdRxPgaAgcgainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Select the mode for AGC analog part:"]
    #[inline(always)]
    pub fn force_agc_gains(&self) -> ForceAgcGainsR {
        ForceAgcGainsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Attenuation at LNA level by step of 6dB with thermometric code:"]
    #[inline(always)]
    pub fn rfd_rx_atten_agcgain(&self) -> RfdRxAttenAgcgainR {
        RfdRxAttenAgcgainR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Attenuation at PGA level by step of 6dB with binary code:"]
    #[inline(always)]
    pub fn rfd_rx_pga_agcgain(&self) -> RfdRxPgaAgcgainR {
        RfdRxPgaAgcgainR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select the mode for AGC analog part:"]
    #[inline(always)]
    pub fn force_agc_gains(&mut self) -> ForceAgcGainsW<'_, AgcAnaEngSpec> {
        ForceAgcGainsW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Attenuation at LNA level by step of 6dB with thermometric code:"]
    #[inline(always)]
    pub fn rfd_rx_atten_agcgain(&mut self) -> RfdRxAttenAgcgainW<'_, AgcAnaEngSpec> {
        RfdRxAttenAgcgainW::new(self, 1)
    }
    #[doc = "Bits 5:7 - Attenuation at PGA level by step of 6dB with binary code:"]
    #[inline(always)]
    pub fn rfd_rx_pga_agcgain(&mut self) -> RfdRxPgaAgcgainW<'_, AgcAnaEngSpec> {
        RfdRxPgaAgcgainW::new(self, 5)
    }
}
#[doc = "AGC_ANA_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgcAnaEngSpec;
impl crate::RegisterSpec for AgcAnaEngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc_ana_eng::R`](R) reader structure"]
impl crate::Readable for AgcAnaEngSpec {}
#[doc = "`write(|w| ..)` method takes [`agc_ana_eng::W`](W) writer structure"]
impl crate::Writable for AgcAnaEngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC_ANA_ENG to value 0"]
impl crate::Resettable for AgcAnaEngSpec {}
