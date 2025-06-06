#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: Ctrl,
    stat: Stat,
    cfg: Cfg,
    irq: Irq,
    rand_seed0: RandSeed0,
    rand_seed1: RandSeed1,
    rand_seed2: RandSeed2,
    rand_seed3: RandSeed3,
    rand_seed4: RandSeed4,
    rand_seed5: RandSeed5,
    rand_seed6: RandSeed6,
    rand_seed7: RandSeed7,
    rand_num0: RandNum0,
    rand_num1: RandNum1,
    rand_num2: RandNum2,
    rand_num3: RandNum3,
    rand_num4: RandNum4,
    rand_num5: RandNum5,
    rand_num6: RandNum6,
    rand_num7: RandNum7,
    cal_cfg: CalCfg,
    cal_result: CalResult,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    ///0x04 -
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    ///0x08 -
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    ///0x0c -
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    ///0x10 -
    #[inline(always)]
    pub const fn rand_seed0(&self) -> &RandSeed0 {
        &self.rand_seed0
    }
    ///0x14 -
    #[inline(always)]
    pub const fn rand_seed1(&self) -> &RandSeed1 {
        &self.rand_seed1
    }
    ///0x18 -
    #[inline(always)]
    pub const fn rand_seed2(&self) -> &RandSeed2 {
        &self.rand_seed2
    }
    ///0x1c -
    #[inline(always)]
    pub const fn rand_seed3(&self) -> &RandSeed3 {
        &self.rand_seed3
    }
    ///0x20 -
    #[inline(always)]
    pub const fn rand_seed4(&self) -> &RandSeed4 {
        &self.rand_seed4
    }
    ///0x24 -
    #[inline(always)]
    pub const fn rand_seed5(&self) -> &RandSeed5 {
        &self.rand_seed5
    }
    ///0x28 -
    #[inline(always)]
    pub const fn rand_seed6(&self) -> &RandSeed6 {
        &self.rand_seed6
    }
    ///0x2c -
    #[inline(always)]
    pub const fn rand_seed7(&self) -> &RandSeed7 {
        &self.rand_seed7
    }
    ///0x30 -
    #[inline(always)]
    pub const fn rand_num0(&self) -> &RandNum0 {
        &self.rand_num0
    }
    ///0x34 -
    #[inline(always)]
    pub const fn rand_num1(&self) -> &RandNum1 {
        &self.rand_num1
    }
    ///0x38 -
    #[inline(always)]
    pub const fn rand_num2(&self) -> &RandNum2 {
        &self.rand_num2
    }
    ///0x3c -
    #[inline(always)]
    pub const fn rand_num3(&self) -> &RandNum3 {
        &self.rand_num3
    }
    ///0x40 -
    #[inline(always)]
    pub const fn rand_num4(&self) -> &RandNum4 {
        &self.rand_num4
    }
    ///0x44 -
    #[inline(always)]
    pub const fn rand_num5(&self) -> &RandNum5 {
        &self.rand_num5
    }
    ///0x48 -
    #[inline(always)]
    pub const fn rand_num6(&self) -> &RandNum6 {
        &self.rand_num6
    }
    ///0x4c -
    #[inline(always)]
    pub const fn rand_num7(&self) -> &RandNum7 {
        &self.rand_num7
    }
    ///0x50 -
    #[inline(always)]
    pub const fn cal_cfg(&self) -> &CalCfg {
        &self.cal_cfg
    }
    ///0x54 -
    #[inline(always)]
    pub const fn cal_result(&self) -> &CalResult {
        &self.cal_result
    }
}
///CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ctrl`]
///module
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CTRLrs>;
///
pub mod ctrl;
///STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@stat`]
///module
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::STATrs>;
///
pub mod stat;
///CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cfg`]
///module
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CFGrs>;
///
pub mod cfg;
///IRQ (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@irq`]
///module
#[doc(alias = "IRQ")]
pub type Irq = crate::Reg<irq::IRQrs>;
///
pub mod irq;
///RAND_SEED0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed0`]
///module
#[doc(alias = "RAND_SEED0")]
pub type RandSeed0 = crate::Reg<rand_seed0::RAND_SEED0rs>;
///
pub mod rand_seed0;
///RAND_SEED1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed1`]
///module
#[doc(alias = "RAND_SEED1")]
pub type RandSeed1 = crate::Reg<rand_seed1::RAND_SEED1rs>;
///
pub mod rand_seed1;
///RAND_SEED2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed2`]
///module
#[doc(alias = "RAND_SEED2")]
pub type RandSeed2 = crate::Reg<rand_seed2::RAND_SEED2rs>;
///
pub mod rand_seed2;
///RAND_SEED3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed3`]
///module
#[doc(alias = "RAND_SEED3")]
pub type RandSeed3 = crate::Reg<rand_seed3::RAND_SEED3rs>;
///
pub mod rand_seed3;
///RAND_SEED4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed4`]
///module
#[doc(alias = "RAND_SEED4")]
pub type RandSeed4 = crate::Reg<rand_seed4::RAND_SEED4rs>;
///
pub mod rand_seed4;
///RAND_SEED5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed5`]
///module
#[doc(alias = "RAND_SEED5")]
pub type RandSeed5 = crate::Reg<rand_seed5::RAND_SEED5rs>;
///
pub mod rand_seed5;
///RAND_SEED6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed6`]
///module
#[doc(alias = "RAND_SEED6")]
pub type RandSeed6 = crate::Reg<rand_seed6::RAND_SEED6rs>;
///
pub mod rand_seed6;
///RAND_SEED7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed7`]
///module
#[doc(alias = "RAND_SEED7")]
pub type RandSeed7 = crate::Reg<rand_seed7::RAND_SEED7rs>;
///
pub mod rand_seed7;
///RAND_NUM0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num0`]
///module
#[doc(alias = "RAND_NUM0")]
pub type RandNum0 = crate::Reg<rand_num0::RAND_NUM0rs>;
///
pub mod rand_num0;
///RAND_NUM1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num1`]
///module
#[doc(alias = "RAND_NUM1")]
pub type RandNum1 = crate::Reg<rand_num1::RAND_NUM1rs>;
///
pub mod rand_num1;
///RAND_NUM2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num2`]
///module
#[doc(alias = "RAND_NUM2")]
pub type RandNum2 = crate::Reg<rand_num2::RAND_NUM2rs>;
///
pub mod rand_num2;
///RAND_NUM3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num3`]
///module
#[doc(alias = "RAND_NUM3")]
pub type RandNum3 = crate::Reg<rand_num3::RAND_NUM3rs>;
///
pub mod rand_num3;
///RAND_NUM4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num4`]
///module
#[doc(alias = "RAND_NUM4")]
pub type RandNum4 = crate::Reg<rand_num4::RAND_NUM4rs>;
///
pub mod rand_num4;
///RAND_NUM5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num5`]
///module
#[doc(alias = "RAND_NUM5")]
pub type RandNum5 = crate::Reg<rand_num5::RAND_NUM5rs>;
///
pub mod rand_num5;
///RAND_NUM6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num6`]
///module
#[doc(alias = "RAND_NUM6")]
pub type RandNum6 = crate::Reg<rand_num6::RAND_NUM6rs>;
///
pub mod rand_num6;
///RAND_NUM7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num7`]
///module
#[doc(alias = "RAND_NUM7")]
pub type RandNum7 = crate::Reg<rand_num7::RAND_NUM7rs>;
///
pub mod rand_num7;
///CAL_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cal_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cal_cfg`]
///module
#[doc(alias = "CAL_CFG")]
pub type CalCfg = crate::Reg<cal_cfg::CAL_CFGrs>;
///
pub mod cal_cfg;
///CAL_RESULT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cal_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cal_result`]
///module
#[doc(alias = "CAL_RESULT")]
pub type CalResult = crate::Reg<cal_result::CAL_RESULTrs>;
///
pub mod cal_result;
