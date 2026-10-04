#[doc = "Register `RTC_TAMPCR` reader"]
pub type R = crate::R<RtcTampcrSpec>;
#[doc = "Register `RTC_TAMPCR` writer"]
pub type W = crate::W<RtcTampcrSpec>;
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled."]
pub type Tamp1eR = crate::BitReader;
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled."]
pub type Tamp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event."]
pub type Tamp1trgR = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event."]
pub type Tamp1trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled."]
pub type TampieR = crate::BitReader;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled."]
pub type TampieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
pub type TamptsR = crate::BitReader;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
pub type TamptsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
pub type TampfreqR = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
pub type TampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level."]
pub type TampfltR = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level."]
pub type TampfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles"]
pub type TampprchR = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles"]
pub type TampprchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins."]
pub type TamppudisR = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins."]
pub type TamppudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled."]
pub type Tamp1ieR = crate::BitReader;
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled."]
pub type Tamp1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1NOERASE` reader - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers."]
pub type Tamp1noeraseR = crate::BitReader;
#[doc = "Field `TAMP1NOERASE` writer - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers."]
pub type Tamp1noeraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1MF` reader - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased."]
pub type Tamp1mfR = crate::BitReader;
#[doc = "Field `TAMP1MF` writer - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased."]
pub type Tamp1mfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled."]
    #[inline(always)]
    pub fn tamp1e(&self) -> Tamp1eR {
        Tamp1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&self) -> Tamp1trgR {
        Tamp1trgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled."]
    #[inline(always)]
    pub fn tampie(&self) -> TampieR {
        TampieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
    #[inline(always)]
    pub fn tampts(&self) -> TamptsR {
        TamptsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TampfreqR {
        TampfreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level."]
    #[inline(always)]
    pub fn tampflt(&self) -> TampfltR {
        TampfltR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles"]
    #[inline(always)]
    pub fn tampprch(&self) -> TampprchR {
        TampprchR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins."]
    #[inline(always)]
    pub fn tamppudis(&self) -> TamppudisR {
        TamppudisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn tamp1ie(&self) -> Tamp1ieR {
        Tamp1ieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers."]
    #[inline(always)]
    pub fn tamp1noerase(&self) -> Tamp1noeraseR {
        Tamp1noeraseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased."]
    #[inline(always)]
    pub fn tamp1mf(&self) -> Tamp1mfR {
        Tamp1mfR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled."]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> Tamp1eW<'_, RtcTampcrSpec> {
        Tamp1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> Tamp1trgW<'_, RtcTampcrSpec> {
        Tamp1trgW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled."]
    #[inline(always)]
    pub fn tampie(&mut self) -> TampieW<'_, RtcTampcrSpec> {
        TampieW::new(self, 2)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
    #[inline(always)]
    pub fn tampts(&mut self) -> TamptsW<'_, RtcTampcrSpec> {
        TamptsW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TampfreqW<'_, RtcTampcrSpec> {
        TampfreqW::new(self, 8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level."]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TampfltW<'_, RtcTampcrSpec> {
        TampfltW::new(self, 11)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TampprchW<'_, RtcTampcrSpec> {
        TampprchW::new(self, 13)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins."]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TamppudisW<'_, RtcTampcrSpec> {
        TamppudisW::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> Tamp1ieW<'_, RtcTampcrSpec> {
        Tamp1ieW::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers."]
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> Tamp1noeraseW<'_, RtcTampcrSpec> {
        Tamp1noeraseW::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased."]
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> Tamp1mfW<'_, RtcTampcrSpec> {
        Tamp1mfW::new(self, 18)
    }
}
#[doc = "RTC_TAMPCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTampcrSpec;
impl crate::RegisterSpec for RtcTampcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tampcr::R`](R) reader structure"]
impl crate::Readable for RtcTampcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_tampcr::W`](W) writer structure"]
impl crate::Writable for RtcTampcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_TAMPCR to value 0"]
impl crate::Resettable for RtcTampcrSpec {}
