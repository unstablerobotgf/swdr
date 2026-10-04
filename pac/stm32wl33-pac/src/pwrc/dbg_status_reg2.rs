#[doc = "Register `DBG_STATUS_REG2` reader"]
pub type R = crate::R<DbgStatusReg2Spec>;
#[doc = "Field `PMU_FSM_STATE` reader - PMU_FSM_STATE\\[3:0\\]: Indicates the current state of the PMU FSM inside the PWRC. - 0000: POR - 0001: RUN - 0010: DS ENTRY - 0011: WAIT1 - 0100: WAIT2 - 0101: WAIT - 0110: WAIT3 - 0111: WAIT4 - 1000: ISOLATION - 1001: DEEPSTOP - 1010: SHUTDOWN - 1011: DEEPSTOP EXIT"]
pub type PmuFsmStateR = crate::FieldReader;
#[doc = "Field `RAM_FSM_STATE` reader - RAM_FSM_STATE\\[1:0\\]: Indicates the current state of the RAM FSM inside the PWRC: - 00: POR - 01: POWER UP - 10: READY - 11: OFF"]
pub type RamFsmStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PMU_FSM_STATE\\[3:0\\]: Indicates the current state of the PMU FSM inside the PWRC. - 0000: POR - 0001: RUN - 0010: DS ENTRY - 0011: WAIT1 - 0100: WAIT2 - 0101: WAIT - 0110: WAIT3 - 0111: WAIT4 - 1000: ISOLATION - 1001: DEEPSTOP - 1010: SHUTDOWN - 1011: DEEPSTOP EXIT"]
    #[inline(always)]
    pub fn pmu_fsm_state(&self) -> PmuFsmStateR {
        PmuFsmStateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - RAM_FSM_STATE\\[1:0\\]: Indicates the current state of the RAM FSM inside the PWRC: - 00: POR - 01: POWER UP - 10: READY - 11: OFF"]
    #[inline(always)]
    pub fn ram_fsm_state(&self) -> RamFsmStateR {
        RamFsmStateR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "DBG_STATUS_REG2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_status_reg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgStatusReg2Spec;
impl crate::RegisterSpec for DbgStatusReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_status_reg2::R`](R) reader structure"]
impl crate::Readable for DbgStatusReg2Spec {}
#[doc = "`reset()` method sets DBG_STATUS_REG2 to value 0x0201"]
impl crate::Resettable for DbgStatusReg2Spec {
    const RESET_VALUE: u32 = 0x0201;
}
