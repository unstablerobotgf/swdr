#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS_2_0` reader - SMS\\[3:0\\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description."]
pub type Sms2_0R = crate::FieldReader;
#[doc = "Field `SMS_2_0` writer - SMS\\[3:0\\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description."]
pub type Sms2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS_2_0` reader - TS\\[4:0\\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer."]
pub type Ts2_0R = crate::FieldReader;
#[doc = "Field `TS_2_0` writer - TS\\[4:0\\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer."]
pub type Ts2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_3` reader - SMS\\[3:0\\]: Slave mode selection. See SMS_LSB description"]
pub type Sms3R = crate::BitReader;
#[doc = "Field `SMS_3` writer - SMS\\[3:0\\]: Slave mode selection. See SMS_LSB description"]
pub type Sms3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_4_3` reader - TS\\[4:0\\]: Trigger selection. See TS_LSB description"]
pub type Ts4_3R = crate::FieldReader;
#[doc = "Field `TS_4_3` writer - TS\\[4:0\\]: Trigger selection. See TS_LSB description"]
pub type Ts4_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - SMS\\[3:0\\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description."]
    #[inline(always)]
    pub fn sms_2_0(&self) -> Sms2_0R {
        Sms2_0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - TS\\[4:0\\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer."]
    #[inline(always)]
    pub fn ts_2_0(&self) -> Ts2_0R {
        Ts2_0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SMS\\[3:0\\]: Slave mode selection. See SMS_LSB description"]
    #[inline(always)]
    pub fn sms_3(&self) -> Sms3R {
        Sms3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - TS\\[4:0\\]: Trigger selection. See TS_LSB description"]
    #[inline(always)]
    pub fn ts_4_3(&self) -> Ts4_3R {
        Ts4_3R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS\\[3:0\\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description."]
    #[inline(always)]
    pub fn sms_2_0(&mut self) -> Sms2_0W<'_, SmcrSpec> {
        Sms2_0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - TS\\[4:0\\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer."]
    #[inline(always)]
    pub fn ts_2_0(&mut self) -> Ts2_0W<'_, SmcrSpec> {
        Ts2_0W::new(self, 4)
    }
    #[doc = "Bit 7 - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bit 16 - SMS\\[3:0\\]: Slave mode selection. See SMS_LSB description"]
    #[inline(always)]
    pub fn sms_3(&mut self) -> Sms3W<'_, SmcrSpec> {
        Sms3W::new(self, 16)
    }
    #[doc = "Bits 20:21 - TS\\[4:0\\]: Trigger selection. See TS_LSB description"]
    #[inline(always)]
    pub fn ts_4_3(&mut self) -> Ts4_3W<'_, SmcrSpec> {
        Ts4_3W::new(self, 20)
    }
}
#[doc = "SMCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {}
