#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aes_cr: AesCr,
    aes_sr: AesSr,
    aes_dinr: AesDinr,
    aes_doutr: AesDoutr,
    aes_keyr0: AesKeyr0,
    aes_keyr1: AesKeyr1,
    aes_keyr2: AesKeyr2,
    aes_keyr3: AesKeyr3,
    aes_ivr0: AesIvr0,
    aes_ivr1: AesIvr1,
    aes_ivr2: AesIvr2,
    aes_ivr3: AesIvr3,
}
impl RegisterBlock {
    #[doc = "0x00 - AES_CR register"]
    #[inline(always)]
    pub const fn aes_cr(&self) -> &AesCr {
        &self.aes_cr
    }
    #[doc = "0x04 - AES_SR register"]
    #[inline(always)]
    pub const fn aes_sr(&self) -> &AesSr {
        &self.aes_sr
    }
    #[doc = "0x08 - AES_DINR register"]
    #[inline(always)]
    pub const fn aes_dinr(&self) -> &AesDinr {
        &self.aes_dinr
    }
    #[doc = "0x0c - AES_DOUTR register"]
    #[inline(always)]
    pub const fn aes_doutr(&self) -> &AesDoutr {
        &self.aes_doutr
    }
    #[doc = "0x10 - AES_KEYRx register"]
    #[inline(always)]
    pub const fn aes_keyr0(&self) -> &AesKeyr0 {
        &self.aes_keyr0
    }
    #[doc = "0x14 - AES_KEYRx register"]
    #[inline(always)]
    pub const fn aes_keyr1(&self) -> &AesKeyr1 {
        &self.aes_keyr1
    }
    #[doc = "0x18 - AES_KEYRx register"]
    #[inline(always)]
    pub const fn aes_keyr2(&self) -> &AesKeyr2 {
        &self.aes_keyr2
    }
    #[doc = "0x1c - AES_KEYRx register"]
    #[inline(always)]
    pub const fn aes_keyr3(&self) -> &AesKeyr3 {
        &self.aes_keyr3
    }
    #[doc = "0x20 - AES_IVRx register"]
    #[inline(always)]
    pub const fn aes_ivr0(&self) -> &AesIvr0 {
        &self.aes_ivr0
    }
    #[doc = "0x24 - AES_IVRx register"]
    #[inline(always)]
    pub const fn aes_ivr1(&self) -> &AesIvr1 {
        &self.aes_ivr1
    }
    #[doc = "0x28 - AES_IVRx register"]
    #[inline(always)]
    pub const fn aes_ivr2(&self) -> &AesIvr2 {
        &self.aes_ivr2
    }
    #[doc = "0x2c - AES_IVRx register"]
    #[inline(always)]
    pub const fn aes_ivr3(&self) -> &AesIvr3 {
        &self.aes_ivr3
    }
}
#[doc = "AES_CR (rw) register accessor: AES_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cr`] module"]
#[doc(alias = "AES_CR")]
pub type AesCr = crate::Reg<aes_cr::AesCrSpec>;
#[doc = "AES_CR register"]
pub mod aes_cr;
#[doc = "AES_SR (r) register accessor: AES_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_sr`] module"]
#[doc(alias = "AES_SR")]
pub type AesSr = crate::Reg<aes_sr::AesSrSpec>;
#[doc = "AES_SR register"]
pub mod aes_sr;
#[doc = "AES_DINR (rw) register accessor: AES_DINR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_dinr`] module"]
#[doc(alias = "AES_DINR")]
pub type AesDinr = crate::Reg<aes_dinr::AesDinrSpec>;
#[doc = "AES_DINR register"]
pub mod aes_dinr;
#[doc = "AES_DOUTR (r) register accessor: AES_DOUTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_doutr`] module"]
#[doc(alias = "AES_DOUTR")]
pub type AesDoutr = crate::Reg<aes_doutr::AesDoutrSpec>;
#[doc = "AES_DOUTR register"]
pub mod aes_doutr;
#[doc = "AES_KEYR0 (rw) register accessor: AES_KEYRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_keyr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr0`] module"]
#[doc(alias = "AES_KEYR0")]
pub type AesKeyr0 = crate::Reg<aes_keyr0::AesKeyr0Spec>;
#[doc = "AES_KEYRx register"]
pub mod aes_keyr0;
#[doc = "AES_KEYR1 (rw) register accessor: AES_KEYRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_keyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr1`] module"]
#[doc(alias = "AES_KEYR1")]
pub type AesKeyr1 = crate::Reg<aes_keyr1::AesKeyr1Spec>;
#[doc = "AES_KEYRx register"]
pub mod aes_keyr1;
#[doc = "AES_KEYR2 (rw) register accessor: AES_KEYRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_keyr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr2`] module"]
#[doc(alias = "AES_KEYR2")]
pub type AesKeyr2 = crate::Reg<aes_keyr2::AesKeyr2Spec>;
#[doc = "AES_KEYRx register"]
pub mod aes_keyr2;
#[doc = "AES_KEYR3 (rw) register accessor: AES_KEYRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_keyr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_keyr3`] module"]
#[doc(alias = "AES_KEYR3")]
pub type AesKeyr3 = crate::Reg<aes_keyr3::AesKeyr3Spec>;
#[doc = "AES_KEYRx register"]
pub mod aes_keyr3;
#[doc = "AES_IVR0 (rw) register accessor: AES_IVRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr0`] module"]
#[doc(alias = "AES_IVR0")]
pub type AesIvr0 = crate::Reg<aes_ivr0::AesIvr0Spec>;
#[doc = "AES_IVRx register"]
pub mod aes_ivr0;
#[doc = "AES_IVR1 (rw) register accessor: AES_IVRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr1`] module"]
#[doc(alias = "AES_IVR1")]
pub type AesIvr1 = crate::Reg<aes_ivr1::AesIvr1Spec>;
#[doc = "AES_IVRx register"]
pub mod aes_ivr1;
#[doc = "AES_IVR2 (rw) register accessor: AES_IVRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr2`] module"]
#[doc(alias = "AES_IVR2")]
pub type AesIvr2 = crate::Reg<aes_ivr2::AesIvr2Spec>;
#[doc = "AES_IVRx register"]
pub mod aes_ivr2;
#[doc = "AES_IVR3 (rw) register accessor: AES_IVRx register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ivr3`] module"]
#[doc(alias = "AES_IVR3")]
pub type AesIvr3 = crate::Reg<aes_ivr3::AesIvr3Spec>;
#[doc = "AES_IVRx register"]
pub mod aes_ivr3;
