#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    wdt_cvr0: WdtCvr0,
    wdt_cvr1: WdtCvr1,
    wdt_cr: WdtCr,
    wdt_ccr: WdtCcr,
    wdt_icr: WdtIcr,
    wdt_sr: WdtSr,
    wdt_wp: WdtWp,
    wdt_fg: WdtFg,
}
impl RegisterBlock {
    ///0x00 - WatchDog Counter Value 0
    #[inline(always)]
    pub const fn wdt_cvr0(&self) -> &WdtCvr0 {
        &self.wdt_cvr0
    }
    ///0x04 - WatchDog Counter Value 1
    #[inline(always)]
    pub const fn wdt_cvr1(&self) -> &WdtCvr1 {
        &self.wdt_cvr1
    }
    ///0x08 - WatchDog Control Register
    #[inline(always)]
    pub const fn wdt_cr(&self) -> &WdtCr {
        &self.wdt_cr
    }
    ///0x0c - WatchDog Counter Control Register
    #[inline(always)]
    pub const fn wdt_ccr(&self) -> &WdtCcr {
        &self.wdt_ccr
    }
    ///0x10 - WatchDog Interrupt Clear Register
    #[inline(always)]
    pub const fn wdt_icr(&self) -> &WdtIcr {
        &self.wdt_icr
    }
    ///0x14 - WatchDog Status Register
    #[inline(always)]
    pub const fn wdt_sr(&self) -> &WdtSr {
        &self.wdt_sr
    }
    ///0x18 - WatchDog Write Protect Register
    #[inline(always)]
    pub const fn wdt_wp(&self) -> &WdtWp {
        &self.wdt_wp
    }
    ///0x1c - WatchDog Flag Register
    #[inline(always)]
    pub const fn wdt_fg(&self) -> &WdtFg {
        &self.wdt_fg
    }
}
///WDT_CVR0 (rw) register accessor: WatchDog Counter Value 0
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_cvr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cvr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_cvr0`]
///module
#[doc(alias = "WDT_CVR0")]
pub type WdtCvr0 = crate::Reg<wdt_cvr0::WDT_CVR0rs>;
///WatchDog Counter Value 0
pub mod wdt_cvr0;
///WDT_CVR1 (rw) register accessor: WatchDog Counter Value 1
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_cvr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cvr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_cvr1`]
///module
#[doc(alias = "WDT_CVR1")]
pub type WdtCvr1 = crate::Reg<wdt_cvr1::WDT_CVR1rs>;
///WatchDog Counter Value 1
pub mod wdt_cvr1;
///WDT_CR (rw) register accessor: WatchDog Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_cr`]
///module
#[doc(alias = "WDT_CR")]
pub type WdtCr = crate::Reg<wdt_cr::WDT_CRrs>;
///WatchDog Control Register
pub mod wdt_cr;
///WDT_CCR (rw) register accessor: WatchDog Counter Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_ccr`]
///module
#[doc(alias = "WDT_CCR")]
pub type WdtCcr = crate::Reg<wdt_ccr::WDT_CCRrs>;
///WatchDog Counter Control Register
pub mod wdt_ccr;
///WDT_ICR (rw) register accessor: WatchDog Interrupt Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_icr`]
///module
#[doc(alias = "WDT_ICR")]
pub type WdtIcr = crate::Reg<wdt_icr::WDT_ICRrs>;
///WatchDog Interrupt Clear Register
pub mod wdt_icr;
///WDT_SR (rw) register accessor: WatchDog Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_sr`]
///module
#[doc(alias = "WDT_SR")]
pub type WdtSr = crate::Reg<wdt_sr::WDT_SRrs>;
///WatchDog Status Register
pub mod wdt_sr;
///WDT_WP (rw) register accessor: WatchDog Write Protect Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_wp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_wp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_wp`]
///module
#[doc(alias = "WDT_WP")]
pub type WdtWp = crate::Reg<wdt_wp::WDT_WPrs>;
///WatchDog Write Protect Register
pub mod wdt_wp;
///WDT_FG (rw) register accessor: WatchDog Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_fg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_fg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdt_fg`]
///module
#[doc(alias = "WDT_FG")]
pub type WdtFg = crate::Reg<wdt_fg::WDT_FGrs>;
///WatchDog Flag Register
pub mod wdt_fg;
