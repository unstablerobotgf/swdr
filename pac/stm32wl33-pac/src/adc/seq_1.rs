#[doc = "Register `SEQ_1` reader"]
pub type R = crate::R<Seq1Spec>;
#[doc = "Register `SEQ_1` writer"]
pub type W = crate::W<Seq1Spec>;
#[doc = "Field `SEQ0` reader - SEQ0\\[3:0\\]: channel number code for first conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
pub type Seq0R = crate::FieldReader;
#[doc = "Field `SEQ0` writer - SEQ0\\[3:0\\]: channel number code for first conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
pub type Seq0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ1` reader - SEQ1\\[3:0\\]: channel number code for second conversion of the sequence. See SEQ0 for code detail."]
pub type Seq1R = crate::FieldReader;
#[doc = "Field `SEQ1` writer - SEQ1\\[3:0\\]: channel number code for second conversion of the sequence. See SEQ0 for code detail."]
pub type Seq1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ2` reader - SEQ2\\[3:0\\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail."]
pub type Seq2R = crate::FieldReader;
#[doc = "Field `SEQ2` writer - SEQ2\\[3:0\\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail."]
pub type Seq2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ3` reader - SEQ3\\[3:0\\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq3R = crate::FieldReader;
#[doc = "Field `SEQ3` writer - SEQ3\\[3:0\\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ4` reader - SEQ4\\[3:0\\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq4R = crate::FieldReader;
#[doc = "Field `SEQ4` writer - SEQ4\\[3:0\\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ5` reader - SEQ5\\[3:0\\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq5R = crate::FieldReader;
#[doc = "Field `SEQ5` writer - SEQ5\\[3:0\\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ6` reader - SEQ6\\[3:0\\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq6R = crate::FieldReader;
#[doc = "Field `SEQ6` writer - SEQ6\\[3:0\\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ7` reader - SEQ7\\[3:0\\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq7R = crate::FieldReader;
#[doc = "Field `SEQ7` writer - SEQ7\\[3:0\\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail."]
pub type Seq7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SEQ0\\[3:0\\]: channel number code for first conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
    #[inline(always)]
    pub fn seq0(&self) -> Seq0R {
        Seq0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SEQ1\\[3:0\\]: channel number code for second conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq1(&self) -> Seq1R {
        Seq1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SEQ2\\[3:0\\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq2(&self) -> Seq2R {
        Seq2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SEQ3\\[3:0\\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq3(&self) -> Seq3R {
        Seq3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SEQ4\\[3:0\\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq4(&self) -> Seq4R {
        Seq4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SEQ5\\[3:0\\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq5(&self) -> Seq5R {
        Seq5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SEQ6\\[3:0\\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq6(&self) -> Seq6R {
        Seq6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SEQ7\\[3:0\\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq7(&self) -> Seq7R {
        Seq7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEQ0\\[3:0\\]: channel number code for first conversion of the sequence 0000: VINM\\[0\\] to ADC single negative input 0001: VINM\\[1\\] to ADC single negative input 0010: VINM\\[2\\] to ADC single negative input 0011: VINM\\[3\\] to ADC single negative input 0100: VINP\\[0\\] to ADC single positive input 0101: VINP\\[1\\] to ADC single positive input 0110: VINP\\[2\\] to ADC single positive input 0111: VINP\\[3\\] to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\] to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\] to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\] to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved"]
    #[inline(always)]
    pub fn seq0(&mut self) -> Seq0W<'_, Seq1Spec> {
        Seq0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - SEQ1\\[3:0\\]: channel number code for second conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq1(&mut self) -> Seq1W<'_, Seq1Spec> {
        Seq1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SEQ2\\[3:0\\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq2(&mut self) -> Seq2W<'_, Seq1Spec> {
        Seq2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - SEQ3\\[3:0\\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq3(&mut self) -> Seq3W<'_, Seq1Spec> {
        Seq3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - SEQ4\\[3:0\\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq4(&mut self) -> Seq4W<'_, Seq1Spec> {
        Seq4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SEQ5\\[3:0\\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq5(&mut self) -> Seq5W<'_, Seq1Spec> {
        Seq5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - SEQ6\\[3:0\\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq6(&mut self) -> Seq6W<'_, Seq1Spec> {
        Seq6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - SEQ7\\[3:0\\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail."]
    #[inline(always)]
    pub fn seq7(&mut self) -> Seq7W<'_, Seq1Spec> {
        Seq7W::new(self, 28)
    }
}
#[doc = "SEQ_1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seq1Spec;
impl crate::RegisterSpec for Seq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_1::R`](R) reader structure"]
impl crate::Readable for Seq1Spec {}
#[doc = "`write(|w| ..)` method takes [`seq_1::W`](W) writer structure"]
impl crate::Writable for Seq1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQ_1 to value 0"]
impl crate::Resettable for Seq1Spec {}
