#[doc = "Register `CRC_INIT` reader"]
pub type R = crate::R<CrcInitSpec>;
#[doc = "Register `CRC_INIT` writer"]
pub type W = crate::W<CrcInitSpec>;
#[doc = "Field `INIT` reader - Programmable initial CRC value This register is used to write the CRC initial value."]
pub type InitR = crate::FieldReader<u32>;
#[doc = "Field `INIT` writer - Programmable initial CRC value This register is used to write the CRC initial value."]
pub type InitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value."]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value."]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, CrcInitSpec> {
        InitW::new(self, 0)
    }
}
#[doc = "CRC_INIT register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcInitSpec;
impl crate::RegisterSpec for CrcInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_init::R`](R) reader structure"]
impl crate::Readable for CrcInitSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_init::W`](W) writer structure"]
impl crate::Writable for CrcInitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_INIT to value 0xffff_ffff"]
impl crate::Resettable for CrcInitSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
