#[doc = "Register `RX_INFO_REG` reader"]
pub type R = crate::R<RxInfoRegSpec>;
#[doc = "Field `RX_PCKTLEN_OUT` reader - Indicates received packet length in bytes:"]
pub type RxPcktlenOutR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates received packet length in bytes:"]
    #[inline(always)]
    pub fn rx_pcktlen_out(&self) -> RxPcktlenOutR {
        RxPcktlenOutR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX_INFO_REG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_info_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxInfoRegSpec;
impl crate::RegisterSpec for RxInfoRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_info_reg::R`](R) reader structure"]
impl crate::Readable for RxInfoRegSpec {}
#[doc = "`reset()` method sets RX_INFO_REG to value 0"]
impl crate::Resettable for RxInfoRegSpec {}
