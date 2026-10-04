#[doc = "Register `SPI_SSPCR1` reader"]
pub type R = crate::R<SpiSspcr1Spec>;
#[doc = "Register `SPI_SSPCR1` writer"]
pub type W = crate::W<SpiSspcr1Spec>;
#[doc = "Field `CPHA` reader - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR` reader - Master selection - 0: Slave configuration - 1: Master configuration"]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - Master selection - 0: Slave configuration - 1: Master configuration"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR` reader - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256"]
pub type BrR = crate::FieldReader;
#[doc = "Field `BR` writer - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPE` reader - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled"]
pub type SpeR = crate::BitReader;
#[doc = "Field `SPE` writer - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled"]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFIRST` reader - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first"]
pub type LsbfirstR = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first"]
pub type LsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI` reader - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored."]
pub type SsiR = crate::BitReader;
#[doc = "Field `SSI` writer - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored."]
pub type SsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled"]
pub type SsmR = crate::BitReader;
#[doc = "Field `SSM` writer - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXONLY` reader - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)"]
pub type RxonlyR = crate::BitReader;
#[doc = "Field `RXONLY` writer - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)"]
pub type RxonlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCL` reader - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length"]
pub type CrclR = crate::BitReader;
#[doc = "Field `CRCL` writer - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length"]
pub type CrclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNEXT` reader - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register"]
pub type CrcnextR = crate::BitReader;
#[doc = "Field `CRCNEXT` writer - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register"]
pub type CrcnextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)"]
pub type BidioeR = crate::BitReader;
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)"]
pub type BidioeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected"]
pub type BidimodeR = crate::BitReader;
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected"]
pub type BidimodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection - 0: Slave configuration - 1: Master configuration"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LsbfirstR {
        LsbfirstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored."]
    #[inline(always)]
    pub fn ssi(&self) -> SsiR {
        SsiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn rxonly(&self) -> RxonlyR {
        RxonlyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length"]
    #[inline(always)]
    pub fn crcl(&self) -> CrclR {
        CrclR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register"]
    #[inline(always)]
    pub fn crcnext(&self) -> CrcnextR {
        CrcnextR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn bidioe(&self) -> BidioeR {
        BidioeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn bidimode(&self) -> BidimodeR {
        BidimodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase - 0: The first clock transition is the first data capture edge - 1: The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, SpiSspcr1Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity - 0: CK to 0 when idle - 1: CK to 1 when idle"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, SpiSspcr1Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Master selection - 0: Slave configuration - 1: Master configuration"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, SpiSspcr1Spec> {
        MstrW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Baud rate control - 000: fPCLK/2 - 001: fPCLK/4 - 010: fPCLK/8 - 011: fPCLK/16 - 100: fPCLK/32 - 101: fPCLK/64 - 110: fPCLK/128 - 111: fPCLK/256"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, SpiSspcr1Spec> {
        BrW::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable - 0: Peripheral disabled - 1: Peripheral enabled"]
    #[inline(always)]
    pub fn spe(&mut self) -> SpeW<'_, SpiSspcr1Spec> {
        SpeW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format - 0: data is transmitted / received with the MSB first - 1: data is transmitted / received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LsbfirstW<'_, SpiSspcr1Spec> {
        LsbfirstW::new(self, 7)
    }
    #[doc = "Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored."]
    #[inline(always)]
    pub fn ssi(&mut self) -> SsiW<'_, SpiSspcr1Spec> {
        SsiW::new(self, 8)
    }
    #[doc = "Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. - 0: Software slave management disabled - 1: Software slave management enabled"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, SpiSspcr1Spec> {
        SsmW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. - 0: Full duplex (Transmit and receive) - 1: Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn rxonly(&mut self) -> RxonlyW<'_, SpiSspcr1Spec> {
        RxonlyW::new(self, 10)
    }
    #[doc = "Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. - 0: 8-bit CRC length - 1: 16-bit CRC length"]
    #[inline(always)]
    pub fn crcl(&mut self) -> CrclW<'_, SpiSspcr1Spec> {
        CrclW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit CRC next - 0: Next transmit value is from Tx buffer - 1: Next transmit value is from Tx CRC register"]
    #[inline(always)]
    pub fn crcnext(&mut self) -> CrcnextW<'_, SpiSspcr1Spec> {
        CrcnextW::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable - 0: CRC calculation disabled - 1: CRC calculation Enabled"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, SpiSspcr1Spec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode - 0: Output disabled (receive-only mode) - 1: Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn bidioe(&mut self) -> BidioeW<'_, SpiSspcr1Spec> {
        BidioeW::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. - 0: 2-line unidirectional data mode selected - 1: 1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn bidimode(&mut self) -> BidimodeW<'_, SpiSspcr1Spec> {
        BidimodeW::new(self, 15)
    }
}
#[doc = "SPI_SSPCR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sspcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSspcr1Spec;
impl crate::RegisterSpec for SpiSspcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sspcr1::R`](R) reader structure"]
impl crate::Readable for SpiSspcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_sspcr1::W`](W) writer structure"]
impl crate::Writable for SpiSspcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SSPCR1 to value 0"]
impl crate::Resettable for SpiSspcr1Spec {}
