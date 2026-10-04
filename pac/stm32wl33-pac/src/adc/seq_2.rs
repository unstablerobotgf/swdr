#[doc = "Register `SEQ_2` reader"]
pub type R = crate::R<Seq2Spec>;
#[doc = "Register `SEQ_2` writer"]
pub type W = crate::W<Seq2Spec>;
#[doc = "Field `SEQ8` reader - SEQ8\\[3:0\\]: channel number code for 9th conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
pub type Seq8R = crate::FieldReader;
#[doc = "Field `SEQ8` writer - SEQ8\\[3:0\\]: channel number code for 9th conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
pub type Seq8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ9` reader - SEQ9\\[3:0\\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq9R = crate::FieldReader;
#[doc = "Field `SEQ9` writer - SEQ9\\[3:0\\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ10` reader - SEQ10\\[3:0\\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq10R = crate::FieldReader;
#[doc = "Field `SEQ10` writer - SEQ10\\[3:0\\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ11` reader - SEQ11\\[3:0\\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq11R = crate::FieldReader;
#[doc = "Field `SEQ11` writer - SEQ11\\[3:0\\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ12` reader - SEQ12\\[3:0\\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq12R = crate::FieldReader;
#[doc = "Field `SEQ12` writer - SEQ12\\[3:0\\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ13` reader - SEQ13\\[3:0\\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq13R = crate::FieldReader;
#[doc = "Field `SEQ13` writer - SEQ13\\[3:0\\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ14` reader - SEQ14\\[3:0\\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq14R = crate::FieldReader;
#[doc = "Field `SEQ14` writer - SEQ14\\[3:0\\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ15` reader - SEQ15\\[3:0\\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq15R = crate::FieldReader;
#[doc = "Field `SEQ15` writer - SEQ15\\[3:0\\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SEQ8\\[3:0\\]: channel number code for 9th conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
    #[inline(always)]
    pub fn seq8(&self) -> Seq8R {
        Seq8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SEQ9\\[3:0\\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq9(&self) -> Seq9R {
        Seq9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SEQ10\\[3:0\\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq10(&self) -> Seq10R {
        Seq10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SEQ11\\[3:0\\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq11(&self) -> Seq11R {
        Seq11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SEQ12\\[3:0\\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq12(&self) -> Seq12R {
        Seq12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SEQ13\\[3:0\\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq13(&self) -> Seq13R {
        Seq13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SEQ14\\[3:0\\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq14(&self) -> Seq14R {
        Seq14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SEQ15\\[3:0\\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq15(&self) -> Seq15R {
        Seq15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEQ8\\[3:0\\]: channel number code for 9th conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
    #[inline(always)]
    pub fn seq8(&mut self) -> Seq8W<'_, Seq2Spec> {
        Seq8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - SEQ9\\[3:0\\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq9(&mut self) -> Seq9W<'_, Seq2Spec> {
        Seq9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SEQ10\\[3:0\\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq10(&mut self) -> Seq10W<'_, Seq2Spec> {
        Seq10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - SEQ11\\[3:0\\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq11(&mut self) -> Seq11W<'_, Seq2Spec> {
        Seq11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - SEQ12\\[3:0\\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq12(&mut self) -> Seq12W<'_, Seq2Spec> {
        Seq12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SEQ13\\[3:0\\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq13(&mut self) -> Seq13W<'_, Seq2Spec> {
        Seq13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - SEQ14\\[3:0\\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq14(&mut self) -> Seq14W<'_, Seq2Spec> {
        Seq14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - SEQ15\\[3:0\\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq15(&mut self) -> Seq15W<'_, Seq2Spec> {
        Seq15W::new(self, 28)
    }
}
#[doc = "SEQ_2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seq2Spec;
impl crate::RegisterSpec for Seq2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_2::R`](R) reader structure"]
impl crate::Readable for Seq2Spec {}
#[doc = "`write(|w| ..)` method takes [`seq_2::W`](W) writer structure"]
impl crate::Writable for Seq2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQ_2 to value 0"]
impl crate::Resettable for Seq2Spec {}
