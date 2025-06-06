#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rstr1: Rstr1,
    rstr2: Rstr2,
    enr1: Enr1,
    enr2: Enr2,
    esr1: Esr1,
    esr2: Esr2,
    ecr1: Ecr1,
    ecr2: Ecr2,
    csr: Csr,
    cfgr: Cfgr,
    usbcr: Usbcr,
    dll1cr: Dll1cr,
    dll2cr: Dll2cr,
    hrccal1: Hrccal1,
    hrccal2: Hrccal2,
    dbgclkr: Dbgclkr,
    dbgr: Dbgr,
    dwcfgr: Dwcfgr,
}
impl RegisterBlock {
    ///0x00 - Reset Register 1
    #[inline(always)]
    pub const fn rstr1(&self) -> &Rstr1 {
        &self.rstr1
    }
    ///0x04 - Reset Register 2
    #[inline(always)]
    pub const fn rstr2(&self) -> &Rstr2 {
        &self.rstr2
    }
    ///0x08 - Enable Register 1
    #[inline(always)]
    pub const fn enr1(&self) -> &Enr1 {
        &self.enr1
    }
    ///0x0c - Enable Register 2
    #[inline(always)]
    pub const fn enr2(&self) -> &Enr2 {
        &self.enr2
    }
    ///0x10 - Enable Set Register 1
    #[inline(always)]
    pub const fn esr1(&self) -> &Esr1 {
        &self.esr1
    }
    ///0x14 - Enable Set Register 2
    #[inline(always)]
    pub const fn esr2(&self) -> &Esr2 {
        &self.esr2
    }
    ///0x18 - Enable Clear Register 1
    #[inline(always)]
    pub const fn ecr1(&self) -> &Ecr1 {
        &self.ecr1
    }
    ///0x1c - Enable Clear Register 2
    #[inline(always)]
    pub const fn ecr2(&self) -> &Ecr2 {
        &self.ecr2
    }
    ///0x20 - Clock Select Register
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    ///0x24 - Clock Configuration Register
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    ///0x28 - USBC Control Register
    #[inline(always)]
    pub const fn usbcr(&self) -> &Usbcr {
        &self.usbcr
    }
    ///0x2c - DLL1 Control Register
    #[inline(always)]
    pub const fn dll1cr(&self) -> &Dll1cr {
        &self.dll1cr
    }
    ///0x30 - DLL2 Control Register
    #[inline(always)]
    pub const fn dll2cr(&self) -> &Dll2cr {
        &self.dll2cr
    }
    ///0x34 - HRC Calibration Register 1
    #[inline(always)]
    pub const fn hrccal1(&self) -> &Hrccal1 {
        &self.hrccal1
    }
    ///0x38 - HRC Calibration Register 2
    #[inline(always)]
    pub const fn hrccal2(&self) -> &Hrccal2 {
        &self.hrccal2
    }
    ///0x3c - Debug Clock Register
    #[inline(always)]
    pub const fn dbgclkr(&self) -> &Dbgclkr {
        &self.dbgclkr
    }
    ///0x40 - Debug Register
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
    ///0x44 - Deep WFI mode Clock Configuration Register
    #[inline(always)]
    pub const fn dwcfgr(&self) -> &Dwcfgr {
        &self.dwcfgr
    }
}
///RSTR1 (rw) register accessor: Reset Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rstr1`]
///module
#[doc(alias = "RSTR1")]
pub type Rstr1 = crate::Reg<rstr1::RSTR1rs>;
///Reset Register 1
pub mod rstr1;
///RSTR2 (rw) register accessor: Reset Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rstr2`]
///module
#[doc(alias = "RSTR2")]
pub type Rstr2 = crate::Reg<rstr2::RSTR2rs>;
///Reset Register 2
pub mod rstr2;
///ENR1 (rw) register accessor: Enable Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@enr1`]
///module
#[doc(alias = "ENR1")]
pub type Enr1 = crate::Reg<enr1::ENR1rs>;
///Enable Register 1
pub mod enr1;
///ENR2 (rw) register accessor: Enable Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@enr2`]
///module
#[doc(alias = "ENR2")]
pub type Enr2 = crate::Reg<enr2::ENR2rs>;
///Enable Register 2
pub mod enr2;
///ESR1 (rw) register accessor: Enable Set Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`esr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@esr1`]
///module
#[doc(alias = "ESR1")]
pub type Esr1 = crate::Reg<esr1::ESR1rs>;
///Enable Set Register 1
pub mod esr1;
///ESR2 (rw) register accessor: Enable Set Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`esr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@esr2`]
///module
#[doc(alias = "ESR2")]
pub type Esr2 = crate::Reg<esr2::ESR2rs>;
///Enable Set Register 2
pub mod esr2;
///ECR1 (rw) register accessor: Enable Clear Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`ecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ecr1`]
///module
#[doc(alias = "ECR1")]
pub type Ecr1 = crate::Reg<ecr1::ECR1rs>;
///Enable Clear Register 1
pub mod ecr1;
///ECR2 (rw) register accessor: Enable Clear Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`ecr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ecr2`]
///module
#[doc(alias = "ECR2")]
pub type Ecr2 = crate::Reg<ecr2::ECR2rs>;
///Enable Clear Register 2
pub mod ecr2;
///CSR (rw) register accessor: Clock Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@csr`]
///module
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CSRrs>;
///Clock Select Register
pub mod csr;
///CFGR (rw) register accessor: Clock Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfgr`]
///module
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CFGRrs>;
///Clock Configuration Register
pub mod cfgr;
///USBCR (rw) register accessor: USBC Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`usbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@usbcr`]
///module
#[doc(alias = "USBCR")]
pub type Usbcr = crate::Reg<usbcr::USBCRrs>;
///USBC Control Register
pub mod usbcr;
///DLL1CR (rw) register accessor: DLL1 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dll1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dll1cr`]
///module
#[doc(alias = "DLL1CR")]
pub type Dll1cr = crate::Reg<dll1cr::DLL1CRrs>;
///DLL1 Control Register
pub mod dll1cr;
///DLL2CR (rw) register accessor: DLL2 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dll2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dll2cr`]
///module
#[doc(alias = "DLL2CR")]
pub type Dll2cr = crate::Reg<dll2cr::DLL2CRrs>;
///DLL2 Control Register
pub mod dll2cr;
///HRCCAL1 (rw) register accessor: HRC Calibration Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`hrccal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hrccal1`]
///module
#[doc(alias = "HRCCAL1")]
pub type Hrccal1 = crate::Reg<hrccal1::HRCCAL1rs>;
///HRC Calibration Register 1
pub mod hrccal1;
///HRCCAL2 (rw) register accessor: HRC Calibration Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`hrccal2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccal2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hrccal2`]
///module
#[doc(alias = "HRCCAL2")]
pub type Hrccal2 = crate::Reg<hrccal2::HRCCAL2rs>;
///HRC Calibration Register 2
pub mod hrccal2;
///DBGCLKR (rw) register accessor: Debug Clock Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgclkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgclkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbgclkr`]
///module
#[doc(alias = "DBGCLKR")]
pub type Dbgclkr = crate::Reg<dbgclkr::DBGCLKRrs>;
///Debug Clock Register
pub mod dbgclkr;
///DBGR (rw) register accessor: Debug Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbgr`]
///module
#[doc(alias = "DBGR")]
pub type Dbgr = crate::Reg<dbgr::DBGRrs>;
///Debug Register
pub mod dbgr;
///DWCFGR (rw) register accessor: Deep WFI mode Clock Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dwcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dwcfgr`]
///module
#[doc(alias = "DWCFGR")]
pub type Dwcfgr = crate::Reg<dwcfgr::DWCFGRrs>;
///Deep WFI mode Clock Configuration Register
pub mod dwcfgr;
