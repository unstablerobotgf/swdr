#[doc = "Register `RFSEQ_IRQ_STATUS` reader"]
pub type R = crate::R<RfseqIrqStatusSpec>;
#[doc = "Register `RFSEQ_IRQ_STATUS` writer"]
pub type W = crate::W<RfseqIrqStatusSpec>;
#[doc = "Field `TX_DONE_F` reader - Transmission done flag"]
pub type TxDoneFR = crate::BitReader;
#[doc = "Field `TX_DONE_F` writer - Transmission done flag"]
pub type TxDoneFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OK_F` reader - Reception ended and OK flag"]
pub type RxOkFR = crate::BitReader;
#[doc = "Field `RX_OK_F` writer - Reception ended and OK flag"]
pub type RxOkFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TIMEOUT_F` reader - Reception timeout flag"]
pub type RxTimeoutFR = crate::BitReader;
#[doc = "Field `RX_TIMEOUT_F` writer - Reception timeout flag"]
pub type RxTimeoutFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_FRROR_F` reader - Reception with CRC error flag"]
pub type RxCrcFrrorFR = crate::BitReader;
#[doc = "Field `RX_CRC_FRROR_F` writer - Reception with CRC error flag"]
pub type RxCrcFrrorFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_RX_TERM_F` reader - Fast RX Termination flag"]
pub type FastRxTermFR = crate::BitReader;
#[doc = "Field `FAST_RX_TERM_F` writer - Fast RX Termination flag"]
pub type FastRxTermFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTIMER_STOP_CDT_F` reader - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
pub type RxtimerStopCdtFR = crate::BitReader;
#[doc = "Field `RXTIMER_STOP_CDT_F` writer - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
pub type RxtimerStopCdtFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SABORT_DONE_F` reader - SABORT command treated and done flag"]
pub type SabortDoneFR = crate::BitReader;
#[doc = "Field `SABORT_DONE_F` writer - SABORT command treated and done flag"]
pub type SabortDoneFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_REJECTED_F` reader - Command rejection flag."]
pub type CommandRejectedFR = crate::BitReader;
#[doc = "Field `COMMAND_REJECTED_F` writer - Command rejection flag."]
pub type CommandRejectedFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_F` reader - Carrier Sense (RSSI over threshold) flag"]
pub type CsFR = crate::BitReader;
#[doc = "Field `CS_F` writer - Carrier Sense (RSSI over threshold) flag"]
pub type CsFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_VALID_F` reader - Valid PREAMBLE detection flag."]
pub type PreambleValidFR = crate::BitReader;
#[doc = "Field `PREAMBLE_VALID_F` writer - Valid PREAMBLE detection flag."]
pub type PreambleValidFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_VALID_F` reader - Valid SYNC word detection flag."]
pub type SyncValidFR = crate::BitReader;
#[doc = "Field `SYNC_VALID_F` writer - Valid SYNC word detection flag."]
pub type SyncValidFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABUFFER0_USED_F` reader - Data Buffer 0 fully read in TX or fully written in RX flag"]
pub type Databuffer0UsedFR = crate::BitReader;
#[doc = "Field `DATABUFFER0_USED_F` writer - Data Buffer 0 fully read in TX or fully written in RX flag"]
pub type Databuffer0UsedFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABUFFER1_USED_F` reader - Data Buffer 1 fully read in TX or fully written in RX flag"]
pub type Databuffer1UsedFR = crate::BitReader;
#[doc = "Field `DATABUFFER1_USED_F` writer - Data Buffer 1 fully read in TX or fully written in RX flag"]
pub type Databuffer1UsedFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ALMOST_FULL_0_F` reader - Data Buffer0 used (written during a RX) up to programmed thresold flag"]
pub type RxAlmostFull0FR = crate::BitReader;
#[doc = "Field `RX_ALMOST_FULL_0_F` writer - Data Buffer0 used (written during a RX) up to programmed thresold flag"]
pub type RxAlmostFull0FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ALMOST_FULL_1_F` reader - Data Buffer1 used (written during a RX) up to programmed thresold flag"]
pub type RxAlmostFull1FR = crate::BitReader;
#[doc = "Field `RX_ALMOST_FULL_1_F` writer - Data Buffer1 used (written during a RX) up to programmed thresold flag"]
pub type RxAlmostFull1FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ALMOST_EMPTY_0_F` reader - Data Buffer0 used (read during a TX) up to programmed thresold flag"]
pub type TxAlmostEmpty0FR = crate::BitReader;
#[doc = "Field `TX_ALMOST_EMPTY_0_F` writer - Data Buffer0 used (read during a TX) up to programmed thresold flag"]
pub type TxAlmostEmpty0FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ALMOST_EMPTY_1_F` reader - Data Buffer1 used (read during a TX) up to programmed thresold flag"]
pub type TxAlmostEmpty1FR = crate::BitReader;
#[doc = "Field `TX_ALMOST_EMPTY_1_F` writer - Data Buffer1 used (read during a TX) up to programmed thresold flag"]
pub type TxAlmostEmpty1FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_ACCESS_ERROR_F` reader - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer)."]
pub type AhbAccessErrorFR = crate::BitReader;
#[doc = "Field `AHB_ACCESS_ERROR_F` writer - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer)."]
pub type AhbAccessErrorFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_ANA_FAILURE_F` reader - Analog HW failure flag (PLL lock / unlock error, calibration error)"]
pub type HwAnaFailureFR = crate::BitReader;
#[doc = "Field `HW_ANA_FAILURE_F` writer - Analog HW failure flag (PLL lock / unlock error, calibration error)"]
pub type HwAnaFailureFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_F` reader - Sequencer completion flag."]
pub type SeqFR = crate::BitReader;
#[doc = "Field `SEQ_F` writer - Sequencer completion flag."]
pub type SeqFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRM_CMD_START_F` reader - RRM-UDRA command list execution started flag."]
pub type RrmCmdStartFR = crate::BitReader;
#[doc = "Field `RRM_CMD_START_F` writer - RRM-UDRA command list execution started flag."]
pub type RrmCmdStartFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRM_CMD_END_F` reader - RRM-UDRA command list execution ended flag."]
pub type RrmCmdEndFR = crate::BitReader;
#[doc = "Field `RRM_CMD_END_F` writer - RRM-UDRA command list execution ended flag."]
pub type RrmCmdEndFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAFEASK_CALIB_DONE_F` reader - End of Safe-ASK PA calibration flag."]
pub type SafeaskCalibDoneFR = crate::BitReader;
#[doc = "Field `SAFEASK_CALIB_DONE_F` writer - End of Safe-ASK PA calibration flag."]
pub type SafeaskCalibDoneFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_CALIB_DONE_F` reader - Valid RSSI value available in the RSSI_RUNNING bit field flag."]
pub type AgcCalibDoneFR = crate::BitReader;
#[doc = "Field `AGC_CALIB_DONE_F` writer - Valid RSSI value available in the RSSI_RUNNING bit field flag."]
pub type AgcCalibDoneFW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmission done flag"]
    #[inline(always)]
    pub fn tx_done_f(&self) -> TxDoneFR {
        TxDoneFR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception ended and OK flag"]
    #[inline(always)]
    pub fn rx_ok_f(&self) -> RxOkFR {
        RxOkFR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reception timeout flag"]
    #[inline(always)]
    pub fn rx_timeout_f(&self) -> RxTimeoutFR {
        RxTimeoutFR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reception with CRC error flag"]
    #[inline(always)]
    pub fn rx_crc_frror_f(&self) -> RxCrcFrrorFR {
        RxCrcFrrorFR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast RX Termination flag"]
    #[inline(always)]
    pub fn fast_rx_term_f(&self) -> FastRxTermFR {
        FastRxTermFR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
    #[inline(always)]
    pub fn rxtimer_stop_cdt_f(&self) -> RxtimerStopCdtFR {
        RxtimerStopCdtFR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SABORT command treated and done flag"]
    #[inline(always)]
    pub fn sabort_done_f(&self) -> SabortDoneFR {
        SabortDoneFR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Command rejection flag."]
    #[inline(always)]
    pub fn command_rejected_f(&self) -> CommandRejectedFR {
        CommandRejectedFR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Carrier Sense (RSSI over threshold) flag"]
    #[inline(always)]
    pub fn cs_f(&self) -> CsFR {
        CsFR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Valid PREAMBLE detection flag."]
    #[inline(always)]
    pub fn preamble_valid_f(&self) -> PreambleValidFR {
        PreambleValidFR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Valid SYNC word detection flag."]
    #[inline(always)]
    pub fn sync_valid_f(&self) -> SyncValidFR {
        SyncValidFR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Data Buffer 0 fully read in TX or fully written in RX flag"]
    #[inline(always)]
    pub fn databuffer0_used_f(&self) -> Databuffer0UsedFR {
        Databuffer0UsedFR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Buffer 1 fully read in TX or fully written in RX flag"]
    #[inline(always)]
    pub fn databuffer1_used_f(&self) -> Databuffer1UsedFR {
        Databuffer1UsedFR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Buffer0 used (written during a RX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn rx_almost_full_0_f(&self) -> RxAlmostFull0FR {
        RxAlmostFull0FR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Data Buffer1 used (written during a RX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn rx_almost_full_1_f(&self) -> RxAlmostFull1FR {
        RxAlmostFull1FR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Buffer0 used (read during a TX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn tx_almost_empty_0_f(&self) -> TxAlmostEmpty0FR {
        TxAlmostEmpty0FR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data Buffer1 used (read during a TX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn tx_almost_empty_1_f(&self) -> TxAlmostEmpty1FR {
        TxAlmostEmpty1FR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer)."]
    #[inline(always)]
    pub fn ahb_access_error_f(&self) -> AhbAccessErrorFR {
        AhbAccessErrorFR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog HW failure flag (PLL lock / unlock error, calibration error)"]
    #[inline(always)]
    pub fn hw_ana_failure_f(&self) -> HwAnaFailureFR {
        HwAnaFailureFR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Sequencer completion flag."]
    #[inline(always)]
    pub fn seq_f(&self) -> SeqFR {
        SeqFR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RRM-UDRA command list execution started flag."]
    #[inline(always)]
    pub fn rrm_cmd_start_f(&self) -> RrmCmdStartFR {
        RrmCmdStartFR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RRM-UDRA command list execution ended flag."]
    #[inline(always)]
    pub fn rrm_cmd_end_f(&self) -> RrmCmdEndFR {
        RrmCmdEndFR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - End of Safe-ASK PA calibration flag."]
    #[inline(always)]
    pub fn safeask_calib_done_f(&self) -> SafeaskCalibDoneFR {
        SafeaskCalibDoneFR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Valid RSSI value available in the RSSI_RUNNING bit field flag."]
    #[inline(always)]
    pub fn agc_calib_done_f(&self) -> AgcCalibDoneFR {
        AgcCalibDoneFR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission done flag"]
    #[inline(always)]
    pub fn tx_done_f(&mut self) -> TxDoneFW<'_, RfseqIrqStatusSpec> {
        TxDoneFW::new(self, 0)
    }
    #[doc = "Bit 1 - Reception ended and OK flag"]
    #[inline(always)]
    pub fn rx_ok_f(&mut self) -> RxOkFW<'_, RfseqIrqStatusSpec> {
        RxOkFW::new(self, 1)
    }
    #[doc = "Bit 2 - Reception timeout flag"]
    #[inline(always)]
    pub fn rx_timeout_f(&mut self) -> RxTimeoutFW<'_, RfseqIrqStatusSpec> {
        RxTimeoutFW::new(self, 2)
    }
    #[doc = "Bit 3 - Reception with CRC error flag"]
    #[inline(always)]
    pub fn rx_crc_frror_f(&mut self) -> RxCrcFrrorFW<'_, RfseqIrqStatusSpec> {
        RxCrcFrrorFW::new(self, 3)
    }
    #[doc = "Bit 4 - Fast RX Termination flag"]
    #[inline(always)]
    pub fn fast_rx_term_f(&mut self) -> FastRxTermFW<'_, RfseqIrqStatusSpec> {
        FastRxTermFW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
    #[inline(always)]
    pub fn rxtimer_stop_cdt_f(&mut self) -> RxtimerStopCdtFW<'_, RfseqIrqStatusSpec> {
        RxtimerStopCdtFW::new(self, 7)
    }
    #[doc = "Bit 8 - SABORT command treated and done flag"]
    #[inline(always)]
    pub fn sabort_done_f(&mut self) -> SabortDoneFW<'_, RfseqIrqStatusSpec> {
        SabortDoneFW::new(self, 8)
    }
    #[doc = "Bit 9 - Command rejection flag."]
    #[inline(always)]
    pub fn command_rejected_f(&mut self) -> CommandRejectedFW<'_, RfseqIrqStatusSpec> {
        CommandRejectedFW::new(self, 9)
    }
    #[doc = "Bit 12 - Carrier Sense (RSSI over threshold) flag"]
    #[inline(always)]
    pub fn cs_f(&mut self) -> CsFW<'_, RfseqIrqStatusSpec> {
        CsFW::new(self, 12)
    }
    #[doc = "Bit 13 - Valid PREAMBLE detection flag."]
    #[inline(always)]
    pub fn preamble_valid_f(&mut self) -> PreambleValidFW<'_, RfseqIrqStatusSpec> {
        PreambleValidFW::new(self, 13)
    }
    #[doc = "Bit 14 - Valid SYNC word detection flag."]
    #[inline(always)]
    pub fn sync_valid_f(&mut self) -> SyncValidFW<'_, RfseqIrqStatusSpec> {
        SyncValidFW::new(self, 14)
    }
    #[doc = "Bit 16 - Data Buffer 0 fully read in TX or fully written in RX flag"]
    #[inline(always)]
    pub fn databuffer0_used_f(&mut self) -> Databuffer0UsedFW<'_, RfseqIrqStatusSpec> {
        Databuffer0UsedFW::new(self, 16)
    }
    #[doc = "Bit 17 - Data Buffer 1 fully read in TX or fully written in RX flag"]
    #[inline(always)]
    pub fn databuffer1_used_f(&mut self) -> Databuffer1UsedFW<'_, RfseqIrqStatusSpec> {
        Databuffer1UsedFW::new(self, 17)
    }
    #[doc = "Bit 18 - Data Buffer0 used (written during a RX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn rx_almost_full_0_f(&mut self) -> RxAlmostFull0FW<'_, RfseqIrqStatusSpec> {
        RxAlmostFull0FW::new(self, 18)
    }
    #[doc = "Bit 19 - Data Buffer1 used (written during a RX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn rx_almost_full_1_f(&mut self) -> RxAlmostFull1FW<'_, RfseqIrqStatusSpec> {
        RxAlmostFull1FW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Buffer0 used (read during a TX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn tx_almost_empty_0_f(&mut self) -> TxAlmostEmpty0FW<'_, RfseqIrqStatusSpec> {
        TxAlmostEmpty0FW::new(self, 20)
    }
    #[doc = "Bit 21 - Data Buffer1 used (read during a TX) up to programmed thresold flag"]
    #[inline(always)]
    pub fn tx_almost_empty_1_f(&mut self) -> TxAlmostEmpty1FW<'_, RfseqIrqStatusSpec> {
        TxAlmostEmpty1FW::new(self, 21)
    }
    #[doc = "Bit 22 - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer)."]
    #[inline(always)]
    pub fn ahb_access_error_f(&mut self) -> AhbAccessErrorFW<'_, RfseqIrqStatusSpec> {
        AhbAccessErrorFW::new(self, 22)
    }
    #[doc = "Bit 24 - Analog HW failure flag (PLL lock / unlock error, calibration error)"]
    #[inline(always)]
    pub fn hw_ana_failure_f(&mut self) -> HwAnaFailureFW<'_, RfseqIrqStatusSpec> {
        HwAnaFailureFW::new(self, 24)
    }
    #[doc = "Bit 26 - Sequencer completion flag."]
    #[inline(always)]
    pub fn seq_f(&mut self) -> SeqFW<'_, RfseqIrqStatusSpec> {
        SeqFW::new(self, 26)
    }
    #[doc = "Bit 27 - RRM-UDRA command list execution started flag."]
    #[inline(always)]
    pub fn rrm_cmd_start_f(&mut self) -> RrmCmdStartFW<'_, RfseqIrqStatusSpec> {
        RrmCmdStartFW::new(self, 27)
    }
    #[doc = "Bit 28 - RRM-UDRA command list execution ended flag."]
    #[inline(always)]
    pub fn rrm_cmd_end_f(&mut self) -> RrmCmdEndFW<'_, RfseqIrqStatusSpec> {
        RrmCmdEndFW::new(self, 28)
    }
    #[doc = "Bit 30 - End of Safe-ASK PA calibration flag."]
    #[inline(always)]
    pub fn safeask_calib_done_f(&mut self) -> SafeaskCalibDoneFW<'_, RfseqIrqStatusSpec> {
        SafeaskCalibDoneFW::new(self, 30)
    }
    #[doc = "Bit 31 - Valid RSSI value available in the RSSI_RUNNING bit field flag."]
    #[inline(always)]
    pub fn agc_calib_done_f(&mut self) -> AgcCalibDoneFW<'_, RfseqIrqStatusSpec> {
        AgcCalibDoneFW::new(self, 31)
    }
}
#[doc = "RFSEQ_IRQ_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfseq_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfseqIrqStatusSpec;
impl crate::RegisterSpec for RfseqIrqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfseq_irq_status::R`](R) reader structure"]
impl crate::Readable for RfseqIrqStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rfseq_irq_status::W`](W) writer structure"]
impl crate::Writable for RfseqIrqStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFSEQ_IRQ_STATUS to value 0"]
impl crate::Resettable for RfseqIrqStatusSpec {}
