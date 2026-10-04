#[doc = "Register `I2C_CR2` reader"]
pub type R = crate::R<I2cCr2Spec>;
#[doc = "Register `I2C_CR2` writer"]
pub type W = crate::W<I2cCr2Spec>;
#[doc = "Field `SADD` reader - Slave address"]
pub type SaddR = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - Slave address"]
pub type SaddW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer."]
pub type RdWrnR = crate::BitReader;
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer."]
pub type RdWrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD10` reader - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode"]
pub type Add10R = crate::BitReader;
#[doc = "Field `ADD10` writer - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode"]
pub type Add10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEAD10R` reader - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
pub type Head10rR = crate::BitReader;
#[doc = "Field `HEAD10R` writer - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
pub type Head10rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte."]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte."]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBYTES` reader - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0."]
pub type NbytesR = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0."]
pub type NbytesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
pub type ReloadR = crate::BitReader;
#[doc = "Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
pub type AutoendR = crate::BitReader;
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
pub type AutoendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECBYTE` reader - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested"]
pub type PecbyteR = crate::BitReader;
#[doc = "Field `PECBYTE` writer - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested"]
pub type PecbyteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    pub fn sadd(&self) -> SaddR {
        SaddR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer."]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RdWrnR {
        RdWrnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn add10(&self) -> Add10R {
        Add10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
    #[inline(always)]
    pub fn head10r(&self) -> Head10rR {
        Head10rR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte."]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0."]
    #[inline(always)]
    pub fn nbytes(&self) -> NbytesR {
        NbytesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    #[inline(always)]
    pub fn autoend(&self) -> AutoendR {
        AutoendR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PecbyteR {
        PecbyteR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    pub fn sadd(&mut self) -> SaddW<'_, I2cCr2Spec> {
        SaddW::new(self, 0)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) - 0: Master requests a write transfer. - 1: Master requests a read transfer."]
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RdWrnW<'_, I2cCr2Spec> {
        RdWrnW::new(self, 10)
    }
    #[doc = "Bit 11 - Ten-bit addressing mode (master mode) - 0: The master operates in 7-bit addressing mode, - 1: The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn add10(&mut self) -> Add10W<'_, I2cCr2Spec> {
        Add10W::new(self, 11)
    }
    #[doc = "Bit 12 - Ten bit (10-bit) address header only read direction (master receiver mode) - 0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction. - 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
    #[inline(always)]
    pub fn head10r(&mut self) -> Head10rW<'_, I2cCr2Spec> {
        Head10rW::new(self, 12)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. - 0: No Start generation. - 1: Restart/Start generation: If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, I2cCr2Spec> {
        StartW::new(self, 13)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: - 0: No Stop generation. - 1: Stop generation after current byte transfer."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, I2cCr2Spec> {
        StopW::new(self, 14)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. - 0: an ACK is sent after current received byte. - 1: a NACK is sent after current received byte."]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, I2cCr2Spec> {
        NackW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0."]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NbytesW<'_, I2cCr2Spec> {
        NbytesW::new(self, 16)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software. - 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow). - 1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<'_, I2cCr2Spec> {
        ReloadW::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. - 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. - 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    #[inline(always)]
    pub fn autoend(&mut self) -> AutoendW<'_, I2cCr2Spec> {
        AutoendW::new(self, 25)
    }
    #[doc = "Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. - 0: No PEC transfer. - 1: PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PecbyteW<'_, I2cCr2Spec> {
        PecbyteW::new(self, 26)
    }
}
#[doc = "I2C_CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCr2Spec;
impl crate::RegisterSpec for I2cCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cr2::R`](R) reader structure"]
impl crate::Readable for I2cCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cr2::W`](W) writer structure"]
impl crate::Writable for I2cCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CR2 to value 0"]
impl crate::Resettable for I2cCr2Spec {}
