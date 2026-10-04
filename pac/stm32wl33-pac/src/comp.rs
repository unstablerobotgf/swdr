#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
}
impl RegisterBlock {
    #[doc = "0x00 - CSR register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
}
#[doc = "CSR (rw) register accessor: CSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "CSR register"]
pub mod csr;
