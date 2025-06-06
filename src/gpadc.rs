#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    adc_cfg_reg1: AdcCfgReg1,
    adc_slot0_reg: AdcSlot0Reg,
    adc_slot1_reg: AdcSlot1Reg,
    adc_slot2_reg: AdcSlot2Reg,
    adc_slot3_reg: AdcSlot3Reg,
    adc_slot4_reg: AdcSlot4Reg,
    adc_slot5_reg: AdcSlot5Reg,
    adc_slot6_reg: AdcSlot6Reg,
    adc_slot7_reg: AdcSlot7Reg,
    adc_rdata0: AdcRdata0,
    adc_rdata1: AdcRdata1,
    adc_rdata2: AdcRdata2,
    adc_rdata3: AdcRdata3,
    adc_dma_rdata: AdcDmaRdata,
    adc_ctrl_reg: AdcCtrlReg,
    adc_ctrl_reg2: AdcCtrlReg2,
    gpadc_status: GpadcStatus,
    gpadc_irq: GpadcIrq,
}
impl RegisterBlock {
    ///0x00 - ADC Analog Config Register 1
    #[inline(always)]
    pub const fn adc_cfg_reg1(&self) -> &AdcCfgReg1 {
        &self.adc_cfg_reg1
    }
    ///0x04 - ADC Slot0 Config Register
    #[inline(always)]
    pub const fn adc_slot0_reg(&self) -> &AdcSlot0Reg {
        &self.adc_slot0_reg
    }
    ///0x08 - ADC Slot1 Config Register
    #[inline(always)]
    pub const fn adc_slot1_reg(&self) -> &AdcSlot1Reg {
        &self.adc_slot1_reg
    }
    ///0x0c - ADC Slot2 Config Register
    #[inline(always)]
    pub const fn adc_slot2_reg(&self) -> &AdcSlot2Reg {
        &self.adc_slot2_reg
    }
    ///0x10 - ADC Slot3 Config Register
    #[inline(always)]
    pub const fn adc_slot3_reg(&self) -> &AdcSlot3Reg {
        &self.adc_slot3_reg
    }
    ///0x14 - ADC Slot4 Config Register
    #[inline(always)]
    pub const fn adc_slot4_reg(&self) -> &AdcSlot4Reg {
        &self.adc_slot4_reg
    }
    ///0x18 - ADC Slot5 Config Register
    #[inline(always)]
    pub const fn adc_slot5_reg(&self) -> &AdcSlot5Reg {
        &self.adc_slot5_reg
    }
    ///0x1c - ADC Slot6 Config Register
    #[inline(always)]
    pub const fn adc_slot6_reg(&self) -> &AdcSlot6Reg {
        &self.adc_slot6_reg
    }
    ///0x20 - ADC Slot7 Config Register
    #[inline(always)]
    pub const fn adc_slot7_reg(&self) -> &AdcSlot7Reg {
        &self.adc_slot7_reg
    }
    ///0x24 - ADC Read Data0
    #[inline(always)]
    pub const fn adc_rdata0(&self) -> &AdcRdata0 {
        &self.adc_rdata0
    }
    ///0x28 - ADC Read Data1
    #[inline(always)]
    pub const fn adc_rdata1(&self) -> &AdcRdata1 {
        &self.adc_rdata1
    }
    ///0x2c - ADC Read Data2
    #[inline(always)]
    pub const fn adc_rdata2(&self) -> &AdcRdata2 {
        &self.adc_rdata2
    }
    ///0x30 - ADC Read Data3
    #[inline(always)]
    pub const fn adc_rdata3(&self) -> &AdcRdata3 {
        &self.adc_rdata3
    }
    ///0x34 - ADC Read Data For DMA
    #[inline(always)]
    pub const fn adc_dma_rdata(&self) -> &AdcDmaRdata {
        &self.adc_dma_rdata
    }
    ///0x38 - ADC Control Register
    #[inline(always)]
    pub const fn adc_ctrl_reg(&self) -> &AdcCtrlReg {
        &self.adc_ctrl_reg
    }
    ///0x3c - ADC Control Register2
    #[inline(always)]
    pub const fn adc_ctrl_reg2(&self) -> &AdcCtrlReg2 {
        &self.adc_ctrl_reg2
    }
    ///0x40 - GPADC Status Register
    #[inline(always)]
    pub const fn gpadc_status(&self) -> &GpadcStatus {
        &self.gpadc_status
    }
    ///0x44 - GPADC IRQ Register
    #[inline(always)]
    pub const fn gpadc_irq(&self) -> &GpadcIrq {
        &self.gpadc_irq
    }
}
///ADC_CFG_REG1 (rw) register accessor: ADC Analog Config Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`adc_cfg_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_cfg_reg1`]
///module
#[doc(alias = "ADC_CFG_REG1")]
pub type AdcCfgReg1 = crate::Reg<adc_cfg_reg1::ADC_CFG_REG1rs>;
///ADC Analog Config Register 1
pub mod adc_cfg_reg1;
///ADC_SLOT0_REG (rw) register accessor: ADC Slot0 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot0_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot0_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot0_reg`]
///module
#[doc(alias = "ADC_SLOT0_REG")]
pub type AdcSlot0Reg = crate::Reg<adc_slot0_reg::ADC_SLOT0_REGrs>;
///ADC Slot0 Config Register
pub mod adc_slot0_reg;
///ADC_SLOT1_REG (rw) register accessor: ADC Slot1 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot1_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot1_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot1_reg`]
///module
#[doc(alias = "ADC_SLOT1_REG")]
pub type AdcSlot1Reg = crate::Reg<adc_slot1_reg::ADC_SLOT1_REGrs>;
///ADC Slot1 Config Register
pub mod adc_slot1_reg;
///ADC_SLOT2_REG (rw) register accessor: ADC Slot2 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot2_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot2_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot2_reg`]
///module
#[doc(alias = "ADC_SLOT2_REG")]
pub type AdcSlot2Reg = crate::Reg<adc_slot2_reg::ADC_SLOT2_REGrs>;
///ADC Slot2 Config Register
pub mod adc_slot2_reg;
///ADC_SLOT3_REG (rw) register accessor: ADC Slot3 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot3_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot3_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot3_reg`]
///module
#[doc(alias = "ADC_SLOT3_REG")]
pub type AdcSlot3Reg = crate::Reg<adc_slot3_reg::ADC_SLOT3_REGrs>;
///ADC Slot3 Config Register
pub mod adc_slot3_reg;
///ADC_SLOT4_REG (rw) register accessor: ADC Slot4 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot4_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot4_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot4_reg`]
///module
#[doc(alias = "ADC_SLOT4_REG")]
pub type AdcSlot4Reg = crate::Reg<adc_slot4_reg::ADC_SLOT4_REGrs>;
///ADC Slot4 Config Register
pub mod adc_slot4_reg;
///ADC_SLOT5_REG (rw) register accessor: ADC Slot5 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot5_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot5_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot5_reg`]
///module
#[doc(alias = "ADC_SLOT5_REG")]
pub type AdcSlot5Reg = crate::Reg<adc_slot5_reg::ADC_SLOT5_REGrs>;
///ADC Slot5 Config Register
pub mod adc_slot5_reg;
///ADC_SLOT6_REG (rw) register accessor: ADC Slot6 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot6_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot6_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot6_reg`]
///module
#[doc(alias = "ADC_SLOT6_REG")]
pub type AdcSlot6Reg = crate::Reg<adc_slot6_reg::ADC_SLOT6_REGrs>;
///ADC Slot6 Config Register
pub mod adc_slot6_reg;
///ADC_SLOT7_REG (rw) register accessor: ADC Slot7 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot7_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot7_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_slot7_reg`]
///module
#[doc(alias = "ADC_SLOT7_REG")]
pub type AdcSlot7Reg = crate::Reg<adc_slot7_reg::ADC_SLOT7_REGrs>;
///ADC Slot7 Config Register
pub mod adc_slot7_reg;
///ADC_RDATA0 (rw) register accessor: ADC Read Data0
///
///You can [`read`](crate::Reg::read) this register and get [`adc_rdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_rdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_rdata0`]
///module
#[doc(alias = "ADC_RDATA0")]
pub type AdcRdata0 = crate::Reg<adc_rdata0::ADC_RDATA0rs>;
///ADC Read Data0
pub mod adc_rdata0;
///ADC_RDATA1 (rw) register accessor: ADC Read Data1
///
///You can [`read`](crate::Reg::read) this register and get [`adc_rdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_rdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_rdata1`]
///module
#[doc(alias = "ADC_RDATA1")]
pub type AdcRdata1 = crate::Reg<adc_rdata1::ADC_RDATA1rs>;
///ADC Read Data1
pub mod adc_rdata1;
///ADC_RDATA2 (rw) register accessor: ADC Read Data2
///
///You can [`read`](crate::Reg::read) this register and get [`adc_rdata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_rdata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_rdata2`]
///module
#[doc(alias = "ADC_RDATA2")]
pub type AdcRdata2 = crate::Reg<adc_rdata2::ADC_RDATA2rs>;
///ADC Read Data2
pub mod adc_rdata2;
///ADC_RDATA3 (rw) register accessor: ADC Read Data3
///
///You can [`read`](crate::Reg::read) this register and get [`adc_rdata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_rdata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_rdata3`]
///module
#[doc(alias = "ADC_RDATA3")]
pub type AdcRdata3 = crate::Reg<adc_rdata3::ADC_RDATA3rs>;
///ADC Read Data3
pub mod adc_rdata3;
///ADC_DMA_RDATA (rw) register accessor: ADC Read Data For DMA
///
///You can [`read`](crate::Reg::read) this register and get [`adc_dma_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dma_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_dma_rdata`]
///module
#[doc(alias = "ADC_DMA_RDATA")]
pub type AdcDmaRdata = crate::Reg<adc_dma_rdata::ADC_DMA_RDATArs>;
///ADC Read Data For DMA
pub mod adc_dma_rdata;
///ADC_CTRL_REG (rw) register accessor: ADC Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ctrl_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ctrl_reg`]
///module
#[doc(alias = "ADC_CTRL_REG")]
pub type AdcCtrlReg = crate::Reg<adc_ctrl_reg::ADC_CTRL_REGrs>;
///ADC Control Register
pub mod adc_ctrl_reg;
///ADC_CTRL_REG2 (rw) register accessor: ADC Control Register2
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ctrl_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ctrl_reg2`]
///module
#[doc(alias = "ADC_CTRL_REG2")]
pub type AdcCtrlReg2 = crate::Reg<adc_ctrl_reg2::ADC_CTRL_REG2rs>;
///ADC Control Register2
pub mod adc_ctrl_reg2;
///GPADC_STATUS (rw) register accessor: GPADC Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`gpadc_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gpadc_status`]
///module
#[doc(alias = "GPADC_STATUS")]
pub type GpadcStatus = crate::Reg<gpadc_status::GPADC_STATUSrs>;
///GPADC Status Register
pub mod gpadc_status;
///GPADC_IRQ (rw) register accessor: GPADC IRQ Register
///
///You can [`read`](crate::Reg::read) this register and get [`gpadc_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@gpadc_irq`]
///module
#[doc(alias = "GPADC_IRQ")]
pub type GpadcIrq = crate::Reg<gpadc_irq::GPADC_IRQrs>;
///GPADC IRQ Register
pub mod gpadc_irq;
