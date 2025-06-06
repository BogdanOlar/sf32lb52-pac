#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: Isr,
    ifcr: Ifcr,
    ccr1: Ccr1,
    cndtr1: Cndtr1,
    cpar1: Cpar1,
    cm0ar1: Cm0ar1,
    cbsr1: Cbsr1,
    ccr2: Ccr2,
    cndtr2: Cndtr2,
    cpar2: Cpar2,
    cm0ar2: Cm0ar2,
    cbsr2: Cbsr2,
    ccr3: Ccr3,
    cndtr3: Cndtr3,
    cpar3: Cpar3,
    cm0ar3: Cm0ar3,
    cbsr3: Cbsr3,
    ccr4: Ccr4,
    cndtr4: Cndtr4,
    cpar4: Cpar4,
    cm0ar4: Cm0ar4,
    cbsr4: Cbsr4,
    ccr5: Ccr5,
    cndtr5: Cndtr5,
    cpar5: Cpar5,
    cm0ar5: Cm0ar5,
    cbsr5: Cbsr5,
    ccr6: Ccr6,
    cndtr6: Cndtr6,
    cpar6: Cpar6,
    cm0ar6: Cm0ar6,
    cbsr6: Cbsr6,
    ccr7: Ccr7,
    cndtr7: Cndtr7,
    cpar7: Cpar7,
    cm0ar7: Cm0ar7,
    cbsr7: Cbsr7,
    ccr8: Ccr8,
    cndtr8: Cndtr8,
    cpar8: Cpar8,
    cm0ar8: Cm0ar8,
    cbsr8: Cbsr8,
    cselr1: Cselr1,
    cselr2: Cselr2,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    ///0x04 -
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    ///0x08 -
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    ///0x0c -
    #[inline(always)]
    pub const fn cndtr1(&self) -> &Cndtr1 {
        &self.cndtr1
    }
    ///0x10 -
    #[inline(always)]
    pub const fn cpar1(&self) -> &Cpar1 {
        &self.cpar1
    }
    ///0x14 -
    #[inline(always)]
    pub const fn cm0ar1(&self) -> &Cm0ar1 {
        &self.cm0ar1
    }
    ///0x18 -
    #[inline(always)]
    pub const fn cbsr1(&self) -> &Cbsr1 {
        &self.cbsr1
    }
    ///0x1c -
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    ///0x20 -
    #[inline(always)]
    pub const fn cndtr2(&self) -> &Cndtr2 {
        &self.cndtr2
    }
    ///0x24 -
    #[inline(always)]
    pub const fn cpar2(&self) -> &Cpar2 {
        &self.cpar2
    }
    ///0x28 -
    #[inline(always)]
    pub const fn cm0ar2(&self) -> &Cm0ar2 {
        &self.cm0ar2
    }
    ///0x2c -
    #[inline(always)]
    pub const fn cbsr2(&self) -> &Cbsr2 {
        &self.cbsr2
    }
    ///0x30 -
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    ///0x34 -
    #[inline(always)]
    pub const fn cndtr3(&self) -> &Cndtr3 {
        &self.cndtr3
    }
    ///0x38 -
    #[inline(always)]
    pub const fn cpar3(&self) -> &Cpar3 {
        &self.cpar3
    }
    ///0x3c -
    #[inline(always)]
    pub const fn cm0ar3(&self) -> &Cm0ar3 {
        &self.cm0ar3
    }
    ///0x40 -
    #[inline(always)]
    pub const fn cbsr3(&self) -> &Cbsr3 {
        &self.cbsr3
    }
    ///0x44 -
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    ///0x48 -
    #[inline(always)]
    pub const fn cndtr4(&self) -> &Cndtr4 {
        &self.cndtr4
    }
    ///0x4c -
    #[inline(always)]
    pub const fn cpar4(&self) -> &Cpar4 {
        &self.cpar4
    }
    ///0x50 -
    #[inline(always)]
    pub const fn cm0ar4(&self) -> &Cm0ar4 {
        &self.cm0ar4
    }
    ///0x54 -
    #[inline(always)]
    pub const fn cbsr4(&self) -> &Cbsr4 {
        &self.cbsr4
    }
    ///0x58 -
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    ///0x5c -
    #[inline(always)]
    pub const fn cndtr5(&self) -> &Cndtr5 {
        &self.cndtr5
    }
    ///0x60 -
    #[inline(always)]
    pub const fn cpar5(&self) -> &Cpar5 {
        &self.cpar5
    }
    ///0x64 -
    #[inline(always)]
    pub const fn cm0ar5(&self) -> &Cm0ar5 {
        &self.cm0ar5
    }
    ///0x68 -
    #[inline(always)]
    pub const fn cbsr5(&self) -> &Cbsr5 {
        &self.cbsr5
    }
    ///0x6c -
    #[inline(always)]
    pub const fn ccr6(&self) -> &Ccr6 {
        &self.ccr6
    }
    ///0x70 -
    #[inline(always)]
    pub const fn cndtr6(&self) -> &Cndtr6 {
        &self.cndtr6
    }
    ///0x74 -
    #[inline(always)]
    pub const fn cpar6(&self) -> &Cpar6 {
        &self.cpar6
    }
    ///0x78 -
    #[inline(always)]
    pub const fn cm0ar6(&self) -> &Cm0ar6 {
        &self.cm0ar6
    }
    ///0x7c -
    #[inline(always)]
    pub const fn cbsr6(&self) -> &Cbsr6 {
        &self.cbsr6
    }
    ///0x80 -
    #[inline(always)]
    pub const fn ccr7(&self) -> &Ccr7 {
        &self.ccr7
    }
    ///0x84 -
    #[inline(always)]
    pub const fn cndtr7(&self) -> &Cndtr7 {
        &self.cndtr7
    }
    ///0x88 -
    #[inline(always)]
    pub const fn cpar7(&self) -> &Cpar7 {
        &self.cpar7
    }
    ///0x8c -
    #[inline(always)]
    pub const fn cm0ar7(&self) -> &Cm0ar7 {
        &self.cm0ar7
    }
    ///0x90 -
    #[inline(always)]
    pub const fn cbsr7(&self) -> &Cbsr7 {
        &self.cbsr7
    }
    ///0x94 -
    #[inline(always)]
    pub const fn ccr8(&self) -> &Ccr8 {
        &self.ccr8
    }
    ///0x98 -
    #[inline(always)]
    pub const fn cndtr8(&self) -> &Cndtr8 {
        &self.cndtr8
    }
    ///0x9c -
    #[inline(always)]
    pub const fn cpar8(&self) -> &Cpar8 {
        &self.cpar8
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn cm0ar8(&self) -> &Cm0ar8 {
        &self.cm0ar8
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn cbsr8(&self) -> &Cbsr8 {
        &self.cbsr8
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn cselr1(&self) -> &Cselr1 {
        &self.cselr1
    }
    ///0xac -
    #[inline(always)]
    pub const fn cselr2(&self) -> &Cselr2 {
        &self.cselr2
    }
}
///ISR (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@isr`]
///module
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::ISRrs>;
///
pub mod isr;
///IFCR (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ifcr`]
///module
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IFCRrs>;
///
pub mod ifcr;
///CCR1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr1`]
///module
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::CCR1rs>;
///
pub mod ccr1;
///CNDTR1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr1`]
///module
#[doc(alias = "CNDTR1")]
pub type Cndtr1 = crate::Reg<cndtr1::CNDTR1rs>;
///
pub mod cndtr1;
///CPAR1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar1`]
///module
#[doc(alias = "CPAR1")]
pub type Cpar1 = crate::Reg<cpar1::CPAR1rs>;
///
pub mod cpar1;
///CM0AR1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar1`]
///module
#[doc(alias = "CM0AR1")]
pub type Cm0ar1 = crate::Reg<cm0ar1::CM0AR1rs>;
///
pub mod cm0ar1;
///CBSR1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr1`]
///module
#[doc(alias = "CBSR1")]
pub type Cbsr1 = crate::Reg<cbsr1::CBSR1rs>;
///
pub mod cbsr1;
///CCR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr2`]
///module
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::CCR2rs>;
///
pub mod ccr2;
///CNDTR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr2`]
///module
#[doc(alias = "CNDTR2")]
pub type Cndtr2 = crate::Reg<cndtr2::CNDTR2rs>;
///
pub mod cndtr2;
///CPAR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar2`]
///module
#[doc(alias = "CPAR2")]
pub type Cpar2 = crate::Reg<cpar2::CPAR2rs>;
///
pub mod cpar2;
///CM0AR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar2`]
///module
#[doc(alias = "CM0AR2")]
pub type Cm0ar2 = crate::Reg<cm0ar2::CM0AR2rs>;
///
pub mod cm0ar2;
///CBSR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr2`]
///module
#[doc(alias = "CBSR2")]
pub type Cbsr2 = crate::Reg<cbsr2::CBSR2rs>;
///
pub mod cbsr2;
///CCR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr3`]
///module
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::CCR3rs>;
///
pub mod ccr3;
///CNDTR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr3`]
///module
#[doc(alias = "CNDTR3")]
pub type Cndtr3 = crate::Reg<cndtr3::CNDTR3rs>;
///
pub mod cndtr3;
///CPAR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar3`]
///module
#[doc(alias = "CPAR3")]
pub type Cpar3 = crate::Reg<cpar3::CPAR3rs>;
///
pub mod cpar3;
///CM0AR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar3`]
///module
#[doc(alias = "CM0AR3")]
pub type Cm0ar3 = crate::Reg<cm0ar3::CM0AR3rs>;
///
pub mod cm0ar3;
///CBSR3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr3`]
///module
#[doc(alias = "CBSR3")]
pub type Cbsr3 = crate::Reg<cbsr3::CBSR3rs>;
///
pub mod cbsr3;
///CCR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr4`]
///module
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::CCR4rs>;
///
pub mod ccr4;
///CNDTR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr4`]
///module
#[doc(alias = "CNDTR4")]
pub type Cndtr4 = crate::Reg<cndtr4::CNDTR4rs>;
///
pub mod cndtr4;
///CPAR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar4`]
///module
#[doc(alias = "CPAR4")]
pub type Cpar4 = crate::Reg<cpar4::CPAR4rs>;
///
pub mod cpar4;
///CM0AR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar4`]
///module
#[doc(alias = "CM0AR4")]
pub type Cm0ar4 = crate::Reg<cm0ar4::CM0AR4rs>;
///
pub mod cm0ar4;
///CBSR4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr4`]
///module
#[doc(alias = "CBSR4")]
pub type Cbsr4 = crate::Reg<cbsr4::CBSR4rs>;
///
pub mod cbsr4;
///CCR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr5`]
///module
#[doc(alias = "CCR5")]
pub type Ccr5 = crate::Reg<ccr5::CCR5rs>;
///
pub mod ccr5;
///CNDTR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr5`]
///module
#[doc(alias = "CNDTR5")]
pub type Cndtr5 = crate::Reg<cndtr5::CNDTR5rs>;
///
pub mod cndtr5;
///CPAR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar5`]
///module
#[doc(alias = "CPAR5")]
pub type Cpar5 = crate::Reg<cpar5::CPAR5rs>;
///
pub mod cpar5;
///CM0AR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar5`]
///module
#[doc(alias = "CM0AR5")]
pub type Cm0ar5 = crate::Reg<cm0ar5::CM0AR5rs>;
///
pub mod cm0ar5;
///CBSR5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr5`]
///module
#[doc(alias = "CBSR5")]
pub type Cbsr5 = crate::Reg<cbsr5::CBSR5rs>;
///
pub mod cbsr5;
///CCR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr6`]
///module
#[doc(alias = "CCR6")]
pub type Ccr6 = crate::Reg<ccr6::CCR6rs>;
///
pub mod ccr6;
///CNDTR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr6`]
///module
#[doc(alias = "CNDTR6")]
pub type Cndtr6 = crate::Reg<cndtr6::CNDTR6rs>;
///
pub mod cndtr6;
///CPAR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar6`]
///module
#[doc(alias = "CPAR6")]
pub type Cpar6 = crate::Reg<cpar6::CPAR6rs>;
///
pub mod cpar6;
///CM0AR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar6`]
///module
#[doc(alias = "CM0AR6")]
pub type Cm0ar6 = crate::Reg<cm0ar6::CM0AR6rs>;
///
pub mod cm0ar6;
///CBSR6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr6`]
///module
#[doc(alias = "CBSR6")]
pub type Cbsr6 = crate::Reg<cbsr6::CBSR6rs>;
///
pub mod cbsr6;
///CCR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr7`]
///module
#[doc(alias = "CCR7")]
pub type Ccr7 = crate::Reg<ccr7::CCR7rs>;
///
pub mod ccr7;
///CNDTR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr7`]
///module
#[doc(alias = "CNDTR7")]
pub type Cndtr7 = crate::Reg<cndtr7::CNDTR7rs>;
///
pub mod cndtr7;
///CPAR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar7`]
///module
#[doc(alias = "CPAR7")]
pub type Cpar7 = crate::Reg<cpar7::CPAR7rs>;
///
pub mod cpar7;
///CM0AR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar7`]
///module
#[doc(alias = "CM0AR7")]
pub type Cm0ar7 = crate::Reg<cm0ar7::CM0AR7rs>;
///
pub mod cm0ar7;
///CBSR7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr7`]
///module
#[doc(alias = "CBSR7")]
pub type Cbsr7 = crate::Reg<cbsr7::CBSR7rs>;
///
pub mod cbsr7;
///CCR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ccr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ccr8`]
///module
#[doc(alias = "CCR8")]
pub type Ccr8 = crate::Reg<ccr8::CCR8rs>;
///
pub mod ccr8;
///CNDTR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cndtr8`]
///module
#[doc(alias = "CNDTR8")]
pub type Cndtr8 = crate::Reg<cndtr8::CNDTR8rs>;
///
pub mod cndtr8;
///CPAR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cpar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cpar8`]
///module
#[doc(alias = "CPAR8")]
pub type Cpar8 = crate::Reg<cpar8::CPAR8rs>;
///
pub mod cpar8;
///CM0AR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cm0ar8`]
///module
#[doc(alias = "CM0AR8")]
pub type Cm0ar8 = crate::Reg<cm0ar8::CM0AR8rs>;
///
pub mod cm0ar8;
///CBSR8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cbsr8`]
///module
#[doc(alias = "CBSR8")]
pub type Cbsr8 = crate::Reg<cbsr8::CBSR8rs>;
///
pub mod cbsr8;
///CSELR1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cselr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cselr1`]
///module
#[doc(alias = "CSELR1")]
pub type Cselr1 = crate::Reg<cselr1::CSELR1rs>;
///
pub mod cselr1;
///CSELR2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`cselr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cselr2`]
///module
#[doc(alias = "CSELR2")]
pub type Cselr2 = crate::Reg<cselr2::CSELR2rs>;
///
pub mod cselr2;
