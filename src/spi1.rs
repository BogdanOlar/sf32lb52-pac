#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    top_ctrl: TopCtrl,
    fifo_ctrl: FifoCtrl,
    inte: Inte,
    to: To,
    data: Data,
    status: Status,
    _reserved6: [u8; 0x0c],
    rwot_ctrl: RwotCtrl,
    rwot_ccm: RwotCcm,
    rwot_cvwrn: RwotCvwrn,
    _reserved9: [u8; 0x0c],
    clk_ctrl: ClkCtrl,
    _reserved10: [u8; 0x14],
    triwire_ctrl: TriwireCtrl,
}
impl RegisterBlock {
    ///0x00 - Top Control Register
    #[inline(always)]
    pub const fn top_ctrl(&self) -> &TopCtrl {
        &self.top_ctrl
    }
    ///0x04 - FIFO Control Register
    #[inline(always)]
    pub const fn fifo_ctrl(&self) -> &FifoCtrl {
        &self.fifo_ctrl
    }
    ///0x08 - Interrupt Enable Register
    #[inline(always)]
    pub const fn inte(&self) -> &Inte {
        &self.inte
    }
    ///0x0c - SPI Time Out Register
    #[inline(always)]
    pub const fn to(&self) -> &To {
        &self.to
    }
    ///0x10 - SPI DATA Register
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    ///0x14 - Status Register
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x24 - RWOT Control Register
    #[inline(always)]
    pub const fn rwot_ctrl(&self) -> &RwotCtrl {
        &self.rwot_ctrl
    }
    ///0x28 - RWOT Counter Cycles Match Register
    #[inline(always)]
    pub const fn rwot_ccm(&self) -> &RwotCcm {
        &self.rwot_ccm
    }
    ///0x2c - RWOT Counter Value Write for Red Request Register
    #[inline(always)]
    pub const fn rwot_cvwrn(&self) -> &RwotCvwrn {
        &self.rwot_cvwrn
    }
    ///0x3c - CLK Control Register
    #[inline(always)]
    pub const fn clk_ctrl(&self) -> &ClkCtrl {
        &self.clk_ctrl
    }
    ///0x54 - Three Wire Mode Control Register
    #[inline(always)]
    pub const fn triwire_ctrl(&self) -> &TriwireCtrl {
        &self.triwire_ctrl
    }
}
///TOP_CTRL (rw) register accessor: Top Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`top_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@top_ctrl`]
///module
#[doc(alias = "TOP_CTRL")]
pub type TopCtrl = crate::Reg<top_ctrl::TOP_CTRLrs>;
///Top Control Register
pub mod top_ctrl;
///FIFO_CTRL (rw) register accessor: FIFO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifo_ctrl`]
///module
#[doc(alias = "FIFO_CTRL")]
pub type FifoCtrl = crate::Reg<fifo_ctrl::FIFO_CTRLrs>;
///FIFO Control Register
pub mod fifo_ctrl;
///INTE (rw) register accessor: Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@inte`]
///module
#[doc(alias = "INTE")]
pub type Inte = crate::Reg<inte::INTErs>;
///Interrupt Enable Register
pub mod inte;
///TO (rw) register accessor: SPI Time Out Register
///
///You can [`read`](crate::Reg::read) this register and get [`to::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@to`]
///module
#[doc(alias = "TO")]
pub type To = crate::Reg<to::TOrs>;
///SPI Time Out Register
pub mod to;
///DATA (rw) register accessor: SPI DATA Register
///
///You can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@data`]
///module
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DATArs>;
///SPI DATA Register
pub mod data;
///STATUS (rw) register accessor: Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@status`]
///module
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
///Status Register
pub mod status;
///RWOT_CTRL (rw) register accessor: RWOT Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`rwot_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwot_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rwot_ctrl`]
///module
#[doc(alias = "RWOT_CTRL")]
pub type RwotCtrl = crate::Reg<rwot_ctrl::RWOT_CTRLrs>;
///RWOT Control Register
pub mod rwot_ctrl;
///RWOT_CCM (rw) register accessor: RWOT Counter Cycles Match Register
///
///You can [`read`](crate::Reg::read) this register and get [`rwot_ccm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwot_ccm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rwot_ccm`]
///module
#[doc(alias = "RWOT_CCM")]
pub type RwotCcm = crate::Reg<rwot_ccm::RWOT_CCMrs>;
///RWOT Counter Cycles Match Register
pub mod rwot_ccm;
///RWOT_CVWRN (rw) register accessor: RWOT Counter Value Write for Red Request Register
///
///You can [`read`](crate::Reg::read) this register and get [`rwot_cvwrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwot_cvwrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rwot_cvwrn`]
///module
#[doc(alias = "RWOT_CVWRN")]
pub type RwotCvwrn = crate::Reg<rwot_cvwrn::RWOT_CVWRNrs>;
///RWOT Counter Value Write for Red Request Register
pub mod rwot_cvwrn;
///CLK_CTRL (rw) register accessor: CLK Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@clk_ctrl`]
///module
#[doc(alias = "CLK_CTRL")]
pub type ClkCtrl = crate::Reg<clk_ctrl::CLK_CTRLrs>;
///CLK Control Register
pub mod clk_ctrl;
///TRIWIRE_CTRL (rw) register accessor: Three Wire Mode Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`triwire_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`triwire_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@triwire_ctrl`]
///module
#[doc(alias = "TRIWIRE_CTRL")]
pub type TriwireCtrl = crate::Reg<triwire_ctrl::TRIWIRE_CTRLrs>;
///Three Wire Mode Control Register
pub mod triwire_ctrl;
