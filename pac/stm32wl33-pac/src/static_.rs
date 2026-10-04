#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pckt_config: PcktConfig,
    sync: Sync,
    sec_sync: SecSync,
    crc_init: CrcInit,
    pckt_ctrl: PcktCtrl,
    databuffer0_ptr: Databuffer0Ptr,
    databuffer1_ptr: Databuffer1Ptr,
    databuffer_size: DatabufferSize,
    pa_level_3_0: PaLevel3_0,
    pa_level_7_4: PaLevel7_4,
    pa_config: PaConfig,
    if_ctrl: IfCtrl,
    as_qi_ctrl: AsQiCtrl,
    iqc_config: IqcConfig,
    dsss_ctrl: DsssCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - PCKT_CONFIG register"]
    #[inline(always)]
    pub const fn pckt_config(&self) -> &PcktConfig {
        &self.pckt_config
    }
    #[doc = "0x04 - SYNC register"]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x08 - SEC_SYNC register"]
    #[inline(always)]
    pub const fn sec_sync(&self) -> &SecSync {
        &self.sec_sync
    }
    #[doc = "0x0c - CRC_INIT register"]
    #[inline(always)]
    pub const fn crc_init(&self) -> &CrcInit {
        &self.crc_init
    }
    #[doc = "0x10 - PCKT_CTRL register"]
    #[inline(always)]
    pub const fn pckt_ctrl(&self) -> &PcktCtrl {
        &self.pckt_ctrl
    }
    #[doc = "0x14 - DATABUFFER0_PTR register"]
    #[inline(always)]
    pub const fn databuffer0_ptr(&self) -> &Databuffer0Ptr {
        &self.databuffer0_ptr
    }
    #[doc = "0x18 - DATABUFFER1_PTR register"]
    #[inline(always)]
    pub const fn databuffer1_ptr(&self) -> &Databuffer1Ptr {
        &self.databuffer1_ptr
    }
    #[doc = "0x1c - DATABUFFER_SIZE register"]
    #[inline(always)]
    pub const fn databuffer_size(&self) -> &DatabufferSize {
        &self.databuffer_size
    }
    #[doc = "0x20 - PA_LEVEL_3_0 register"]
    #[inline(always)]
    pub const fn pa_level_3_0(&self) -> &PaLevel3_0 {
        &self.pa_level_3_0
    }
    #[doc = "0x24 - PA_LEVEL_7_4 register"]
    #[inline(always)]
    pub const fn pa_level_7_4(&self) -> &PaLevel7_4 {
        &self.pa_level_7_4
    }
    #[doc = "0x28 - PA_CONFIG register"]
    #[inline(always)]
    pub const fn pa_config(&self) -> &PaConfig {
        &self.pa_config
    }
    #[doc = "0x2c - IF_CTRL register"]
    #[inline(always)]
    pub const fn if_ctrl(&self) -> &IfCtrl {
        &self.if_ctrl
    }
    #[doc = "0x30 - AS_QI_CTRL register"]
    #[inline(always)]
    pub const fn as_qi_ctrl(&self) -> &AsQiCtrl {
        &self.as_qi_ctrl
    }
    #[doc = "0x34 - IQC_CONFIG register"]
    #[inline(always)]
    pub const fn iqc_config(&self) -> &IqcConfig {
        &self.iqc_config
    }
    #[doc = "0x38 - DSSS_CTRL register"]
    #[inline(always)]
    pub const fn dsss_ctrl(&self) -> &DsssCtrl {
        &self.dsss_ctrl
    }
}
#[doc = "PCKT_CONFIG (rw) register accessor: PCKT_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pckt_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pckt_config`] module"]
#[doc(alias = "PCKT_CONFIG")]
pub type PcktConfig = crate::Reg<pckt_config::PcktConfigSpec>;
#[doc = "PCKT_CONFIG register"]
pub mod pckt_config;
#[doc = "SYNC (rw) register accessor: SYNC register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`] module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "SYNC register"]
pub mod sync;
#[doc = "SEC_SYNC (rw) register accessor: SEC_SYNC register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_sync`] module"]
#[doc(alias = "SEC_SYNC")]
pub type SecSync = crate::Reg<sec_sync::SecSyncSpec>;
#[doc = "SEC_SYNC register"]
pub mod sec_sync;
#[doc = "CRC_INIT (rw) register accessor: CRC_INIT register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_init`] module"]
#[doc(alias = "CRC_INIT")]
pub type CrcInit = crate::Reg<crc_init::CrcInitSpec>;
#[doc = "CRC_INIT register"]
pub mod crc_init;
#[doc = "PCKT_CTRL (rw) register accessor: PCKT_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`pckt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pckt_ctrl`] module"]
#[doc(alias = "PCKT_CTRL")]
pub type PcktCtrl = crate::Reg<pckt_ctrl::PcktCtrlSpec>;
#[doc = "PCKT_CTRL register"]
pub mod pckt_ctrl;
#[doc = "DATABUFFER0_PTR (rw) register accessor: DATABUFFER0_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer0_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer0_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databuffer0_ptr`] module"]
#[doc(alias = "DATABUFFER0_PTR")]
pub type Databuffer0Ptr = crate::Reg<databuffer0_ptr::Databuffer0PtrSpec>;
#[doc = "DATABUFFER0_PTR register"]
pub mod databuffer0_ptr;
#[doc = "DATABUFFER1_PTR (rw) register accessor: DATABUFFER1_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer1_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer1_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databuffer1_ptr`] module"]
#[doc(alias = "DATABUFFER1_PTR")]
pub type Databuffer1Ptr = crate::Reg<databuffer1_ptr::Databuffer1PtrSpec>;
#[doc = "DATABUFFER1_PTR register"]
pub mod databuffer1_ptr;
#[doc = "DATABUFFER_SIZE (rw) register accessor: DATABUFFER_SIZE register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databuffer_size`] module"]
#[doc(alias = "DATABUFFER_SIZE")]
pub type DatabufferSize = crate::Reg<databuffer_size::DatabufferSizeSpec>;
#[doc = "DATABUFFER_SIZE register"]
pub mod databuffer_size;
#[doc = "PA_LEVEL_3_0 (rw) register accessor: PA_LEVEL_3_0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_level_3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_level_3_0`] module"]
#[doc(alias = "PA_LEVEL_3_0")]
pub type PaLevel3_0 = crate::Reg<pa_level_3_0::PaLevel3_0Spec>;
#[doc = "PA_LEVEL_3_0 register"]
pub mod pa_level_3_0;
#[doc = "PA_LEVEL_7_4 (rw) register accessor: PA_LEVEL_7_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_level_7_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_7_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_level_7_4`] module"]
#[doc(alias = "PA_LEVEL_7_4")]
pub type PaLevel7_4 = crate::Reg<pa_level_7_4::PaLevel7_4Spec>;
#[doc = "PA_LEVEL_7_4 register"]
pub mod pa_level_7_4;
#[doc = "PA_CONFIG (rw) register accessor: PA_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_config`] module"]
#[doc(alias = "PA_CONFIG")]
pub type PaConfig = crate::Reg<pa_config::PaConfigSpec>;
#[doc = "PA_CONFIG register"]
pub mod pa_config;
#[doc = "IF_CTRL (rw) register accessor: IF_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_ctrl`] module"]
#[doc(alias = "IF_CTRL")]
pub type IfCtrl = crate::Reg<if_ctrl::IfCtrlSpec>;
#[doc = "IF_CTRL register"]
pub mod if_ctrl;
#[doc = "AS_QI_CTRL (rw) register accessor: AS_QI_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`as_qi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`as_qi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@as_qi_ctrl`] module"]
#[doc(alias = "AS_QI_CTRL")]
pub type AsQiCtrl = crate::Reg<as_qi_ctrl::AsQiCtrlSpec>;
#[doc = "AS_QI_CTRL register"]
pub mod as_qi_ctrl;
#[doc = "IQC_CONFIG (rw) register accessor: IQC_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqc_config`] module"]
#[doc(alias = "IQC_CONFIG")]
pub type IqcConfig = crate::Reg<iqc_config::IqcConfigSpec>;
#[doc = "IQC_CONFIG register"]
pub mod iqc_config;
#[doc = "DSSS_CTRL (rw) register accessor: DSSS_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsss_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsss_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsss_ctrl`] module"]
#[doc(alias = "DSSS_CTRL")]
pub type DsssCtrl = crate::Reg<dsss_ctrl::DsssCtrlSpec>;
#[doc = "DSSS_CTRL register"]
pub mod dsss_ctrl;
