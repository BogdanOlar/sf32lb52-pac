#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tsen_ctrl_reg: TsenCtrlReg,
    tsen_rdata: TsenRdata,
    tsen_irq: TsenIrq,
}
impl RegisterBlock {
    ///0x00 - TSEN Analog Control Register
    #[inline(always)]
    pub const fn tsen_ctrl_reg(&self) -> &TsenCtrlReg {
        &self.tsen_ctrl_reg
    }
    ///0x04 - Tsen Read Data
    #[inline(always)]
    pub const fn tsen_rdata(&self) -> &TsenRdata {
        &self.tsen_rdata
    }
    ///0x08 - Tsen IRQ Register
    #[inline(always)]
    pub const fn tsen_irq(&self) -> &TsenIrq {
        &self.tsen_irq
    }
}
///TSEN_CTRL_REG (rw) register accessor: TSEN Analog Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`tsen_ctrl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen_ctrl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tsen_ctrl_reg`]
///module
#[doc(alias = "TSEN_CTRL_REG")]
pub type TsenCtrlReg = crate::Reg<tsen_ctrl_reg::TSEN_CTRL_REGrs>;
///TSEN Analog Control Register
pub mod tsen_ctrl_reg;
///TSEN_RDATA (rw) register accessor: Tsen Read Data
///
///You can [`read`](crate::Reg::read) this register and get [`tsen_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tsen_rdata`]
///module
#[doc(alias = "TSEN_RDATA")]
pub type TsenRdata = crate::Reg<tsen_rdata::TSEN_RDATArs>;
///Tsen Read Data
pub mod tsen_rdata;
///TSEN_IRQ (rw) register accessor: Tsen IRQ Register
///
///You can [`read`](crate::Reg::read) this register and get [`tsen_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tsen_irq`]
///module
#[doc(alias = "TSEN_IRQ")]
pub type TsenIrq = crate::Reg<tsen_irq::TSEN_IRQrs>;
///Tsen IRQ Register
pub mod tsen_irq;
