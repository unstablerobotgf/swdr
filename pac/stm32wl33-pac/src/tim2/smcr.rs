#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS_2_0` reader - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal."]
pub type Sms2_0R = crate::FieldReader;
#[doc = "Field `SMS_2_0` writer - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal."]
pub type Sms2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCCS` reader - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF"]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF"]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_2_0` reader - TS\\[4:0\\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) 00110: Filtered Timer Input 2 (TI2FP2) 00111: External Trigger input (ETRF) Others: Reserved See Table Note:: TIM2 internal trigger connection on page 395 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts2_0R = crate::FieldReader;
#[doc = "Field `TS_2_0` writer - TS\\[4:0\\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) 00110: Filtered Timer Input 2 (TI2FP2) 00111: External Trigger input (ETRF) Others: Reserved See Table Note:: TIM2 internal trigger connection on page 395 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type Ts2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - MSM: Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - MSM: Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - ETF\\[3:0\\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
pub type EtfR = crate::FieldReader;
#[doc = "Field `ETF` writer - ETF\\[3:0\\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
pub type EtfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - ETPS\\[1:0\\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
pub type EtpsR = crate::FieldReader;
#[doc = "Field `ETPS` writer - ETPS\\[1:0\\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
pub type EtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type EceR = crate::BitReader;
#[doc = "Field `ECE` writer - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type EceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge."]
pub type EtpR = crate::BitReader;
#[doc = "Field `ETP` writer - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge."]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_3` reader - SMS\\[3\\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0"]
pub type Sms3R = crate::BitReader;
#[doc = "Field `SMS_3` writer - SMS\\[3\\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0"]
pub type Sms3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_4_3` reader - Trigger selection. See TS_2_0_ description"]
pub type Ts4_3R = crate::FieldReader;
#[doc = "Field `TS_4_3` writer - Trigger selection. See TS_2_0_ description"]
pub type Ts4_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal."]
    #[inline(always)]
    pub fn sms_2_0(&self) -> Sms2_0R {
        Sms2_0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - TS\\[4:0\\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) 00110: Filtered Timer Input 2 (TI2FP2) 00111: External Trigger input (ETRF) Others: Reserved See Table Note:: TIM2 internal trigger connection on page 395 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts_2_0(&self) -> Ts2_0R {
        Ts2_0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MSM: Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ETF\\[3:0\\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn etf(&self) -> EtfR {
        EtfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - ETPS\\[1:0\\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn etps(&self) -> EtpsR {
        EtpsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&self) -> EceR {
        EceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SMS\\[3\\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0"]
    #[inline(always)]
    pub fn sms_3(&self) -> Sms3R {
        Sms3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection. See TS_2_0_ description"]
    #[inline(always)]
    pub fn ts_4_3(&self) -> Ts4_3R {
        Ts4_3R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal."]
    #[inline(always)]
    pub fn sms_2_0(&mut self) -> Sms2_0W<'_, SmcrSpec> {
        Sms2_0W::new(self, 0)
    }
    #[doc = "Bit 3 - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn occs(&mut self) -> OccsW<'_, SmcrSpec> {
        OccsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - TS\\[4:0\\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) 00110: Filtered Timer Input 2 (TI2FP2) 00111: External Trigger input (ETRF) Others: Reserved See Table Note:: TIM2 internal trigger connection on page 395 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts_2_0(&mut self) -> Ts2_0W<'_, SmcrSpec> {
        Ts2_0W::new(self, 4)
    }
    #[doc = "Bit 7 - MSM: Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bits 8:11 - ETF\\[3:0\\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn etf(&mut self) -> EtfW<'_, SmcrSpec> {
        EtfW::new(self, 8)
    }
    #[doc = "Bits 12:13 - ETPS\\[1:0\\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn etps(&mut self) -> EtpsW<'_, SmcrSpec> {
        EtpsW::new(self, 12)
    }
    #[doc = "Bit 14 - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&mut self) -> EceW<'_, SmcrSpec> {
        EceW::new(self, 14)
    }
    #[doc = "Bit 15 - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, SmcrSpec> {
        EtpW::new(self, 15)
    }
    #[doc = "Bit 16 - SMS\\[3\\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0"]
    #[inline(always)]
    pub fn sms_3(&mut self) -> Sms3W<'_, SmcrSpec> {
        Sms3W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection. See TS_2_0_ description"]
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
