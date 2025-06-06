#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pmr: Pmr,
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    acr: Acr,
    lscr: Lscr,
    dscr: Dscr,
    sbcr: Sbcr,
    wer: Wer,
    wsr: Wsr,
    wcr: Wcr,
    issr: Issr,
    anacr: Anacr,
    gtimr: Gtimr,
    reserve0: Reserve0,
    reserve1: Reserve1,
}
impl RegisterBlock {
    ///0x00 - Power Mode Register
    #[inline(always)]
    pub const fn pmr(&self) -> &Pmr {
        &self.pmr
    }
    ///0x04 - Control Register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    ///0x08 - Control Register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    ///0x0c - Control Register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    ///0x10 - Active Mode Control register
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    ///0x14 - Light Sleep Ctrl Register
    #[inline(always)]
    pub const fn lscr(&self) -> &Lscr {
        &self.lscr
    }
    ///0x18 - Deep Sleep Ctrl Register
    #[inline(always)]
    pub const fn dscr(&self) -> &Dscr {
        &self.dscr
    }
    ///0x1c - Standby Mode Ctrl Register
    #[inline(always)]
    pub const fn sbcr(&self) -> &Sbcr {
        &self.sbcr
    }
    ///0x20 - Wakeup Enable register
    #[inline(always)]
    pub const fn wer(&self) -> &Wer {
        &self.wer
    }
    ///0x24 - Wakeup Status register
    #[inline(always)]
    pub const fn wsr(&self) -> &Wsr {
        &self.wsr
    }
    ///0x28 - Wakeup Clear register
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    ///0x2c - Inter System Wakeup Register
    #[inline(always)]
    pub const fn issr(&self) -> &Issr {
        &self.issr
    }
    ///0x30 - Analog Control Register
    #[inline(always)]
    pub const fn anacr(&self) -> &Anacr {
        &self.anacr
    }
    ///0x34 - Global Timer Register
    #[inline(always)]
    pub const fn gtimr(&self) -> &Gtimr {
        &self.gtimr
    }
    ///0x38 - Reserved Register 0
    #[inline(always)]
    pub const fn reserve0(&self) -> &Reserve0 {
        &self.reserve0
    }
    ///0x3c - Reserved Register 1
    #[inline(always)]
    pub const fn reserve1(&self) -> &Reserve1 {
        &self.reserve1
    }
}
///PMR (rw) register accessor: Power Mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`pmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pmr`]
///module
#[doc(alias = "PMR")]
pub type Pmr = crate::Reg<pmr::PMRrs>;
///Power Mode Register
pub mod pmr;
///CR1 (rw) register accessor: Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr1`]
///module
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::CR1rs>;
///Control Register 1
pub mod cr1;
///CR2 (rw) register accessor: Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr2`]
///module
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::CR2rs>;
///Control Register 2
pub mod cr2;
///CR3 (rw) register accessor: Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr3`]
///module
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::CR3rs>;
///Control Register 3
pub mod cr3;
///ACR (rw) register accessor: Active Mode Control register
///
///You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@acr`]
///module
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::ACRrs>;
///Active Mode Control register
pub mod acr;
///LSCR (rw) register accessor: Light Sleep Ctrl Register
///
///You can [`read`](crate::Reg::read) this register and get [`lscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lscr`]
///module
#[doc(alias = "LSCR")]
pub type Lscr = crate::Reg<lscr::LSCRrs>;
///Light Sleep Ctrl Register
pub mod lscr;
///DSCR (rw) register accessor: Deep Sleep Ctrl Register
///
///You can [`read`](crate::Reg::read) this register and get [`dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dscr`]
///module
#[doc(alias = "DSCR")]
pub type Dscr = crate::Reg<dscr::DSCRrs>;
///Deep Sleep Ctrl Register
pub mod dscr;
///SBCR (rw) register accessor: Standby Mode Ctrl Register
///
///You can [`read`](crate::Reg::read) this register and get [`sbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sbcr`]
///module
#[doc(alias = "SBCR")]
pub type Sbcr = crate::Reg<sbcr::SBCRrs>;
///Standby Mode Ctrl Register
pub mod sbcr;
///WER (rw) register accessor: Wakeup Enable register
///
///You can [`read`](crate::Reg::read) this register and get [`wer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wer`]
///module
#[doc(alias = "WER")]
pub type Wer = crate::Reg<wer::WERrs>;
///Wakeup Enable register
pub mod wer;
///WSR (rw) register accessor: Wakeup Status register
///
///You can [`read`](crate::Reg::read) this register and get [`wsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wsr`]
///module
#[doc(alias = "WSR")]
pub type Wsr = crate::Reg<wsr::WSRrs>;
///Wakeup Status register
pub mod wsr;
///WCR (rw) register accessor: Wakeup Clear register
///
///You can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wcr`]
///module
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WCRrs>;
///Wakeup Clear register
pub mod wcr;
///ISSR (rw) register accessor: Inter System Wakeup Register
///
///You can [`read`](crate::Reg::read) this register and get [`issr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`issr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@issr`]
///module
#[doc(alias = "ISSR")]
pub type Issr = crate::Reg<issr::ISSRrs>;
///Inter System Wakeup Register
pub mod issr;
///ANACR (rw) register accessor: Analog Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`anacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@anacr`]
///module
#[doc(alias = "ANACR")]
pub type Anacr = crate::Reg<anacr::ANACRrs>;
///Analog Control Register
pub mod anacr;
///GTIMR (rw) register accessor: Global Timer Register
///
///You can [`read`](crate::Reg::read) this register and get [`gtimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gtimr`]
///module
#[doc(alias = "GTIMR")]
pub type Gtimr = crate::Reg<gtimr::GTIMRrs>;
///Global Timer Register
pub mod gtimr;
///RESERVE0 (rw) register accessor: Reserved Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`reserve0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserve0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reserve0`]
///module
#[doc(alias = "RESERVE0")]
pub type Reserve0 = crate::Reg<reserve0::RESERVE0rs>;
///Reserved Register 0
pub mod reserve0;
///RESERVE1 (rw) register accessor: Reserved Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`reserve1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserve1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reserve1`]
///module
#[doc(alias = "RESERVE1")]
pub type Reserve1 = crate::Reg<reserve1::RESERVE1rs>;
///Reserved Register 1
pub mod reserve1;
