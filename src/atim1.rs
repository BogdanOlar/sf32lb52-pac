#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    smcr: Smcr,
    dier: Dier,
    sr: Sr,
    egr: Egr,
    ccmr1: Ccmr1,
    ccmr2: Ccmr2,
    ccer: Ccer,
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
    rcr: Rcr,
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    ccr4: Ccr4,
    bdtr: Bdtr,
    _reserved18: [u8; 0x0c],
    ccmr3: Ccmr3,
    ccr5: Ccr5,
    ccr6: Ccr6,
    af1: Af1,
    af2: Af2,
}
impl RegisterBlock {
    ///0x00 - TIM control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    ///0x04 - TIM control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    ///0x08 - TIM slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &Smcr {
        &self.smcr
    }
    ///0x0c - TIM DMA/Interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &Dier {
        &self.dier
    }
    ///0x10 - TIM status register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x14 - Event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &Egr {
        &self.egr
    }
    ///0x18 - TIM capture/compare mode register 1
    #[inline(always)]
    pub const fn ccmr1(&self) -> &Ccmr1 {
        &self.ccmr1
    }
    ///0x1c - TIM capture/compare mode register 2
    #[inline(always)]
    pub const fn ccmr2(&self) -> &Ccmr2 {
        &self.ccmr2
    }
    ///0x20 - Capture/Compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &Ccer {
        &self.ccer
    }
    ///0x24 - Counter
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    ///0x28 - Prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    ///0x2c - Auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    ///0x30 - Repetition counter register
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    ///0x34 - Capture/Compare register 1
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    ///0x38 - Capture/Compare register 2
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    ///0x3c - Capture/Compare register 3
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    ///0x40 - Capture/Compare register 4
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    ///0x44 - TIM break and dead-time register
    #[inline(always)]
    pub const fn bdtr(&self) -> &Bdtr {
        &self.bdtr
    }
    ///0x54 - TIM capture/compare mode register 3
    #[inline(always)]
    pub const fn ccmr3(&self) -> &Ccmr3 {
        &self.ccmr3
    }
    ///0x58 - Capture/Compare register 5
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    ///0x5c - Capture/Compare register 6
    #[inline(always)]
    pub const fn ccr6(&self) -> &Ccr6 {
        &self.ccr6
    }
    ///0x60 - Alternate function option register
    #[inline(always)]
    pub const fn af1(&self) -> &Af1 {
        &self.af1
    }
    ///0x64 - Alternate function option register 2
    #[inline(always)]
    pub const fn af2(&self) -> &Af2 {
        &self.af2
    }
}
///CR1 (rw) register accessor: TIM control register 1
///
///You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr1`]
///module
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::CR1rs>;
///TIM control register 1
pub mod cr1;
///CR2 (rw) register accessor: TIM control register 2
///
///You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr2`]
///module
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::CR2rs>;
///TIM control register 2
pub mod cr2;
///SMCR (rw) register accessor: TIM slave mode control register
///
///You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@smcr`]
///module
#[doc(alias = "SMCR")]
pub type Smcr = crate::Reg<smcr::SMCRrs>;
///TIM slave mode control register
pub mod smcr;
///DIER (rw) register accessor: TIM DMA/Interrupt enable register
///
///You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dier`]
///module
#[doc(alias = "DIER")]
pub type Dier = crate::Reg<dier::DIERrs>;
///TIM DMA/Interrupt enable register
pub mod dier;
///SR (rw) register accessor: TIM status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sr`]
///module
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SRrs>;
///TIM status register
pub mod sr;
///EGR (rw) register accessor: Event generation register
///
///You can [`read`](crate::Reg::read) this register and get [`egr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@egr`]
///module
#[doc(alias = "EGR")]
pub type Egr = crate::Reg<egr::EGRrs>;
///Event generation register
pub mod egr;
///CCMR1 (rw) register accessor: TIM capture/compare mode register 1
///
///You can [`read`](crate::Reg::read) this register and get [`ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccmr1`]
///module
#[doc(alias = "CCMR1")]
pub type Ccmr1 = crate::Reg<ccmr1::CCMR1rs>;
///TIM capture/compare mode register 1
pub mod ccmr1;
///CCMR2 (rw) register accessor: TIM capture/compare mode register 2
///
///You can [`read`](crate::Reg::read) this register and get [`ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccmr2`]
///module
#[doc(alias = "CCMR2")]
pub type Ccmr2 = crate::Reg<ccmr2::CCMR2rs>;
///TIM capture/compare mode register 2
pub mod ccmr2;
///CCER (rw) register accessor: Capture/Compare enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccer`]
///module
#[doc(alias = "CCER")]
pub type Ccer = crate::Reg<ccer::CCERrs>;
///Capture/Compare enable register
pub mod ccer;
///CNT (rw) register accessor: Counter
///
///You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cnt`]
///module
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CNTrs>;
///Counter
pub mod cnt;
///PSC (rw) register accessor: Prescaler
///
///You can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@psc`]
///module
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PSCrs>;
///Prescaler
pub mod psc;
///ARR (rw) register accessor: Auto-reload register
///
///You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@arr`]
///module
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ARRrs>;
///Auto-reload register
pub mod arr;
///RCR (rw) register accessor: Repetition counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr`]
///module
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RCRrs>;
///Repetition counter register
pub mod rcr;
///CCR1 (rw) register accessor: Capture/Compare register 1
///
///You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr1`]
///module
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::CCR1rs>;
///Capture/Compare register 1
pub mod ccr1;
///CCR2 (rw) register accessor: Capture/Compare register 2
///
///You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr2`]
///module
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::CCR2rs>;
///Capture/Compare register 2
pub mod ccr2;
///CCR3 (rw) register accessor: Capture/Compare register 3
///
///You can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr3`]
///module
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::CCR3rs>;
///Capture/Compare register 3
pub mod ccr3;
///CCR4 (rw) register accessor: Capture/Compare register 4
///
///You can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr4`]
///module
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::CCR4rs>;
///Capture/Compare register 4
pub mod ccr4;
///BDTR (rw) register accessor: TIM break and dead-time register
///
///You can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bdtr`]
///module
#[doc(alias = "BDTR")]
pub type Bdtr = crate::Reg<bdtr::BDTRrs>;
///TIM break and dead-time register
pub mod bdtr;
///CCMR3 (rw) register accessor: TIM capture/compare mode register 3
///
///You can [`read`](crate::Reg::read) this register and get [`ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccmr3`]
///module
#[doc(alias = "CCMR3")]
pub type Ccmr3 = crate::Reg<ccmr3::CCMR3rs>;
///TIM capture/compare mode register 3
pub mod ccmr3;
///CCR5 (rw) register accessor: Capture/Compare register 5
///
///You can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr5`]
///module
#[doc(alias = "CCR5")]
pub type Ccr5 = crate::Reg<ccr5::CCR5rs>;
///Capture/Compare register 5
pub mod ccr5;
///CCR6 (rw) register accessor: Capture/Compare register 6
///
///You can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr6`]
///module
#[doc(alias = "CCR6")]
pub type Ccr6 = crate::Reg<ccr6::CCR6rs>;
///Capture/Compare register 6
pub mod ccr6;
///AF1 (rw) register accessor: Alternate function option register
///
///You can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@af1`]
///module
#[doc(alias = "AF1")]
pub type Af1 = crate::Reg<af1::AF1rs>;
///Alternate function option register
pub mod af1;
///AF2 (rw) register accessor: Alternate function option register 2
///
///You can [`read`](crate::Reg::read) this register and get [`af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@af2`]
///module
#[doc(alias = "AF2")]
pub type Af2 = crate::Reg<af2::AF2rs>;
///Alternate function option register 2
pub mod af2;
