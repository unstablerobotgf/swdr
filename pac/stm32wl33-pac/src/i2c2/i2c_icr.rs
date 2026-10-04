#[doc = "Register `I2C_ICR` reader"]
pub type R = crate::R<I2cIcrSpec>;
#[doc = "Register `I2C_ICR` writer"]
pub type W = crate::W<I2cIcrSpec>;
#[doc = "Field `ADDRCF` writer - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
pub type AddrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
pub type NackcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCF` writer - Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
pub type StopcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRCF` writer - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
pub type BerrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOCF` writer - Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
pub type ArlocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
pub type OvrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECCF` writer - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section 22.3: I2C implementation."]
pub type PeccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section 22.3: I2C implementation."]
pub type TimoutcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTCF` writer - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section 22.3: I2C implementation."]
pub type AlertcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
    #[inline(always)]
    pub fn addrcf(&mut self) -> AddrcfW<'_, I2cIcrSpec> {
        AddrcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
    #[inline(always)]
    pub fn nackcf(&mut self) -> NackcfW<'_, I2cIcrSpec> {
        NackcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn stopcf(&mut self) -> StopcfW<'_, I2cIcrSpec> {
        StopcfW::new(self, 5)
    }
    #[doc = "Bit 8 - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn berrcf(&mut self) -> BerrcfW<'_, I2cIcrSpec> {
        BerrcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn arlocf(&mut self) -> ArlocfW<'_, I2cIcrSpec> {
        ArlocfW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OvrcfW<'_, I2cIcrSpec> {
        OvrcfW::new(self, 10)
    }
    #[doc = "Bit 11 - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section 22.3: I2C implementation."]
    #[inline(always)]
    pub fn peccf(&mut self) -> PeccfW<'_, I2cIcrSpec> {
        PeccfW::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section 22.3: I2C implementation."]
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TimoutcfW<'_, I2cIcrSpec> {
        TimoutcfW::new(self, 12)
    }
    #[doc = "Bit 13 - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section 22.3: I2C implementation."]
    #[inline(always)]
    pub fn alertcf(&mut self) -> AlertcfW<'_, I2cIcrSpec> {
        AlertcfW::new(self, 13)
    }
}
#[doc = "I2C_ICR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cIcrSpec;
impl crate::RegisterSpec for I2cIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_icr::R`](R) reader structure"]
impl crate::Readable for I2cIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_icr::W`](W) writer structure"]
impl crate::Writable for I2cIcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_ICR to value 0"]
impl crate::Resettable for I2cIcrSpec {}
