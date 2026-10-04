#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBG_SLEEP` reader - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state."]
pub type DbgSleepR = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state."]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state."]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state."]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state."]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DbgSleepR {
        DbgSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state."]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state."]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<'_, CrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state."]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DbgStopW<'_, CrSpec> {
        DbgStopW::new(self, 1)
    }
}
#[doc = "CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
