#[doc = "Register `LCSC_SR` reader"]
pub type R = crate::R<LcscSrSpec>;
#[doc = "Field `CLKWISE_STATE` reader - The current state of the LCSC clockwise FSM:"]
pub type ClkwiseStateR = crate::FieldReader;
#[doc = "Field `ACLKWISE_STATE` reader - The current state of the LCSC anti clockwise FSM:"]
pub type AclkwiseStateR = crate::FieldReader;
#[doc = "Field `LAST_DIR` reader - The last direction detected:"]
pub type LastDirR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - The current state of the LCSC clockwise FSM:"]
    #[inline(always)]
    pub fn clkwise_state(&self) -> ClkwiseStateR {
        ClkwiseStateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The current state of the LCSC anti clockwise FSM:"]
    #[inline(always)]
    pub fn aclkwise_state(&self) -> AclkwiseStateR {
        AclkwiseStateR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The last direction detected:"]
    #[inline(always)]
    pub fn last_dir(&self) -> LastDirR {
        LastDirR::new(((self.bits >> 4) & 3) as u8)
    }
}
#[doc = "LCSC_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscSrSpec;
impl crate::RegisterSpec for LcscSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_sr::R`](R) reader structure"]
impl crate::Readable for LcscSrSpec {}
#[doc = "`reset()` method sets LCSC_SR to value 0"]
impl crate::Resettable for LcscSrSpec {}
