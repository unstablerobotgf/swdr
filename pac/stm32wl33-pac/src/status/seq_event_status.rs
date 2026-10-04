#[doc = "Register `SEQ_EVENT_STATUS` reader"]
pub type R = crate::R<SeqEventStatusSpec>;
#[doc = "Field `SEQ_EVENT_STATUS` reader - Current value of the seq_event_status used by the Sequencer for next action mask comparison."]
pub type SeqEventStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of the seq_event_status used by the Sequencer for next action mask comparison."]
    #[inline(always)]
    pub fn seq_event_status(&self) -> SeqEventStatusR {
        SeqEventStatusR::new(self.bits)
    }
}
#[doc = "SEQ_EVENT_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_event_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqEventStatusSpec;
impl crate::RegisterSpec for SeqEventStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_event_status::R`](R) reader structure"]
impl crate::Readable for SeqEventStatusSpec {}
#[doc = "`reset()` method sets SEQ_EVENT_STATUS to value 0"]
impl crate::Resettable for SeqEventStatusSpec {}
