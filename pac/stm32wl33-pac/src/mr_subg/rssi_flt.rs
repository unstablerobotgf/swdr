#[doc = "Register `RSSI_FLT` reader"]
pub type R = crate::R<RssiFltSpec>;
#[doc = "Register `RSSI_FLT` writer"]
pub type W = crate::W<RssiFltSpec>;
#[doc = "Field `OOK_PEAK_DECAY` reader - Peak decay control for OOK: 3 slow decay; 0 fast decay"]
pub type OokPeakDecayR = crate::FieldReader;
#[doc = "Field `OOK_PEAK_DECAY` writer - Peak decay control for OOK: 3 slow decay; 0 fast decay"]
pub type OokPeakDecayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FREEZE_ON_SYNC_OOK_PEAK_DECAY` reader - Freeze on sync OK peak decay"]
pub type FreezeOnSyncOokPeakDecayR = crate::BitReader;
#[doc = "Field `FREEZE_ON_SYNC_OOK_PEAK_DECAY` writer - Freeze on sync OK peak decay"]
pub type FreezeOnSyncOokPeakDecayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSSI_FLT` reader - Gain of the RSSI filter"]
pub type RssiFltR = crate::FieldReader;
#[doc = "Field `RSSI_FLT` writer - Gain of the RSSI filter"]
pub type RssiFltW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Peak decay control for OOK: 3 slow decay; 0 fast decay"]
    #[inline(always)]
    pub fn ook_peak_decay(&self) -> OokPeakDecayR {
        OokPeakDecayR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Freeze on sync OK peak decay"]
    #[inline(always)]
    pub fn freeze_on_sync_ook_peak_decay(&self) -> FreezeOnSyncOokPeakDecayR {
        FreezeOnSyncOokPeakDecayR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Gain of the RSSI filter"]
    #[inline(always)]
    pub fn rssi_flt(&self) -> RssiFltR {
        RssiFltR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peak decay control for OOK: 3 slow decay; 0 fast decay"]
    #[inline(always)]
    pub fn ook_peak_decay(&mut self) -> OokPeakDecayW<'_, RssiFltSpec> {
        OokPeakDecayW::new(self, 0)
    }
    #[doc = "Bit 3 - Freeze on sync OK peak decay"]
    #[inline(always)]
    pub fn freeze_on_sync_ook_peak_decay(&mut self) -> FreezeOnSyncOokPeakDecayW<'_, RssiFltSpec> {
        FreezeOnSyncOokPeakDecayW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Gain of the RSSI filter"]
    #[inline(always)]
    pub fn rssi_flt(&mut self) -> RssiFltW<'_, RssiFltSpec> {
        RssiFltW::new(self, 4)
    }
}
#[doc = "RSSI_FLT register\n\nYou can [`read`](crate::Reg::read) this register and get [`rssi_flt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rssi_flt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssiFltSpec;
impl crate::RegisterSpec for RssiFltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rssi_flt::R`](R) reader structure"]
impl crate::Readable for RssiFltSpec {}
#[doc = "`write(|w| ..)` method takes [`rssi_flt::W`](W) writer structure"]
impl crate::Writable for RssiFltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSSI_FLT to value 0xe0"]
impl crate::Resettable for RssiFltSpec {
    const RESET_VALUE: u32 = 0xe0;
}
