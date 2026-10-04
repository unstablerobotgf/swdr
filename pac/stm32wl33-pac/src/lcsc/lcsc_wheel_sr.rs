#[doc = "Register `LCSC_WHEEL_SR` reader"]
pub type R = crate::R<LcscWheelSrSpec>;
#[doc = "Field `CLKWISE` reader - Number of Clock Wise revolutions"]
pub type ClkwiseR = crate::FieldReader<u16>;
#[doc = "Field `ACLKWISE` reader - Number of Anti Clock Wise revolutions"]
pub type AclkwiseR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Clock Wise revolutions"]
    #[inline(always)]
    pub fn clkwise(&self) -> ClkwiseR {
        ClkwiseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of Anti Clock Wise revolutions"]
    #[inline(always)]
    pub fn aclkwise(&self) -> AclkwiseR {
        AclkwiseR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "LCSC_WHEEL_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_wheel_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscWheelSrSpec;
impl crate::RegisterSpec for LcscWheelSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_wheel_sr::R`](R) reader structure"]
impl crate::Readable for LcscWheelSrSpec {}
#[doc = "`reset()` method sets LCSC_WHEEL_SR to value 0"]
impl crate::Resettable for LcscWheelSrSpec {}
