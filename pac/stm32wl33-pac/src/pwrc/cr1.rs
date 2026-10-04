#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LPMS` reader - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep. - 0: Deep Stop mode (default) - 1: Shutdown mode"]
pub type LpmsR = crate::BitReader;
#[doc = "Field `LPMS` writer - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep. - 0: Deep Stop mode (default) - 1: Shutdown mode"]
pub type LpmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSDNBOR` reader - ENSDNBOR: Enable BOR supply monitoring during shutdown mode. - 1: the PD_ALL_SHUTDOWN signal is not set during SHUTDOWN mode - 0: the PD_ALL_SHUTDOWN signal is set during SHUTDOWN mode."]
pub type EnsdnborR = crate::BitReader;
#[doc = "Field `ENSDNBOR` writer - ENSDNBOR: Enable BOR supply monitoring during shutdown mode. - 1: the PD_ALL_SHUTDOWN signal is not set during SHUTDOWN mode - 0: the PD_ALL_SHUTDOWN signal is set during SHUTDOWN mode."]
pub type EnsdnborW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS_RUN_AUTO` reader - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. - 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) - 1: IBIAS control is automatic (default)."]
pub type IbiasRunAutoR = crate::BitReader;
#[doc = "Field `IBIAS_RUN_AUTO` writer - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. - 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) - 1: IBIAS control is automatic (default)."]
pub type IbiasRunAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS_RUN_STATE` reader - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. - 0: IBIAS control is disabled (default). - 1: IBIAS control is enabled."]
pub type IbiasRunStateR = crate::BitReader;
#[doc = "Field `IBIAS_RUN_STATE` writer - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. - 0: IBIAS control is disabled (default). - 1: IBIAS control is enabled."]
pub type IbiasRunStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APC` reader - APC Apply Pull-up and pull-down configuration from CPU - 1: the I/O pull-up and pull-down configurations defined in the PUCRx and PDCRx registers is applied. - 0: the PUCRx and PDCRx are not used to control the I/O pull-up and pull-down configuration of the product I/Os."]
pub type ApcR = crate::BitReader;
#[doc = "Field `APC` writer - APC Apply Pull-up and pull-down configuration from CPU - 1: the I/O pull-up and pull-down configurations defined in the PUCRx and PDCRx registers is applied. - 0: the PUCRx and PDCRx are not used to control the I/O pull-up and pull-down configuration of the product I/Os."]
pub type ApcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBORH` reader - ENBORH: enable BORH configuration - 1: BORH is enabled, threshold level depends on SELBOR\\[1:0\\] - 0: BORH off (VBOR0): threshold level for above 1.60V voltage operation."]
pub type EnborhR = crate::BitReader;
#[doc = "Field `ENBORH` writer - ENBORH: enable BORH configuration - 1: BORH is enabled, threshold level depends on SELBOR\\[1:0\\] - 0: BORH off (VBOR0): threshold level for above 1.60V voltage operation."]
pub type EnborhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELBORH` reader - SELBORH\\[1:0\\]: BORH selection of Vbor threshold - 11: BORH Level 4(VBOR4): threshold level for above 2.81 V voltage operation. - 10: BORH Level 3 (VBOR3): threshold level for above 2.52 V voltage operation - 01: BORH Level 2 (VBOR2): threshold level for above 2.21 V voltage operation - 00: BORH Level 1 (VBOR1): threshold level for above 2.0V voltage operation."]
pub type SelborhR = crate::FieldReader;
#[doc = "Field `SELBORH` writer - SELBORH\\[1:0\\]: BORH selection of Vbor threshold - 11: BORH Level 4(VBOR4): threshold level for above 2.81 V voltage operation. - 10: BORH Level 3 (VBOR3): threshold level for above 2.52 V voltage operation - 01: BORH Level 2 (VBOR2): threshold level for above 2.21 V voltage operation - 00: BORH Level 1 (VBOR1): threshold level for above 2.0V voltage operation."]
pub type SelborhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENBORL` reader - ENBORL: Enable BORL reset supervising during RUN mode. - 0: No BORL is monitored during RUN mode. - 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN."]
pub type EnborlR = crate::BitReader;
#[doc = "Field `ENBORL` writer - ENBORL: Enable BORL reset supervising during RUN mode. - 0: No BORL is monitored during RUN mode. - 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN."]
pub type EnborlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep. - 0: Deep Stop mode (default) - 1: Shutdown mode"]
    #[inline(always)]
    pub fn lpms(&self) -> LpmsR {
        LpmsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENSDNBOR: Enable BOR supply monitoring during shutdown mode. - 1: the PD_ALL_SHUTDOWN signal is not set during SHUTDOWN mode - 0: the PD_ALL_SHUTDOWN signal is set during SHUTDOWN mode."]
    #[inline(always)]
    pub fn ensdnbor(&self) -> EnsdnborR {
        EnsdnborR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. - 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) - 1: IBIAS control is automatic (default)."]
    #[inline(always)]
    pub fn ibias_run_auto(&self) -> IbiasRunAutoR {
        IbiasRunAutoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. - 0: IBIAS control is disabled (default). - 1: IBIAS control is enabled."]
    #[inline(always)]
    pub fn ibias_run_state(&self) -> IbiasRunStateR {
        IbiasRunStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APC Apply Pull-up and pull-down configuration from CPU - 1: the I/O pull-up and pull-down configurations defined in the PUCRx and PDCRx registers is applied. - 0: the PUCRx and PDCRx are not used to control the I/O pull-up and pull-down configuration of the product I/Os."]
    #[inline(always)]
    pub fn apc(&self) -> ApcR {
        ApcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENBORH: enable BORH configuration - 1: BORH is enabled, threshold level depends on SELBOR\\[1:0\\] - 0: BORH off (VBOR0): threshold level for above 1.60V voltage operation."]
    #[inline(always)]
    pub fn enborh(&self) -> EnborhR {
        EnborhR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SELBORH\\[1:0\\]: BORH selection of Vbor threshold - 11: BORH Level 4(VBOR4): threshold level for above 2.81 V voltage operation. - 10: BORH Level 3 (VBOR3): threshold level for above 2.52 V voltage operation - 01: BORH Level 2 (VBOR2): threshold level for above 2.21 V voltage operation - 00: BORH Level 1 (VBOR1): threshold level for above 2.0V voltage operation."]
    #[inline(always)]
    pub fn selborh(&self) -> SelborhR {
        SelborhR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - ENBORL: Enable BORL reset supervising during RUN mode. - 0: No BORL is monitored during RUN mode. - 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN."]
    #[inline(always)]
    pub fn enborl(&self) -> EnborlR {
        EnborlR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep. - 0: Deep Stop mode (default) - 1: Shutdown mode"]
    #[inline(always)]
    pub fn lpms(&mut self) -> LpmsW<'_, Cr1Spec> {
        LpmsW::new(self, 0)
    }
    #[doc = "Bit 1 - ENSDNBOR: Enable BOR supply monitoring during shutdown mode. - 1: the PD_ALL_SHUTDOWN signal is not set during SHUTDOWN mode - 0: the PD_ALL_SHUTDOWN signal is set during SHUTDOWN mode."]
    #[inline(always)]
    pub fn ensdnbor(&mut self) -> EnsdnborW<'_, Cr1Spec> {
        EnsdnborW::new(self, 1)
    }
    #[doc = "Bit 2 - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. - 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) - 1: IBIAS control is automatic (default)."]
    #[inline(always)]
    pub fn ibias_run_auto(&mut self) -> IbiasRunAutoW<'_, Cr1Spec> {
        IbiasRunAutoW::new(self, 2)
    }
    #[doc = "Bit 3 - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. - 0: IBIAS control is disabled (default). - 1: IBIAS control is enabled."]
    #[inline(always)]
    pub fn ibias_run_state(&mut self) -> IbiasRunStateW<'_, Cr1Spec> {
        IbiasRunStateW::new(self, 3)
    }
    #[doc = "Bit 4 - APC Apply Pull-up and pull-down configuration from CPU - 1: the I/O pull-up and pull-down configurations defined in the PUCRx and PDCRx registers is applied. - 0: the PUCRx and PDCRx are not used to control the I/O pull-up and pull-down configuration of the product I/Os."]
    #[inline(always)]
    pub fn apc(&mut self) -> ApcW<'_, Cr1Spec> {
        ApcW::new(self, 4)
    }
    #[doc = "Bit 5 - ENBORH: enable BORH configuration - 1: BORH is enabled, threshold level depends on SELBOR\\[1:0\\] - 0: BORH off (VBOR0): threshold level for above 1.60V voltage operation."]
    #[inline(always)]
    pub fn enborh(&mut self) -> EnborhW<'_, Cr1Spec> {
        EnborhW::new(self, 5)
    }
    #[doc = "Bits 6:7 - SELBORH\\[1:0\\]: BORH selection of Vbor threshold - 11: BORH Level 4(VBOR4): threshold level for above 2.81 V voltage operation. - 10: BORH Level 3 (VBOR3): threshold level for above 2.52 V voltage operation - 01: BORH Level 2 (VBOR2): threshold level for above 2.21 V voltage operation - 00: BORH Level 1 (VBOR1): threshold level for above 2.0V voltage operation."]
    #[inline(always)]
    pub fn selborh(&mut self) -> SelborhW<'_, Cr1Spec> {
        SelborhW::new(self, 6)
    }
    #[doc = "Bit 8 - ENBORL: Enable BORL reset supervising during RUN mode. - 0: No BORL is monitored during RUN mode. - 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN."]
    #[inline(always)]
    pub fn enborl(&mut self) -> EnborlW<'_, Cr1Spec> {
        EnborlW::new(self, 8)
    }
}
#[doc = "CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0x0114"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0x0114;
}
