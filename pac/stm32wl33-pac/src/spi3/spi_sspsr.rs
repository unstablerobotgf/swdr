#[doc = "Register `SPI_SSPSR` reader"]
pub type R = crate::R<SpiSspsrSpec>;
#[doc = "Register `SPI_SSPSR` writer"]
pub type W = crate::W<SpiSspsrSpec>;
#[doc = "Field `RXNE` reader - Receive buffer not empty - 0: Rx buffer empty - 1: Rx buffer not empty"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TXE` reader - Transmit buffer empty - 0: No more empty space in Tx buffer. (software shall not write data to the Tx buffer). - 1: At least one empty space in Tx buffer. (software may write data to the Tx buffer)."]
pub type TxeR = crate::BitReader;
#[doc = "Field `CHSIDE` reader - Channel side - 0: Channel Left has to be transmitted or has been received - 1: Channel Right has to be transmitted or has been received"]
pub type ChsideR = crate::BitReader;
#[doc = "Field `UDR` reader - Underrun flag - 0: No underrun occurred - 1: Underrun occurred"]
pub type UdrR = crate::BitReader;
#[doc = "Field `CRCERR` reader - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0."]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0."]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` reader - Mode fault - 0: No mode fault occurred - 1: Mode fault occurred"]
pub type ModfR = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun flag - 0: No overrun occurred - 1: Overrun occurred"]
pub type OvrR = crate::BitReader;
#[doc = "Field `BSY` reader - Busy flag - 0: SPI (or I2S) not busy - 1: SPI (or I2S) is busy in communication or Tx buffer is not empty This flag is set and cleared by hardware."]
pub type BsyR = crate::BitReader;
#[doc = "Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to Section 18.5.10: SPI error flags and Section 18.7.6: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software. - 0: No frame format error - 1: A frame format error occurred"]
pub type FreR = crate::BitReader;
#[doc = "Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full"]
pub type FrlvlR = crate::FieldReader;
#[doc = "Field `FTLVL` reader - FIFO Transmission Level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)"]
pub type FtlvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive buffer not empty - 0: Rx buffer empty - 1: Rx buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty - 0: No more empty space in Tx buffer. (software shall not write data to the Tx buffer). - 1: At least one empty space in Tx buffer. (software may write data to the Tx buffer)."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel side - 0: Channel Left has to be transmitted or has been received - 1: Channel Right has to be transmitted or has been received"]
    #[inline(always)]
    pub fn chside(&self) -> ChsideR {
        ChsideR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underrun flag - 0: No underrun occurred - 1: Underrun occurred"]
    #[inline(always)]
    pub fn udr(&self) -> UdrR {
        UdrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0."]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault - 0: No mode fault occurred - 1: Mode fault occurred"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag - 0: No overrun occurred - 1: Overrun occurred"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag - 0: SPI (or I2S) not busy - 1: SPI (or I2S) is busy in communication or Tx buffer is not empty This flag is set and cleared by hardware."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to Section 18.5.10: SPI error flags and Section 18.7.6: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software. - 0: No frame format error - 1: A frame format error occurred"]
    #[inline(always)]
    pub fn fre(&self) -> FreR {
        FreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full"]
    #[inline(always)]
    pub fn frlvl(&self) -> FrlvlR {
        FrlvlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - FIFO Transmission Level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)"]
    #[inline(always)]
    pub fn ftlvl(&self) -> FtlvlR {
        FtlvlR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0."]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<'_, SpiSspsrSpec> {
        CrcerrW::new(self, 4)
    }
}
#[doc = "SPI_SSPSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSspsrSpec;
impl crate::RegisterSpec for SpiSspsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sspsr::R`](R) reader structure"]
impl crate::Readable for SpiSspsrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sspsr::W`](W) writer structure"]
impl crate::Writable for SpiSspsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SSPSR to value 0x02"]
impl crate::Resettable for SpiSspsrSpec {
    const RESET_VALUE: u32 = 0x02;
}
