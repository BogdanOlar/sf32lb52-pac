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
    target: Target,
    actual: Actual,
    pre_wkup: PreWkup,
    slp_cfg: SlpCfg,
    slp_ctrl: SlpCtrl,
    anacr: Anacr,
    gtimr: Gtimr,
    reserve0: Reserve0,
    reserve1: Reserve1,
    _reserved21: [u8; 0xac],
    spr: Spr,
    pcr: Pcr,
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
    ///0x2c - Inter System Status Register
    #[inline(always)]
    pub const fn issr(&self) -> &Issr {
        &self.issr
    }
    ///0x30 - BT sleep time target
    #[inline(always)]
    pub const fn target(&self) -> &Target {
        &self.target
    }
    ///0x34 - BT actual sleep time
    #[inline(always)]
    pub const fn actual(&self) -> &Actual {
        &self.actual
    }
    ///0x38 - time before bt awake
    #[inline(always)]
    pub const fn pre_wkup(&self) -> &PreWkup {
        &self.pre_wkup
    }
    ///0x3c - BT sleep configuration
    #[inline(always)]
    pub const fn slp_cfg(&self) -> &SlpCfg {
        &self.slp_cfg
    }
    ///0x40 - BT sleep control
    #[inline(always)]
    pub const fn slp_ctrl(&self) -> &SlpCtrl {
        &self.slp_ctrl
    }
    ///0x44 - Analog Control Register
    #[inline(always)]
    pub const fn anacr(&self) -> &Anacr {
        &self.anacr
    }
    ///0x48 - Global Timer Register
    #[inline(always)]
    pub const fn gtimr(&self) -> &Gtimr {
        &self.gtimr
    }
    ///0x4c - Reserved Register 0
    #[inline(always)]
    pub const fn reserve0(&self) -> &Reserve0 {
        &self.reserve0
    }
    ///0x50 - Reserved Register 1
    #[inline(always)]
    pub const fn reserve1(&self) -> &Reserve1 {
        &self.reserve1
    }
    ///0x100 - Stack Pointer Register
    #[inline(always)]
    pub const fn spr(&self) -> &Spr {
        &self.spr
    }
    ///0x104 - Pointer Counter Register
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
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
///ISSR (rw) register accessor: Inter System Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`issr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`issr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@issr`]
///module
#[doc(alias = "ISSR")]
pub type Issr = crate::Reg<issr::ISSRrs>;
///Inter System Status Register
pub mod issr;
///TARGET (rw) register accessor: BT sleep time target
///
///You can [`read`](crate::Reg::read) this register and get [`target::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@target`]
///module
#[doc(alias = "TARGET")]
pub type Target = crate::Reg<target::TARGETrs>;
///BT sleep time target
pub mod target;
///ACTUAL (rw) register accessor: BT actual sleep time
///
///You can [`read`](crate::Reg::read) this register and get [`actual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@actual`]
///module
#[doc(alias = "ACTUAL")]
pub type Actual = crate::Reg<actual::ACTUALrs>;
///BT actual sleep time
pub mod actual;
///PRE_WKUP (rw) register accessor: time before bt awake
///
///You can [`read`](crate::Reg::read) this register and get [`pre_wkup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_wkup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pre_wkup`]
///module
#[doc(alias = "PRE_WKUP")]
pub type PreWkup = crate::Reg<pre_wkup::PRE_WKUPrs>;
///time before bt awake
pub mod pre_wkup;
///SLP_CFG (rw) register accessor: BT sleep configuration
///
///You can [`read`](crate::Reg::read) this register and get [`slp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@slp_cfg`]
///module
#[doc(alias = "SLP_CFG")]
pub type SlpCfg = crate::Reg<slp_cfg::SLP_CFGrs>;
///BT sleep configuration
pub mod slp_cfg;
///SLP_CTRL (rw) register accessor: BT sleep control
///
///You can [`read`](crate::Reg::read) this register and get [`slp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@slp_ctrl`]
///module
#[doc(alias = "SLP_CTRL")]
pub type SlpCtrl = crate::Reg<slp_ctrl::SLP_CTRLrs>;
///BT sleep control
pub mod slp_ctrl;
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
///SPR (rw) register accessor: Stack Pointer Register
///
///You can [`read`](crate::Reg::read) this register and get [`spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@spr`]
///module
#[doc(alias = "SPR")]
pub type Spr = crate::Reg<spr::SPRrs>;
///Stack Pointer Register
pub mod spr;
///PCR (rw) register accessor: Pointer Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pcr`]
///module
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PCRrs>;
///Pointer Counter Register
pub mod pcr;
