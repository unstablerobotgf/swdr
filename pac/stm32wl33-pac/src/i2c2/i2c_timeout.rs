#[doc = "Register `I2C_TIMEOUT` reader"]
pub type R = crate::R<I2cTimeoutSpec>;
#[doc = "Register `I2C_TIMEOUT` writer"]
pub type W = crate::W<I2cTimeoutSpec>;
#[doc = "Field `TIMEOUTA` reader - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
pub type TimeoutaR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
pub type TimeoutaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TIDLE` reader - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0."]
pub type TidleR = crate::BitReader;
#[doc = "Field `TIDLE` writer - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0."]
pub type TidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTEN` reader - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
pub type TimeoutenR = crate::BitReader;
#[doc = "Field `TIMEOUTEN` writer - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
pub type TimeoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` reader - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
pub type TimeoutbR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
pub type TimeoutbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TEXTEN` reader - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
pub type TextenR = crate::BitReader;
#[doc = "Field `TEXTEN` writer - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
pub type TextenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn timeouta(&self) -> TimeoutaR {
        TimeoutaR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn tidle(&self) -> TidleR {
        TidleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn timeouten(&self) -> TimeoutenR {
        TimeoutenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
    #[inline(always)]
    pub fn timeoutb(&self) -> TimeoutbR {
        TimeoutbR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn texten(&self) -> TextenR {
        TextenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, I2cTimeoutSpec> {
        TimeoutaW::new(self, 0)
    }
    #[doc = "Bit 12 - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn tidle(&mut self) -> TidleW<'_, I2cTimeoutSpec> {
        TidleW::new(self, 12)
    }
    #[doc = "Bit 15 - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn timeouten(&mut self) -> TimeoutenW<'_, I2cTimeoutSpec> {
        TimeoutenW::new(self, 15)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, I2cTimeoutSpec> {
        TimeoutbW::new(self, 16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1)."]
    #[inline(always)]
    pub fn texten(&mut self) -> TextenW<'_, I2cTimeoutSpec> {
        TextenW::new(self, 31)
    }
}
#[doc = "I2C_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cTimeoutSpec;
impl crate::RegisterSpec for I2cTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_timeout::R`](R) reader structure"]
impl crate::Readable for I2cTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_timeout::W`](W) writer structure"]
impl crate::Writable for I2cTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_TIMEOUT to value 0"]
impl crate::Resettable for I2cTimeoutSpec {}
