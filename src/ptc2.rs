#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: Isr,
    icr: Icr,
    ier: Ier,
    _reserved3: [u8; 0x04],
    tcr1: Tcr1,
    tar1: Tar1,
    tdr1: Tdr1,
    rcr1: Rcr1,
    tcr2: Tcr2,
    tar2: Tar2,
    tdr2: Tdr2,
    rcr2: Rcr2,
    tcr3: Tcr3,
    tar3: Tar3,
    tdr3: Tdr3,
    rcr3: Rcr3,
    tcr4: Tcr4,
    tar4: Tar4,
    tdr4: Tdr4,
    rcr4: Rcr4,
    tcr5: Tcr5,
    tar5: Tar5,
    tdr5: Tdr5,
    rcr5: Rcr5,
    tcr6: Tcr6,
    tar6: Tar6,
    tdr6: Tdr6,
    rcr6: Rcr6,
    tcr7: Tcr7,
    tar7: Tar7,
    tdr7: Tdr7,
    rcr7: Rcr7,
    tcr8: Tcr8,
    tar8: Tar8,
    tdr8: Tdr8,
    rcr8: Rcr8,
    _reserved35: [u8; 0x40],
    mem1: Mem1,
    mem2: Mem2,
    mem3: Mem3,
    mem4: Mem4,
    gpio31_0: Gpio31_0,
    gpio63_32: Gpio63_32,
    gpio95_64: Gpio95_64,
}
impl RegisterBlock {
    ///0x00 - interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    ///0x04 - interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    ///0x08 - interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    ///0x10 - task 1 control register
    #[inline(always)]
    pub const fn tcr1(&self) -> &Tcr1 {
        &self.tcr1
    }
    ///0x14 - task 1 address register
    #[inline(always)]
    pub const fn tar1(&self) -> &Tar1 {
        &self.tar1
    }
    ///0x18 - task 1 data register
    #[inline(always)]
    pub const fn tdr1(&self) -> &Tdr1 {
        &self.tdr1
    }
    ///0x1c - task 1 repetition and delay counter register
    #[inline(always)]
    pub const fn rcr1(&self) -> &Rcr1 {
        &self.rcr1
    }
    ///0x20 -
    #[inline(always)]
    pub const fn tcr2(&self) -> &Tcr2 {
        &self.tcr2
    }
    ///0x24 -
    #[inline(always)]
    pub const fn tar2(&self) -> &Tar2 {
        &self.tar2
    }
    ///0x28 -
    #[inline(always)]
    pub const fn tdr2(&self) -> &Tdr2 {
        &self.tdr2
    }
    ///0x2c - task 2 repetition and delay counter register
    #[inline(always)]
    pub const fn rcr2(&self) -> &Rcr2 {
        &self.rcr2
    }
    ///0x30 -
    #[inline(always)]
    pub const fn tcr3(&self) -> &Tcr3 {
        &self.tcr3
    }
    ///0x34 -
    #[inline(always)]
    pub const fn tar3(&self) -> &Tar3 {
        &self.tar3
    }
    ///0x38 -
    #[inline(always)]
    pub const fn tdr3(&self) -> &Tdr3 {
        &self.tdr3
    }
    ///0x3c - task 3 repetition and delay counter register
    #[inline(always)]
    pub const fn rcr3(&self) -> &Rcr3 {
        &self.rcr3
    }
    ///0x40 -
    #[inline(always)]
    pub const fn tcr4(&self) -> &Tcr4 {
        &self.tcr4
    }
    ///0x44 -
    #[inline(always)]
    pub const fn tar4(&self) -> &Tar4 {
        &self.tar4
    }
    ///0x48 -
    #[inline(always)]
    pub const fn tdr4(&self) -> &Tdr4 {
        &self.tdr4
    }
    ///0x4c - task 4 repetition and delay counter register
    #[inline(always)]
    pub const fn rcr4(&self) -> &Rcr4 {
        &self.rcr4
    }
    ///0x50 -
    #[inline(always)]
    pub const fn tcr5(&self) -> &Tcr5 {
        &self.tcr5
    }
    ///0x54 -
    #[inline(always)]
    pub const fn tar5(&self) -> &Tar5 {
        &self.tar5
    }
    ///0x58 -
    #[inline(always)]
    pub const fn tdr5(&self) -> &Tdr5 {
        &self.tdr5
    }
    ///0x5c - task 5 repetition counter register
    #[inline(always)]
    pub const fn rcr5(&self) -> &Rcr5 {
        &self.rcr5
    }
    ///0x60 -
    #[inline(always)]
    pub const fn tcr6(&self) -> &Tcr6 {
        &self.tcr6
    }
    ///0x64 -
    #[inline(always)]
    pub const fn tar6(&self) -> &Tar6 {
        &self.tar6
    }
    ///0x68 -
    #[inline(always)]
    pub const fn tdr6(&self) -> &Tdr6 {
        &self.tdr6
    }
    ///0x6c - task 6 repetition counter register
    #[inline(always)]
    pub const fn rcr6(&self) -> &Rcr6 {
        &self.rcr6
    }
    ///0x70 -
    #[inline(always)]
    pub const fn tcr7(&self) -> &Tcr7 {
        &self.tcr7
    }
    ///0x74 -
    #[inline(always)]
    pub const fn tar7(&self) -> &Tar7 {
        &self.tar7
    }
    ///0x78 -
    #[inline(always)]
    pub const fn tdr7(&self) -> &Tdr7 {
        &self.tdr7
    }
    ///0x7c - task 7 repetition counter register
    #[inline(always)]
    pub const fn rcr7(&self) -> &Rcr7 {
        &self.rcr7
    }
    ///0x80 -
    #[inline(always)]
    pub const fn tcr8(&self) -> &Tcr8 {
        &self.tcr8
    }
    ///0x84 -
    #[inline(always)]
    pub const fn tar8(&self) -> &Tar8 {
        &self.tar8
    }
    ///0x88 -
    #[inline(always)]
    pub const fn tdr8(&self) -> &Tdr8 {
        &self.tdr8
    }
    ///0x8c - task 8 repetition counter register
    #[inline(always)]
    pub const fn rcr8(&self) -> &Rcr8 {
        &self.rcr8
    }
    ///0xd0 - temporary memory 1
    #[inline(always)]
    pub const fn mem1(&self) -> &Mem1 {
        &self.mem1
    }
    ///0xd4 - temporary memory 2
    #[inline(always)]
    pub const fn mem2(&self) -> &Mem2 {
        &self.mem2
    }
    ///0xd8 - temporary memory 3
    #[inline(always)]
    pub const fn mem3(&self) -> &Mem3 {
        &self.mem3
    }
    ///0xdc - temporary memory 4
    #[inline(always)]
    pub const fn mem4(&self) -> &Mem4 {
        &self.mem4
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn gpio31_0(&self) -> &Gpio31_0 {
        &self.gpio31_0
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn gpio63_32(&self) -> &Gpio63_32 {
        &self.gpio63_32
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn gpio95_64(&self) -> &Gpio95_64 {
        &self.gpio95_64
    }
}
///ISR (rw) register accessor: interrupt status register
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@isr`]
///module
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::ISRrs>;
///interrupt status register
pub mod isr;
///ICR (rw) register accessor: interrupt clear register
///
///You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@icr`]
///module
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::ICRrs>;
///interrupt clear register
pub mod icr;
///IER (rw) register accessor: interrupt enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ier`]
///module
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IERrs>;
///interrupt enable register
pub mod ier;
///TCR1 (rw) register accessor: task 1 control register
///
///You can [`read`](crate::Reg::read) this register and get [`tcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr1`]
///module
#[doc(alias = "TCR1")]
pub type Tcr1 = crate::Reg<tcr1::TCR1rs>;
///task 1 control register
pub mod tcr1;
///TAR1 (rw) register accessor: task 1 address register
///
///You can [`read`](crate::Reg::read) this register and get [`tar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar1`]
///module
#[doc(alias = "TAR1")]
pub type Tar1 = crate::Reg<tar1::TAR1rs>;
///task 1 address register
pub mod tar1;
///TDR1 (rw) register accessor: task 1 data register
///
///You can [`read`](crate::Reg::read) this register and get [`tdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr1`]
///module
#[doc(alias = "TDR1")]
pub type Tdr1 = crate::Reg<tdr1::TDR1rs>;
///task 1 data register
pub mod tdr1;
///RCR1 (rw) register accessor: task 1 repetition and delay counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr1`]
///module
#[doc(alias = "RCR1")]
pub type Rcr1 = crate::Reg<rcr1::RCR1rs>;
///task 1 repetition and delay counter register
pub mod rcr1;
///TCR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr2`]
///module
#[doc(alias = "TCR2")]
pub type Tcr2 = crate::Reg<tcr2::TCR2rs>;
///
pub mod tcr2;
///TAR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar2`]
///module
#[doc(alias = "TAR2")]
pub type Tar2 = crate::Reg<tar2::TAR2rs>;
///
pub mod tar2;
///TDR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr2`]
///module
#[doc(alias = "TDR2")]
pub type Tdr2 = crate::Reg<tdr2::TDR2rs>;
///
pub mod tdr2;
///RCR2 (rw) register accessor: task 2 repetition and delay counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr2`]
///module
#[doc(alias = "RCR2")]
pub type Rcr2 = crate::Reg<rcr2::RCR2rs>;
///task 2 repetition and delay counter register
pub mod rcr2;
///TCR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr3`]
///module
#[doc(alias = "TCR3")]
pub type Tcr3 = crate::Reg<tcr3::TCR3rs>;
///
pub mod tcr3;
///TAR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar3`]
///module
#[doc(alias = "TAR3")]
pub type Tar3 = crate::Reg<tar3::TAR3rs>;
///
pub mod tar3;
///TDR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr3`]
///module
#[doc(alias = "TDR3")]
pub type Tdr3 = crate::Reg<tdr3::TDR3rs>;
///
pub mod tdr3;
///RCR3 (rw) register accessor: task 3 repetition and delay counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr3`]
///module
#[doc(alias = "RCR3")]
pub type Rcr3 = crate::Reg<rcr3::RCR3rs>;
///task 3 repetition and delay counter register
pub mod rcr3;
///TCR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr4`]
///module
#[doc(alias = "TCR4")]
pub type Tcr4 = crate::Reg<tcr4::TCR4rs>;
///
pub mod tcr4;
///TAR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar4`]
///module
#[doc(alias = "TAR4")]
pub type Tar4 = crate::Reg<tar4::TAR4rs>;
///
pub mod tar4;
///TDR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr4`]
///module
#[doc(alias = "TDR4")]
pub type Tdr4 = crate::Reg<tdr4::TDR4rs>;
///
pub mod tdr4;
///RCR4 (rw) register accessor: task 4 repetition and delay counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr4`]
///module
#[doc(alias = "RCR4")]
pub type Rcr4 = crate::Reg<rcr4::RCR4rs>;
///task 4 repetition and delay counter register
pub mod rcr4;
///TCR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr5`]
///module
#[doc(alias = "TCR5")]
pub type Tcr5 = crate::Reg<tcr5::TCR5rs>;
///
pub mod tcr5;
///TAR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar5`]
///module
#[doc(alias = "TAR5")]
pub type Tar5 = crate::Reg<tar5::TAR5rs>;
///
pub mod tar5;
///TDR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr5`]
///module
#[doc(alias = "TDR5")]
pub type Tdr5 = crate::Reg<tdr5::TDR5rs>;
///
pub mod tdr5;
///RCR5 (rw) register accessor: task 5 repetition counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr5`]
///module
#[doc(alias = "RCR5")]
pub type Rcr5 = crate::Reg<rcr5::RCR5rs>;
///task 5 repetition counter register
pub mod rcr5;
///TCR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr6`]
///module
#[doc(alias = "TCR6")]
pub type Tcr6 = crate::Reg<tcr6::TCR6rs>;
///
pub mod tcr6;
///TAR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar6`]
///module
#[doc(alias = "TAR6")]
pub type Tar6 = crate::Reg<tar6::TAR6rs>;
///
pub mod tar6;
///TDR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr6`]
///module
#[doc(alias = "TDR6")]
pub type Tdr6 = crate::Reg<tdr6::TDR6rs>;
///
pub mod tdr6;
///RCR6 (rw) register accessor: task 6 repetition counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr6`]
///module
#[doc(alias = "RCR6")]
pub type Rcr6 = crate::Reg<rcr6::RCR6rs>;
///task 6 repetition counter register
pub mod rcr6;
///TCR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr7`]
///module
#[doc(alias = "TCR7")]
pub type Tcr7 = crate::Reg<tcr7::TCR7rs>;
///
pub mod tcr7;
///TAR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar7`]
///module
#[doc(alias = "TAR7")]
pub type Tar7 = crate::Reg<tar7::TAR7rs>;
///
pub mod tar7;
///TDR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr7`]
///module
#[doc(alias = "TDR7")]
pub type Tdr7 = crate::Reg<tdr7::TDR7rs>;
///
pub mod tdr7;
///RCR7 (rw) register accessor: task 7 repetition counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr7`]
///module
#[doc(alias = "RCR7")]
pub type Rcr7 = crate::Reg<rcr7::RCR7rs>;
///task 7 repetition counter register
pub mod rcr7;
///TCR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tcr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tcr8`]
///module
#[doc(alias = "TCR8")]
pub type Tcr8 = crate::Reg<tcr8::TCR8rs>;
///
pub mod tcr8;
///TAR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tar8`]
///module
#[doc(alias = "TAR8")]
pub type Tar8 = crate::Reg<tar8::TAR8rs>;
///
pub mod tar8;
///TDR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tdr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr8`]
///module
#[doc(alias = "TDR8")]
pub type Tdr8 = crate::Reg<tdr8::TDR8rs>;
///
pub mod tdr8;
///RCR8 (rw) register accessor: task 8 repetition counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rcr8`]
///module
#[doc(alias = "RCR8")]
pub type Rcr8 = crate::Reg<rcr8::RCR8rs>;
///task 8 repetition counter register
pub mod rcr8;
///MEM1 (rw) register accessor: temporary memory 1
///
///You can [`read`](crate::Reg::read) this register and get [`mem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mem1`]
///module
#[doc(alias = "MEM1")]
pub type Mem1 = crate::Reg<mem1::MEM1rs>;
///temporary memory 1
pub mod mem1;
///MEM2 (rw) register accessor: temporary memory 2
///
///You can [`read`](crate::Reg::read) this register and get [`mem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mem2`]
///module
#[doc(alias = "MEM2")]
pub type Mem2 = crate::Reg<mem2::MEM2rs>;
///temporary memory 2
pub mod mem2;
///MEM3 (rw) register accessor: temporary memory 3
///
///You can [`read`](crate::Reg::read) this register and get [`mem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mem3`]
///module
#[doc(alias = "MEM3")]
pub type Mem3 = crate::Reg<mem3::MEM3rs>;
///temporary memory 3
pub mod mem3;
///MEM4 (rw) register accessor: temporary memory 4
///
///You can [`read`](crate::Reg::read) this register and get [`mem4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mem4`]
///module
#[doc(alias = "MEM4")]
pub type Mem4 = crate::Reg<mem4::MEM4rs>;
///temporary memory 4
pub mod mem4;
///GPIO31_0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`gpio31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gpio31_0`]
///module
#[doc(alias = "GPIO31_0")]
pub type Gpio31_0 = crate::Reg<gpio31_0::GPIO31_0rs>;
///
pub mod gpio31_0;
///GPIO63_32 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`gpio63_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio63_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gpio63_32`]
///module
#[doc(alias = "GPIO63_32")]
pub type Gpio63_32 = crate::Reg<gpio63_32::GPIO63_32rs>;
///
pub mod gpio63_32;
///GPIO95_64 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`gpio95_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio95_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gpio95_64`]
///module
#[doc(alias = "GPIO95_64")]
pub type Gpio95_64 = crate::Reg<gpio95_64::GPIO95_64rs>;
///
pub mod gpio95_64;
