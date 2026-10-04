#[doc = "Register `SPI_SSPDR` reader"]
pub type R = crate::R<SpiSspdrSpec>;
#[doc = "Register `SPI_SSPDR` writer"]
pub type W = crate::W<SpiSspdrSpec>;
#[doc = "Field `DR` reader - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
pub type DrR = crate::FieldReader<u16>;
#[doc = "Field `DR` writer - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section 18.5.8: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<'_, SpiSspdrSpec> {
        DrW::new(self, 0)
    }
}
#[doc = "SPI_SSPDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSspdrSpec;
impl crate::RegisterSpec for SpiSspdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sspdr::R`](R) reader structure"]
impl crate::Readable for SpiSspdrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sspdr::W`](W) writer structure"]
impl crate::Writable for SpiSspdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SSPDR to value 0"]
impl crate::Resettable for SpiSspdrSpec {}
