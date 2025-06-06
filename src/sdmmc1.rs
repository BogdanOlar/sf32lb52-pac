#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sr: Sr,
    ccr: Ccr,
    car: Car,
    rir: Rir,
    rar1: Rar1,
    rar2: Rar2,
    rar3: Rar3,
    rar4: Rar4,
    tor: Tor,
    dcr: Dcr,
    dlr: Dlr,
    ier: Ier,
    clkcr: Clkcr,
    _reserved13: [u8; 0x08],
    cdr: Cdr,
    dbgr1: Dbgr1,
    dbgr2: Dbgr2,
    ceata: Ceata,
    _reserved17: [u8; 0x08],
    dsr: Dsr,
    cdcr: Cdcr,
    casr: Casr,
    cacr: Cacr,
    cacnt: Cacnt,
    caoff: Caoff,
    _reserved23: [u8; 0x0194],
    fifo: Fifo,
}
impl RegisterBlock {
    ///0x00 - command and data status register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x04 - command control register
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    ///0x08 - command argument register
    #[inline(always)]
    pub const fn car(&self) -> &Car {
        &self.car
    }
    ///0x0c - response command index register
    #[inline(always)]
    pub const fn rir(&self) -> &Rir {
        &self.rir
    }
    ///0x10 - response command argument1 register
    #[inline(always)]
    pub const fn rar1(&self) -> &Rar1 {
        &self.rar1
    }
    ///0x14 - response command argument2 register
    #[inline(always)]
    pub const fn rar2(&self) -> &Rar2 {
        &self.rar2
    }
    ///0x18 - response command argument3 register
    #[inline(always)]
    pub const fn rar3(&self) -> &Rar3 {
        &self.rar3
    }
    ///0x1c - response command argument4 register
    #[inline(always)]
    pub const fn rar4(&self) -> &Rar4 {
        &self.rar4
    }
    ///0x20 - timeout count register
    #[inline(always)]
    pub const fn tor(&self) -> &Tor {
        &self.tor
    }
    ///0x24 - data control register
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    ///0x28 - data length register
    #[inline(always)]
    pub const fn dlr(&self) -> &Dlr {
        &self.dlr
    }
    ///0x2c - command and data interrupt mask register
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    ///0x30 - clock control register
    #[inline(always)]
    pub const fn clkcr(&self) -> &Clkcr {
        &self.clkcr
    }
    ///0x3c - card interface control and card detect register
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
    ///0x40 - card debug port1 register
    #[inline(always)]
    pub const fn dbgr1(&self) -> &Dbgr1 {
        &self.dbgr1
    }
    ///0x44 - card debug port2 register
    #[inline(always)]
    pub const fn dbgr2(&self) -> &Dbgr2 {
        &self.dbgr2
    }
    ///0x48 - CE-ATA/SDIO mode register
    #[inline(always)]
    pub const fn ceata(&self) -> &Ceata {
        &self.ceata
    }
    ///0x54 - data status register
    #[inline(always)]
    pub const fn dsr(&self) -> &Dsr {
        &self.dsr
    }
    ///0x58 - clock duty cycle register
    #[inline(always)]
    pub const fn cdcr(&self) -> &Cdcr {
        &self.cdcr
    }
    ///0x5c - cache status register
    #[inline(always)]
    pub const fn casr(&self) -> &Casr {
        &self.casr
    }
    ///0x60 - cache control register
    #[inline(always)]
    pub const fn cacr(&self) -> &Cacr {
        &self.cacr
    }
    ///0x64 - cache counter register
    #[inline(always)]
    pub const fn cacnt(&self) -> &Cacnt {
        &self.cacnt
    }
    ///0x68 - cache offset register
    #[inline(always)]
    pub const fn caoff(&self) -> &Caoff {
        &self.caoff
    }
    ///0x200 - FIFO entry
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
}
///SR (rw) register accessor: command and data status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sr`]
///module
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SRrs>;
///command and data status register
pub mod sr;
///CCR (rw) register accessor: command control register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr`]
///module
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CCRrs>;
///command control register
pub mod ccr;
///CAR (rw) register accessor: command argument register
///
///You can [`read`](crate::Reg::read) this register and get [`car::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`car::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@car`]
///module
#[doc(alias = "CAR")]
pub type Car = crate::Reg<car::CARrs>;
///command argument register
pub mod car;
///RIR (rw) register accessor: response command index register
///
///You can [`read`](crate::Reg::read) this register and get [`rir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rir`]
///module
#[doc(alias = "RIR")]
pub type Rir = crate::Reg<rir::RIRrs>;
///response command index register
pub mod rir;
///RAR1 (rw) register accessor: response command argument1 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rar1`]
///module
#[doc(alias = "RAR1")]
pub type Rar1 = crate::Reg<rar1::RAR1rs>;
///response command argument1 register
pub mod rar1;
///RAR2 (rw) register accessor: response command argument2 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rar2`]
///module
#[doc(alias = "RAR2")]
pub type Rar2 = crate::Reg<rar2::RAR2rs>;
///response command argument2 register
pub mod rar2;
///RAR3 (rw) register accessor: response command argument3 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rar3`]
///module
#[doc(alias = "RAR3")]
pub type Rar3 = crate::Reg<rar3::RAR3rs>;
///response command argument3 register
pub mod rar3;
///RAR4 (rw) register accessor: response command argument4 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rar4`]
///module
#[doc(alias = "RAR4")]
pub type Rar4 = crate::Reg<rar4::RAR4rs>;
///response command argument4 register
pub mod rar4;
///TOR (rw) register accessor: timeout count register
///
///You can [`read`](crate::Reg::read) this register and get [`tor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tor`]
///module
#[doc(alias = "TOR")]
pub type Tor = crate::Reg<tor::TORrs>;
///timeout count register
pub mod tor;
///DCR (rw) register accessor: data control register
///
///You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcr`]
///module
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DCRrs>;
///data control register
pub mod dcr;
///DLR (rw) register accessor: data length register
///
///You can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dlr`]
///module
#[doc(alias = "DLR")]
pub type Dlr = crate::Reg<dlr::DLRrs>;
///data length register
pub mod dlr;
///IER (rw) register accessor: command and data interrupt mask register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ier`]
///module
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IERrs>;
///command and data interrupt mask register
pub mod ier;
///CLKCR (rw) register accessor: clock control register
///
///You can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@clkcr`]
///module
#[doc(alias = "CLKCR")]
pub type Clkcr = crate::Reg<clkcr::CLKCRrs>;
///clock control register
pub mod clkcr;
///CDR (rw) register accessor: card interface control and card detect register
///
///You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cdr`]
///module
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CDRrs>;
///card interface control and card detect register
pub mod cdr;
///DBGR1 (rw) register accessor: card debug port1 register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbgr1`]
///module
#[doc(alias = "DBGR1")]
pub type Dbgr1 = crate::Reg<dbgr1::DBGR1rs>;
///card debug port1 register
pub mod dbgr1;
///DBGR2 (rw) register accessor: card debug port2 register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbgr2`]
///module
#[doc(alias = "DBGR2")]
pub type Dbgr2 = crate::Reg<dbgr2::DBGR2rs>;
///card debug port2 register
pub mod dbgr2;
///CEATA (rw) register accessor: CE-ATA/SDIO mode register
///
///You can [`read`](crate::Reg::read) this register and get [`ceata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ceata`]
///module
#[doc(alias = "CEATA")]
pub type Ceata = crate::Reg<ceata::CEATArs>;
///CE-ATA/SDIO mode register
pub mod ceata;
///DSR (rw) register accessor: data status register
///
///You can [`read`](crate::Reg::read) this register and get [`dsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dsr`]
///module
#[doc(alias = "DSR")]
pub type Dsr = crate::Reg<dsr::DSRrs>;
///data status register
pub mod dsr;
///CDCR (rw) register accessor: clock duty cycle register
///
///You can [`read`](crate::Reg::read) this register and get [`cdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cdcr`]
///module
#[doc(alias = "CDCR")]
pub type Cdcr = crate::Reg<cdcr::CDCRrs>;
///clock duty cycle register
pub mod cdcr;
///CASR (rw) register accessor: cache status register
///
///You can [`read`](crate::Reg::read) this register and get [`casr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`casr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@casr`]
///module
#[doc(alias = "CASR")]
pub type Casr = crate::Reg<casr::CASRrs>;
///cache status register
pub mod casr;
///CACR (rw) register accessor: cache control register
///
///You can [`read`](crate::Reg::read) this register and get [`cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cacr`]
///module
#[doc(alias = "CACR")]
pub type Cacr = crate::Reg<cacr::CACRrs>;
///cache control register
pub mod cacr;
///CACNT (rw) register accessor: cache counter register
///
///You can [`read`](crate::Reg::read) this register and get [`cacnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cacnt`]
///module
#[doc(alias = "CACNT")]
pub type Cacnt = crate::Reg<cacnt::CACNTrs>;
///cache counter register
pub mod cacnt;
///CAOFF (rw) register accessor: cache offset register
///
///You can [`read`](crate::Reg::read) this register and get [`caoff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caoff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@caoff`]
///module
#[doc(alias = "CAOFF")]
pub type Caoff = crate::Reg<caoff::CAOFFrs>;
///cache offset register
pub mod caoff;
///FIFO (rw) register accessor: FIFO entry
///
///You can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifo`]
///module
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FIFOrs>;
///FIFO entry
pub mod fifo;
