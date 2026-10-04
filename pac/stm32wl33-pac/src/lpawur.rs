#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frame_config0: FrameConfig0,
    frame_config1: FrameConfig1,
    frame_sync_config: FrameSyncConfig,
    rfip_config: RfipConfig,
    rf_config: RfConfig,
    agc_config: AgcConfig,
    _reserved6: [u8; 0x04],
    payload_0: Payload0,
    payload_1: Payload1,
}
impl RegisterBlock {
    #[doc = "0x00 - FRAME_CONFIG0 register"]
    #[inline(always)]
    pub const fn frame_config0(&self) -> &FrameConfig0 {
        &self.frame_config0
    }
    #[doc = "0x04 - FRAME_CONFIG1 register"]
    #[inline(always)]
    pub const fn frame_config1(&self) -> &FrameConfig1 {
        &self.frame_config1
    }
    #[doc = "0x08 - FRAME_SYNC_CONFIG register"]
    #[inline(always)]
    pub const fn frame_sync_config(&self) -> &FrameSyncConfig {
        &self.frame_sync_config
    }
    #[doc = "0x0c - RFIP_CONFIG register"]
    #[inline(always)]
    pub const fn rfip_config(&self) -> &RfipConfig {
        &self.rfip_config
    }
    #[doc = "0x10 - RF_CONFIG register"]
    #[inline(always)]
    pub const fn rf_config(&self) -> &RfConfig {
        &self.rf_config
    }
    #[doc = "0x14 - AGC_CONFIG register"]
    #[inline(always)]
    pub const fn agc_config(&self) -> &AgcConfig {
        &self.agc_config
    }
    #[doc = "0x1c - PAYLOAD_0 register"]
    #[inline(always)]
    pub const fn payload_0(&self) -> &Payload0 {
        &self.payload_0
    }
    #[doc = "0x20 - PAYLOAD_1 register"]
    #[inline(always)]
    pub const fn payload_1(&self) -> &Payload1 {
        &self.payload_1
    }
}
#[doc = "FRAME_CONFIG0 (rw) register accessor: FRAME_CONFIG0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_config0`] module"]
#[doc(alias = "FRAME_CONFIG0")]
pub type FrameConfig0 = crate::Reg<frame_config0::FrameConfig0Spec>;
#[doc = "FRAME_CONFIG0 register"]
pub mod frame_config0;
#[doc = "FRAME_CONFIG1 (rw) register accessor: FRAME_CONFIG1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_config1`] module"]
#[doc(alias = "FRAME_CONFIG1")]
pub type FrameConfig1 = crate::Reg<frame_config1::FrameConfig1Spec>;
#[doc = "FRAME_CONFIG1 register"]
pub mod frame_config1;
#[doc = "FRAME_SYNC_CONFIG (rw) register accessor: FRAME_SYNC_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_sync_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_sync_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_sync_config`] module"]
#[doc(alias = "FRAME_SYNC_CONFIG")]
pub type FrameSyncConfig = crate::Reg<frame_sync_config::FrameSyncConfigSpec>;
#[doc = "FRAME_SYNC_CONFIG register"]
pub mod frame_sync_config;
#[doc = "RFIP_CONFIG (rw) register accessor: RFIP_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfip_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfip_config`] module"]
#[doc(alias = "RFIP_CONFIG")]
pub type RfipConfig = crate::Reg<rfip_config::RfipConfigSpec>;
#[doc = "RFIP_CONFIG register"]
pub mod rfip_config;
#[doc = "RF_CONFIG (rw) register accessor: RF_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_config`] module"]
#[doc(alias = "RF_CONFIG")]
pub type RfConfig = crate::Reg<rf_config::RfConfigSpec>;
#[doc = "RF_CONFIG register"]
pub mod rf_config;
#[doc = "AGC_CONFIG (rw) register accessor: AGC_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agc_config`] module"]
#[doc(alias = "AGC_CONFIG")]
pub type AgcConfig = crate::Reg<agc_config::AgcConfigSpec>;
#[doc = "AGC_CONFIG register"]
pub mod agc_config;
#[doc = "PAYLOAD_0 (r) register accessor: PAYLOAD_0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`payload_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@payload_0`] module"]
#[doc(alias = "PAYLOAD_0")]
pub type Payload0 = crate::Reg<payload_0::Payload0Spec>;
#[doc = "PAYLOAD_0 register"]
pub mod payload_0;
#[doc = "PAYLOAD_1 (r) register accessor: PAYLOAD_1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`payload_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@payload_1`] module"]
#[doc(alias = "PAYLOAD_1")]
pub type Payload1 = crate::Reg<payload_1::Payload1Spec>;
#[doc = "PAYLOAD_1 register"]
pub mod payload_1;
