#[doc = "Register `SPI2S_I2SCFGR` reader"]
pub type R = crate::R<Spi2sI2scfgrSpec>;
#[doc = "Register `SPI2S_I2SCFGR` writer"]
pub type W = crate::W<Spi2sI2scfgrSpec>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in."]
pub type ChlenR = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in."]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed"]
pub type DatlenR = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed"]
pub type DatlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKPOL` reader - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level"]
pub type CkpolR = crate::BitReader;
#[doc = "Field `CKPOL` writer - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level"]
pub type CkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSTD` reader - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard"]
pub type I2sstdR = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard"]
pub type I2sstdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PcmsyncR = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PcmsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SCFG` reader - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive"]
pub type I2scfgR = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive"]
pub type I2scfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SE` reader - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode."]
pub type I2seR = crate::BitReader;
#[doc = "Field `I2SE` writer - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode."]
pub type I2seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SMOD` reader - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled."]
pub type I2smodR = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled."]
pub type I2smodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Astren {
    #[doc = "0: The Asynchronous start is disabled. When the I2S is enabled in slave mode, the I2S slave starts the transfer when the I2S clock is received and an appropriate transition (depending on the protocol selected) is detected on the WS signal."]
    B0x0 = 0,
    #[doc = "1: The Asynchronous start is enabled. When the I2S is enabled in slave mode, the I2S slave starts immediately the transfer when the I2S clock is received from the master without checking the expected transition of WS signal."]
    B0x1 = 1,
}
impl From<Astren> for bool {
    #[inline(always)]
    fn from(variant: Astren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASTREN` reader - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards."]
pub type AstrenR = crate::BitReader<Astren>;
impl AstrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Astren {
        match self.bits {
            false => Astren::B0x0,
            true => Astren::B0x1,
        }
    }
    #[doc = "The Asynchronous start is disabled. When the I2S is enabled in slave mode, the I2S slave starts the transfer when the I2S clock is received and an appropriate transition (depending on the protocol selected) is detected on the WS signal."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Astren::B0x0
    }
    #[doc = "The Asynchronous start is enabled. When the I2S is enabled in slave mode, the I2S slave starts immediately the transfer when the I2S clock is received from the master without checking the expected transition of WS signal."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Astren::B0x1
    }
}
#[doc = "Field `ASTREN` writer - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards."]
pub type AstrenW<'a, REG> = crate::BitWriter<'a, REG, Astren>;
impl<'a, REG> AstrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Asynchronous start is disabled. When the I2S is enabled in slave mode, the I2S slave starts the transfer when the I2S clock is received and an appropriate transition (depending on the protocol selected) is detected on the WS signal."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Astren::B0x0)
    }
    #[doc = "The Asynchronous start is enabled. When the I2S is enabled in slave mode, the I2S slave starts immediately the transfer when the I2S clock is received from the master without checking the expected transition of WS signal."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Astren::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in."]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed"]
    #[inline(always)]
    pub fn datlen(&self) -> DatlenR {
        DatlenR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level"]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2sstdR {
        I2sstdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    pub fn pcmsync(&self) -> PcmsyncR {
        PcmsyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2scfgR {
        I2scfgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn i2se(&self) -> I2seR {
        I2seR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    pub fn i2smod(&self) -> I2smodR {
        I2smodR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards."]
    #[inline(always)]
    pub fn astren(&self) -> AstrenR {
        AstrenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in."]
    #[inline(always)]
    pub fn chlen(&mut self) -> ChlenW<'_, Spi2sI2scfgrSpec> {
        ChlenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DatlenW<'_, Spi2sI2scfgrSpec> {
        DatlenW::new(self, 1)
    }
    #[doc = "Bit 3 - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<'_, Spi2sI2scfgrSpec> {
        CkpolW::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2sstdW<'_, Spi2sI2scfgrSpec> {
        I2sstdW::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PcmsyncW<'_, Spi2sI2scfgrSpec> {
        PcmsyncW::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive"]
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2scfgW<'_, Spi2sI2scfgrSpec> {
        I2scfgW::new(self, 8)
    }
    #[doc = "Bit 10 - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn i2se(&mut self) -> I2seW<'_, Spi2sI2scfgrSpec> {
        I2seW::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2smodW<'_, Spi2sI2scfgrSpec> {
        I2smodW::new(self, 11)
    }
    #[doc = "Bit 12 - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards."]
    #[inline(always)]
    pub fn astren(&mut self) -> AstrenW<'_, Spi2sI2scfgrSpec> {
        AstrenW::new(self, 12)
    }
}
#[doc = "SPI2S_I2SCFGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2s_i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi2sI2scfgrSpec;
impl crate::RegisterSpec for Spi2sI2scfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi2s_i2scfgr::R`](R) reader structure"]
impl crate::Readable for Spi2sI2scfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi2s_i2scfgr::W`](W) writer structure"]
impl crate::Writable for Spi2sI2scfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI2S_I2SCFGR to value 0"]
impl crate::Resettable for Spi2sI2scfgrSpec {}
