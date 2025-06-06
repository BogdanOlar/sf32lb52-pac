#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: Cr,
    tcr: Tcr,
    ier: Ier,
    sr: Sr,
    dbr: Dbr,
    sar: Sar,
    lcr: Lcr,
    wcr: Wcr,
    rccr: Rccr,
    bmr: Bmr,
    dnr: Dnr,
    _reserved11: [u8; 0x04],
    fifo: Fifo,
}
impl RegisterBlock {
    ///0x00 - Control register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x04 - Transfer Control register
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    ///0x08 - Interrupt Enable register
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    ///0x0c - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x10 - Data Buffer register
    #[inline(always)]
    pub const fn dbr(&self) -> &Dbr {
        &self.dbr
    }
    ///0x14 - Slave Address Register
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    ///0x18 - Load Count Register
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    ///0x1c - Wait Count Register
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    ///0x20 - Bus Reset Cycle Counter Register
    #[inline(always)]
    pub const fn rccr(&self) -> &Rccr {
        &self.rccr
    }
    ///0x24 - Bus Monitor Register
    #[inline(always)]
    pub const fn bmr(&self) -> &Bmr {
        &self.bmr
    }
    ///0x28 - DMA number register
    #[inline(always)]
    pub const fn dnr(&self) -> &Dnr {
        &self.dnr
    }
    ///0x30 - FIFO Register
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
}
///CR (rw) register accessor: Control register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr`]
///module
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CRrs>;
///Control register
pub mod cr;
///TCR (rw) register accessor: Transfer Control register
///
///You can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr`]
///module
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TCRrs>;
///Transfer Control register
pub mod tcr;
///IER (rw) register accessor: Interrupt Enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ier`]
///module
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IERrs>;
///Interrupt Enable register
pub mod ier;
///SR (rw) register accessor: Status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sr`]
///module
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
///DBR (rw) register accessor: Data Buffer register
///
///You can [`read`](crate::Reg::read) this register and get [`dbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbr`]
///module
#[doc(alias = "DBR")]
pub type Dbr = crate::Reg<dbr::DBRrs>;
///Data Buffer register
pub mod dbr;
///SAR (rw) register accessor: Slave Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sar`]
///module
#[doc(alias = "SAR")]
pub type Sar = crate::Reg<sar::SARrs>;
///Slave Address Register
pub mod sar;
///LCR (rw) register accessor: Load Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcr`]
///module
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LCRrs>;
///Load Count Register
pub mod lcr;
///WCR (rw) register accessor: Wait Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wcr`]
///module
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WCRrs>;
///Wait Count Register
pub mod wcr;
///RCCR (rw) register accessor: Bus Reset Cycle Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`rccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rccr`]
///module
#[doc(alias = "RCCR")]
pub type Rccr = crate::Reg<rccr::RCCRrs>;
///Bus Reset Cycle Counter Register
pub mod rccr;
///BMR (rw) register accessor: Bus Monitor Register
///
///You can [`read`](crate::Reg::read) this register and get [`bmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bmr`]
///module
#[doc(alias = "BMR")]
pub type Bmr = crate::Reg<bmr::BMRrs>;
///Bus Monitor Register
pub mod bmr;
///DNR (rw) register accessor: DMA number register
///
///You can [`read`](crate::Reg::read) this register and get [`dnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dnr`]
///module
#[doc(alias = "DNR")]
pub type Dnr = crate::Reg<dnr::DNRrs>;
///DMA number register
pub mod dnr;
///FIFO (rw) register accessor: FIFO Register
///
///You can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifo`]
///module
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FIFOrs>;
///FIFO Register
pub mod fifo;
