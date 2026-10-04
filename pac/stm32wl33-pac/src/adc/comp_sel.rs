#[doc = "Register `COMP_SEL` reader"]
pub type R = crate::R<CompSelSpec>;
#[doc = "Register `COMP_SEL` writer"]
pub type W = crate::W<CompSelSpec>;
#[doc = "Field `OFFSET_GAIN0` reader - OFFSET_GAIN0\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain0R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN0` writer - OFFSET_GAIN0\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN1` reader - OFFSET_GAIN1\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V. This field also selects the gain/offset for Temperature Sensor input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain1R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN1` writer - OFFSET_GAIN1\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V. This field also selects the gain/offset for Temperature Sensor input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN2` reader - OFFSET_GAIN2\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain2R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN2` writer - OFFSET_GAIN2\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN3` reader - OFFSET_GAIN3\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain3R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN3` writer - OFFSET_GAIN3\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN4` reader - OFFSET_GAIN4\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain4R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN4` writer - OFFSET_GAIN4\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN5` reader - OFFSET_GAIN5\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain5R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN5` writer - OFFSET_GAIN5\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN6` reader - OFFSET_GAIN6\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V. This field also selects the gain/offset for VBAT input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain6R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN6` writer - OFFSET_GAIN6\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V. This field also selects the gain/offset for VBAT input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN7` reader - OFFSET_GAIN7\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain7R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN7` writer - OFFSET_GAIN7\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSET_GAIN8` reader - OFFSET_GAIN8\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain8R = crate::FieldReader;
#[doc = "Field `OFFSET_GAIN8` writer - OFFSET_GAIN8\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
pub type OffsetGain8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - OFFSET_GAIN0\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain0(&self) -> OffsetGain0R {
        OffsetGain0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - OFFSET_GAIN1\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V. This field also selects the gain/offset for Temperature Sensor input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain1(&self) -> OffsetGain1R {
        OffsetGain1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - OFFSET_GAIN2\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain2(&self) -> OffsetGain2R {
        OffsetGain2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - OFFSET_GAIN3\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain3(&self) -> OffsetGain3R {
        OffsetGain3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - OFFSET_GAIN4\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain4(&self) -> OffsetGain4R {
        OffsetGain4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - OFFSET_GAIN5\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain5(&self) -> OffsetGain5R {
        OffsetGain5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - OFFSET_GAIN6\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V. This field also selects the gain/offset for VBAT input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain6(&self) -> OffsetGain6R {
        OffsetGain6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - OFFSET_GAIN7\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain7(&self) -> OffsetGain7R {
        OffsetGain7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - OFFSET_GAIN8\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain8(&self) -> OffsetGain8R {
        OffsetGain8R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OFFSET_GAIN0\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain0(&mut self) -> OffsetGain0W<'_, CompSelSpec> {
        OffsetGain0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - OFFSET_GAIN1\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V. This field also selects the gain/offset for Temperature Sensor input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain1(&mut self) -> OffsetGain1W<'_, CompSelSpec> {
        OffsetGain1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - OFFSET_GAIN2\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain2(&mut self) -> OffsetGain2W<'_, CompSelSpec> {
        OffsetGain2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - OFFSET_GAIN3\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain3(&mut self) -> OffsetGain3W<'_, CompSelSpec> {
        OffsetGain3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - OFFSET_GAIN4\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain4(&mut self) -> OffsetGain4W<'_, CompSelSpec> {
        OffsetGain4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - OFFSET_GAIN5\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain5(&mut self) -> OffsetGain5W<'_, CompSelSpec> {
        OffsetGain5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - OFFSET_GAIN6\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V. This field also selects the gain/offset for VBAT input:: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain6(&mut self) -> OffsetGain6W<'_, CompSelSpec> {
        OffsetGain6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - OFFSET_GAIN7\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain7(&mut self) -> OffsetGain7W<'_, CompSelSpec> {
        OffsetGain7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - OFFSET_GAIN8\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4"]
    #[inline(always)]
    pub fn offset_gain8(&mut self) -> OffsetGain8W<'_, CompSelSpec> {
        OffsetGain8W::new(self, 16)
    }
}
#[doc = "COMP_SEL register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompSelSpec;
impl crate::RegisterSpec for CompSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_sel::R`](R) reader structure"]
impl crate::Readable for CompSelSpec {}
#[doc = "`write(|w| ..)` method takes [`comp_sel::W`](W) writer structure"]
impl crate::Writable for CompSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_SEL to value 0"]
impl crate::Resettable for CompSelSpec {}
