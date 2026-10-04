#[doc = "Register `SPI_SSPCR2` reader"]
pub type R = crate::R<SpiSspcr2Spec>;
#[doc = "Register `SPI_SSPCR2` writer"]
pub type W = crate::W<SpiSspcr2Spec>;
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set. - 0: Rx buffer DMA disabled - 1: Rx buffer DMA enabled"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set. - 0: Rx buffer DMA disabled - 1: Rx buffer DMA enabled"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set. - 0: Tx buffer DMA disabled - 1: Tx buffer DMA enabled"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set. - 0: Tx buffer DMA disabled - 1: Tx buffer DMA enabled"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SS output enable - 0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration - 1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
pub type SsoeR = crate::BitReader;
#[doc = "Field `SSOE` writer - SS output enable - 0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration - 1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
pub type SsoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSP` reader - NSS pulse management This bit is used in master mode only. it allow the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. - 0: No NSS pulse - 1: NSS pulse generated"]
pub type NsspR = crate::BitReader;
#[doc = "Field `NSSP` writer - NSS pulse management This bit is used in master mode only. it allow the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. - 0: No NSS pulse - 1: NSS pulse generated"]
pub type NsspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRF` reader - Frame format - 0: SPI Motorola mode - 1 SPI TI mode"]
pub type FrfR = crate::BitReader;
#[doc = "Field `FRF` writer - Frame format - 0: SPI Motorola mode - 1 SPI TI mode"]
pub type FrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode). - 0: Error interrupt is masked - 1: Error interrupt is enabled"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode). - 0: Error interrupt is masked - 1: Error interrupt is enabled"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable - 0: RXNE interrupt masked - 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable - 0: RXNE interrupt masked - 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable - 0: TXE interrupt masked - 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable - 0: TXE interrupt masked - 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS` reader - Data size These bits configure the data length for SPI transfers: - 0000: Not used - 0001: Not used - 0010: Not used - 0011: 4-bit - 0100: 5-bit - 0101: 6-bit - 0110: 7-bit - 0111: 8-bit - 1000: 9-bit - 1001: 10-bit - 1010: 11-bit - 1011: 12-bit - 1100: 13-bit - 1101: 14-bit - 1110: 15-bit - 1111: 16-bit If software attempts to write one of the 'Not used' values, they are forced to the value '0111'(8-bit)."]
pub type DsR = crate::FieldReader;
#[doc = "Field `DS` writer - Data size These bits configure the data length for SPI transfers: - 0000: Not used - 0001: Not used - 0010: Not used - 0011: 4-bit - 0100: 5-bit - 0101: 6-bit - 0110: 7-bit - 0111: 8-bit - 1000: 9-bit - 1001: 10-bit - 1010: 11-bit - 1011: 12-bit - 1100: 13-bit - 1101: 14-bit - 1110: 15-bit - 1111: 16-bit If software attempts to write one of the 'Not used' values, they are forced to the value '0111'(8-bit)."]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRXTH` reader - FIFO reception threshold FRXTH shall be set according the read access (16-bit or 8-bit) to the FIFO. This bit is used to set the threshold of the RXFIFO that triggers an RXNE event - 0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit) - 1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
pub type FrxthR = crate::BitReader;
#[doc = "Field `FRXTH` writer - FIFO reception threshold FRXTH shall be set according the read access (16-bit or 8-bit) to the FIFO. This bit is used to set the threshold of the RXFIFO that triggers an RXNE event - 0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit) - 1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
pub type FrxthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
pub type LdmaRxR = crate::BitReader;
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
pub type LdmaRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
pub type LdmaTxR = crate::BitReader;
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
pub type LdmaTxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set. - 0: Rx buffer DMA disabled - 1: Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set. - 0: Tx buffer DMA disabled - 1: Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable - 0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration - 1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    #[inline(always)]
    pub fn ssoe(&self) -> SsoeR {
        SsoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management This bit is used in master mode only. it allow the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. - 0: No NSS pulse - 1: NSS pulse generated"]
    #[inline(always)]
    pub fn nssp(&self) -> NsspR {
        NsspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format - 0: SPI Motorola mode - 1 SPI TI mode"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode). - 0: Error interrupt is masked - 1: Error interrupt is enabled"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable - 0: RXNE interrupt masked - 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable - 0: TXE interrupt masked - 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size These bits configure the data length for SPI transfers: - 0000: Not used - 0001: Not used - 0010: Not used - 0011: 4-bit - 0100: 5-bit - 0101: 6-bit - 0110: 7-bit - 0111: 8-bit - 1000: 9-bit - 1001: 10-bit - 1010: 11-bit - 1011: 12-bit - 1100: 13-bit - 1101: 14-bit - 1110: 15-bit - 1111: 16-bit If software attempts to write one of the 'Not used' values, they are forced to the value '0111'(8-bit)."]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold FRXTH shall be set according the read access (16-bit or 8-bit) to the FIFO. This bit is used to set the threshold of the RXFIFO that triggers an RXNE event - 0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit) - 1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn frxth(&self) -> FrxthR {
        FrxthR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LdmaRxR {
        LdmaRxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LdmaTxR {
        LdmaTxR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set. - 0: Rx buffer DMA disabled - 1: Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<'_, SpiSspcr2Spec> {
        RxdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set. - 0: Tx buffer DMA disabled - 1: Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<'_, SpiSspcr2Spec> {
        TxdmaenW::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable - 0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration - 1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SsoeW<'_, SpiSspcr2Spec> {
        SsoeW::new(self, 2)
    }
    #[doc = "Bit 3 - NSS pulse management This bit is used in master mode only. it allow the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. - 0: No NSS pulse - 1: NSS pulse generated"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NsspW<'_, SpiSspcr2Spec> {
        NsspW::new(self, 3)
    }
    #[doc = "Bit 4 - Frame format - 0: SPI Motorola mode - 1 SPI TI mode"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, SpiSspcr2Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode). - 0: Error interrupt is masked - 1: Error interrupt is enabled"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, SpiSspcr2Spec> {
        ErrieW::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable - 0: RXNE interrupt masked - 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<'_, SpiSspcr2Spec> {
        RxneieW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable - 0: TXE interrupt masked - 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<'_, SpiSspcr2Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data size These bits configure the data length for SPI transfers: - 0000: Not used - 0001: Not used - 0010: Not used - 0011: 4-bit - 0100: 5-bit - 0101: 6-bit - 0110: 7-bit - 0111: 8-bit - 1000: 9-bit - 1001: 10-bit - 1010: 11-bit - 1011: 12-bit - 1100: 13-bit - 1101: 14-bit - 1110: 15-bit - 1111: 16-bit If software attempts to write one of the 'Not used' values, they are forced to the value '0111'(8-bit)."]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<'_, SpiSspcr2Spec> {
        DsW::new(self, 8)
    }
    #[doc = "Bit 12 - FIFO reception threshold FRXTH shall be set according the read access (16-bit or 8-bit) to the FIFO. This bit is used to set the threshold of the RXFIFO that triggers an RXNE event - 0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit) - 1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn frxth(&mut self) -> FrxthW<'_, SpiSspcr2Spec> {
        FrxthW::new(self, 12)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LdmaRxW<'_, SpiSspcr2Spec> {
        LdmaRxW::new(self, 13)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length = 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). - 0: Number of data to transfer is even - 1: Number of data to transfer is odd"]
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LdmaTxW<'_, SpiSspcr2Spec> {
        LdmaTxW::new(self, 14)
    }
}
#[doc = "SPI_SSPCR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSspcr2Spec;
impl crate::RegisterSpec for SpiSspcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sspcr2::R`](R) reader structure"]
impl crate::Readable for SpiSspcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_sspcr2::W`](W) writer structure"]
impl crate::Writable for SpiSspcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SSPCR2 to value 0x0700"]
impl crate::Resettable for SpiSspcr2Spec {
    const RESET_VALUE: u32 = 0x0700;
}
