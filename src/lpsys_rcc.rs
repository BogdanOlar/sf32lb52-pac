#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rstr1: Rstr1,
    enr1: Enr1,
    esr1: Esr1,
    ecr1: Ecr1,
    csr: Csr,
    cfgr: Cfgr,
    dbgr: Dbgr,
}
impl RegisterBlock {
    ///0x00 - Reset Register 1
    #[inline(always)]
    pub const fn rstr1(&self) -> &Rstr1 {
        &self.rstr1
    }
    ///0x04 - Enable Register 1
    #[inline(always)]
    pub const fn enr1(&self) -> &Enr1 {
        &self.enr1
    }
    ///0x08 - Enable Set Register 1
    #[inline(always)]
    pub const fn esr1(&self) -> &Esr1 {
        &self.esr1
    }
    ///0x0c - Enable Clear Register 1
    #[inline(always)]
    pub const fn ecr1(&self) -> &Ecr1 {
        &self.ecr1
    }
    ///0x10 - Clock Select Register
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    ///0x14 - Clock Configuration Register
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    ///0x18 - Debug Register
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
}
///RSTR1 (rw) register accessor: Reset Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rstr1`]
///module
#[doc(alias = "RSTR1")]
pub type Rstr1 = crate::Reg<rstr1::RSTR1rs>;
///Reset Register 1
pub mod rstr1;
///ENR1 (rw) register accessor: Enable Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@enr1`]
///module
#[doc(alias = "ENR1")]
pub type Enr1 = crate::Reg<enr1::ENR1rs>;
///Enable Register 1
pub mod enr1;
///ESR1 (rw) register accessor: Enable Set Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`esr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@esr1`]
///module
#[doc(alias = "ESR1")]
pub type Esr1 = crate::Reg<esr1::ESR1rs>;
///Enable Set Register 1
pub mod esr1;
///ECR1 (rw) register accessor: Enable Clear Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`ecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ecr1`]
///module
#[doc(alias = "ECR1")]
pub type Ecr1 = crate::Reg<ecr1::ECR1rs>;
///Enable Clear Register 1
pub mod ecr1;
///CSR (rw) register accessor: Clock Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@csr`]
///module
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CSRrs>;
///Clock Select Register
pub mod csr;
///CFGR (rw) register accessor: Clock Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfgr`]
///module
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CFGRrs>;
///Clock Configuration Register
pub mod cfgr;
///DBGR (rw) register accessor: Debug Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbgr`]
///module
#[doc(alias = "DBGR")]
pub type Dbgr = crate::Reg<dbgr::DBGRrs>;
///Debug Register
pub mod dbgr;
