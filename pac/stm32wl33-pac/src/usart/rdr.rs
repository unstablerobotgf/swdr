#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RDR` reader - RDR\\[8:0\\]: Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see Figure 124). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
pub type RdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - RDR\\[8:0\\]: Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see Figure 124). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "RDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {}
