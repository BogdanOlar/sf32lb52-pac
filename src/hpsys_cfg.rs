#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bmr: Bmr,
    idr: Idr,
    swcr: Swcr,
    scr: Scr,
    syscr: Syscr,
    rtc_tr: RtcTr,
    rtc_dr: RtcDr,
    ulpmcr: Ulpmcr,
    dbgr: Dbgr,
    mdbgr: Mdbgr,
    _reserved10: [u8; 0x14],
    lpirq: Lpirq,
    usbcr: Usbcr,
    sys_rsvd: SysRsvd,
    i2c1_pinr: I2c1Pinr,
    i2c2_pinr: I2c2Pinr,
    i2c3_pinr: I2c3Pinr,
    i2c4_pinr: I2c4Pinr,
    usart1_pinr: Usart1Pinr,
    usart2_pinr: Usart2Pinr,
    usart3_pinr: Usart3Pinr,
    gptim1_pinr: Gptim1Pinr,
    gptim2_pinr: Gptim2Pinr,
    etr_pinr: EtrPinr,
    lptim1_pinr: Lptim1Pinr,
    lptim2_pinr: Lptim2Pinr,
    atim1_pinr1: Atim1Pinr1,
    atim1_pinr2: Atim1Pinr2,
    atim1_pinr3: Atim1Pinr3,
    pta_pinr: PtaPinr,
    anau_cr: AnauCr,
    anau_rsvd: AnauRsvd,
    anatr: Anatr,
    cau2_cr: Cau2Cr,
    cau2_rsvd: Cau2Rsvd,
}
impl RegisterBlock {
    ///0x00 - Boot Mode Register
    #[inline(always)]
    pub const fn bmr(&self) -> &Bmr {
        &self.bmr
    }
    ///0x04 - ID Register
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    ///0x08 - SW Control Register
    #[inline(always)]
    pub const fn swcr(&self) -> &Swcr {
        &self.swcr
    }
    ///0x0c - Security Control Register
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    ///0x10 - System Configure Register
    #[inline(always)]
    pub const fn syscr(&self) -> &Syscr {
        &self.syscr
    }
    ///0x14 - Mirrored RTC Time Register
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RtcTr {
        &self.rtc_tr
    }
    ///0x18 - Mirrored RTC Date Register
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RtcDr {
        &self.rtc_dr
    }
    ///0x1c - ULP Memory Control register
    #[inline(always)]
    pub const fn ulpmcr(&self) -> &Ulpmcr {
        &self.ulpmcr
    }
    ///0x20 - Debug Select Register
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
    ///0x24 - Memory Debug Register
    #[inline(always)]
    pub const fn mdbgr(&self) -> &Mdbgr {
        &self.mdbgr
    }
    ///0x3c - Interrupt Selection for LCPU
    #[inline(always)]
    pub const fn lpirq(&self) -> &Lpirq {
        &self.lpirq
    }
    ///0x40 - USB Control register
    #[inline(always)]
    pub const fn usbcr(&self) -> &Usbcr {
        &self.usbcr
    }
    ///0x44 - HPSYS RSVD Register
    #[inline(always)]
    pub const fn sys_rsvd(&self) -> &SysRsvd {
        &self.sys_rsvd
    }
    ///0x48 - I2C1 Pin Register
    #[inline(always)]
    pub const fn i2c1_pinr(&self) -> &I2c1Pinr {
        &self.i2c1_pinr
    }
    ///0x4c - I2C2 Pin Register
    #[inline(always)]
    pub const fn i2c2_pinr(&self) -> &I2c2Pinr {
        &self.i2c2_pinr
    }
    ///0x50 - I2C3 Pin Register
    #[inline(always)]
    pub const fn i2c3_pinr(&self) -> &I2c3Pinr {
        &self.i2c3_pinr
    }
    ///0x54 - I2C4 Pin Register
    #[inline(always)]
    pub const fn i2c4_pinr(&self) -> &I2c4Pinr {
        &self.i2c4_pinr
    }
    ///0x58 - USART1 Pin Register
    #[inline(always)]
    pub const fn usart1_pinr(&self) -> &Usart1Pinr {
        &self.usart1_pinr
    }
    ///0x5c - USART2 Pin Register
    #[inline(always)]
    pub const fn usart2_pinr(&self) -> &Usart2Pinr {
        &self.usart2_pinr
    }
    ///0x60 - USART3 Pin Register
    #[inline(always)]
    pub const fn usart3_pinr(&self) -> &Usart3Pinr {
        &self.usart3_pinr
    }
    ///0x64 - GPTIM1 Pin Register
    #[inline(always)]
    pub const fn gptim1_pinr(&self) -> &Gptim1Pinr {
        &self.gptim1_pinr
    }
    ///0x68 - GPTIM2 Pin Register
    #[inline(always)]
    pub const fn gptim2_pinr(&self) -> &Gptim2Pinr {
        &self.gptim2_pinr
    }
    ///0x6c - GPTIM ETR Pin Register
    #[inline(always)]
    pub const fn etr_pinr(&self) -> &EtrPinr {
        &self.etr_pinr
    }
    ///0x70 - LPTIM1 Pin Register
    #[inline(always)]
    pub const fn lptim1_pinr(&self) -> &Lptim1Pinr {
        &self.lptim1_pinr
    }
    ///0x74 - LPTIM2 Pin Register
    #[inline(always)]
    pub const fn lptim2_pinr(&self) -> &Lptim2Pinr {
        &self.lptim2_pinr
    }
    ///0x78 - ATIM1 Pin Register 1
    #[inline(always)]
    pub const fn atim1_pinr1(&self) -> &Atim1Pinr1 {
        &self.atim1_pinr1
    }
    ///0x7c - ATIM1 Pin Register 2
    #[inline(always)]
    pub const fn atim1_pinr2(&self) -> &Atim1Pinr2 {
        &self.atim1_pinr2
    }
    ///0x80 - ATIM1 Pin Register 3
    #[inline(always)]
    pub const fn atim1_pinr3(&self) -> &Atim1Pinr3 {
        &self.atim1_pinr3
    }
    ///0x84 - PTA Pin Register
    #[inline(always)]
    pub const fn pta_pinr(&self) -> &PtaPinr {
        &self.pta_pinr
    }
    ///0x88 - ANAU Control Register
    #[inline(always)]
    pub const fn anau_cr(&self) -> &AnauCr {
        &self.anau_cr
    }
    ///0x8c - ANAU Reserve Register
    #[inline(always)]
    pub const fn anau_rsvd(&self) -> &AnauRsvd {
        &self.anau_rsvd
    }
    ///0x90 - Analog Test Register
    #[inline(always)]
    pub const fn anatr(&self) -> &Anatr {
        &self.anatr
    }
    ///0x94 - CAU2 Control Register
    #[inline(always)]
    pub const fn cau2_cr(&self) -> &Cau2Cr {
        &self.cau2_cr
    }
    ///0x98 - CAU2 RSVD Register1
    #[inline(always)]
    pub const fn cau2_rsvd(&self) -> &Cau2Rsvd {
        &self.cau2_rsvd
    }
}
///BMR (rw) register accessor: Boot Mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`bmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bmr`]
///module
#[doc(alias = "BMR")]
pub type Bmr = crate::Reg<bmr::BMRrs>;
///Boot Mode Register
pub mod bmr;
///IDR (rw) register accessor: ID Register
///
///You can [`read`](crate::Reg::read) this register and get [`idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@idr`]
///module
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IDRrs>;
///ID Register
pub mod idr;
///SWCR (rw) register accessor: SW Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`swcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@swcr`]
///module
#[doc(alias = "SWCR")]
pub type Swcr = crate::Reg<swcr::SWCRrs>;
///SW Control Register
pub mod swcr;
///SCR (rw) register accessor: Security Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scr`]
///module
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::SCRrs>;
///Security Control Register
pub mod scr;
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
///DBGR (rw) register accessor: Debug Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbgr`]
///module
#[doc(alias = "DBGR")]
pub type Dbgr = crate::Reg<dbgr::DBGRrs>;
///Debug Select Register
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
///LPIRQ (rw) register accessor: Interrupt Selection for LCPU
///
///You can [`read`](crate::Reg::read) this register and get [`lpirq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpirq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lpirq`]
///module
#[doc(alias = "LPIRQ")]
pub type Lpirq = crate::Reg<lpirq::LPIRQrs>;
///Interrupt Selection for LCPU
pub mod lpirq;
///USBCR (rw) register accessor: USB Control register
///
///You can [`read`](crate::Reg::read) this register and get [`usbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usbcr`]
///module
#[doc(alias = "USBCR")]
pub type Usbcr = crate::Reg<usbcr::USBCRrs>;
///USB Control register
pub mod usbcr;
///SYS_RSVD (rw) register accessor: HPSYS RSVD Register
///
///You can [`read`](crate::Reg::read) this register and get [`sys_rsvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_rsvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sys_rsvd`]
///module
#[doc(alias = "SYS_RSVD")]
pub type SysRsvd = crate::Reg<sys_rsvd::SYS_RSVDrs>;
///HPSYS RSVD Register
pub mod sys_rsvd;
///I2C1_PINR (rw) register accessor: I2C1 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2c1_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@i2c1_pinr`]
///module
#[doc(alias = "I2C1_PINR")]
pub type I2c1Pinr = crate::Reg<i2c1_pinr::I2C1_PINRrs>;
///I2C1 Pin Register
pub mod i2c1_pinr;
///I2C2_PINR (rw) register accessor: I2C2 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2c2_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@i2c2_pinr`]
///module
#[doc(alias = "I2C2_PINR")]
pub type I2c2Pinr = crate::Reg<i2c2_pinr::I2C2_PINRrs>;
///I2C2 Pin Register
pub mod i2c2_pinr;
///I2C3_PINR (rw) register accessor: I2C3 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2c3_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c3_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@i2c3_pinr`]
///module
#[doc(alias = "I2C3_PINR")]
pub type I2c3Pinr = crate::Reg<i2c3_pinr::I2C3_PINRrs>;
///I2C3 Pin Register
pub mod i2c3_pinr;
///I2C4_PINR (rw) register accessor: I2C4 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2c4_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c4_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@i2c4_pinr`]
///module
#[doc(alias = "I2C4_PINR")]
pub type I2c4Pinr = crate::Reg<i2c4_pinr::I2C4_PINRrs>;
///I2C4 Pin Register
pub mod i2c4_pinr;
///USART1_PINR (rw) register accessor: USART1 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart1_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usart1_pinr`]
///module
#[doc(alias = "USART1_PINR")]
pub type Usart1Pinr = crate::Reg<usart1_pinr::USART1_PINRrs>;
///USART1 Pin Register
pub mod usart1_pinr;
///USART2_PINR (rw) register accessor: USART2 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart2_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart2_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usart2_pinr`]
///module
#[doc(alias = "USART2_PINR")]
pub type Usart2Pinr = crate::Reg<usart2_pinr::USART2_PINRrs>;
///USART2 Pin Register
pub mod usart2_pinr;
///USART3_PINR (rw) register accessor: USART3 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart3_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart3_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usart3_pinr`]
///module
#[doc(alias = "USART3_PINR")]
pub type Usart3Pinr = crate::Reg<usart3_pinr::USART3_PINRrs>;
///USART3 Pin Register
pub mod usart3_pinr;
///GPTIM1_PINR (rw) register accessor: GPTIM1 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`gptim1_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptim1_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gptim1_pinr`]
///module
#[doc(alias = "GPTIM1_PINR")]
pub type Gptim1Pinr = crate::Reg<gptim1_pinr::GPTIM1_PINRrs>;
///GPTIM1 Pin Register
pub mod gptim1_pinr;
///GPTIM2_PINR (rw) register accessor: GPTIM2 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`gptim2_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptim2_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gptim2_pinr`]
///module
#[doc(alias = "GPTIM2_PINR")]
pub type Gptim2Pinr = crate::Reg<gptim2_pinr::GPTIM2_PINRrs>;
///GPTIM2 Pin Register
pub mod gptim2_pinr;
///ETR_PINR (rw) register accessor: GPTIM ETR Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`etr_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etr_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@etr_pinr`]
///module
#[doc(alias = "ETR_PINR")]
pub type EtrPinr = crate::Reg<etr_pinr::ETR_PINRrs>;
///GPTIM ETR Pin Register
pub mod etr_pinr;
///LPTIM1_PINR (rw) register accessor: LPTIM1 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`lptim1_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lptim1_pinr`]
///module
#[doc(alias = "LPTIM1_PINR")]
pub type Lptim1Pinr = crate::Reg<lptim1_pinr::LPTIM1_PINRrs>;
///LPTIM1 Pin Register
pub mod lptim1_pinr;
///LPTIM2_PINR (rw) register accessor: LPTIM2 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`lptim2_pinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_pinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lptim2_pinr`]
///module
#[doc(alias = "LPTIM2_PINR")]
pub type Lptim2Pinr = crate::Reg<lptim2_pinr::LPTIM2_PINRrs>;
///LPTIM2 Pin Register
pub mod lptim2_pinr;
///ATIM1_PINR1 (rw) register accessor: ATIM1 Pin Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`atim1_pinr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim1_pinr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@atim1_pinr1`]
///module
#[doc(alias = "ATIM1_PINR1")]
pub type Atim1Pinr1 = crate::Reg<atim1_pinr1::ATIM1_PINR1rs>;
///ATIM1 Pin Register 1
pub mod atim1_pinr1;
///ATIM1_PINR2 (rw) register accessor: ATIM1 Pin Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`atim1_pinr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim1_pinr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@atim1_pinr2`]
///module
#[doc(alias = "ATIM1_PINR2")]
pub type Atim1Pinr2 = crate::Reg<atim1_pinr2::ATIM1_PINR2rs>;
///ATIM1 Pin Register 2
pub mod atim1_pinr2;
///ATIM1_PINR3 (rw) register accessor: ATIM1 Pin Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`atim1_pinr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim1_pinr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@atim1_pinr3`]
///module
#[doc(alias = "ATIM1_PINR3")]
pub type Atim1Pinr3 = crate::Reg<atim1_pinr3::ATIM1_PINR3rs>;
///ATIM1 Pin Register 3
pub mod atim1_pinr3;
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
///ANAU_CR (rw) register accessor: ANAU Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`anau_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anau_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@anau_cr`]
///module
#[doc(alias = "ANAU_CR")]
pub type AnauCr = crate::Reg<anau_cr::ANAU_CRrs>;
///ANAU Control Register
pub mod anau_cr;
///ANAU_RSVD (rw) register accessor: ANAU Reserve Register
///
///You can [`read`](crate::Reg::read) this register and get [`anau_rsvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anau_rsvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@anau_rsvd`]
///module
#[doc(alias = "ANAU_RSVD")]
pub type AnauRsvd = crate::Reg<anau_rsvd::ANAU_RSVDrs>;
///ANAU Reserve Register
pub mod anau_rsvd;
///ANATR (rw) register accessor: Analog Test Register
///
///You can [`read`](crate::Reg::read) this register and get [`anatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@anatr`]
///module
#[doc(alias = "ANATR")]
pub type Anatr = crate::Reg<anatr::ANATRrs>;
///Analog Test Register
pub mod anatr;
///CAU2_CR (rw) register accessor: CAU2 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`cau2_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau2_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cau2_cr`]
///module
#[doc(alias = "CAU2_CR")]
pub type Cau2Cr = crate::Reg<cau2_cr::CAU2_CRrs>;
///CAU2 Control Register
pub mod cau2_cr;
///CAU2_RSVD (rw) register accessor: CAU2 RSVD Register1
///
///You can [`read`](crate::Reg::read) this register and get [`cau2_rsvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau2_rsvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cau2_rsvd`]
///module
#[doc(alias = "CAU2_RSVD")]
pub type Cau2Rsvd = crate::Reg<cau2_rsvd::CAU2_RSVDrs>;
///CAU2 RSVD Register1
pub mod cau2_rsvd;
