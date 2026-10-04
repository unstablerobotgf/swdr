#[doc = "Register `SINGEN_ANA_ENG` reader"]
pub type R = crate::R<SingenAnaEngSpec>;
#[doc = "Register `SINGEN_ANA_ENG` writer"]
pub type W = crate::W<SingenAnaEngSpec>;
#[doc = "Field `RFD_SINGEN_ENA` reader - Enable SINGEN signal for the RFSUBGanalog IP."]
pub type RfdSingenEnaR = crate::BitReader;
#[doc = "Field `RFD_SINGEN_ENA` writer - Enable SINGEN signal for the RFSUBGanalog IP."]
pub type RfdSingenEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFD_SINGEN_DIV2_PUP` reader - This bit value is directly connected to the RFSUBG analog IP pin."]
pub type RfdSingenDiv2PupR = crate::BitReader;
#[doc = "Field `RFD_SINGEN_DIV2_PUP` writer - This bit value is directly connected to the RFSUBG analog IP pin."]
pub type RfdSingenDiv2PupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFD_SINGEN_LBE` reader - This bit value is directly connected to the RFSUBG analog IP pin."]
pub type RfdSingenLbeR = crate::BitReader;
#[doc = "Field `RFD_SINGEN_LBE` writer - This bit value is directly connected to the RFSUBG analog IP pin."]
pub type RfdSingenLbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable SINGEN signal for the RFSUBGanalog IP."]
    #[inline(always)]
    pub fn rfd_singen_ena(&self) -> RfdSingenEnaR {
        RfdSingenEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit value is directly connected to the RFSUBG analog IP pin."]
    #[inline(always)]
    pub fn rfd_singen_div2_pup(&self) -> RfdSingenDiv2PupR {
        RfdSingenDiv2PupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit value is directly connected to the RFSUBG analog IP pin."]
    #[inline(always)]
    pub fn rfd_singen_lbe(&self) -> RfdSingenLbeR {
        RfdSingenLbeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SINGEN signal for the RFSUBGanalog IP."]
    #[inline(always)]
    pub fn rfd_singen_ena(&mut self) -> RfdSingenEnaW<'_, SingenAnaEngSpec> {
        RfdSingenEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit value is directly connected to the RFSUBG analog IP pin."]
    #[inline(always)]
    pub fn rfd_singen_div2_pup(&mut self) -> RfdSingenDiv2PupW<'_, SingenAnaEngSpec> {
        RfdSingenDiv2PupW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit value is directly connected to the RFSUBG analog IP pin."]
    #[inline(always)]
    pub fn rfd_singen_lbe(&mut self) -> RfdSingenLbeW<'_, SingenAnaEngSpec> {
        RfdSingenLbeW::new(self, 2)
    }
}
#[doc = "SINGEN_ANA_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingenAnaEngSpec;
impl crate::RegisterSpec for SingenAnaEngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singen_ana_eng::R`](R) reader structure"]
impl crate::Readable for SingenAnaEngSpec {}
#[doc = "`write(|w| ..)` method takes [`singen_ana_eng::W`](W) writer structure"]
impl crate::Writable for SingenAnaEngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGEN_ANA_ENG to value 0"]
impl crate::Resettable for SingenAnaEngSpec {}
