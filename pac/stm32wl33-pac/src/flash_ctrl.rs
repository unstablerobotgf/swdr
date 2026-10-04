#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    command: Command,
    config: Config,
    irqstat: Irqstat,
    irqmask: Irqmask,
    irqraw: Irqraw,
    size: Size,
    address: Address,
    _reserved7: [u8; 0x08],
    lfsrval: Lfsrval,
    _reserved8: [u8; 0x0c],
    pageprot0: Pageprot0,
    pageprot1: Pageprot1,
    _reserved10: [u8; 0x04],
    data0: Data0,
    data1: Data1,
    data2: Data2,
    data3: Data3,
    unlock012: Unlock012,
    unlock3: Unlock3,
}
impl RegisterBlock {
    #[doc = "0x00 - COMMAND register"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x04 - CONFIG register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x08 - IRQSTAT register"]
    #[inline(always)]
    pub const fn irqstat(&self) -> &Irqstat {
        &self.irqstat
    }
    #[doc = "0x0c - IRQMASK register"]
    #[inline(always)]
    pub const fn irqmask(&self) -> &Irqmask {
        &self.irqmask
    }
    #[doc = "0x10 - IRQRAW register"]
    #[inline(always)]
    pub const fn irqraw(&self) -> &Irqraw {
        &self.irqraw
    }
    #[doc = "0x14 - SIZE register"]
    #[inline(always)]
    pub const fn size(&self) -> &Size {
        &self.size
    }
    #[doc = "0x18 - ADDRESS register"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x24 - LFSRVAL register"]
    #[inline(always)]
    pub const fn lfsrval(&self) -> &Lfsrval {
        &self.lfsrval
    }
    #[doc = "0x34 - PAGEPROT0 register"]
    #[inline(always)]
    pub const fn pageprot0(&self) -> &Pageprot0 {
        &self.pageprot0
    }
    #[doc = "0x38 - PAGEPROT1 register"]
    #[inline(always)]
    pub const fn pageprot1(&self) -> &Pageprot1 {
        &self.pageprot1
    }
    #[doc = "0x40 - DATA0 register"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x44 - DATA1 register"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x48 - DATA2 register"]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x4c - DATA3 register"]
    #[inline(always)]
    pub const fn data3(&self) -> &Data3 {
        &self.data3
    }
    #[doc = "0x50 - UNLOCK012 register"]
    #[inline(always)]
    pub const fn unlock012(&self) -> &Unlock012 {
        &self.unlock012
    }
    #[doc = "0x54 - UNLOCK3 register"]
    #[inline(always)]
    pub const fn unlock3(&self) -> &Unlock3 {
        &self.unlock3
    }
}
#[doc = "COMMAND (rw) register accessor: COMMAND register\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "COMMAND register"]
pub mod command;
#[doc = "CONFIG (rw) register accessor: CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "CONFIG register"]
pub mod config;
#[doc = "IRQSTAT (rw) register accessor: IRQSTAT register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstat`] module"]
#[doc(alias = "IRQSTAT")]
pub type Irqstat = crate::Reg<irqstat::IrqstatSpec>;
#[doc = "IRQSTAT register"]
pub mod irqstat;
#[doc = "IRQMASK (rw) register accessor: IRQMASK register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqmask`] module"]
#[doc(alias = "IRQMASK")]
pub type Irqmask = crate::Reg<irqmask::IrqmaskSpec>;
#[doc = "IRQMASK register"]
pub mod irqmask;
#[doc = "IRQRAW (rw) register accessor: IRQRAW register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqraw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqraw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqraw`] module"]
#[doc(alias = "IRQRAW")]
pub type Irqraw = crate::Reg<irqraw::IrqrawSpec>;
#[doc = "IRQRAW register"]
pub mod irqraw;
#[doc = "SIZE (r) register accessor: SIZE register\n\nYou can [`read`](crate::Reg::read) this register and get [`size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@size`] module"]
#[doc(alias = "SIZE")]
pub type Size = crate::Reg<size::SizeSpec>;
#[doc = "SIZE register"]
pub mod size;
#[doc = "ADDRESS (rw) register accessor: ADDRESS register\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`] module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "ADDRESS register"]
pub mod address;
#[doc = "LFSRVAL (r) register accessor: LFSRVAL register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsrval::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsrval`] module"]
#[doc(alias = "LFSRVAL")]
pub type Lfsrval = crate::Reg<lfsrval::LfsrvalSpec>;
#[doc = "LFSRVAL register"]
pub mod lfsrval;
#[doc = "PAGEPROT0 (rw) register accessor: PAGEPROT0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pageprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pageprot0`] module"]
#[doc(alias = "PAGEPROT0")]
pub type Pageprot0 = crate::Reg<pageprot0::Pageprot0Spec>;
#[doc = "PAGEPROT0 register"]
pub mod pageprot0;
#[doc = "PAGEPROT1 (rw) register accessor: PAGEPROT1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pageprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pageprot1`] module"]
#[doc(alias = "PAGEPROT1")]
pub type Pageprot1 = crate::Reg<pageprot1::Pageprot1Spec>;
#[doc = "PAGEPROT1 register"]
pub mod pageprot1;
#[doc = "DATA0 (rw) register accessor: DATA0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`] module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "DATA0 register"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: DATA1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`] module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "DATA1 register"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: DATA2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`] module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "DATA2 register"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: DATA3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3`] module"]
#[doc(alias = "DATA3")]
pub type Data3 = crate::Reg<data3::Data3Spec>;
#[doc = "DATA3 register"]
pub mod data3;
#[doc = "UNLOCK012 (rw) register accessor: UNLOCK012 register\n\nYou can [`read`](crate::Reg::read) this register and get [`unlock012::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock012::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock012`] module"]
#[doc(alias = "UNLOCK012")]
pub type Unlock012 = crate::Reg<unlock012::Unlock012Spec>;
#[doc = "UNLOCK012 register"]
pub mod unlock012;
#[doc = "UNLOCK3 (rw) register accessor: UNLOCK3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`unlock3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock3`] module"]
#[doc(alias = "UNLOCK3")]
pub type Unlock3 = crate::Reg<unlock3::Unlock3Spec>;
#[doc = "UNLOCK3 register"]
pub mod unlock3;
