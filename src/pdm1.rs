#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfg0: Cfg0,
    cfg1: Cfg1,
    sinc_cfg: SincCfg,
    _reserved3: [u8; 0x08],
    hpf_cfg: HpfCfg,
    pga_cfg: PgaCfg,
    _reserved5: [u8; 0x18],
    lpf_cfg6: LpfCfg6,
    fifo_cfg: FifoCfg,
    _reserved7: [u8; 0x08],
    fifo_st: FifoSt,
    int_st: IntSt,
    int_msk: IntMsk,
    int_clr: IntClr,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    ///0x04 -
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    ///0x08 -
    #[inline(always)]
    pub const fn sinc_cfg(&self) -> &SincCfg {
        &self.sinc_cfg
    }
    ///0x14 -
    #[inline(always)]
    pub const fn hpf_cfg(&self) -> &HpfCfg {
        &self.hpf_cfg
    }
    ///0x18 -
    #[inline(always)]
    pub const fn pga_cfg(&self) -> &PgaCfg {
        &self.pga_cfg
    }
    ///0x34 -
    #[inline(always)]
    pub const fn lpf_cfg6(&self) -> &LpfCfg6 {
        &self.lpf_cfg6
    }
    ///0x38 -
    #[inline(always)]
    pub const fn fifo_cfg(&self) -> &FifoCfg {
        &self.fifo_cfg
    }
    ///0x44 -
    #[inline(always)]
    pub const fn fifo_st(&self) -> &FifoSt {
        &self.fifo_st
    }
    ///0x48 -
    #[inline(always)]
    pub const fn int_st(&self) -> &IntSt {
        &self.int_st
    }
    ///0x4c -
    #[inline(always)]
    pub const fn int_msk(&self) -> &IntMsk {
        &self.int_msk
    }
    ///0x50 -
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
}
///CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfg0`]
///module
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::CFG0rs>;
///
pub mod cfg0;
///CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfg1`]
///module
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::CFG1rs>;
///
pub mod cfg1;
///SINC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`sinc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sinc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sinc_cfg`]
///module
#[doc(alias = "SINC_CFG")]
pub type SincCfg = crate::Reg<sinc_cfg::SINC_CFGrs>;
///
pub mod sinc_cfg;
///HPF_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hpf_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpf_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hpf_cfg`]
///module
#[doc(alias = "HPF_CFG")]
pub type HpfCfg = crate::Reg<hpf_cfg::HPF_CFGrs>;
///
pub mod hpf_cfg;
///PGA_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pga_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pga_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pga_cfg`]
///module
#[doc(alias = "PGA_CFG")]
pub type PgaCfg = crate::Reg<pga_cfg::PGA_CFGrs>;
///
pub mod pga_cfg;
///LPF_CFG6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lpf_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpf_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lpf_cfg6`]
///module
#[doc(alias = "LPF_CFG6")]
pub type LpfCfg6 = crate::Reg<lpf_cfg6::LPF_CFG6rs>;
///
pub mod lpf_cfg6;
///FIFO_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifo_cfg`]
///module
#[doc(alias = "FIFO_CFG")]
pub type FifoCfg = crate::Reg<fifo_cfg::FIFO_CFGrs>;
///
pub mod fifo_cfg;
///FIFO_ST (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifo_st`]
///module
#[doc(alias = "FIFO_ST")]
pub type FifoSt = crate::Reg<fifo_st::FIFO_STrs>;
///
pub mod fifo_st;
///INT_ST (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`int_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_st`]
///module
#[doc(alias = "INT_ST")]
pub type IntSt = crate::Reg<int_st::INT_STrs>;
///
pub mod int_st;
///INT_MSK (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`int_msk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_msk`]
///module
#[doc(alias = "INT_MSK")]
pub type IntMsk = crate::Reg<int_msk::INT_MSKrs>;
///
pub mod int_msk;
///INT_CLR (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_clr`]
///module
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::INT_CLRrs>;
///
pub mod int_clr;
