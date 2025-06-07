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
    ///0x10 - Random seed value0. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed0(&self) -> &RandSeed0 {
        &self.rand_seed0
    }
    ///0x14 - random seed value1. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed1(&self) -> &RandSeed1 {
        &self.rand_seed1
    }
    ///0x18 - random seed value2. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed2(&self) -> &RandSeed2 {
        &self.rand_seed2
    }
    ///0x1c - random seed value3. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed3(&self) -> &RandSeed3 {
        &self.rand_seed3
    }
    ///0x20 - random seed value4. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed4(&self) -> &RandSeed4 {
        &self.rand_seed4
    }
    ///0x24 - random seed value5. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed5(&self) -> &RandSeed5 {
        &self.rand_seed5
    }
    ///0x28 - random seed value6. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed6(&self) -> &RandSeed6 {
        &self.rand_seed6
    }
    ///0x2c - random seed value7. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub const fn rand_seed7(&self) -> &RandSeed7 {
        &self.rand_seed7
    }
    ///0x30 - random number value0
    #[inline(always)]
    pub const fn rand_num0(&self) -> &RandNum0 {
        &self.rand_num0
    }
    ///0x34 - random number value1
    #[inline(always)]
    pub const fn rand_num1(&self) -> &RandNum1 {
        &self.rand_num1
    }
    ///0x38 - random number value2
    #[inline(always)]
    pub const fn rand_num2(&self) -> &RandNum2 {
        &self.rand_num2
    }
    ///0x3c - random number value3
    #[inline(always)]
    pub const fn rand_num3(&self) -> &RandNum3 {
        &self.rand_num3
    }
    ///0x40 - random number value4
    #[inline(always)]
    pub const fn rand_num4(&self) -> &RandNum4 {
        &self.rand_num4
    }
    ///0x44 - random number value5
    #[inline(always)]
    pub const fn rand_num5(&self) -> &RandNum5 {
        &self.rand_num5
    }
    ///0x48 - random number value6
    #[inline(always)]
    pub const fn rand_num6(&self) -> &RandNum6 {
        &self.rand_num6
    }
    ///0x4c - random number value7
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
///RAND_SEED0 (rw) register accessor: Random seed value0. If using external random seed, write value to this register will update the random seed in use.
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_seed0`]
///module
#[doc(alias = "RAND_SEED0")]
pub type RandSeed0 = crate::Reg<rand_seed0::RAND_SEED0rs>;
///Random seed value0. If using external random seed, write value to this register will update the random seed in use.
pub mod rand_seed0;
pub use rand_seed0 as rand_seed1;
pub use rand_seed0 as rand_seed2;
pub use rand_seed0 as rand_seed3;
pub use rand_seed0 as rand_seed4;
pub use rand_seed0 as rand_seed5;
pub use rand_seed0 as rand_seed6;
pub use rand_seed0 as rand_seed7;
pub use RandSeed0 as RandSeed1;
pub use RandSeed0 as RandSeed2;
pub use RandSeed0 as RandSeed3;
pub use RandSeed0 as RandSeed4;
pub use RandSeed0 as RandSeed5;
pub use RandSeed0 as RandSeed6;
pub use RandSeed0 as RandSeed7;
///RAND_NUM0 (rw) register accessor: random number value0
///
///You can [`read`](crate::Reg::read) this register and get [`rand_num0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_num0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rand_num0`]
///module
#[doc(alias = "RAND_NUM0")]
pub type RandNum0 = crate::Reg<rand_num0::RAND_NUM0rs>;
///random number value0
pub mod rand_num0;
pub use rand_num0 as rand_num1;
pub use rand_num0 as rand_num2;
pub use rand_num0 as rand_num3;
pub use rand_num0 as rand_num4;
pub use rand_num0 as rand_num5;
pub use rand_num0 as rand_num6;
pub use rand_num0 as rand_num7;
pub use RandNum0 as RandNum1;
pub use RandNum0 as RandNum2;
pub use RandNum0 as RandNum3;
pub use RandNum0 as RandNum4;
pub use RandNum0 as RandNum5;
pub use RandNum0 as RandNum6;
pub use RandNum0 as RandNum7;
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
