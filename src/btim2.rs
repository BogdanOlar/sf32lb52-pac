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
    _reserved6: [u8; 0x0c],
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
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
