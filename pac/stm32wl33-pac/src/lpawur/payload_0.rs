#[doc = "Register `PAYLOAD_0` reader"]
pub type R = crate::R<Payload0Spec>;
#[doc = "Field `PAYLOAD_0` reader - First part of the payload (Least significant Byte First)"]
pub type Payload0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - First part of the payload (Least significant Byte First)"]
    #[inline(always)]
    pub fn payload_0(&self) -> Payload0R {
        Payload0R::new(self.bits)
    }
}
#[doc = "PAYLOAD_0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`payload_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Payload0Spec;
impl crate::RegisterSpec for Payload0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`payload_0::R`](R) reader structure"]
impl crate::Readable for Payload0Spec {}
#[doc = "`reset()` method sets PAYLOAD_0 to value 0"]
impl crate::Resettable for Payload0Spec {}
