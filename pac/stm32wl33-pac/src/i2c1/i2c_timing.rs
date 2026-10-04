#[doc = "Register `I2C_TIMING` reader"]
pub type R = crate::R<I2cTimingSpec>;
#[doc = "Register `I2C_TIMING` writer"]
pub type W = crate::W<I2cTimingSpec>;
#[doc = "Field `SCLL` reader - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings."]
pub type ScllR = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings."]
pub type ScllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing."]
pub type SclhR = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing."]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDADEL` reader - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing."]
pub type SdadelR = crate::FieldReader;
#[doc = "Field `SDADEL` writer - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing."]
pub type SdadelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLDEL` reader - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing."]
pub type ScldelR = crate::FieldReader;
#[doc = "Field `SCLDEL` writer - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing."]
pub type ScldelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESC` reader - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings."]
    #[inline(always)]
    pub fn scll(&self) -> ScllR {
        ScllR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing."]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing."]
    #[inline(always)]
    pub fn sdadel(&self) -> SdadelR {
        SdadelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing."]
    #[inline(always)]
    pub fn scldel(&self) -> ScldelR {
        ScldelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings."]
    #[inline(always)]
    pub fn scll(&mut self) -> ScllW<'_, I2cTimingSpec> {
        ScllW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing."]
    #[inline(always)]
    pub fn sclh(&mut self) -> SclhW<'_, I2cTimingSpec> {
        SclhW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing."]
    #[inline(always)]
    pub fn sdadel(&mut self) -> SdadelW<'_, I2cTimingSpec> {
        SdadelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing."]
    #[inline(always)]
    pub fn scldel(&mut self) -> ScldelW<'_, I2cTimingSpec> {
        ScldelW::new(self, 20)
    }
    #[doc = "Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, I2cTimingSpec> {
        PrescW::new(self, 28)
    }
}
#[doc = "I2C_TIMING register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cTimingSpec;
impl crate::RegisterSpec for I2cTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_timing::R`](R) reader structure"]
impl crate::Readable for I2cTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_timing::W`](W) writer structure"]
impl crate::Writable for I2cTimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_TIMING to value 0"]
impl crate::Resettable for I2cTimingSpec {}
