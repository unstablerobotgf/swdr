#[doc = "Register `RFSEQ_IRQ_ENABLE` reader"]
pub type R = crate::R<RfseqIrqEnableSpec>;
#[doc = "Register `RFSEQ_IRQ_ENABLE` writer"]
pub type W = crate::W<RfseqIrqEnableSpec>;
#[doc = "Field `TX_DONE_E` reader - Enable interrupt on TX_DONE_F flag"]
pub type TxDoneER = crate::BitReader;
#[doc = "Field `TX_DONE_E` writer - Enable interrupt on TX_DONE_F flag"]
pub type TxDoneEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OK_E` reader - Enable interrupt on RX_OK_F flag"]
pub type RxOkER = crate::BitReader;
#[doc = "Field `RX_OK_E` writer - Enable interrupt on RX_OK_F flag"]
pub type RxOkEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TIMEOUT_E` reader - Enable interrupt on RX_TIMEOUT_F flag"]
pub type RxTimeoutER = crate::BitReader;
#[doc = "Field `RX_TIMEOUT_E` writer - Enable interrupt on RX_TIMEOUT_F flag"]
pub type RxTimeoutEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_ERROR_E` reader - Enable interrupt on RX_CRC_ERROR_F flag"]
pub type RxCrcErrorER = crate::BitReader;
#[doc = "Field `RX_CRC_ERROR_E` writer - Enable interrupt on RX_CRC_ERROR_F flag"]
pub type RxCrcErrorEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_RX_TERM_E` reader - Enable interrupt on FAST_RX_TERM_F flag"]
pub type FastRxTermER = crate::BitReader;
#[doc = "Field `FAST_RX_TERM_E` writer - Enable interrupt on FAST_RX_TERM_F flag"]
pub type FastRxTermEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTIMER_STOP_CDT_E` reader - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
pub type RxtimerStopCdtER = crate::BitReader;
#[doc = "Field `RXTIMER_STOP_CDT_E` writer - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
pub type RxtimerStopCdtEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SABORT_DONE_E` reader - Enable interrupt on SABORT command treated and done flag"]
pub type SabortDoneER = crate::BitReader;
#[doc = "Field `SABORT_DONE_E` writer - Enable interrupt on SABORT command treated and done flag"]
pub type SabortDoneEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_REJECTED_E` reader - Enable interrupt on COMMAND_REJECTED flag"]
pub type CommandRejectedER = crate::BitReader;
#[doc = "Field `COMMAND_REJECTED_E` writer - Enable interrupt on COMMAND_REJECTED flag"]
pub type CommandRejectedEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_E` reader - Enable interrupt on CS_F flag"]
pub type CsER = crate::BitReader;
#[doc = "Field `CS_E` writer - Enable interrupt on CS_F flag"]
pub type CsEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_VALID_E` reader - Enable interrupt on PREAMBLE_VALID_F flag"]
pub type PreambleValidER = crate::BitReader;
#[doc = "Field `PREAMBLE_VALID_E` writer - Enable interrupt on PREAMBLE_VALID_F flag"]
pub type PreambleValidEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_VALID_E` reader - Enable interrupt on SYNC_VALID_F flag"]
pub type SyncValidER = crate::BitReader;
#[doc = "Field `SYNC_VALID_E` writer - Enable interrupt on SYNC_VALID_F flag"]
pub type SyncValidEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABUFFER0_USED_E` reader - Enable interrupt on DATABUFFER0_USED_F flag"]
pub type Databuffer0UsedER = crate::BitReader;
#[doc = "Field `DATABUFFER0_USED_E` writer - Enable interrupt on DATABUFFER0_USED_F flag"]
pub type Databuffer0UsedEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATABUFFER1_USED_E` reader - Enable interrupt on DATABUFFER1_USED_F flag"]
pub type Databuffer1UsedER = crate::BitReader;
#[doc = "Field `DATABUFFER1_USED_E` writer - Enable interrupt on DATABUFFER1_USED_F flag"]
pub type Databuffer1UsedEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ALMOST_FULL_0_E` reader - Enable interrupt on RX_ALMOST_FULL_0_F flag"]
pub type RxAlmostFull0ER = crate::BitReader;
#[doc = "Field `RX_ALMOST_FULL_0_E` writer - Enable interrupt on RX_ALMOST_FULL_0_F flag"]
pub type RxAlmostFull0EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ALMOST_FULL_1_E` reader - Enable interrupt on RX_ALMOST_FULL_1_F flag"]
pub type RxAlmostFull1ER = crate::BitReader;
#[doc = "Field `RX_ALMOST_FULL_1_E` writer - Enable interrupt on RX_ALMOST_FULL_1_F flag"]
pub type RxAlmostFull1EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ALMOST_EMPTY_0_E` reader - Enable interrupt on TX_ALMOST_EMPTY_0_F flag"]
pub type TxAlmostEmpty0ER = crate::BitReader;
#[doc = "Field `TX_ALMOST_EMPTY_0_E` writer - Enable interrupt on TX_ALMOST_EMPTY_0_F flag"]
pub type TxAlmostEmpty0EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ALMOST_EMPTY_1_E` reader - Enable interrupt on TX_ALMOST_EMPTY_1_F flag"]
pub type TxAlmostEmpty1ER = crate::BitReader;
#[doc = "Field `TX_ALMOST_EMPTY_1_E` writer - Enable interrupt on TX_ALMOST_EMPTY_1_F flag"]
pub type TxAlmostEmpty1EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_ACCESS_ERROR_E` reader - Enable interrupt on AHB_ACCESS_ERROR_F flag"]
pub type AhbAccessErrorER = crate::BitReader;
#[doc = "Field `AHB_ACCESS_ERROR_E` writer - Enable interrupt on AHB_ACCESS_ERROR_F flag"]
pub type AhbAccessErrorEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_ANA_FAILURE_E` reader - Enable interrupt on HW_ANA_FAILURE_F flag"]
pub type HwAnaFailureER = crate::BitReader;
#[doc = "Field `HW_ANA_FAILURE_E` writer - Enable interrupt on HW_ANA_FAILURE_F flag"]
pub type HwAnaFailureEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_E` reader - Enable interrupt on SEQ_F flag"]
pub type SeqER = crate::BitReader;
#[doc = "Field `SEQ_E` writer - Enable interrupt on SEQ_F flag"]
pub type SeqEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRM_CMD_START_E` reader - Enable interrupt on RRM_CMD_END_F flag"]
pub type RrmCmdStartER = crate::BitReader;
#[doc = "Field `RRM_CMD_START_E` writer - Enable interrupt on RRM_CMD_END_F flag"]
pub type RrmCmdStartEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRM_CMD_END_E` reader - Enable interrupt on RRM_CMD_END_F flag"]
pub type RrmCmdEndER = crate::BitReader;
#[doc = "Field `RRM_CMD_END_E` writer - Enable interrupt on RRM_CMD_END_F flag"]
pub type RrmCmdEndEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAFEASK_CALIB_DONE_E` reader - Enable interrupt on SAFEASK_CALIB_DONE_F flag"]
pub type SafeaskCalibDoneER = crate::BitReader;
#[doc = "Field `SAFEASK_CALIB_DONE_E` writer - Enable interrupt on SAFEASK_CALIB_DONE_F flag"]
pub type SafeaskCalibDoneEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_CALIB_DONE_E` reader - Enable interrupt on AGC_CALIB_DONE_F flag"]
pub type AgcCalibDoneER = crate::BitReader;
#[doc = "Field `AGC_CALIB_DONE_E` writer - Enable interrupt on AGC_CALIB_DONE_F flag"]
pub type AgcCalibDoneEW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable interrupt on TX_DONE_F flag"]
    #[inline(always)]
    pub fn tx_done_e(&self) -> TxDoneER {
        TxDoneER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt on RX_OK_F flag"]
    #[inline(always)]
    pub fn rx_ok_e(&self) -> RxOkER {
        RxOkER::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt on RX_TIMEOUT_F flag"]
    #[inline(always)]
    pub fn rx_timeout_e(&self) -> RxTimeoutER {
        RxTimeoutER::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable interrupt on RX_CRC_ERROR_F flag"]
    #[inline(always)]
    pub fn rx_crc_error_e(&self) -> RxCrcErrorER {
        RxCrcErrorER::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable interrupt on FAST_RX_TERM_F flag"]
    #[inline(always)]
    pub fn fast_rx_term_e(&self) -> FastRxTermER {
        FastRxTermER::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
    #[inline(always)]
    pub fn rxtimer_stop_cdt_e(&self) -> RxtimerStopCdtER {
        RxtimerStopCdtER::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable interrupt on SABORT command treated and done flag"]
    #[inline(always)]
    pub fn sabort_done_e(&self) -> SabortDoneER {
        SabortDoneER::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable interrupt on COMMAND_REJECTED flag"]
    #[inline(always)]
    pub fn command_rejected_e(&self) -> CommandRejectedER {
        CommandRejectedER::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable interrupt on CS_F flag"]
    #[inline(always)]
    pub fn cs_e(&self) -> CsER {
        CsER::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable interrupt on PREAMBLE_VALID_F flag"]
    #[inline(always)]
    pub fn preamble_valid_e(&self) -> PreambleValidER {
        PreambleValidER::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable interrupt on SYNC_VALID_F flag"]
    #[inline(always)]
    pub fn sync_valid_e(&self) -> SyncValidER {
        SyncValidER::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable interrupt on DATABUFFER0_USED_F flag"]
    #[inline(always)]
    pub fn databuffer0_used_e(&self) -> Databuffer0UsedER {
        Databuffer0UsedER::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable interrupt on DATABUFFER1_USED_F flag"]
    #[inline(always)]
    pub fn databuffer1_used_e(&self) -> Databuffer1UsedER {
        Databuffer1UsedER::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable interrupt on RX_ALMOST_FULL_0_F flag"]
    #[inline(always)]
    pub fn rx_almost_full_0_e(&self) -> RxAlmostFull0ER {
        RxAlmostFull0ER::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable interrupt on RX_ALMOST_FULL_1_F flag"]
    #[inline(always)]
    pub fn rx_almost_full_1_e(&self) -> RxAlmostFull1ER {
        RxAlmostFull1ER::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable interrupt on TX_ALMOST_EMPTY_0_F flag"]
    #[inline(always)]
    pub fn tx_almost_empty_0_e(&self) -> TxAlmostEmpty0ER {
        TxAlmostEmpty0ER::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable interrupt on TX_ALMOST_EMPTY_1_F flag"]
    #[inline(always)]
    pub fn tx_almost_empty_1_e(&self) -> TxAlmostEmpty1ER {
        TxAlmostEmpty1ER::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable interrupt on AHB_ACCESS_ERROR_F flag"]
    #[inline(always)]
    pub fn ahb_access_error_e(&self) -> AhbAccessErrorER {
        AhbAccessErrorER::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable interrupt on HW_ANA_FAILURE_F flag"]
    #[inline(always)]
    pub fn hw_ana_failure_e(&self) -> HwAnaFailureER {
        HwAnaFailureER::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable interrupt on SEQ_F flag"]
    #[inline(always)]
    pub fn seq_e(&self) -> SeqER {
        SeqER::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable interrupt on RRM_CMD_END_F flag"]
    #[inline(always)]
    pub fn rrm_cmd_start_e(&self) -> RrmCmdStartER {
        RrmCmdStartER::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable interrupt on RRM_CMD_END_F flag"]
    #[inline(always)]
    pub fn rrm_cmd_end_e(&self) -> RrmCmdEndER {
        RrmCmdEndER::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable interrupt on SAFEASK_CALIB_DONE_F flag"]
    #[inline(always)]
    pub fn safeask_calib_done_e(&self) -> SafeaskCalibDoneER {
        SafeaskCalibDoneER::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable interrupt on AGC_CALIB_DONE_F flag"]
    #[inline(always)]
    pub fn agc_calib_done_e(&self) -> AgcCalibDoneER {
        AgcCalibDoneER::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on TX_DONE_F flag"]
    #[inline(always)]
    pub fn tx_done_e(&mut self) -> TxDoneEW<'_, RfseqIrqEnableSpec> {
        TxDoneEW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt on RX_OK_F flag"]
    #[inline(always)]
    pub fn rx_ok_e(&mut self) -> RxOkEW<'_, RfseqIrqEnableSpec> {
        RxOkEW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable interrupt on RX_TIMEOUT_F flag"]
    #[inline(always)]
    pub fn rx_timeout_e(&mut self) -> RxTimeoutEW<'_, RfseqIrqEnableSpec> {
        RxTimeoutEW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable interrupt on RX_CRC_ERROR_F flag"]
    #[inline(always)]
    pub fn rx_crc_error_e(&mut self) -> RxCrcErrorEW<'_, RfseqIrqEnableSpec> {
        RxCrcErrorEW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable interrupt on FAST_RX_TERM_F flag"]
    #[inline(always)]
    pub fn fast_rx_term_e(&mut self) -> FastRxTermEW<'_, RfseqIrqEnableSpec> {
        FastRxTermEW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag"]
    #[inline(always)]
    pub fn rxtimer_stop_cdt_e(&mut self) -> RxtimerStopCdtEW<'_, RfseqIrqEnableSpec> {
        RxtimerStopCdtEW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable interrupt on SABORT command treated and done flag"]
    #[inline(always)]
    pub fn sabort_done_e(&mut self) -> SabortDoneEW<'_, RfseqIrqEnableSpec> {
        SabortDoneEW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable interrupt on COMMAND_REJECTED flag"]
    #[inline(always)]
    pub fn command_rejected_e(&mut self) -> CommandRejectedEW<'_, RfseqIrqEnableSpec> {
        CommandRejectedEW::new(self, 9)
    }
    #[doc = "Bit 12 - Enable interrupt on CS_F flag"]
    #[inline(always)]
    pub fn cs_e(&mut self) -> CsEW<'_, RfseqIrqEnableSpec> {
        CsEW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable interrupt on PREAMBLE_VALID_F flag"]
    #[inline(always)]
    pub fn preamble_valid_e(&mut self) -> PreambleValidEW<'_, RfseqIrqEnableSpec> {
        PreambleValidEW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable interrupt on SYNC_VALID_F flag"]
    #[inline(always)]
    pub fn sync_valid_e(&mut self) -> SyncValidEW<'_, RfseqIrqEnableSpec> {
        SyncValidEW::new(self, 14)
    }
    #[doc = "Bit 16 - Enable interrupt on DATABUFFER0_USED_F flag"]
    #[inline(always)]
    pub fn databuffer0_used_e(&mut self) -> Databuffer0UsedEW<'_, RfseqIrqEnableSpec> {
        Databuffer0UsedEW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable interrupt on DATABUFFER1_USED_F flag"]
    #[inline(always)]
    pub fn databuffer1_used_e(&mut self) -> Databuffer1UsedEW<'_, RfseqIrqEnableSpec> {
        Databuffer1UsedEW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable interrupt on RX_ALMOST_FULL_0_F flag"]
    #[inline(always)]
    pub fn rx_almost_full_0_e(&mut self) -> RxAlmostFull0EW<'_, RfseqIrqEnableSpec> {
        RxAlmostFull0EW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable interrupt on RX_ALMOST_FULL_1_F flag"]
    #[inline(always)]
    pub fn rx_almost_full_1_e(&mut self) -> RxAlmostFull1EW<'_, RfseqIrqEnableSpec> {
        RxAlmostFull1EW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable interrupt on TX_ALMOST_EMPTY_0_F flag"]
    #[inline(always)]
    pub fn tx_almost_empty_0_e(&mut self) -> TxAlmostEmpty0EW<'_, RfseqIrqEnableSpec> {
        TxAlmostEmpty0EW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable interrupt on TX_ALMOST_EMPTY_1_F flag"]
    #[inline(always)]
    pub fn tx_almost_empty_1_e(&mut self) -> TxAlmostEmpty1EW<'_, RfseqIrqEnableSpec> {
        TxAlmostEmpty1EW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable interrupt on AHB_ACCESS_ERROR_F flag"]
    #[inline(always)]
    pub fn ahb_access_error_e(&mut self) -> AhbAccessErrorEW<'_, RfseqIrqEnableSpec> {
        AhbAccessErrorEW::new(self, 22)
    }
    #[doc = "Bit 24 - Enable interrupt on HW_ANA_FAILURE_F flag"]
    #[inline(always)]
    pub fn hw_ana_failure_e(&mut self) -> HwAnaFailureEW<'_, RfseqIrqEnableSpec> {
        HwAnaFailureEW::new(self, 24)
    }
    #[doc = "Bit 26 - Enable interrupt on SEQ_F flag"]
    #[inline(always)]
    pub fn seq_e(&mut self) -> SeqEW<'_, RfseqIrqEnableSpec> {
        SeqEW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable interrupt on RRM_CMD_END_F flag"]
    #[inline(always)]
    pub fn rrm_cmd_start_e(&mut self) -> RrmCmdStartEW<'_, RfseqIrqEnableSpec> {
        RrmCmdStartEW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable interrupt on RRM_CMD_END_F flag"]
    #[inline(always)]
    pub fn rrm_cmd_end_e(&mut self) -> RrmCmdEndEW<'_, RfseqIrqEnableSpec> {
        RrmCmdEndEW::new(self, 28)
    }
    #[doc = "Bit 30 - Enable interrupt on SAFEASK_CALIB_DONE_F flag"]
    #[inline(always)]
    pub fn safeask_calib_done_e(&mut self) -> SafeaskCalibDoneEW<'_, RfseqIrqEnableSpec> {
        SafeaskCalibDoneEW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable interrupt on AGC_CALIB_DONE_F flag"]
    #[inline(always)]
    pub fn agc_calib_done_e(&mut self) -> AgcCalibDoneEW<'_, RfseqIrqEnableSpec> {
        AgcCalibDoneEW::new(self, 31)
    }
}
#[doc = "RFSEQ_IRQ_ENABLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfseq_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfseqIrqEnableSpec;
impl crate::RegisterSpec for RfseqIrqEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfseq_irq_enable::R`](R) reader structure"]
impl crate::Readable for RfseqIrqEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`rfseq_irq_enable::W`](W) writer structure"]
impl crate::Writable for RfseqIrqEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFSEQ_IRQ_ENABLE to value 0"]
impl crate::Resettable for RfseqIrqEnableSpec {}
