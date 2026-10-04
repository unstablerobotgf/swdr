#[doc = "Register `RTC_ALRMASSR` reader"]
pub type R = crate::R<RtcAlrmassrSpec>;
#[doc = "Register `RTC_ALRMASSR` writer"]
pub type W = crate::W<RtcAlrmassrSpec>;
#[doc = "Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
pub type SsR = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\\[14:1\\] are dont care in Alarm A comparison. Only SS\\[0\\] is compared. 2: SS\\[14:2\\] are dont care in Alarm A comparison. Only SS\\[1:0\\] are compared. 3: SS\\[14:3\\] are dont care in Alarm A comparison. Only SS\\[2:0\\] are compared. ... 12: SS\\[14:12\\] are dont care in Alarm A comparison. SS\\[11:0\\] are compared. 13: SS\\[14:13\\] are dont care in Alarm A comparison. SS\\[12:0\\] are compared. 14: SS\\[14\\] is dont care in Alarm A comparison. SS\\[13:0\\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssR = crate::FieldReader;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\\[14:1\\] are dont care in Alarm A comparison. Only SS\\[0\\] is compared. 2: SS\\[14:2\\] are dont care in Alarm A comparison. Only SS\\[1:0\\] are compared. 3: SS\\[14:3\\] are dont care in Alarm A comparison. Only SS\\[2:0\\] are compared. ... 12: SS\\[14:12\\] are dont care in Alarm A comparison. SS\\[11:0\\] are compared. 13: SS\\[14:13\\] are dont care in Alarm A comparison. SS\\[12:0\\] are compared. 14: SS\\[14\\] is dont care in Alarm A comparison. SS\\[13:0\\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\\[14:1\\] are dont care in Alarm A comparison. Only SS\\[0\\] is compared. 2: SS\\[14:2\\] are dont care in Alarm A comparison. Only SS\\[1:0\\] are compared. 3: SS\\[14:3\\] are dont care in Alarm A comparison. Only SS\\[2:0\\] are compared. ... 12: SS\\[14:12\\] are dont care in Alarm A comparison. SS\\[11:0\\] are compared. 13: SS\\[14:13\\] are dont care in Alarm A comparison. SS\\[12:0\\] are compared. 14: SS\\[14\\] is dont care in Alarm A comparison. SS\\[13:0\\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MaskssR {
        MaskssR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<'_, RtcAlrmassrSpec> {
        SsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\\[14:1\\] are dont care in Alarm A comparison. Only SS\\[0\\] is compared. 2: SS\\[14:2\\] are dont care in Alarm A comparison. Only SS\\[1:0\\] are compared. 3: SS\\[14:3\\] are dont care in Alarm A comparison. Only SS\\[2:0\\] are compared. ... 12: SS\\[14:12\\] are dont care in Alarm A comparison. SS\\[11:0\\] are compared. 13: SS\\[14:13\\] are dont care in Alarm A comparison. SS\\[12:0\\] are compared. 14: SS\\[14\\] is dont care in Alarm A comparison. SS\\[13:0\\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&mut self) -> MaskssW<'_, RtcAlrmassrSpec> {
        MaskssW::new(self, 24)
    }
}
#[doc = "RTC_ALRMASSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmassr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmassr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrmassrSpec;
impl crate::RegisterSpec for RtcAlrmassrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmassr::R`](R) reader structure"]
impl crate::Readable for RtcAlrmassrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmassr::W`](W) writer structure"]
impl crate::Writable for RtcAlrmassrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ALRMASSR to value 0"]
impl crate::Resettable for RtcAlrmassrSpec {}
