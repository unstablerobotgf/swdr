#[doc = "Register `AES_SR` reader"]
pub type R = crate::R<AesSrSpec>;
#[doc = "Field `CCF` reader - CCF: Computation complete flag"]
pub type CcfR = crate::BitReader;
#[doc = "Field `RDERR` reader - RDERR: Read error flag"]
pub type RderrR = crate::BitReader;
#[doc = "Field `WRERR` reader - WRERR: Write error flag"]
pub type WrerrR = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY: Busy flag"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CCF: Computation complete flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RDERR: Read error flag"]
    #[inline(always)]
    pub fn rderr(&self) -> RderrR {
        RderrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WRERR: Write error flag"]
    #[inline(always)]
    pub fn wrerr(&self) -> WrerrR {
        WrerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY: Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "AES_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesSrSpec;
impl crate::RegisterSpec for AesSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_sr::R`](R) reader structure"]
impl crate::Readable for AesSrSpec {}
#[doc = "`reset()` method sets AES_SR to value 0"]
impl crate::Resettable for AesSrSpec {}
