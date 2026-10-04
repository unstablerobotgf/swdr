#[doc = "Register `RX_INDICATOR` reader"]
pub type R = crate::R<RxIndicatorSpec>;
#[doc = "Field `RSSI_LEVEL_ON_SYNC` reader - RSSI level captured at the end of the SYNC word detection of the received packet."]
pub type RssiLevelOnSyncR = crate::FieldReader<u16>;
#[doc = "Field `RSSI_LEVEL_RUN` reader - Continuous level of the output of the measured RSSI value"]
pub type RssiLevelRunR = crate::FieldReader<u16>;
#[doc = "Field `AGC_WORD` reader - AGC word of the received packet."]
pub type AgcWordR = crate::FieldReader;
#[doc = "Field `ANT_SELECT` reader - Currently selected antenna"]
pub type AntSelectR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RSSI level captured at the end of the SYNC word detection of the received packet."]
    #[inline(always)]
    pub fn rssi_level_on_sync(&self) -> RssiLevelOnSyncR {
        RssiLevelOnSyncR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - Continuous level of the output of the measured RSSI value"]
    #[inline(always)]
    pub fn rssi_level_run(&self) -> RssiLevelRunR {
        RssiLevelRunR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:27 - AGC word of the received packet."]
    #[inline(always)]
    pub fn agc_word(&self) -> AgcWordR {
        AgcWordR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Currently selected antenna"]
    #[inline(always)]
    pub fn ant_select(&self) -> AntSelectR {
        AntSelectR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "RX_INDICATOR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_indicator::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxIndicatorSpec;
impl crate::RegisterSpec for RxIndicatorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_indicator::R`](R) reader structure"]
impl crate::Readable for RxIndicatorSpec {}
#[doc = "`reset()` method sets RX_INDICATOR to value 0"]
impl crate::Resettable for RxIndicatorSpec {}
