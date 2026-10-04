#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rfseq_irq_status: RfseqIrqStatus,
    rfseq_status_detail: RfseqStatusDetail,
    radio_fsm_info: RadioFsmInfo,
    rx_indicator: RxIndicator,
    rx_info_reg: RxInfoReg,
    rx_crc_reg: RxCrcReg,
    qi_info: QiInfo,
    databuffer_info: DatabufferInfo,
    time_capture: TimeCapture,
    iqc_correction_out: IqcCorrectionOut,
    pa_safeask_out: PaSafeaskOut,
    vco_calib_out: VcoCalibOut,
    seq_info: SeqInfo,
    seq_event_status: SeqEventStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - RFSEQ_IRQ_STATUS register"]
    #[inline(always)]
    pub const fn rfseq_irq_status(&self) -> &RfseqIrqStatus {
        &self.rfseq_irq_status
    }
    #[doc = "0x04 - RFSEQ_STATUS_DETAIL register"]
    #[inline(always)]
    pub const fn rfseq_status_detail(&self) -> &RfseqStatusDetail {
        &self.rfseq_status_detail
    }
    #[doc = "0x08 - RADIO_FSM_INFO register"]
    #[inline(always)]
    pub const fn radio_fsm_info(&self) -> &RadioFsmInfo {
        &self.radio_fsm_info
    }
    #[doc = "0x0c - RX_INDICATOR register"]
    #[inline(always)]
    pub const fn rx_indicator(&self) -> &RxIndicator {
        &self.rx_indicator
    }
    #[doc = "0x10 - RX_INFO_REG register"]
    #[inline(always)]
    pub const fn rx_info_reg(&self) -> &RxInfoReg {
        &self.rx_info_reg
    }
    #[doc = "0x14 - RX_CRC_REG register"]
    #[inline(always)]
    pub const fn rx_crc_reg(&self) -> &RxCrcReg {
        &self.rx_crc_reg
    }
    #[doc = "0x18 - QI_INFO register"]
    #[inline(always)]
    pub const fn qi_info(&self) -> &QiInfo {
        &self.qi_info
    }
    #[doc = "0x1c - DATABUFFER_INFO register"]
    #[inline(always)]
    pub const fn databuffer_info(&self) -> &DatabufferInfo {
        &self.databuffer_info
    }
    #[doc = "0x20 - TIME_CAPTURE register"]
    #[inline(always)]
    pub const fn time_capture(&self) -> &TimeCapture {
        &self.time_capture
    }
    #[doc = "0x24 - IQC_CORRECTION_OUT register"]
    #[inline(always)]
    pub const fn iqc_correction_out(&self) -> &IqcCorrectionOut {
        &self.iqc_correction_out
    }
    #[doc = "0x28 - PA_SAFEASK_OUT register"]
    #[inline(always)]
    pub const fn pa_safeask_out(&self) -> &PaSafeaskOut {
        &self.pa_safeask_out
    }
    #[doc = "0x2c - VCO_CALIB_OUT register"]
    #[inline(always)]
    pub const fn vco_calib_out(&self) -> &VcoCalibOut {
        &self.vco_calib_out
    }
    #[doc = "0x30 - SEQ_INFO register"]
    #[inline(always)]
    pub const fn seq_info(&self) -> &SeqInfo {
        &self.seq_info
    }
    #[doc = "0x34 - SEQ_EVENT_STATUS register"]
    #[inline(always)]
    pub const fn seq_event_status(&self) -> &SeqEventStatus {
        &self.seq_event_status
    }
}
#[doc = "RFSEQ_IRQ_STATUS (rw) register accessor: RFSEQ_IRQ_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfseq_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfseq_irq_status`] module"]
#[doc(alias = "RFSEQ_IRQ_STATUS")]
pub type RfseqIrqStatus = crate::Reg<rfseq_irq_status::RfseqIrqStatusSpec>;
#[doc = "RFSEQ_IRQ_STATUS register"]
pub mod rfseq_irq_status;
#[doc = "RFSEQ_STATUS_DETAIL (rw) register accessor: RFSEQ_STATUS_DETAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfseq_status_detail::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_status_detail::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfseq_status_detail`] module"]
#[doc(alias = "RFSEQ_STATUS_DETAIL")]
pub type RfseqStatusDetail = crate::Reg<rfseq_status_detail::RfseqStatusDetailSpec>;
#[doc = "RFSEQ_STATUS_DETAIL register"]
pub mod rfseq_status_detail;
#[doc = "RADIO_FSM_INFO (r) register accessor: RADIO_FSM_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`radio_fsm_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radio_fsm_info`] module"]
#[doc(alias = "RADIO_FSM_INFO")]
pub type RadioFsmInfo = crate::Reg<radio_fsm_info::RadioFsmInfoSpec>;
#[doc = "RADIO_FSM_INFO register"]
pub mod radio_fsm_info;
#[doc = "RX_INDICATOR (r) register accessor: RX_INDICATOR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_indicator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_indicator`] module"]
#[doc(alias = "RX_INDICATOR")]
pub type RxIndicator = crate::Reg<rx_indicator::RxIndicatorSpec>;
#[doc = "RX_INDICATOR register"]
pub mod rx_indicator;
#[doc = "RX_INFO_REG (r) register accessor: RX_INFO_REG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_info_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_info_reg`] module"]
#[doc(alias = "RX_INFO_REG")]
pub type RxInfoReg = crate::Reg<rx_info_reg::RxInfoRegSpec>;
#[doc = "RX_INFO_REG register"]
pub mod rx_info_reg;
#[doc = "RX_CRC_REG (r) register accessor: RX_CRC_REG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_reg`] module"]
#[doc(alias = "RX_CRC_REG")]
pub type RxCrcReg = crate::Reg<rx_crc_reg::RxCrcRegSpec>;
#[doc = "RX_CRC_REG register"]
pub mod rx_crc_reg;
#[doc = "QI_INFO (r) register accessor: QI_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`qi_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qi_info`] module"]
#[doc(alias = "QI_INFO")]
pub type QiInfo = crate::Reg<qi_info::QiInfoSpec>;
#[doc = "QI_INFO register"]
pub mod qi_info;
#[doc = "DATABUFFER_INFO (r) register accessor: DATABUFFER_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databuffer_info`] module"]
#[doc(alias = "DATABUFFER_INFO")]
pub type DatabufferInfo = crate::Reg<databuffer_info::DatabufferInfoSpec>;
#[doc = "DATABUFFER_INFO register"]
pub mod databuffer_info;
#[doc = "TIME_CAPTURE (r) register accessor: TIME_CAPTURE register\n\nYou can [`read`](crate::Reg::read) this register and get [`time_capture::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_capture`] module"]
#[doc(alias = "TIME_CAPTURE")]
pub type TimeCapture = crate::Reg<time_capture::TimeCaptureSpec>;
#[doc = "TIME_CAPTURE register"]
pub mod time_capture;
#[doc = "IQC_CORRECTION_OUT (r) register accessor: IQC_CORRECTION_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_correction_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqc_correction_out`] module"]
#[doc(alias = "IQC_CORRECTION_OUT")]
pub type IqcCorrectionOut = crate::Reg<iqc_correction_out::IqcCorrectionOutSpec>;
#[doc = "IQC_CORRECTION_OUT register"]
pub mod iqc_correction_out;
#[doc = "PA_SAFEASK_OUT (r) register accessor: PA_SAFEASK_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_safeask_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_safeask_out`] module"]
#[doc(alias = "PA_SAFEASK_OUT")]
pub type PaSafeaskOut = crate::Reg<pa_safeask_out::PaSafeaskOutSpec>;
#[doc = "PA_SAFEASK_OUT register"]
pub mod pa_safeask_out;
#[doc = "VCO_CALIB_OUT (r) register accessor: VCO_CALIB_OUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`vco_calib_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vco_calib_out`] module"]
#[doc(alias = "VCO_CALIB_OUT")]
pub type VcoCalibOut = crate::Reg<vco_calib_out::VcoCalibOutSpec>;
#[doc = "VCO_CALIB_OUT register"]
pub mod vco_calib_out;
#[doc = "SEQ_INFO (r) register accessor: SEQ_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_info`] module"]
#[doc(alias = "SEQ_INFO")]
pub type SeqInfo = crate::Reg<seq_info::SeqInfoSpec>;
#[doc = "SEQ_INFO register"]
pub mod seq_info;
#[doc = "SEQ_EVENT_STATUS (r) register accessor: SEQ_EVENT_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_event_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_event_status`] module"]
#[doc(alias = "SEQ_EVENT_STATUS")]
pub type SeqEventStatus = crate::Reg<seq_event_status::SeqEventStatusSpec>;
#[doc = "SEQ_EVENT_STATUS register"]
pub mod seq_event_status;
