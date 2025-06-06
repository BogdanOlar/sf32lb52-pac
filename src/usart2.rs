#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    brr: Brr,
    _reserved4: [u8; 0x08],
    rqr: Rqr,
    isr: Isr,
    icr: Icr,
    rdr: Rdr,
    tdr: Tdr,
    miscr: Miscr,
    drdr: Drdr,
    dtdr: Dtdr,
    exr: Exr,
}
impl RegisterBlock {
    ///0x00 - Control Register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    ///0x04 - Control Register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    ///0x08 - Control Register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    ///0x0c - Baud Rate Register
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    ///0x18 - Request Register
    #[inline(always)]
    pub const fn rqr(&self) -> &Rqr {
        &self.rqr
    }
    ///0x1c - Interrupt and Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    ///0x20 - Interrupt flag Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    ///0x24 - Receive Data Register
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    ///0x28 - Transmit Data Register
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    ///0x2c - Miscellaneous Register
    #[inline(always)]
    pub const fn miscr(&self) -> &Miscr {
        &self.miscr
    }
    ///0x30 - Debug Receive Data Register
    #[inline(always)]
    pub const fn drdr(&self) -> &Drdr {
        &self.drdr
    }
    ///0x34 - Debug Receive Data Register
    #[inline(always)]
    pub const fn dtdr(&self) -> &Dtdr {
        &self.dtdr
    }
    ///0x38 - Mutual Exclusive Register
    #[inline(always)]
    pub const fn exr(&self) -> &Exr {
        &self.exr
    }
}
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
///BRR (rw) register accessor: Baud Rate Register
///
///You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@brr`]
///module
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BRRrs>;
///Baud Rate Register
pub mod brr;
///RQR (rw) register accessor: Request Register
///
///You can [`read`](crate::Reg::read) this register and get [`rqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rqr`]
///module
#[doc(alias = "RQR")]
pub type Rqr = crate::Reg<rqr::RQRrs>;
///Request Register
pub mod rqr;
///ISR (rw) register accessor: Interrupt and Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@isr`]
///module
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::ISRrs>;
///Interrupt and Status Register
pub mod isr;
///ICR (rw) register accessor: Interrupt flag Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@icr`]
///module
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::ICRrs>;
///Interrupt flag Clear Register
pub mod icr;
///RDR (rw) register accessor: Receive Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rdr`]
///module
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RDRrs>;
///Receive Data Register
pub mod rdr;
///TDR (rw) register accessor: Transmit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tdr`]
///module
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TDRrs>;
///Transmit Data Register
pub mod tdr;
///MISCR (rw) register accessor: Miscellaneous Register
///
///You can [`read`](crate::Reg::read) this register and get [`miscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@miscr`]
///module
#[doc(alias = "MISCR")]
pub type Miscr = crate::Reg<miscr::MISCRrs>;
///Miscellaneous Register
pub mod miscr;
///DRDR (rw) register accessor: Debug Receive Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`drdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@drdr`]
///module
#[doc(alias = "DRDR")]
pub type Drdr = crate::Reg<drdr::DRDRrs>;
///Debug Receive Data Register
pub mod drdr;
///DTDR (rw) register accessor: Debug Receive Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`dtdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dtdr`]
///module
#[doc(alias = "DTDR")]
pub type Dtdr = crate::Reg<dtdr::DTDRrs>;
///Debug Receive Data Register
pub mod dtdr;
///EXR (rw) register accessor: Mutual Exclusive Register
///
///You can [`read`](crate::Reg::read) this register and get [`exr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@exr`]
///module
#[doc(alias = "EXR")]
pub type Exr = crate::Reg<exr::EXRrs>;
///Mutual Exclusive Register
pub mod exr;
