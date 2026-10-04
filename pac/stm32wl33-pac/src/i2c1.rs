#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_cr1: I2cCr1,
    i2c_cr2: I2cCr2,
    i2c_oar1: I2cOar1,
    i2c_oar2: I2cOar2,
    i2c_timing: I2cTiming,
    i2c_timeout: I2cTimeout,
    i2c_isr: I2cIsr,
    i2c_icr: I2cIcr,
    i2c_pec: I2cPec,
    i2c_rxdr: I2cRxdr,
    i2c_txdr: I2cTxdr,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C_CR1 register"]
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2cCr1 {
        &self.i2c_cr1
    }
    #[doc = "0x04 - I2C_CR2 register"]
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2cCr2 {
        &self.i2c_cr2
    }
    #[doc = "0x08 - I2C_OAR1 register"]
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2cOar1 {
        &self.i2c_oar1
    }
    #[doc = "0x0c - I2C_OAR2 register"]
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2cOar2 {
        &self.i2c_oar2
    }
    #[doc = "0x10 - I2C_TIMING register"]
    #[inline(always)]
    pub const fn i2c_timing(&self) -> &I2cTiming {
        &self.i2c_timing
    }
    #[doc = "0x14 - I2C_TIMEOUT register"]
    #[inline(always)]
    pub const fn i2c_timeout(&self) -> &I2cTimeout {
        &self.i2c_timeout
    }
    #[doc = "0x18 - I2C_ISR register"]
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2cIsr {
        &self.i2c_isr
    }
    #[doc = "0x1c - I2C_ICR register"]
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2cIcr {
        &self.i2c_icr
    }
    #[doc = "0x20 - I2C_PEC register"]
    #[inline(always)]
    pub const fn i2c_pec(&self) -> &I2cPec {
        &self.i2c_pec
    }
    #[doc = "0x24 - I2C_RXDR register"]
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2cRxdr {
        &self.i2c_rxdr
    }
    #[doc = "0x28 - I2C_TXDR register"]
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2cTxdr {
        &self.i2c_txdr
    }
}
#[doc = "I2C_CR1 (rw) register accessor: I2C_CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr1`] module"]
#[doc(alias = "I2C_CR1")]
pub type I2cCr1 = crate::Reg<i2c_cr1::I2cCr1Spec>;
#[doc = "I2C_CR1 register"]
pub mod i2c_cr1;
#[doc = "I2C_CR2 (rw) register accessor: I2C_CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr2`] module"]
#[doc(alias = "I2C_CR2")]
pub type I2cCr2 = crate::Reg<i2c_cr2::I2cCr2Spec>;
#[doc = "I2C_CR2 register"]
pub mod i2c_cr2;
#[doc = "I2C_OAR1 (rw) register accessor: I2C_OAR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar1`] module"]
#[doc(alias = "I2C_OAR1")]
pub type I2cOar1 = crate::Reg<i2c_oar1::I2cOar1Spec>;
#[doc = "I2C_OAR1 register"]
pub mod i2c_oar1;
#[doc = "I2C_OAR2 (rw) register accessor: I2C_OAR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar2`] module"]
#[doc(alias = "I2C_OAR2")]
pub type I2cOar2 = crate::Reg<i2c_oar2::I2cOar2Spec>;
#[doc = "I2C_OAR2 register"]
pub mod i2c_oar2;
#[doc = "I2C_TIMING (rw) register accessor: I2C_TIMING register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timing`] module"]
#[doc(alias = "I2C_TIMING")]
pub type I2cTiming = crate::Reg<i2c_timing::I2cTimingSpec>;
#[doc = "I2C_TIMING register"]
pub mod i2c_timing;
#[doc = "I2C_TIMEOUT (rw) register accessor: I2C_TIMEOUT register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timeout`] module"]
#[doc(alias = "I2C_TIMEOUT")]
pub type I2cTimeout = crate::Reg<i2c_timeout::I2cTimeoutSpec>;
#[doc = "I2C_TIMEOUT register"]
pub mod i2c_timeout;
#[doc = "I2C_ISR (rw) register accessor: I2C_ISR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_isr`] module"]
#[doc(alias = "I2C_ISR")]
pub type I2cIsr = crate::Reg<i2c_isr::I2cIsrSpec>;
#[doc = "I2C_ISR register"]
pub mod i2c_isr;
#[doc = "I2C_ICR (rw) register accessor: I2C_ICR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_icr`] module"]
#[doc(alias = "I2C_ICR")]
pub type I2cIcr = crate::Reg<i2c_icr::I2cIcrSpec>;
#[doc = "I2C_ICR register"]
pub mod i2c_icr;
#[doc = "I2C_PEC (r) register accessor: I2C_PEC register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_pec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_pec`] module"]
#[doc(alias = "I2C_PEC")]
pub type I2cPec = crate::Reg<i2c_pec::I2cPecSpec>;
#[doc = "I2C_PEC register"]
pub mod i2c_pec;
#[doc = "I2C_RXDR (r) register accessor: I2C_RXDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_rxdr`] module"]
#[doc(alias = "I2C_RXDR")]
pub type I2cRxdr = crate::Reg<i2c_rxdr::I2cRxdrSpec>;
#[doc = "I2C_RXDR register"]
pub mod i2c_rxdr;
#[doc = "I2C_TXDR (rw) register accessor: I2C_TXDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_txdr`] module"]
#[doc(alias = "I2C_TXDR")]
pub type I2cTxdr = crate::Reg<i2c_txdr::I2cTxdrSpec>;
#[doc = "I2C_TXDR register"]
pub mod i2c_txdr;
