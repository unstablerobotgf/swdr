#[doc = "Register `RF_CONFIG` reader"]
pub type R = crate::R<RfConfigSpec>;
#[doc = "Register `RF_CONFIG` writer"]
pub type W = crate::W<RfConfigSpec>;
#[doc = "Field `ED_SWITCH` reader - - 0 : Normal operation (default)"]
pub type EdSwitchR = crate::BitReader;
#[doc = "Field `ED_SWITCH` writer - - 0 : Normal operation (default)"]
pub type EdSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - Calibrate 4kHz clock (programmable divider)"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Calibrate 4kHz clock (programmable divider)"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AGC_LOW_LVL` reader - AGC level (Low) (default value: 0x2)"]
pub type AgcLowLvlR = crate::FieldReader;
#[doc = "Field `AGC_LOW_LVL` writer - AGC level (Low) (default value: 0x2)"]
pub type AgcLowLvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ED_DC_CTRL` reader - DC current subtraction enabling signal (default value: 0x1)"]
pub type EdDcCtrlR = crate::BitReader;
#[doc = "Field `ED_DC_CTRL` writer - DC current subtraction enabling signal (default value: 0x1)"]
pub type EdDcCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_HIGH_LVL` reader - AGC level (High) (default value: 0x4)"]
pub type AgcHighLvlR = crate::FieldReader;
#[doc = "Field `AGC_HIGH_LVL` writer - AGC level (High) (default value: 0x4)"]
pub type AgcHighLvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ED_ICAL` reader - Current versus VBAT calibration for ED"]
pub type EdIcalR = crate::FieldReader;
#[doc = "Field `ED_ICAL` writer - Current versus VBAT calibration for ED"]
pub type EdIcalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LPF3_CAL` reader - "]
pub type Lpf3CalR = crate::BitReader;
#[doc = "Field `LPF3_CAL` writer - "]
pub type Lpf3CalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - - 0 : Normal operation (default)"]
    #[inline(always)]
    pub fn ed_switch(&self) -> EdSwitchR {
        EdSwitchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Calibrate 4kHz clock (programmable divider)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 11:12 - AGC level (Low) (default value: 0x2)"]
    #[inline(always)]
    pub fn agc_low_lvl(&self) -> AgcLowLvlR {
        AgcLowLvlR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - DC current subtraction enabling signal (default value: 0x1)"]
    #[inline(always)]
    pub fn ed_dc_ctrl(&self) -> EdDcCtrlR {
        EdDcCtrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - AGC level (High) (default value: 0x4)"]
    #[inline(always)]
    pub fn agc_high_lvl(&self) -> AgcHighLvlR {
        AgcHighLvlR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:20 - Current versus VBAT calibration for ED"]
    #[inline(always)]
    pub fn ed_ical(&self) -> EdIcalR {
        EdIcalR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lpf3_cal(&self) -> Lpf3CalR {
        Lpf3CalR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - - 0 : Normal operation (default)"]
    #[inline(always)]
    pub fn ed_switch(&mut self) -> EdSwitchW<'_, RfConfigSpec> {
        EdSwitchW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Calibrate 4kHz clock (programmable divider)"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, RfConfigSpec> {
        ClkdivW::new(self, 1)
    }
    #[doc = "Bits 11:12 - AGC level (Low) (default value: 0x2)"]
    #[inline(always)]
    pub fn agc_low_lvl(&mut self) -> AgcLowLvlW<'_, RfConfigSpec> {
        AgcLowLvlW::new(self, 11)
    }
    #[doc = "Bit 13 - DC current subtraction enabling signal (default value: 0x1)"]
    #[inline(always)]
    pub fn ed_dc_ctrl(&mut self) -> EdDcCtrlW<'_, RfConfigSpec> {
        EdDcCtrlW::new(self, 13)
    }
    #[doc = "Bits 14:17 - AGC level (High) (default value: 0x4)"]
    #[inline(always)]
    pub fn agc_high_lvl(&mut self) -> AgcHighLvlW<'_, RfConfigSpec> {
        AgcHighLvlW::new(self, 14)
    }
    #[doc = "Bits 18:20 - Current versus VBAT calibration for ED"]
    #[inline(always)]
    pub fn ed_ical(&mut self) -> EdIcalW<'_, RfConfigSpec> {
        EdIcalW::new(self, 18)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lpf3_cal(&mut self) -> Lpf3CalW<'_, RfConfigSpec> {
        Lpf3CalW::new(self, 21)
    }
}
#[doc = "RF_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfConfigSpec;
impl crate::RegisterSpec for RfConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_config::R`](R) reader structure"]
impl crate::Readable for RfConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_config::W`](W) writer structure"]
impl crate::Writable for RfConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_CONFIG to value 0x0001_33ee"]
impl crate::Resettable for RfConfigSpec {
    const RESET_VALUE: u32 = 0x0001_33ee;
}
