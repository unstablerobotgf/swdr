#[doc = "Register `RFHSECR` reader"]
pub type R = crate::R<RfhsecrSpec>;
#[doc = "Field `XOTUNE` reader - RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset."]
pub type XotuneR = crate::FieldReader;
#[doc = "Field `AMPLREADY` reader - RF-HSE Amplitude Control Ready output"]
pub type AmplreadyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset."]
    #[inline(always)]
    pub fn xotune(&self) -> XotuneR {
        XotuneR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - RF-HSE Amplitude Control Ready output"]
    #[inline(always)]
    pub fn amplready(&self) -> AmplreadyR {
        AmplreadyR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "RFHSECR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfhsecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfhsecrSpec;
impl crate::RegisterSpec for RfhsecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfhsecr::R`](R) reader structure"]
impl crate::Readable for RfhsecrSpec {}
#[doc = "`reset()` method sets RFHSECR to value 0"]
impl crate::Resettable for RfhsecrSpec {}
