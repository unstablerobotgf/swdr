#[doc = "Register `CRC_DR` reader"]
pub type R = crate::R<CrcDrSpec>;
#[doc = "Register `CRC_DR` writer"]
pub type W = crate::W<CrcDrSpec>;
#[doc = "Field `DR` reader - Data register bits. This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
pub type DrR = crate::FieldReader<u32>;
#[doc = "Field `DR` writer - Data register bits. This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data register bits. This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data register bits. This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<'_, CrcDrSpec> {
        DrW::new(self, 0)
    }
}
#[doc = "CRC_DR register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcDrSpec;
impl crate::RegisterSpec for CrcDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_dr::R`](R) reader structure"]
impl crate::Readable for CrcDrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_dr::W`](W) writer structure"]
impl crate::Writable for CrcDrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_DR to value 0xffff_ffff"]
impl crate::Resettable for CrcDrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
