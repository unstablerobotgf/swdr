#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcd_cr: LcdCr,
    lcd_fcr: LcdFcr,
    lcd_sr: LcdSr,
    lcd_clr: LcdClr,
    _reserved4: [u8; 0x04],
    lcd_ram_com0: LcdRamCom0,
    _reserved5: [u8; 0x04],
    lcd_ram_com1: LcdRamCom1,
    _reserved6: [u8; 0x04],
    lcd_ram_com2: LcdRamCom2,
    _reserved7: [u8; 0x04],
    lcd_ram_com3: LcdRamCom3,
    _reserved8: [u8; 0x04],
    lcd_ram_com4: LcdRamCom4,
    _reserved9: [u8; 0x04],
    lcd_ram_com5: LcdRamCom5,
    _reserved10: [u8; 0x04],
    lcd_ram_com6: LcdRamCom6,
    _reserved11: [u8; 0x04],
    lcd_ram_com7: LcdRamCom7,
}
impl RegisterBlock {
    #[doc = "0x00 - LCD_CR register"]
    #[inline(always)]
    pub const fn lcd_cr(&self) -> &LcdCr {
        &self.lcd_cr
    }
    #[doc = "0x04 - LCD_FCR register"]
    #[inline(always)]
    pub const fn lcd_fcr(&self) -> &LcdFcr {
        &self.lcd_fcr
    }
    #[doc = "0x08 - LCD_SR register"]
    #[inline(always)]
    pub const fn lcd_sr(&self) -> &LcdSr {
        &self.lcd_sr
    }
    #[doc = "0x0c - LCD_CLR register"]
    #[inline(always)]
    pub const fn lcd_clr(&self) -> &LcdClr {
        &self.lcd_clr
    }
    #[doc = "0x14 - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com0(&self) -> &LcdRamCom0 {
        &self.lcd_ram_com0
    }
    #[doc = "0x1c - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com1(&self) -> &LcdRamCom1 {
        &self.lcd_ram_com1
    }
    #[doc = "0x24 - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com2(&self) -> &LcdRamCom2 {
        &self.lcd_ram_com2
    }
    #[doc = "0x2c - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com3(&self) -> &LcdRamCom3 {
        &self.lcd_ram_com3
    }
    #[doc = "0x34 - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com4(&self) -> &LcdRamCom4 {
        &self.lcd_ram_com4
    }
    #[doc = "0x3c - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com5(&self) -> &LcdRamCom5 {
        &self.lcd_ram_com5
    }
    #[doc = "0x44 - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com6(&self) -> &LcdRamCom6 {
        &self.lcd_ram_com6
    }
    #[doc = "0x4c - LCD_RAM_COMx register"]
    #[inline(always)]
    pub const fn lcd_ram_com7(&self) -> &LcdRamCom7 {
        &self.lcd_ram_com7
    }
}
#[doc = "LCD_CR (rw) register accessor: LCD_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cr`] module"]
#[doc(alias = "LCD_CR")]
pub type LcdCr = crate::Reg<lcd_cr::LcdCrSpec>;
#[doc = "LCD_CR register"]
pub mod lcd_cr;
#[doc = "LCD_FCR (rw) register accessor: LCD_FCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_fcr`] module"]
#[doc(alias = "LCD_FCR")]
pub type LcdFcr = crate::Reg<lcd_fcr::LcdFcrSpec>;
#[doc = "LCD_FCR register"]
pub mod lcd_fcr;
#[doc = "LCD_SR (r) register accessor: LCD_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_sr`] module"]
#[doc(alias = "LCD_SR")]
pub type LcdSr = crate::Reg<lcd_sr::LcdSrSpec>;
#[doc = "LCD_SR register"]
pub mod lcd_sr;
#[doc = "LCD_CLR (rw) register accessor: LCD_CLR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_clr`] module"]
#[doc(alias = "LCD_CLR")]
pub type LcdClr = crate::Reg<lcd_clr::LcdClrSpec>;
#[doc = "LCD_CLR register"]
pub mod lcd_clr;
#[doc = "LCD_RAM_COM0 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com0`] module"]
#[doc(alias = "LCD_RAM_COM0")]
pub type LcdRamCom0 = crate::Reg<lcd_ram_com0::LcdRamCom0Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com0;
#[doc = "LCD_RAM_COM1 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com1`] module"]
#[doc(alias = "LCD_RAM_COM1")]
pub type LcdRamCom1 = crate::Reg<lcd_ram_com1::LcdRamCom1Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com1;
#[doc = "LCD_RAM_COM2 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com2`] module"]
#[doc(alias = "LCD_RAM_COM2")]
pub type LcdRamCom2 = crate::Reg<lcd_ram_com2::LcdRamCom2Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com2;
#[doc = "LCD_RAM_COM3 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com3`] module"]
#[doc(alias = "LCD_RAM_COM3")]
pub type LcdRamCom3 = crate::Reg<lcd_ram_com3::LcdRamCom3Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com3;
#[doc = "LCD_RAM_COM4 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com4`] module"]
#[doc(alias = "LCD_RAM_COM4")]
pub type LcdRamCom4 = crate::Reg<lcd_ram_com4::LcdRamCom4Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com4;
#[doc = "LCD_RAM_COM5 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com5`] module"]
#[doc(alias = "LCD_RAM_COM5")]
pub type LcdRamCom5 = crate::Reg<lcd_ram_com5::LcdRamCom5Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com5;
#[doc = "LCD_RAM_COM6 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com6`] module"]
#[doc(alias = "LCD_RAM_COM6")]
pub type LcdRamCom6 = crate::Reg<lcd_ram_com6::LcdRamCom6Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com6;
#[doc = "LCD_RAM_COM7 (rw) register accessor: LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ram_com7`] module"]
#[doc(alias = "LCD_RAM_COM7")]
pub type LcdRamCom7 = crate::Reg<lcd_ram_com7::LcdRamCom7Spec>;
#[doc = "LCD_RAM_COMx register"]
pub mod lcd_ram_com7;
