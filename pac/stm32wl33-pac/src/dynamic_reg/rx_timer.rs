#[doc = "Register `RX_TIMER` reader"]
pub type R = crate::R<RxTimerSpec>;
#[doc = "Register `RX_TIMER` writer"]
pub type W = crate::W<RxTimerSpec>;
#[doc = "Field `RX_TIMEOUT` reader - RX timer timeout (relative duration in interpolated absolute time unit)"]
pub type RxTimeoutR = crate::FieldReader<u32>;
#[doc = "Field `RX_TIMEOUT` writer - RX timer timeout (relative duration in interpolated absolute time unit)"]
pub type RxTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `RX_CS_TIMEOUT_MASK` reader - - 0: CS flag does not contribute to timeout disabling"]
pub type RxCsTimeoutMaskR = crate::BitReader;
#[doc = "Field `RX_CS_TIMEOUT_MASK` writer - - 0: CS flag does not contribute to timeout disabling"]
pub type RxCsTimeoutMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PQI_TIMEOUT_MASK` reader - - 0: PREAMBLE valid flag does not contribute to timeout disabling"]
pub type RxPqiTimeoutMaskR = crate::BitReader;
#[doc = "Field `RX_PQI_TIMEOUT_MASK` writer - - 0: PREAMBLE valid flag does not contribute to timeout disabling"]
pub type RxPqiTimeoutMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SQI_TIMEOUT_MASK` reader - - 0: SYNC valid flag does not contribute to timeout disabling"]
pub type RxSqiTimeoutMaskR = crate::BitReader;
#[doc = "Field `RX_SQI_TIMEOUT_MASK` writer - - 0: SYNC valid flag does not contribute to timeout disabling"]
pub type RxSqiTimeoutMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OR_nAND_SELECT` reader - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask"]
pub type RxOrNAndSelectR = crate::BitReader;
#[doc = "Field `RX_OR_nAND_SELECT` writer - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask"]
pub type RxOrNAndSelectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:22 - RX timer timeout (relative duration in interpolated absolute time unit)"]
    #[inline(always)]
    pub fn rx_timeout(&self) -> RxTimeoutR {
        RxTimeoutR::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bit 28 - - 0: CS flag does not contribute to timeout disabling"]
    #[inline(always)]
    pub fn rx_cs_timeout_mask(&self) -> RxCsTimeoutMaskR {
        RxCsTimeoutMaskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - - 0: PREAMBLE valid flag does not contribute to timeout disabling"]
    #[inline(always)]
    pub fn rx_pqi_timeout_mask(&self) -> RxPqiTimeoutMaskR {
        RxPqiTimeoutMaskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - - 0: SYNC valid flag does not contribute to timeout disabling"]
    #[inline(always)]
    pub fn rx_sqi_timeout_mask(&self) -> RxSqiTimeoutMaskR {
        RxSqiTimeoutMaskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask"]
    #[inline(always)]
    pub fn rx_or_n_and_select(&self) -> RxOrNAndSelectR {
        RxOrNAndSelectR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:22 - RX timer timeout (relative duration in interpolated absolute time unit)"]
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RxTimeoutW<'_, RxTimerSpec> {
        RxTimeoutW::new(self, 0)
    }
    #[doc = "Bit 28 - - 0: CS flag does not contribute to timeout disabling"]
    #[inline(always)]
    pub fn rx_cs_timeout_mask(&mut self) -> RxCsTimeoutMaskW<'_, RxTimerSpec> {
        RxCsTimeoutMaskW::new(self, 28)
    }
    #[doc = "Bit 29 - - 0: PREAMBLE valid flag does not contribute to timeout disabling"]
    #[inline(always)]
    pub fn rx_pqi_timeout_mask(&mut self) -> RxPqiTimeoutMaskW<'_, RxTimerSpec> {
        RxPqiTimeoutMaskW::new(self, 29)
    }
    #[doc = "Bit 30 - - 0: SYNC valid flag does not contribute to timeout disabling"]
    #[inline(always)]
    pub fn rx_sqi_timeout_mask(&mut self) -> RxSqiTimeoutMaskW<'_, RxTimerSpec> {
        RxSqiTimeoutMaskW::new(self, 30)
    }
    #[doc = "Bit 31 - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask"]
    #[inline(always)]
    pub fn rx_or_n_and_select(&mut self) -> RxOrNAndSelectW<'_, RxTimerSpec> {
        RxOrNAndSelectW::new(self, 31)
    }
}
#[doc = "RX_TIMER register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxTimerSpec;
impl crate::RegisterSpec for RxTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_timer::R`](R) reader structure"]
impl crate::Readable for RxTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_timer::W`](W) writer structure"]
impl crate::Writable for RxTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_TIMER to value 0"]
impl crate::Resettable for RxTimerSpec {}
