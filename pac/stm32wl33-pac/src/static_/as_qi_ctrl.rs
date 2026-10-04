#[doc = "Register `AS_QI_CTRL` reader"]
pub type R = crate::R<AsQiCtrlSpec>;
#[doc = "Register `AS_QI_CTRL` writer"]
pub type W = crate::W<AsQiCtrlSpec>;
#[doc = "Field `RSSI_THR` reader - Signal detect threshold in 1 dB resolution."]
pub type RssiThrR = crate::FieldReader<u16>;
#[doc = "Field `RSSI_THR` writer - Signal detect threshold in 1 dB resolution."]
pub type RssiThrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PQI_THR` reader - PQI threshold (if 0 then )."]
pub type PqiThrR = crate::FieldReader;
#[doc = "Field `PQI_THR` writer - PQI threshold (if 0 then )."]
pub type PqiThrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CS_MODE` reader - Carrier Sense mode selection"]
pub type CsModeR = crate::FieldReader;
#[doc = "Field `CS_MODE` writer - Carrier Sense mode selection"]
pub type CsModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SQI_EN` reader - SQI enable"]
pub type SqiEnR = crate::BitReader;
#[doc = "Field `SQI_EN` writer - SQI enable"]
pub type SqiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQI_THR` reader - SQI threshold defining the precision requested to detect the SYNC word."]
pub type SqiThrR = crate::FieldReader;
#[doc = "Field `SQI_THR` writer - SQI threshold defining the precision requested to detect the SYNC word."]
pub type SqiThrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AS_EQU_CTRL` reader - ISI cancellation equalizer"]
pub type AsEquCtrlR = crate::FieldReader;
#[doc = "Field `AS_EQU_CTRL` writer - ISI cancellation equalizer"]
pub type AsEquCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AS_MEAS_TIME` reader - Select the RSSI measurement duration during Antenna switching procedure"]
pub type AsMeasTimeR = crate::FieldReader;
#[doc = "Field `AS_MEAS_TIME` writer - Select the RSSI measurement duration during Antenna switching procedure"]
pub type AsMeasTimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AS_CS_BLANKING` reader - Blank received data if signal is below the CS threshold"]
pub type AsCsBlankingR = crate::BitReader;
#[doc = "Field `AS_CS_BLANKING` writer - Blank received data if signal is below the CS threshold"]
pub type AsCsBlankingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Signal detect threshold in 1 dB resolution."]
    #[inline(always)]
    pub fn rssi_thr(&self) -> RssiThrR {
        RssiThrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:12 - PQI threshold (if 0 then )."]
    #[inline(always)]
    pub fn pqi_thr(&self) -> PqiThrR {
        PqiThrR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - Carrier Sense mode selection"]
    #[inline(always)]
    pub fn cs_mode(&self) -> CsModeR {
        CsModeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SQI enable"]
    #[inline(always)]
    pub fn sqi_en(&self) -> SqiEnR {
        SqiEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SQI threshold defining the precision requested to detect the SYNC word."]
    #[inline(always)]
    pub fn sqi_thr(&self) -> SqiThrR {
        SqiThrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 26:27 - ISI cancellation equalizer"]
    #[inline(always)]
    pub fn as_equ_ctrl(&self) -> AsEquCtrlR {
        AsEquCtrlR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Select the RSSI measurement duration during Antenna switching procedure"]
    #[inline(always)]
    pub fn as_meas_time(&self) -> AsMeasTimeR {
        AsMeasTimeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Blank received data if signal is below the CS threshold"]
    #[inline(always)]
    pub fn as_cs_blanking(&self) -> AsCsBlankingR {
        AsCsBlankingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Signal detect threshold in 1 dB resolution."]
    #[inline(always)]
    pub fn rssi_thr(&mut self) -> RssiThrW<'_, AsQiCtrlSpec> {
        RssiThrW::new(self, 0)
    }
    #[doc = "Bits 9:12 - PQI threshold (if 0 then )."]
    #[inline(always)]
    pub fn pqi_thr(&mut self) -> PqiThrW<'_, AsQiCtrlSpec> {
        PqiThrW::new(self, 9)
    }
    #[doc = "Bits 13:14 - Carrier Sense mode selection"]
    #[inline(always)]
    pub fn cs_mode(&mut self) -> CsModeW<'_, AsQiCtrlSpec> {
        CsModeW::new(self, 13)
    }
    #[doc = "Bit 15 - SQI enable"]
    #[inline(always)]
    pub fn sqi_en(&mut self) -> SqiEnW<'_, AsQiCtrlSpec> {
        SqiEnW::new(self, 15)
    }
    #[doc = "Bits 16:18 - SQI threshold defining the precision requested to detect the SYNC word."]
    #[inline(always)]
    pub fn sqi_thr(&mut self) -> SqiThrW<'_, AsQiCtrlSpec> {
        SqiThrW::new(self, 16)
    }
    #[doc = "Bits 26:27 - ISI cancellation equalizer"]
    #[inline(always)]
    pub fn as_equ_ctrl(&mut self) -> AsEquCtrlW<'_, AsQiCtrlSpec> {
        AsEquCtrlW::new(self, 26)
    }
    #[doc = "Bits 28:30 - Select the RSSI measurement duration during Antenna switching procedure"]
    #[inline(always)]
    pub fn as_meas_time(&mut self) -> AsMeasTimeW<'_, AsQiCtrlSpec> {
        AsMeasTimeW::new(self, 28)
    }
    #[doc = "Bit 31 - Blank received data if signal is below the CS threshold"]
    #[inline(always)]
    pub fn as_cs_blanking(&mut self) -> AsCsBlankingW<'_, AsQiCtrlSpec> {
        AsCsBlankingW::new(self, 31)
    }
}
#[doc = "AS_QI_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`as_qi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`as_qi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsQiCtrlSpec;
impl crate::RegisterSpec for AsQiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`as_qi_ctrl::R`](R) reader structure"]
impl crate::Readable for AsQiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`as_qi_ctrl::W`](W) writer structure"]
impl crate::Writable for AsQiCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AS_QI_CTRL to value 0x5800_8028"]
impl crate::Resettable for AsQiCtrlSpec {
    const RESET_VALUE: u32 = 0x5800_8028;
}
