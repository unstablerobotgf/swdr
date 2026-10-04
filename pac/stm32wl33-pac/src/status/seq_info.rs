#[doc = "Register `SEQ_INFO` reader"]
pub type R = crate::R<SeqInfoSpec>;
#[doc = "Field `SEQ_FSM_STATE` reader - Current state of the Sequencer"]
pub type SeqFsmStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Current state of the Sequencer"]
    #[inline(always)]
    pub fn seq_fsm_state(&self) -> SeqFsmStateR {
        SeqFsmStateR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "SEQ_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqInfoSpec;
impl crate::RegisterSpec for SeqInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_info::R`](R) reader structure"]
impl crate::Readable for SeqInfoSpec {}
#[doc = "`reset()` method sets SEQ_INFO to value 0"]
impl crate::Resettable for SeqInfoSpec {}
