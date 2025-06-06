#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    syscr: Syscr,
    rtc_tr: RtcTr,
    rtc_dr: RtcDr,
    ulpmcr: Ulpmcr,
    dbgr: Dbgr,
    mdbgr: Mdbgr,
    _reserved6: [u8; 0x20],
    usart4_pinr: Usart4Pinr,
    usart5_pinr: Usart5Pinr,
    pta_pinr: PtaPinr,
}
impl RegisterBlock {
    ///0x00 - System Configure Register
    #[inline(always)]
    pub const fn syscr(&self) -> &Syscr {
        &self.syscr
    }
    ///0x04 - Mirrored RTC Time Register
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RtcTr {
        &self.rtc_tr
    }
    ///0x08 - Mirrored RTC Date Register
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RtcDr {
        &self.rtc_dr
    }
    ///0x0c - ULP Memory Control register
    #[inline(always)]
    pub const fn ulpmcr(&self) -> &Ulpmcr {
        &self.ulpmcr
    }
    ///0x10 - Debug Register
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
    ///0x14 - Memory Debug Register
    #[inline(always)]
    pub const fn mdbgr(&self) -> &Mdbgr {
        &self.mdbgr
    }
    ///0x38 - USART4 Pin Register
    #[inline(always)]
    pub const fn usart4_pinr(&self) -> &Usart4Pinr {
        &self.usart4_pinr
    }
    ///0x3c - USART5 Pin Register
    #[inline(always)]
    pub const fn usart5_pinr(&self) -> &Usart5Pinr {
        &self.usart5_pinr
    }
    ///0x40 - PTA Pin Register
    #[inline(always)]
    pub const fn pta_pinr(&self) -> &PtaPinr {
        &self.pta_pinr
    }
}
///SYSCR (rw) register accessor: System Configure Register
///
///You can [`read`](crate::Reg::read) this register and get [`syscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@syscr`]
///module
#[doc(alias = "SYSCR")]
pub type Syscr = crate::Reg<syscr::SYSCRrs>;
///System Configure Register
pub mod syscr;
///RTC_TR (rw) register accessor: Mirrored RTC Time Register
///
///You can [`read`](crate::Reg::read) this register and get [`rtc_tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rtc_tr`]
///module
#[doc(alias = "RTC_TR")]
pub type RtcTr = crate::Reg<rtc_tr::RTC_TRrs>;
///Mirrored RTC Time Register
pub mod rtc_tr;
///RTC_DR (rw) register accessor: Mirrored RTC Date Register
///
///You can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rtc_dr`]
///module
#[doc(alias = "RTC_DR")]
pub type RtcDr = crate::Reg<rtc_dr::RTC_DRrs>;
///Mirrored RTC Date Register
pub mod rtc_dr;
///ULPMCR (rw) register accessor: ULP Memory Control register
///
///You can [`read`](crate::Reg::read) this register and get [`ulpmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulpmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ulpmcr`]
///module
#[doc(alias = "ULPMCR")]
pub type Ulpmcr = crate::Reg<ulpmcr::ULPMCRrs>;
///ULP Memory Control register
pub mod ulpmcr;
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
///MDBGR (rw) register accessor: Memory Debug Register
///
///You can [`read`](crate::Reg::read) this register and get [`mdbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mdbgr`]
///module
#[doc(alias = "MDBGR")]
pub type Mdbgr = crate::Reg<mdbgr::MDBGRrs>;
///Memory Debug Register
pub mod mdbgr;
///USART4_PINR (rw) register accessor: USART4 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart4_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart4_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usart4_pinr`]
///module
#[doc(alias = "USART4_PINR")]
pub type Usart4Pinr = crate::Reg<usart4_pinr::USART4_PINRrs>;
///USART4 Pin Register
pub mod usart4_pinr;
///USART5_PINR (rw) register accessor: USART5 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart5_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart5_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usart5_pinr`]
///module
#[doc(alias = "USART5_PINR")]
pub type Usart5Pinr = crate::Reg<usart5_pinr::USART5_PINRrs>;
///USART5 Pin Register
pub mod usart5_pinr;
///PTA_PINR (rw) register accessor: PTA Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`pta_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pta_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pta_pinr`]
///module
#[doc(alias = "PTA_PINR")]
pub type PtaPinr = crate::Reg<pta_pinr::PTA_PINRrs>;
///PTA Pin Register
pub mod pta_pinr;
