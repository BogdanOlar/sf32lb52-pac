#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: Isr,
    icr: Icr,
    ier: Ier,
    cfgr: Cfgr,
    cr: Cr,
    cmp: Cmp,
    arr: Arr,
    cnt: Cnt,
    rcr: Rcr,
}
impl RegisterBlock {
    ///0x00 - LPTIM interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    ///0x04 - LPTIM interrupt and status clear register
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    ///0x08 - LPTIM interrupt and wakeup enable register
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x14 - LPTIM compare register
    #[inline(always)]
    pub const fn cmp(&self) -> &Cmp {
        &self.cmp
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    ///0x20 - LPTIM repetition register
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
}
///ISR (rw) register accessor: LPTIM interrupt and status register
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@isr`]
///module
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::ISRrs>;
///LPTIM interrupt and status register
pub mod isr;
///ICR (rw) register accessor: LPTIM interrupt and status clear register
///
///You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@icr`]
///module
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::ICRrs>;
///LPTIM interrupt and status clear register
pub mod icr;
///IER (rw) register accessor: LPTIM interrupt and wakeup enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ier`]
///module
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IERrs>;
///LPTIM interrupt and wakeup enable register
pub mod ier;
///CFGR (rw) register accessor: LPTIM configuration register
///
///You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfgr`]
///module
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CFGRrs>;
///LPTIM configuration register
pub mod cfgr;
///CR (rw) register accessor: LPTIM control register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr`]
///module
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CRrs>;
///LPTIM control register
pub mod cr;
///CMP (rw) register accessor: LPTIM compare register
///
///You can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmp`]
///module
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CMPrs>;
///LPTIM compare register
pub mod cmp;
///ARR (rw) register accessor: LPTIM autoreload register
///
///You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@arr`]
///module
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ARRrs>;
///LPTIM autoreload register
pub mod arr;
///CNT (rw) register accessor: LPTIM counter register
///
///You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cnt`]
///module
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CNTrs>;
///LPTIM counter register
pub mod cnt;
///RCR (rw) register accessor: LPTIM repetition register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr`]
///module
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RCRrs>;
///LPTIM repetition register
pub mod rcr;
