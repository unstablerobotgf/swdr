#[doc = "Register `SPI_SSPCRCPR` reader"]
pub type R = crate::R<SpiSspcrcprSpec>;
#[doc = "Register `SPI_SSPCRCPR` writer"]
pub type W = crate::W<SpiSspcrcprSpec>;
#[doc = "Field `CRCPOLY` reader - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required."]
pub type CrcpolyR = crate::FieldReader<u16>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required."]
pub type CrcpolyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required."]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required."]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CrcpolyW<'_, SpiSspcrcprSpec> {
        CrcpolyW::new(self, 0)
    }
}
#[doc = "SPI_SSPCRCPR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspcrcpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcrcpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSspcrcprSpec;
impl crate::RegisterSpec for SpiSspcrcprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sspcrcpr::R`](R) reader structure"]
impl crate::Readable for SpiSspcrcprSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sspcrcpr::W`](W) writer structure"]
impl crate::Writable for SpiSspcrcprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SSPCRCPR to value 0x07"]
impl crate::Resettable for SpiSspcrcprSpec {
    const RESET_VALUE: u32 = 0x07;
}
