#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dr: Dr,
    sr: Sr,
    cr: Cr,
    _reserved3: [u8; 0x04],
    init: Init,
    pol: Pol,
}
impl RegisterBlock {
    ///0x00 - Data register
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    ///0x04 - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x08 - Control register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x10 - Initial CRC value
    #[inline(always)]
    pub const fn init(&self) -> &Init {
        &self.init
    }
    ///0x14 - CRC polynomial
    #[inline(always)]
    pub const fn pol(&self) -> &Pol {
        &self.pol
    }
}
///DR (rw) register accessor: Data register
///
///You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dr`]
///module
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DRrs>;
///Data register
pub mod dr;
///SR (rw) register accessor: Status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sr`]
///module
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
///CR (rw) register accessor: Control register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr`]
///module
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CRrs>;
///Control register
pub mod cr;
///INIT (rw) register accessor: Initial CRC value
///
///You can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@init`]
///module
#[doc(alias = "INIT")]
pub type Init = crate::Reg<init::INITrs>;
///Initial CRC value
pub mod init;
///POL (rw) register accessor: CRC polynomial
///
///You can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pol`]
///module
#[doc(alias = "POL")]
pub type Pol = crate::Reg<pol::POLrs>;
///CRC polynomial
pub mod pol;
