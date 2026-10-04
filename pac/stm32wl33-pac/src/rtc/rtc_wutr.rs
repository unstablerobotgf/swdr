#[doc = "Register `RTC_WUTR` reader"]
pub type R = crate::R<RtcWutrSpec>;
#[doc = "Register `RTC_WUTR` writer"]
pub type W = crate::W<RtcWutrSpec>;
#[doc = "Field `WUT` reader - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\] bits of the RTC_CR register When WUCKSEL\\[2\\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\] effectively becomes WUT\\[16\\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\] to 0x0000 with WUCKSEL\\[2:0\\] =011 (RTCCLK/2) is forbidden."]
pub type WutR = crate::FieldReader<u16>;
#[doc = "Field `WUT` writer - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\] bits of the RTC_CR register When WUCKSEL\\[2\\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\] effectively becomes WUT\\[16\\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\] to 0x0000 with WUCKSEL\\[2:0\\] =011 (RTCCLK/2) is forbidden."]
pub type WutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\] bits of the RTC_CR register When WUCKSEL\\[2\\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\] effectively becomes WUT\\[16\\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\] to 0x0000 with WUCKSEL\\[2:0\\] =011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    pub fn wut(&self) -> WutR {
        WutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\] bits of the RTC_CR register When WUCKSEL\\[2\\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\] effectively becomes WUT\\[16\\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\] to 0x0000 with WUCKSEL\\[2:0\\] =011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    pub fn wut(&mut self) -> WutW<'_, RtcWutrSpec> {
        WutW::new(self, 0)
    }
}
#[doc = "RTC_WUTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_wutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcWutrSpec;
impl crate::RegisterSpec for RtcWutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_wutr::R`](R) reader structure"]
impl crate::Readable for RtcWutrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_wutr::W`](W) writer structure"]
impl crate::Writable for RtcWutrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_WUTR to value 0xffff"]
impl crate::Resettable for RtcWutrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
