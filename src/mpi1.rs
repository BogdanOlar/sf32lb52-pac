#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: Cr,
    dr: Dr,
    dcr: Dcr,
    psclr: Psclr,
    sr: Sr,
    scr: Scr,
    cmdr1: Cmdr1,
    ar1: Ar1,
    abr1: Abr1,
    dlr1: Dlr1,
    ccr1: Ccr1,
    cmdr2: Cmdr2,
    ar2: Ar2,
    abr2: Abr2,
    dlr2: Dlr2,
    ccr2: Ccr2,
    hcmdr: Hcmdr,
    hrabr: Hrabr,
    hrccr: Hrccr,
    hwabr: Hwabr,
    hwccr: Hwccr,
    fifocr: Fifocr,
    miscr: Miscr,
    ctrsar: Ctrsar,
    ctrear: Ctrear,
    noncea: Noncea,
    nonceb: Nonceb,
    aasar: Aasar,
    aaear: Aaear,
    aaoar: Aaoar,
    cir: Cir,
    smr: Smr,
    smkr: Smkr,
    timr: Timr,
    wdtr: Wdtr,
    prsar: Prsar,
    prear: Prear,
    calcr: Calcr,
    _reserved38: [u8; 0x04],
    apm32cr: Apm32cr,
    cr2: Cr2,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x04 - Data Register
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    ///0x08 - Device Control Register
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    ///0x0c - Prescaler Register
    #[inline(always)]
    pub const fn psclr(&self) -> &Psclr {
        &self.psclr
    }
    ///0x10 - Status Register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x14 - Status Clear Register
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    ///0x18 - Command Register
    #[inline(always)]
    pub const fn cmdr1(&self) -> &Cmdr1 {
        &self.cmdr1
    }
    ///0x1c - Address Register
    #[inline(always)]
    pub const fn ar1(&self) -> &Ar1 {
        &self.ar1
    }
    ///0x20 - Alternate Byte Register
    #[inline(always)]
    pub const fn abr1(&self) -> &Abr1 {
        &self.abr1
    }
    ///0x24 - Data Length Register
    #[inline(always)]
    pub const fn dlr1(&self) -> &Dlr1 {
        &self.dlr1
    }
    ///0x28 - Communication Configuration Register
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    ///0x2c - Command Register
    #[inline(always)]
    pub const fn cmdr2(&self) -> &Cmdr2 {
        &self.cmdr2
    }
    ///0x30 - Address Register
    #[inline(always)]
    pub const fn ar2(&self) -> &Ar2 {
        &self.ar2
    }
    ///0x34 - Alternate Byte Register
    #[inline(always)]
    pub const fn abr2(&self) -> &Abr2 {
        &self.abr2
    }
    ///0x38 - Data Length Register
    #[inline(always)]
    pub const fn dlr2(&self) -> &Dlr2 {
        &self.dlr2
    }
    ///0x3c - Communication Configuration Register
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    ///0x40 - AHB Command Register
    #[inline(always)]
    pub const fn hcmdr(&self) -> &Hcmdr {
        &self.hcmdr
    }
    ///0x44 - AHB Read Alternate Byte Register
    #[inline(always)]
    pub const fn hrabr(&self) -> &Hrabr {
        &self.hrabr
    }
    ///0x48 - AHB Read Communication Configuration Register
    #[inline(always)]
    pub const fn hrccr(&self) -> &Hrccr {
        &self.hrccr
    }
    ///0x4c - AHB Write Alternate Byte Register
    #[inline(always)]
    pub const fn hwabr(&self) -> &Hwabr {
        &self.hwabr
    }
    ///0x50 - AHB Write Communication Configuration Register
    #[inline(always)]
    pub const fn hwccr(&self) -> &Hwccr {
        &self.hwccr
    }
    ///0x54 - FIFO Control Register
    #[inline(always)]
    pub const fn fifocr(&self) -> &Fifocr {
        &self.fifocr
    }
    ///0x58 - Miscelaneous Register
    #[inline(always)]
    pub const fn miscr(&self) -> &Miscr {
        &self.miscr
    }
    ///0x5c - CTR Starting Address Register
    #[inline(always)]
    pub const fn ctrsar(&self) -> &Ctrsar {
        &self.ctrsar
    }
    ///0x60 - CTR Ending Address Register
    #[inline(always)]
    pub const fn ctrear(&self) -> &Ctrear {
        &self.ctrear
    }
    ///0x64 - Nonce A Register
    #[inline(always)]
    pub const fn noncea(&self) -> &Noncea {
        &self.noncea
    }
    ///0x68 - Nonce B Register
    #[inline(always)]
    pub const fn nonceb(&self) -> &Nonceb {
        &self.nonceb
    }
    ///0x6c - Address Aliasing Start Address Register
    #[inline(always)]
    pub const fn aasar(&self) -> &Aasar {
        &self.aasar
    }
    ///0x70 - Address Aliasing Ending Address Register
    #[inline(always)]
    pub const fn aaear(&self) -> &Aaear {
        &self.aaear
    }
    ///0x74 - Address Aliasing Offset Address Register
    #[inline(always)]
    pub const fn aaoar(&self) -> &Aaoar {
        &self.aaoar
    }
    ///0x78 - Command Interval Register
    #[inline(always)]
    pub const fn cir(&self) -> &Cir {
        &self.cir
    }
    ///0x7c - Status Match Register
    #[inline(always)]
    pub const fn smr(&self) -> &Smr {
        &self.smr
    }
    ///0x80 - Status Mask Register
    #[inline(always)]
    pub const fn smkr(&self) -> &Smkr {
        &self.smkr
    }
    ///0x84 - Timer Register
    #[inline(always)]
    pub const fn timr(&self) -> &Timr {
        &self.timr
    }
    ///0x88 - WDT Register
    #[inline(always)]
    pub const fn wdtr(&self) -> &Wdtr {
        &self.wdtr
    }
    ///0x8c - Prefetch Starting Address Register
    #[inline(always)]
    pub const fn prsar(&self) -> &Prsar {
        &self.prsar
    }
    ///0x90 - Prefetch Ending Address Register
    #[inline(always)]
    pub const fn prear(&self) -> &Prear {
        &self.prear
    }
    ///0x94 - Calibration Clock Register
    #[inline(always)]
    pub const fn calcr(&self) -> &Calcr {
        &self.calcr
    }
    ///0x9c - APM32 Control Register
    #[inline(always)]
    pub const fn apm32cr(&self) -> &Apm32cr {
        &self.apm32cr
    }
    ///0xa0 - Control Register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
}
///CR (rw) register accessor: Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr`]
///module
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CRrs>;
///Control Register
pub mod cr;
///DR (rw) register accessor: Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dr`]
///module
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DRrs>;
///Data Register
pub mod dr;
///DCR (rw) register accessor: Device Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dcr`]
///module
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DCRrs>;
///Device Control Register
pub mod dcr;
///PSCLR (rw) register accessor: Prescaler Register
///
///You can [`read`](crate::Reg::read) this register and get [`psclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@psclr`]
///module
#[doc(alias = "PSCLR")]
pub type Psclr = crate::Reg<psclr::PSCLRrs>;
///Prescaler Register
pub mod psclr;
///SR (rw) register accessor: Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sr`]
///module
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SRrs>;
///Status Register
pub mod sr;
///SCR (rw) register accessor: Status Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@scr`]
///module
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::SCRrs>;
///Status Clear Register
pub mod scr;
///CMDR1 (rw) register accessor: Command Register
///
///You can [`read`](crate::Reg::read) this register and get [`cmdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmdr1`]
///module
#[doc(alias = "CMDR1")]
pub type Cmdr1 = crate::Reg<cmdr1::CMDR1rs>;
///Command Register
pub mod cmdr1;
///AR1 (rw) register accessor: Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ar1`]
///module
#[doc(alias = "AR1")]
pub type Ar1 = crate::Reg<ar1::AR1rs>;
///Address Register
pub mod ar1;
///ABR1 (rw) register accessor: Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`abr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@abr1`]
///module
#[doc(alias = "ABR1")]
pub type Abr1 = crate::Reg<abr1::ABR1rs>;
///Alternate Byte Register
pub mod abr1;
///DLR1 (rw) register accessor: Data Length Register
///
///You can [`read`](crate::Reg::read) this register and get [`dlr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dlr1`]
///module
#[doc(alias = "DLR1")]
pub type Dlr1 = crate::Reg<dlr1::DLR1rs>;
///Data Length Register
pub mod dlr1;
///CCR1 (rw) register accessor: Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr1`]
///module
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::CCR1rs>;
///Communication Configuration Register
pub mod ccr1;
///CMDR2 (rw) register accessor: Command Register
///
///You can [`read`](crate::Reg::read) this register and get [`cmdr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cmdr2`]
///module
#[doc(alias = "CMDR2")]
pub type Cmdr2 = crate::Reg<cmdr2::CMDR2rs>;
///Command Register
pub mod cmdr2;
///AR2 (rw) register accessor: Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ar2`]
///module
#[doc(alias = "AR2")]
pub type Ar2 = crate::Reg<ar2::AR2rs>;
///Address Register
pub mod ar2;
///ABR2 (rw) register accessor: Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`abr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@abr2`]
///module
#[doc(alias = "ABR2")]
pub type Abr2 = crate::Reg<abr2::ABR2rs>;
///Alternate Byte Register
pub mod abr2;
///DLR2 (rw) register accessor: Data Length Register
///
///You can [`read`](crate::Reg::read) this register and get [`dlr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dlr2`]
///module
#[doc(alias = "DLR2")]
pub type Dlr2 = crate::Reg<dlr2::DLR2rs>;
///Data Length Register
pub mod dlr2;
///CCR2 (rw) register accessor: Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr2`]
///module
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::CCR2rs>;
///Communication Configuration Register
pub mod ccr2;
///HCMDR (rw) register accessor: AHB Command Register
///
///You can [`read`](crate::Reg::read) this register and get [`hcmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hcmdr`]
///module
#[doc(alias = "HCMDR")]
pub type Hcmdr = crate::Reg<hcmdr::HCMDRrs>;
///AHB Command Register
pub mod hcmdr;
///HRABR (rw) register accessor: AHB Read Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`hrabr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrabr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hrabr`]
///module
#[doc(alias = "HRABR")]
pub type Hrabr = crate::Reg<hrabr::HRABRrs>;
///AHB Read Alternate Byte Register
pub mod hrabr;
///HRCCR (rw) register accessor: AHB Read Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`hrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hrccr`]
///module
#[doc(alias = "HRCCR")]
pub type Hrccr = crate::Reg<hrccr::HRCCRrs>;
///AHB Read Communication Configuration Register
pub mod hrccr;
///HWABR (rw) register accessor: AHB Write Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`hwabr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwabr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hwabr`]
///module
#[doc(alias = "HWABR")]
pub type Hwabr = crate::Reg<hwabr::HWABRrs>;
///AHB Write Alternate Byte Register
pub mod hwabr;
///HWCCR (rw) register accessor: AHB Write Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`hwccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hwccr`]
///module
#[doc(alias = "HWCCR")]
pub type Hwccr = crate::Reg<hwccr::HWCCRrs>;
///AHB Write Communication Configuration Register
pub mod hwccr;
///FIFOCR (rw) register accessor: FIFO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`fifocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifocr`]
///module
#[doc(alias = "FIFOCR")]
pub type Fifocr = crate::Reg<fifocr::FIFOCRrs>;
///FIFO Control Register
pub mod fifocr;
///MISCR (rw) register accessor: Miscelaneous Register
///
///You can [`read`](crate::Reg::read) this register and get [`miscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@miscr`]
///module
#[doc(alias = "MISCR")]
pub type Miscr = crate::Reg<miscr::MISCRrs>;
///Miscelaneous Register
pub mod miscr;
///CTRSAR (rw) register accessor: CTR Starting Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrsar`]
///module
#[doc(alias = "CTRSAR")]
pub type Ctrsar = crate::Reg<ctrsar::CTRSARrs>;
///CTR Starting Address Register
pub mod ctrsar;
///CTREAR (rw) register accessor: CTR Ending Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrear`]
///module
#[doc(alias = "CTREAR")]
pub type Ctrear = crate::Reg<ctrear::CTREARrs>;
///CTR Ending Address Register
pub mod ctrear;
///NONCEA (rw) register accessor: Nonce A Register
///
///You can [`read`](crate::Reg::read) this register and get [`noncea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noncea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@noncea`]
///module
#[doc(alias = "NONCEA")]
pub type Noncea = crate::Reg<noncea::NONCEArs>;
///Nonce A Register
pub mod noncea;
///NONCEB (rw) register accessor: Nonce B Register
///
///You can [`read`](crate::Reg::read) this register and get [`nonceb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonceb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@nonceb`]
///module
#[doc(alias = "NONCEB")]
pub type Nonceb = crate::Reg<nonceb::NONCEBrs>;
///Nonce B Register
pub mod nonceb;
///AASAR (rw) register accessor: Address Aliasing Start Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`aasar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aasar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aasar`]
///module
#[doc(alias = "AASAR")]
pub type Aasar = crate::Reg<aasar::AASARrs>;
///Address Aliasing Start Address Register
pub mod aasar;
///AAEAR (rw) register accessor: Address Aliasing Ending Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`aaear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aaear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aaear`]
///module
#[doc(alias = "AAEAR")]
pub type Aaear = crate::Reg<aaear::AAEARrs>;
///Address Aliasing Ending Address Register
pub mod aaear;
///AAOAR (rw) register accessor: Address Aliasing Offset Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`aaoar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aaoar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aaoar`]
///module
#[doc(alias = "AAOAR")]
pub type Aaoar = crate::Reg<aaoar::AAOARrs>;
///Address Aliasing Offset Address Register
pub mod aaoar;
///CIR (rw) register accessor: Command Interval Register
///
///You can [`read`](crate::Reg::read) this register and get [`cir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cir`]
///module
#[doc(alias = "CIR")]
pub type Cir = crate::Reg<cir::CIRrs>;
///Command Interval Register
pub mod cir;
///SMR (rw) register accessor: Status Match Register
///
///You can [`read`](crate::Reg::read) this register and get [`smr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@smr`]
///module
#[doc(alias = "SMR")]
pub type Smr = crate::Reg<smr::SMRrs>;
///Status Match Register
pub mod smr;
///SMKR (rw) register accessor: Status Mask Register
///
///You can [`read`](crate::Reg::read) this register and get [`smkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@smkr`]
///module
#[doc(alias = "SMKR")]
pub type Smkr = crate::Reg<smkr::SMKRrs>;
///Status Mask Register
pub mod smkr;
///TIMR (rw) register accessor: Timer Register
///
///You can [`read`](crate::Reg::read) this register and get [`timr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@timr`]
///module
#[doc(alias = "TIMR")]
pub type Timr = crate::Reg<timr::TIMRrs>;
///Timer Register
pub mod timr;
///WDTR (rw) register accessor: WDT Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wdtr`]
///module
#[doc(alias = "WDTR")]
pub type Wdtr = crate::Reg<wdtr::WDTRrs>;
///WDT Register
pub mod wdtr;
///PRSAR (rw) register accessor: Prefetch Starting Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`prsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@prsar`]
///module
#[doc(alias = "PRSAR")]
pub type Prsar = crate::Reg<prsar::PRSARrs>;
///Prefetch Starting Address Register
pub mod prsar;
///PREAR (rw) register accessor: Prefetch Ending Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`prear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@prear`]
///module
#[doc(alias = "PREAR")]
pub type Prear = crate::Reg<prear::PREARrs>;
///Prefetch Ending Address Register
pub mod prear;
///CALCR (rw) register accessor: Calibration Clock Register
///
///You can [`read`](crate::Reg::read) this register and get [`calcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@calcr`]
///module
#[doc(alias = "CALCR")]
pub type Calcr = crate::Reg<calcr::CALCRrs>;
///Calibration Clock Register
pub mod calcr;
///APM32CR (rw) register accessor: APM32 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`apm32cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apm32cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@apm32cr`]
///module
#[doc(alias = "APM32CR")]
pub type Apm32cr = crate::Reg<apm32cr::APM32CRrs>;
///APM32 Control Register
pub mod apm32cr;
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
