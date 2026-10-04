#[doc = "Register `SEQUENCER_CTRL` reader"]
pub type R = crate::R<SequencerCtrlSpec>;
#[doc = "Register `SEQUENCER_CTRL` writer"]
pub type W = crate::W<SequencerCtrlSpec>;
#[doc = "Field `GEN_SEQ_TRIGGER` writer - Action bit: write 1 to generate a trigger event on Sequencer."]
pub type GenSeqTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_SEQ` reader - Enable/disable the Sequencer"]
pub type DisableSeqR = crate::BitReader;
#[doc = "Field `DISABLE_SEQ` writer - Enable/disable the Sequencer"]
pub type DisableSeqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable/disable the Sequencer"]
    #[inline(always)]
    pub fn disable_seq(&self) -> DisableSeqR {
        DisableSeqR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Action bit: write 1 to generate a trigger event on Sequencer."]
    #[inline(always)]
    pub fn gen_seq_trigger(&mut self) -> GenSeqTriggerW<'_, SequencerCtrlSpec> {
        GenSeqTriggerW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable/disable the Sequencer"]
    #[inline(always)]
    pub fn disable_seq(&mut self) -> DisableSeqW<'_, SequencerCtrlSpec> {
        DisableSeqW::new(self, 1)
    }
}
#[doc = "SEQUENCER_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`sequencer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sequencer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SequencerCtrlSpec;
impl crate::RegisterSpec for SequencerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sequencer_ctrl::R`](R) reader structure"]
impl crate::Readable for SequencerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sequencer_ctrl::W`](W) writer structure"]
impl crate::Writable for SequencerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQUENCER_CTRL to value 0"]
impl crate::Resettable for SequencerCtrlSpec {}
