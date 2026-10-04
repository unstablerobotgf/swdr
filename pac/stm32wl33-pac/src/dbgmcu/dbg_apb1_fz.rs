#[doc = "Register `DBG_APB1_FZ` reader"]
pub type R = crate::R<DbgApb1FzSpec>;
#[doc = "Register `DBG_APB1_FZ` writer"]
pub type W = crate::W<DbgApb1FzSpec>;
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode."]
pub type DbgI2c1StopR = crate::BitReader;
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode."]
pub type DbgI2c1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode."]
pub type DbgI2c2StopR = crate::BitReader;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode."]
pub type DbgI2c2StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DbgI2c1StopR {
        DbgI2c1StopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DbgI2c2StopR {
        DbgI2c2StopR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DbgI2c1StopW<'_, DbgApb1FzSpec> {
        DbgI2c1StopW::new(self, 21)
    }
    #[doc = "Bit 23 - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode."]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DbgI2c2StopW<'_, DbgApb1FzSpec> {
        DbgI2c2StopW::new(self, 23)
    }
}
#[doc = "DBG_APB1_FZ register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_apb1_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb1_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgApb1FzSpec;
impl crate::RegisterSpec for DbgApb1FzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb1_fz::R`](R) reader structure"]
impl crate::Readable for DbgApb1FzSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb1_fz::W`](W) writer structure"]
impl crate::Writable for DbgApb1FzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_APB1_FZ to value 0"]
impl crate::Resettable for DbgApb1FzSpec {}
