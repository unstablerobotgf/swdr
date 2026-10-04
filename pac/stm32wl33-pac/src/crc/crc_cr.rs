#[doc = "Register `CRC_CR` reader"]
pub type R = crate::R<CrcCrSpec>;
#[doc = "Register `CRC_CR` writer"]
pub type W = crate::W<CrcCrSpec>;
#[doc = "Field `RESET` reader - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial"]
pub type PolysizeR = crate::FieldReader;
#[doc = "Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial"]
pub type PolysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_IN` reader - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word"]
pub type RevInR = crate::FieldReader;
#[doc = "Field `REV_IN` writer - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word"]
pub type RevInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format"]
pub type RevOutR = crate::BitReader;
#[doc = "Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format"]
pub type RevOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial"]
    #[inline(always)]
    pub fn polysize(&self) -> PolysizeR {
        PolysizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word"]
    #[inline(always)]
    pub fn rev_in(&self) -> RevInR {
        RevInR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format"]
    #[inline(always)]
    pub fn rev_out(&self) -> RevOutR {
        RevOutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CrcCrSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial"]
    #[inline(always)]
    pub fn polysize(&mut self) -> PolysizeW<'_, CrcCrSpec> {
        PolysizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word"]
    #[inline(always)]
    pub fn rev_in(&mut self) -> RevInW<'_, CrcCrSpec> {
        RevInW::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format"]
    #[inline(always)]
    pub fn rev_out(&mut self) -> RevOutW<'_, CrcCrSpec> {
        RevOutW::new(self, 7)
    }
}
#[doc = "CRC_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCrSpec;
impl crate::RegisterSpec for CrcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_cr::R`](R) reader structure"]
impl crate::Readable for CrcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_cr::W`](W) writer structure"]
impl crate::Writable for CrcCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_CR to value 0"]
impl crate::Resettable for CrcCrSpec {}
