#[doc = "Register `DBG_STATUS_REG1` reader"]
pub type R = crate::R<DbgStatusReg1Spec>;
#[doc = "Field `SMPS_FSM_STATE` reader - SMPS_FSM_STATE\\[2:0\\]: Indicates the current state of the SMPS FSM inside the PWRC.: - 000: STARTUP - 001: SMPS_REQ - 010: SMPS_RUN - 011: STOP - 100: NOSMPS - 101: PRECHARGE - 110: NOSMPS_BOF"]
pub type SmpsFsmStateR = crate::FieldReader;
#[doc = "Field `FLASH_FSM_STATE` reader - FLASH_FSM_STATE\\[2:0\\]: Indicates the current state of the FLASH FSM inside the PWRC: - 000: STATE1: FLASH POR - 001: STATE2: FLASH PWRUP - 010: STATE3: FLASH READY - 101: STATE4: FLASH SWITCH OFF - 110: STATE5: FLASH PWR DOWN"]
pub type FlashFsmStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - SMPS_FSM_STATE\\[2:0\\]: Indicates the current state of the SMPS FSM inside the PWRC.: - 000: STARTUP - 001: SMPS_REQ - 010: SMPS_RUN - 011: STOP - 100: NOSMPS - 101: PRECHARGE - 110: NOSMPS_BOF"]
    #[inline(always)]
    pub fn smps_fsm_state(&self) -> SmpsFsmStateR {
        SmpsFsmStateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - FLASH_FSM_STATE\\[2:0\\]: Indicates the current state of the FLASH FSM inside the PWRC: - 000: STATE1: FLASH POR - 001: STATE2: FLASH PWRUP - 010: STATE3: FLASH READY - 101: STATE4: FLASH SWITCH OFF - 110: STATE5: FLASH PWR DOWN"]
    #[inline(always)]
    pub fn flash_fsm_state(&self) -> FlashFsmStateR {
        FlashFsmStateR::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "DBG_STATUS_REG1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_status_reg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgStatusReg1Spec;
impl crate::RegisterSpec for DbgStatusReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_status_reg1::R`](R) reader structure"]
impl crate::Readable for DbgStatusReg1Spec {}
#[doc = "`reset()` method sets DBG_STATUS_REG1 to value 0x0202"]
impl crate::Resettable for DbgStatusReg1Spec {
    const RESET_VALUE: u32 = 0x0202;
}
