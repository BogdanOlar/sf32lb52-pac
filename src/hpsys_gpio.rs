#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dir0: Dir0,
    dor0: Dor0,
    dosr0: Dosr0,
    docr0: Docr0,
    doer0: Doer0,
    doesr0: Doesr0,
    doecr0: Doecr0,
    ier0: Ier0,
    iesr0: Iesr0,
    iecr0: Iecr0,
    itr0: Itr0,
    itsr0: Itsr0,
    itcr0: Itcr0,
    iphr0: Iphr0,
    iphsr0: Iphsr0,
    iphcr0: Iphcr0,
    iplr0: Iplr0,
    iplsr0: Iplsr0,
    iplcr0: Iplcr0,
    isr0: Isr0,
    _reserved20: [u8; 0x10],
    oemr0: Oemr0,
    oemsr0: Oemsr0,
    oemcr0: Oemcr0,
    _reserved23: [u8; 0x14],
    dir1: Dir1,
    dor1: Dor1,
    dosr1: Dosr1,
    docr1: Docr1,
    doer1: Doer1,
    doesr1: Doesr1,
    doecr1: Doecr1,
    ier1: Ier1,
    iesr1: Iesr1,
    iecr1: Iecr1,
    itr1: Itr1,
    itsr1: Itsr1,
    itcr1: Itcr1,
    iphr1: Iphr1,
    iphsr1: Iphsr1,
    iphcr1: Iphcr1,
    iplr1: Iplr1,
    iplsr1: Iplsr1,
    iplcr1: Iplcr1,
    isr1: Isr1,
    _reserved43: [u8; 0x10],
    oemr1: Oemr1,
    oemsr1: Oemsr1,
    oemcr1: Oemcr1,
}
impl RegisterBlock {
    ///0x00 - Data Input Register
    #[inline(always)]
    pub const fn dir0(&self) -> &Dir0 {
        &self.dir0
    }
    ///0x04 - Data Output Register
    #[inline(always)]
    pub const fn dor0(&self) -> &Dor0 {
        &self.dor0
    }
    ///0x08 - Data Output Set Register
    #[inline(always)]
    pub const fn dosr0(&self) -> &Dosr0 {
        &self.dosr0
    }
    ///0x0c - Data Output Clear Register
    #[inline(always)]
    pub const fn docr0(&self) -> &Docr0 {
        &self.docr0
    }
    ///0x10 - Data Output Enable Register
    #[inline(always)]
    pub const fn doer0(&self) -> &Doer0 {
        &self.doer0
    }
    ///0x14 - Data Output Enable Set Register
    #[inline(always)]
    pub const fn doesr0(&self) -> &Doesr0 {
        &self.doesr0
    }
    ///0x18 - Data Output Enable Clear Register
    #[inline(always)]
    pub const fn doecr0(&self) -> &Doecr0 {
        &self.doecr0
    }
    ///0x1c - Interrupt Enable Register
    #[inline(always)]
    pub const fn ier0(&self) -> &Ier0 {
        &self.ier0
    }
    ///0x20 - Interrupt Enable Set Register
    #[inline(always)]
    pub const fn iesr0(&self) -> &Iesr0 {
        &self.iesr0
    }
    ///0x24 - Interrupt Enable Clear Register
    #[inline(always)]
    pub const fn iecr0(&self) -> &Iecr0 {
        &self.iecr0
    }
    ///0x28 - Interrupt Type Register
    #[inline(always)]
    pub const fn itr0(&self) -> &Itr0 {
        &self.itr0
    }
    ///0x2c - Interrupt Type Set Register
    #[inline(always)]
    pub const fn itsr0(&self) -> &Itsr0 {
        &self.itsr0
    }
    ///0x30 - Interrupt Type Clear Register
    #[inline(always)]
    pub const fn itcr0(&self) -> &Itcr0 {
        &self.itcr0
    }
    ///0x34 - Interrupt Polarity High Register
    #[inline(always)]
    pub const fn iphr0(&self) -> &Iphr0 {
        &self.iphr0
    }
    ///0x38 - Interrupt Polarity High Set Register
    #[inline(always)]
    pub const fn iphsr0(&self) -> &Iphsr0 {
        &self.iphsr0
    }
    ///0x3c - Interrupt Polarity High Clear Register
    #[inline(always)]
    pub const fn iphcr0(&self) -> &Iphcr0 {
        &self.iphcr0
    }
    ///0x40 - Interrupt Polarity Low Register
    #[inline(always)]
    pub const fn iplr0(&self) -> &Iplr0 {
        &self.iplr0
    }
    ///0x44 - Interrupt Polarity Low Set Register
    #[inline(always)]
    pub const fn iplsr0(&self) -> &Iplsr0 {
        &self.iplsr0
    }
    ///0x48 - Interrupt Polarity Low Clear Register
    #[inline(always)]
    pub const fn iplcr0(&self) -> &Iplcr0 {
        &self.iplcr0
    }
    ///0x4c - Interrupt Status Register
    #[inline(always)]
    pub const fn isr0(&self) -> &Isr0 {
        &self.isr0
    }
    ///0x60 - output mode Register
    #[inline(always)]
    pub const fn oemr0(&self) -> &Oemr0 {
        &self.oemr0
    }
    ///0x64 - output mode Set Register
    #[inline(always)]
    pub const fn oemsr0(&self) -> &Oemsr0 {
        &self.oemsr0
    }
    ///0x68 - output mode Clear Register
    #[inline(always)]
    pub const fn oemcr0(&self) -> &Oemcr0 {
        &self.oemcr0
    }
    ///0x80 - Data Input Register
    #[inline(always)]
    pub const fn dir1(&self) -> &Dir1 {
        &self.dir1
    }
    ///0x84 - Data Output Register
    #[inline(always)]
    pub const fn dor1(&self) -> &Dor1 {
        &self.dor1
    }
    ///0x88 - Data Output Set Register
    #[inline(always)]
    pub const fn dosr1(&self) -> &Dosr1 {
        &self.dosr1
    }
    ///0x8c - Data Output Clear Register
    #[inline(always)]
    pub const fn docr1(&self) -> &Docr1 {
        &self.docr1
    }
    ///0x90 - Data Output Enable Register
    #[inline(always)]
    pub const fn doer1(&self) -> &Doer1 {
        &self.doer1
    }
    ///0x94 - Data Output Enable Set Register
    #[inline(always)]
    pub const fn doesr1(&self) -> &Doesr1 {
        &self.doesr1
    }
    ///0x98 - Data Output Enable Clear Register
    #[inline(always)]
    pub const fn doecr1(&self) -> &Doecr1 {
        &self.doecr1
    }
    ///0x9c - Interrupt Enable Register
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    ///0xa0 - Interrupt Enable Set Register
    #[inline(always)]
    pub const fn iesr1(&self) -> &Iesr1 {
        &self.iesr1
    }
    ///0xa4 - Interrupt Enable Clear Register
    #[inline(always)]
    pub const fn iecr1(&self) -> &Iecr1 {
        &self.iecr1
    }
    ///0xa8 - Interrupt Type Register
    #[inline(always)]
    pub const fn itr1(&self) -> &Itr1 {
        &self.itr1
    }
    ///0xac - Interrupt Type Set Register
    #[inline(always)]
    pub const fn itsr1(&self) -> &Itsr1 {
        &self.itsr1
    }
    ///0xb0 - Interrupt Type Clear Register
    #[inline(always)]
    pub const fn itcr1(&self) -> &Itcr1 {
        &self.itcr1
    }
    ///0xb4 - Interrupt Polarity High Register
    #[inline(always)]
    pub const fn iphr1(&self) -> &Iphr1 {
        &self.iphr1
    }
    ///0xb8 - Interrupt Polarity High Set Register
    #[inline(always)]
    pub const fn iphsr1(&self) -> &Iphsr1 {
        &self.iphsr1
    }
    ///0xbc - Interrupt Polarity High Clear Register
    #[inline(always)]
    pub const fn iphcr1(&self) -> &Iphcr1 {
        &self.iphcr1
    }
    ///0xc0 - Interrupt Polarity Low Register
    #[inline(always)]
    pub const fn iplr1(&self) -> &Iplr1 {
        &self.iplr1
    }
    ///0xc4 - Interrupt Polarity Low Set Register
    #[inline(always)]
    pub const fn iplsr1(&self) -> &Iplsr1 {
        &self.iplsr1
    }
    ///0xc8 - Interrupt Polarity Low Clear Register
    #[inline(always)]
    pub const fn iplcr1(&self) -> &Iplcr1 {
        &self.iplcr1
    }
    ///0xcc - Interrupt Status Register
    #[inline(always)]
    pub const fn isr1(&self) -> &Isr1 {
        &self.isr1
    }
    ///0xe0 - output mode Register
    #[inline(always)]
    pub const fn oemr1(&self) -> &Oemr1 {
        &self.oemr1
    }
    ///0xe4 - output mode Set Register
    #[inline(always)]
    pub const fn oemsr1(&self) -> &Oemsr1 {
        &self.oemsr1
    }
    ///0xe8 - output mode Clear Register
    #[inline(always)]
    pub const fn oemcr1(&self) -> &Oemcr1 {
        &self.oemcr1
    }
}
///DIR0 (rw) register accessor: Data Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`dir0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dir0`]
///module
#[doc(alias = "DIR0")]
pub type Dir0 = crate::Reg<dir0::DIR0rs>;
///Data Input Register
pub mod dir0;
///DOR0 (rw) register accessor: Data Output Register
///
///You can [`read`](crate::Reg::read) this register and get [`dor0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dor0`]
///module
#[doc(alias = "DOR0")]
pub type Dor0 = crate::Reg<dor0::DOR0rs>;
///Data Output Register
pub mod dor0;
///DOSR0 (rw) register accessor: Data Output Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`dosr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dosr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dosr0`]
///module
#[doc(alias = "DOSR0")]
pub type Dosr0 = crate::Reg<dosr0::DOSR0rs>;
///Data Output Set Register
pub mod dosr0;
///DOCR0 (rw) register accessor: Data Output Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`docr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@docr0`]
///module
#[doc(alias = "DOCR0")]
pub type Docr0 = crate::Reg<docr0::DOCR0rs>;
///Data Output Clear Register
pub mod docr0;
///DOER0 (rw) register accessor: Data Output Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`doer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@doer0`]
///module
#[doc(alias = "DOER0")]
pub type Doer0 = crate::Reg<doer0::DOER0rs>;
///Data Output Enable Register
pub mod doer0;
///DOESR0 (rw) register accessor: Data Output Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`doesr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doesr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@doesr0`]
///module
#[doc(alias = "DOESR0")]
pub type Doesr0 = crate::Reg<doesr0::DOESR0rs>;
///Data Output Enable Set Register
pub mod doesr0;
///DOECR0 (rw) register accessor: Data Output Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`doecr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doecr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@doecr0`]
///module
#[doc(alias = "DOECR0")]
pub type Doecr0 = crate::Reg<doecr0::DOECR0rs>;
///Data Output Enable Clear Register
pub mod doecr0;
///IER0 (rw) register accessor: Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ier0`]
///module
#[doc(alias = "IER0")]
pub type Ier0 = crate::Reg<ier0::IER0rs>;
///Interrupt Enable Register
pub mod ier0;
///IESR0 (rw) register accessor: Interrupt Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iesr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iesr0`]
///module
#[doc(alias = "IESR0")]
pub type Iesr0 = crate::Reg<iesr0::IESR0rs>;
///Interrupt Enable Set Register
pub mod iesr0;
///IECR0 (rw) register accessor: Interrupt Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iecr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iecr0`]
///module
#[doc(alias = "IECR0")]
pub type Iecr0 = crate::Reg<iecr0::IECR0rs>;
///Interrupt Enable Clear Register
pub mod iecr0;
///ITR0 (rw) register accessor: Interrupt Type Register
///
///You can [`read`](crate::Reg::read) this register and get [`itr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@itr0`]
///module
#[doc(alias = "ITR0")]
pub type Itr0 = crate::Reg<itr0::ITR0rs>;
///Interrupt Type Register
pub mod itr0;
///ITSR0 (rw) register accessor: Interrupt Type Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`itsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@itsr0`]
///module
#[doc(alias = "ITSR0")]
pub type Itsr0 = crate::Reg<itsr0::ITSR0rs>;
///Interrupt Type Set Register
pub mod itsr0;
///ITCR0 (rw) register accessor: Interrupt Type Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`itcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@itcr0`]
///module
#[doc(alias = "ITCR0")]
pub type Itcr0 = crate::Reg<itcr0::ITCR0rs>;
///Interrupt Type Clear Register
pub mod itcr0;
///IPHR0 (rw) register accessor: Interrupt Polarity High Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iphr0`]
///module
#[doc(alias = "IPHR0")]
pub type Iphr0 = crate::Reg<iphr0::IPHR0rs>;
///Interrupt Polarity High Register
pub mod iphr0;
///IPHSR0 (rw) register accessor: Interrupt Polarity High Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iphsr0`]
///module
#[doc(alias = "IPHSR0")]
pub type Iphsr0 = crate::Reg<iphsr0::IPHSR0rs>;
///Interrupt Polarity High Set Register
pub mod iphsr0;
///IPHCR0 (rw) register accessor: Interrupt Polarity High Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iphcr0`]
///module
#[doc(alias = "IPHCR0")]
pub type Iphcr0 = crate::Reg<iphcr0::IPHCR0rs>;
///Interrupt Polarity High Clear Register
pub mod iphcr0;
///IPLR0 (rw) register accessor: Interrupt Polarity Low Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iplr0`]
///module
#[doc(alias = "IPLR0")]
pub type Iplr0 = crate::Reg<iplr0::IPLR0rs>;
///Interrupt Polarity Low Register
pub mod iplr0;
///IPLSR0 (rw) register accessor: Interrupt Polarity Low Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iplsr0`]
///module
#[doc(alias = "IPLSR0")]
pub type Iplsr0 = crate::Reg<iplsr0::IPLSR0rs>;
///Interrupt Polarity Low Set Register
pub mod iplsr0;
///IPLCR0 (rw) register accessor: Interrupt Polarity Low Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iplcr0`]
///module
#[doc(alias = "IPLCR0")]
pub type Iplcr0 = crate::Reg<iplcr0::IPLCR0rs>;
///Interrupt Polarity Low Clear Register
pub mod iplcr0;
///ISR0 (rw) register accessor: Interrupt Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`isr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@isr0`]
///module
#[doc(alias = "ISR0")]
pub type Isr0 = crate::Reg<isr0::ISR0rs>;
///Interrupt Status Register
pub mod isr0;
///OEMR0 (rw) register accessor: output mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@oemr0`]
///module
#[doc(alias = "OEMR0")]
pub type Oemr0 = crate::Reg<oemr0::OEMR0rs>;
///output mode Register
pub mod oemr0;
///OEMSR0 (rw) register accessor: output mode Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@oemsr0`]
///module
#[doc(alias = "OEMSR0")]
pub type Oemsr0 = crate::Reg<oemsr0::OEMSR0rs>;
///output mode Set Register
pub mod oemsr0;
///OEMCR0 (rw) register accessor: output mode Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@oemcr0`]
///module
#[doc(alias = "OEMCR0")]
pub type Oemcr0 = crate::Reg<oemcr0::OEMCR0rs>;
///output mode Clear Register
pub mod oemcr0;
///DIR1 (rw) register accessor: Data Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`dir1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dir1`]
///module
#[doc(alias = "DIR1")]
pub type Dir1 = crate::Reg<dir1::DIR1rs>;
///Data Input Register
pub mod dir1;
///DOR1 (rw) register accessor: Data Output Register
///
///You can [`read`](crate::Reg::read) this register and get [`dor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dor1`]
///module
#[doc(alias = "DOR1")]
pub type Dor1 = crate::Reg<dor1::DOR1rs>;
///Data Output Register
pub mod dor1;
///DOSR1 (rw) register accessor: Data Output Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`dosr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dosr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dosr1`]
///module
#[doc(alias = "DOSR1")]
pub type Dosr1 = crate::Reg<dosr1::DOSR1rs>;
///Data Output Set Register
pub mod dosr1;
///DOCR1 (rw) register accessor: Data Output Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`docr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@docr1`]
///module
#[doc(alias = "DOCR1")]
pub type Docr1 = crate::Reg<docr1::DOCR1rs>;
///Data Output Clear Register
pub mod docr1;
///DOER1 (rw) register accessor: Data Output Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`doer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@doer1`]
///module
#[doc(alias = "DOER1")]
pub type Doer1 = crate::Reg<doer1::DOER1rs>;
///Data Output Enable Register
pub mod doer1;
///DOESR1 (rw) register accessor: Data Output Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`doesr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doesr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@doesr1`]
///module
#[doc(alias = "DOESR1")]
pub type Doesr1 = crate::Reg<doesr1::DOESR1rs>;
///Data Output Enable Set Register
pub mod doesr1;
///DOECR1 (rw) register accessor: Data Output Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`doecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@doecr1`]
///module
#[doc(alias = "DOECR1")]
pub type Doecr1 = crate::Reg<doecr1::DOECR1rs>;
///Data Output Enable Clear Register
pub mod doecr1;
///IER1 (rw) register accessor: Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ier1`]
///module
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::IER1rs>;
///Interrupt Enable Register
pub mod ier1;
///IESR1 (rw) register accessor: Interrupt Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iesr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iesr1`]
///module
#[doc(alias = "IESR1")]
pub type Iesr1 = crate::Reg<iesr1::IESR1rs>;
///Interrupt Enable Set Register
pub mod iesr1;
///IECR1 (rw) register accessor: Interrupt Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iecr1`]
///module
#[doc(alias = "IECR1")]
pub type Iecr1 = crate::Reg<iecr1::IECR1rs>;
///Interrupt Enable Clear Register
pub mod iecr1;
///ITR1 (rw) register accessor: Interrupt Type Register
///
///You can [`read`](crate::Reg::read) this register and get [`itr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@itr1`]
///module
#[doc(alias = "ITR1")]
pub type Itr1 = crate::Reg<itr1::ITR1rs>;
///Interrupt Type Register
pub mod itr1;
///ITSR1 (rw) register accessor: Interrupt Type Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`itsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@itsr1`]
///module
#[doc(alias = "ITSR1")]
pub type Itsr1 = crate::Reg<itsr1::ITSR1rs>;
///Interrupt Type Set Register
pub mod itsr1;
///ITCR1 (rw) register accessor: Interrupt Type Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`itcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@itcr1`]
///module
#[doc(alias = "ITCR1")]
pub type Itcr1 = crate::Reg<itcr1::ITCR1rs>;
///Interrupt Type Clear Register
pub mod itcr1;
///IPHR1 (rw) register accessor: Interrupt Polarity High Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iphr1`]
///module
#[doc(alias = "IPHR1")]
pub type Iphr1 = crate::Reg<iphr1::IPHR1rs>;
///Interrupt Polarity High Register
pub mod iphr1;
///IPHSR1 (rw) register accessor: Interrupt Polarity High Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iphsr1`]
///module
#[doc(alias = "IPHSR1")]
pub type Iphsr1 = crate::Reg<iphsr1::IPHSR1rs>;
///Interrupt Polarity High Set Register
pub mod iphsr1;
///IPHCR1 (rw) register accessor: Interrupt Polarity High Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iphcr1`]
///module
#[doc(alias = "IPHCR1")]
pub type Iphcr1 = crate::Reg<iphcr1::IPHCR1rs>;
///Interrupt Polarity High Clear Register
pub mod iphcr1;
///IPLR1 (rw) register accessor: Interrupt Polarity Low Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iplr1`]
///module
#[doc(alias = "IPLR1")]
pub type Iplr1 = crate::Reg<iplr1::IPLR1rs>;
///Interrupt Polarity Low Register
pub mod iplr1;
///IPLSR1 (rw) register accessor: Interrupt Polarity Low Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iplsr1`]
///module
#[doc(alias = "IPLSR1")]
pub type Iplsr1 = crate::Reg<iplsr1::IPLSR1rs>;
///Interrupt Polarity Low Set Register
pub mod iplsr1;
///IPLCR1 (rw) register accessor: Interrupt Polarity Low Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iplcr1`]
///module
#[doc(alias = "IPLCR1")]
pub type Iplcr1 = crate::Reg<iplcr1::IPLCR1rs>;
///Interrupt Polarity Low Clear Register
pub mod iplcr1;
///ISR1 (rw) register accessor: Interrupt Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`isr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@isr1`]
///module
#[doc(alias = "ISR1")]
pub type Isr1 = crate::Reg<isr1::ISR1rs>;
///Interrupt Status Register
pub mod isr1;
///OEMR1 (rw) register accessor: output mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@oemr1`]
///module
#[doc(alias = "OEMR1")]
pub type Oemr1 = crate::Reg<oemr1::OEMR1rs>;
///output mode Register
pub mod oemr1;
///OEMSR1 (rw) register accessor: output mode Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@oemsr1`]
///module
#[doc(alias = "OEMSR1")]
pub type Oemsr1 = crate::Reg<oemsr1::OEMSR1rs>;
///output mode Set Register
pub mod oemsr1;
///OEMCR1 (rw) register accessor: output mode Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@oemcr1`]
///module
#[doc(alias = "OEMCR1")]
pub type Oemcr1 = crate::Reg<oemcr1::OEMCR1rs>;
///output mode Clear Register
pub mod oemcr1;
