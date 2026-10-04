#[doc = "Register `PAYLOAD_1` reader"]
pub type R = crate::R<Payload1Spec>;
#[doc = "Field `PAYLOAD_1` reader - Second part of the payload (Least significant Byte First)"]
pub type Payload1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Second part of the payload (Least significant Byte First)"]
    #[inline(always)]
    pub fn payload_1(&self) -> Payload1R {
        Payload1R::new(self.bits)
    }
}
#[doc = "PAYLOAD_1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`payload_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Payload1Spec;
impl crate::RegisterSpec for Payload1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`payload_1::R`](R) reader structure"]
impl crate::Readable for Payload1Spec {}
#[doc = "`reset()` method sets PAYLOAD_1 to value 0"]
impl crate::Resettable for Payload1Spec {}
