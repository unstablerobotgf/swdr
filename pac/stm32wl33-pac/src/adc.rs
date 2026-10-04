#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version_id: VersionId,
    conf: Conf,
    ctrl: Ctrl,
    _reserved3: [u8; 0x08],
    switch: Switch,
    _reserved4: [u8; 0x04],
    ds_conf: DsConf,
    seq_1: Seq1,
    seq_2: Seq2,
    comp_1: Comp1,
    comp_2: Comp2,
    comp_3: Comp3,
    comp_4: Comp4,
    comp_sel: CompSel,
    wd_th: WdTh,
    wd_conf: WdConf,
    ds_dataout: DsDataout,
    _reserved15: [u8; 0x04],
    irq_status: IrqStatus,
    irq_enable: IrqEnable,
    _reserved17: [u8; 0x0c],
    test_conf: TestConf,
    dtb_conf: DtbConf,
}
impl RegisterBlock {
    #[doc = "0x00 - VERSION_ID register"]
    #[inline(always)]
    pub const fn version_id(&self) -> &VersionId {
        &self.version_id
    }
    #[doc = "0x04 - CONF register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x08 - CTRL register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x14 - SWITCH register"]
    #[inline(always)]
    pub const fn switch(&self) -> &Switch {
        &self.switch
    }
    #[doc = "0x1c - DS_CONF register"]
    #[inline(always)]
    pub const fn ds_conf(&self) -> &DsConf {
        &self.ds_conf
    }
    #[doc = "0x20 - SEQ_1 register"]
    #[inline(always)]
    pub const fn seq_1(&self) -> &Seq1 {
        &self.seq_1
    }
    #[doc = "0x24 - SEQ_2 register"]
    #[inline(always)]
    pub const fn seq_2(&self) -> &Seq2 {
        &self.seq_2
    }
    #[doc = "0x28 - COMP_1 register"]
    #[inline(always)]
    pub const fn comp_1(&self) -> &Comp1 {
        &self.comp_1
    }
    #[doc = "0x2c - COMP_2 register"]
    #[inline(always)]
    pub const fn comp_2(&self) -> &Comp2 {
        &self.comp_2
    }
    #[doc = "0x30 - COMP_3 register"]
    #[inline(always)]
    pub const fn comp_3(&self) -> &Comp3 {
        &self.comp_3
    }
    #[doc = "0x34 - COMP_4 register"]
    #[inline(always)]
    pub const fn comp_4(&self) -> &Comp4 {
        &self.comp_4
    }
    #[doc = "0x38 - COMP_SEL register"]
    #[inline(always)]
    pub const fn comp_sel(&self) -> &CompSel {
        &self.comp_sel
    }
    #[doc = "0x3c - WD_TH register"]
    #[inline(always)]
    pub const fn wd_th(&self) -> &WdTh {
        &self.wd_th
    }
    #[doc = "0x40 - WD_CONF register"]
    #[inline(always)]
    pub const fn wd_conf(&self) -> &WdConf {
        &self.wd_conf
    }
    #[doc = "0x44 - DS_DATAOUT register"]
    #[inline(always)]
    pub const fn ds_dataout(&self) -> &DsDataout {
        &self.ds_dataout
    }
    #[doc = "0x4c - IRQ_STATUS register"]
    #[inline(always)]
    pub const fn irq_status(&self) -> &IrqStatus {
        &self.irq_status
    }
    #[doc = "0x50 - IRQ_ENABLE register"]
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IrqEnable {
        &self.irq_enable
    }
    #[doc = "0x60 - TEST_CONF register"]
    #[inline(always)]
    pub const fn test_conf(&self) -> &TestConf {
        &self.test_conf
    }
    #[doc = "0x64 - DTB_CONF register"]
    #[inline(always)]
    pub const fn dtb_conf(&self) -> &DtbConf {
        &self.dtb_conf
    }
}
#[doc = "VERSION_ID (r) register accessor: VERSION_ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`version_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_id`] module"]
#[doc(alias = "VERSION_ID")]
pub type VersionId = crate::Reg<version_id::VersionIdSpec>;
#[doc = "VERSION_ID register"]
pub mod version_id;
#[doc = "CONF (rw) register accessor: CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "CONF register"]
pub mod conf;
#[doc = "CTRL (rw) register accessor: CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "CTRL register"]
pub mod ctrl;
#[doc = "SWITCH (rw) register accessor: SWITCH register\n\nYou can [`read`](crate::Reg::read) this register and get [`switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@switch`] module"]
#[doc(alias = "SWITCH")]
pub type Switch = crate::Reg<switch::SwitchSpec>;
#[doc = "SWITCH register"]
pub mod switch;
#[doc = "DS_CONF (rw) register accessor: DS_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds_conf`] module"]
#[doc(alias = "DS_CONF")]
pub type DsConf = crate::Reg<ds_conf::DsConfSpec>;
#[doc = "DS_CONF register"]
pub mod ds_conf;
#[doc = "SEQ_1 (rw) register accessor: SEQ_1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_1`] module"]
#[doc(alias = "SEQ_1")]
pub type Seq1 = crate::Reg<seq_1::Seq1Spec>;
#[doc = "SEQ_1 register"]
pub mod seq_1;
#[doc = "SEQ_2 (rw) register accessor: SEQ_2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_2`] module"]
#[doc(alias = "SEQ_2")]
pub type Seq2 = crate::Reg<seq_2::Seq2Spec>;
#[doc = "SEQ_2 register"]
pub mod seq_2;
#[doc = "COMP_1 (rw) register accessor: COMP_1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_1`] module"]
#[doc(alias = "COMP_1")]
pub type Comp1 = crate::Reg<comp_1::Comp1Spec>;
#[doc = "COMP_1 register"]
pub mod comp_1;
#[doc = "COMP_2 (rw) register accessor: COMP_2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_2`] module"]
#[doc(alias = "COMP_2")]
pub type Comp2 = crate::Reg<comp_2::Comp2Spec>;
#[doc = "COMP_2 register"]
pub mod comp_2;
#[doc = "COMP_3 (rw) register accessor: COMP_3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_3`] module"]
#[doc(alias = "COMP_3")]
pub type Comp3 = crate::Reg<comp_3::Comp3Spec>;
#[doc = "COMP_3 register"]
pub mod comp_3;
#[doc = "COMP_4 (rw) register accessor: COMP_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_4`] module"]
#[doc(alias = "COMP_4")]
pub type Comp4 = crate::Reg<comp_4::Comp4Spec>;
#[doc = "COMP_4 register"]
pub mod comp_4;
#[doc = "COMP_SEL (rw) register accessor: COMP_SEL register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_sel`] module"]
#[doc(alias = "COMP_SEL")]
pub type CompSel = crate::Reg<comp_sel::CompSelSpec>;
#[doc = "COMP_SEL register"]
pub mod comp_sel;
#[doc = "WD_TH (rw) register accessor: WD_TH register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_th`] module"]
#[doc(alias = "WD_TH")]
pub type WdTh = crate::Reg<wd_th::WdThSpec>;
#[doc = "WD_TH register"]
pub mod wd_th;
#[doc = "WD_CONF (rw) register accessor: WD_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_conf`] module"]
#[doc(alias = "WD_CONF")]
pub type WdConf = crate::Reg<wd_conf::WdConfSpec>;
#[doc = "WD_CONF register"]
pub mod wd_conf;
#[doc = "DS_DATAOUT (r) register accessor: DS_DATAOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_dataout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds_dataout`] module"]
#[doc(alias = "DS_DATAOUT")]
pub type DsDataout = crate::Reg<ds_dataout::DsDataoutSpec>;
#[doc = "DS_DATAOUT register"]
pub mod ds_dataout;
#[doc = "IRQ_STATUS (rw) register accessor: IRQ_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_status`] module"]
#[doc(alias = "IRQ_STATUS")]
pub type IrqStatus = crate::Reg<irq_status::IrqStatusSpec>;
#[doc = "IRQ_STATUS register"]
pub mod irq_status;
#[doc = "IRQ_ENABLE (rw) register accessor: IRQ_ENABLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`] module"]
#[doc(alias = "IRQ_ENABLE")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
#[doc = "IRQ_ENABLE register"]
pub mod irq_enable;
#[doc = "TEST_CONF (rw) register accessor: TEST_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_conf`] module"]
#[doc(alias = "TEST_CONF")]
pub type TestConf = crate::Reg<test_conf::TestConfSpec>;
#[doc = "TEST_CONF register"]
pub mod test_conf;
#[doc = "DTB_CONF (rw) register accessor: DTB_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtb_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb_conf`] module"]
#[doc(alias = "DTB_CONF")]
pub type DtbConf = crate::Reg<dtb_conf::DtbConfSpec>;
#[doc = "DTB_CONF register"]
pub mod dtb_conf;
