#[doc = "Register `RADIO_FSM_INFO` reader"]
pub type R = crate::R<RadioFsmInfoSpec>;
#[doc = "Field `RADIO_FSM_STATE` reader - State of the Radio FSM"]
pub type RadioFsmStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - State of the Radio FSM"]
    #[inline(always)]
    pub fn radio_fsm_state(&self) -> RadioFsmStateR {
        RadioFsmStateR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "RADIO_FSM_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`radio_fsm_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RadioFsmInfoSpec;
impl crate::RegisterSpec for RadioFsmInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`radio_fsm_info::R`](R) reader structure"]
impl crate::Readable for RadioFsmInfoSpec {}
#[doc = "`reset()` method sets RADIO_FSM_INFO to value 0"]
impl crate::Resettable for RadioFsmInfoSpec {}
