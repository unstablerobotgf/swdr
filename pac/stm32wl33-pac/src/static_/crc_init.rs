#[doc = "Register `CRC_INIT` reader"]
pub type R = crate::R<CrcInitSpec>;
#[doc = "Register `CRC_INIT` writer"]
pub type W = crate::W<CrcInitSpec>;
#[doc = "Field `CRC_INIT_VAL` reader - CRC intialization value"]
pub type CrcInitValR = crate::FieldReader<u32>;
#[doc = "Field `CRC_INIT_VAL` writer - CRC intialization value"]
pub type CrcInitValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC intialization value"]
    #[inline(always)]
    pub fn crc_init_val(&self) -> CrcInitValR {
        CrcInitValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC intialization value"]
    #[inline(always)]
    pub fn crc_init_val(&mut self) -> CrcInitValW<'_, CrcInitSpec> {
        CrcInitValW::new(self, 0)
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
#[doc = "`reset()` method sets CRC_INIT to value 0"]
impl crate::Resettable for CrcInitSpec {}
