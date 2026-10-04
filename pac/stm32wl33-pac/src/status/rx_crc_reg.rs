#[doc = "Register `RX_CRC_REG` reader"]
pub type R = crate::R<RxCrcRegSpec>;
#[doc = "Field `RX_CRC_OUT` reader - CRC field of the received packet (read-only info)"]
pub type RxCrcOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC field of the received packet (read-only info)"]
    #[inline(always)]
    pub fn rx_crc_out(&self) -> RxCrcOutR {
        RxCrcOutR::new(self.bits)
    }
}
#[doc = "RX_CRC_REG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcRegSpec;
impl crate::RegisterSpec for RxCrcRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_reg::R`](R) reader structure"]
impl crate::Readable for RxCrcRegSpec {}
#[doc = "`reset()` method sets RX_CRC_REG to value 0"]
impl crate::Resettable for RxCrcRegSpec {}
