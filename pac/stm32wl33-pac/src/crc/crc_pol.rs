#[doc = "Register `CRC_POL` reader"]
pub type R = crate::R<CrcPolSpec>;
#[doc = "Register `CRC_POL` writer"]
pub type W = crate::W<CrcPolSpec>;
#[doc = "Field `POL` reader - POL\\[31:0\\]: Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
pub type PolR = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - POL\\[31:0\\]: Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
pub type PolW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - POL\\[31:0\\]: Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - POL\\[31:0\\]: Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, CrcPolSpec> {
        PolW::new(self, 0)
    }
}
#[doc = "CRC_POL register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcPolSpec;
impl crate::RegisterSpec for CrcPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pol::R`](R) reader structure"]
impl crate::Readable for CrcPolSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_pol::W`](W) writer structure"]
impl crate::Writable for CrcPolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_POL to value 0x04c1_1db7"]
impl crate::Resettable for CrcPolSpec {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
