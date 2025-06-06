#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: Isr,
    ifcr: Ifcr,
    ccr: Ccr,
    cndtr: Cndtr,
    srcar: Srcar,
    dstar: Dstar,
}
impl RegisterBlock {
    ///0x00 - interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    ///0x04 - interrupt clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    ///0x08 - channel control register
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    ///0x0c - number of data register
    #[inline(always)]
    pub const fn cndtr(&self) -> &Cndtr {
        &self.cndtr
    }
    ///0x10 - source address register
    #[inline(always)]
    pub const fn srcar(&self) -> &Srcar {
        &self.srcar
    }
    ///0x14 - destination 0 address register
    #[inline(always)]
    pub const fn dstar(&self) -> &Dstar {
        &self.dstar
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
///IFCR (rw) register accessor: interrupt clear register
///
///You can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifcr`]
///module
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IFCRrs>;
///interrupt clear register
pub mod ifcr;
///CCR (rw) register accessor: channel control register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr`]
///module
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CCRrs>;
///channel control register
pub mod ccr;
///CNDTR (rw) register accessor: number of data register
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr`]
///module
#[doc(alias = "CNDTR")]
pub type Cndtr = crate::Reg<cndtr::CNDTRrs>;
///number of data register
pub mod cndtr;
///SRCAR (rw) register accessor: source address register
///
///You can [`read`](crate::Reg::read) this register and get [`srcar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@srcar`]
///module
#[doc(alias = "SRCAR")]
pub type Srcar = crate::Reg<srcar::SRCARrs>;
///source address register
pub mod srcar;
///DSTAR (rw) register accessor: destination 0 address register
///
///You can [`read`](crate::Reg::read) this register and get [`dstar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dstar`]
///module
#[doc(alias = "DSTAR")]
pub type Dstar = crate::Reg<dstar::DSTARrs>;
///destination 0 address register
pub mod dstar;
