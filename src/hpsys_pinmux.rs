#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pad_sa00: PadSa00,
    pad_sa01: PadSa01,
    pad_sa02: PadSa02,
    pad_sa03: PadSa03,
    pad_sa04: PadSa04,
    pad_sa05: PadSa05,
    pad_sa06: PadSa06,
    pad_sa07: PadSa07,
    pad_sa08: PadSa08,
    pad_sa09: PadSa09,
    pad_sa10: PadSa10,
    pad_sa11: PadSa11,
    pad_sa12: PadSa12,
    pad_pa00: PadPa00,
    pad_pa01: PadPa01,
    pad_pa02: PadPa02,
    pad_pa03: PadPa03,
    pad_pa04: PadPa04,
    pad_pa05: PadPa05,
    pad_pa06: PadPa06,
    pad_pa07: PadPa07,
    pad_pa08: PadPa08,
    pad_pa09: PadPa09,
    pad_pa10: PadPa10,
    pad_pa11: PadPa11,
    pad_pa12: PadPa12,
    pad_pa13: PadPa13,
    pad_pa14: PadPa14,
    pad_pa15: PadPa15,
    pad_pa16: PadPa16,
    pad_pa17: PadPa17,
    pad_pa18: PadPa18,
    pad_pa19: PadPa19,
    pad_pa20: PadPa20,
    pad_pa21: PadPa21,
    pad_pa22: PadPa22,
    pad_pa23: PadPa23,
    pad_pa24: PadPa24,
    pad_pa25: PadPa25,
    pad_pa26: PadPa26,
    pad_pa27: PadPa27,
    pad_pa28: PadPa28,
    pad_pa29: PadPa29,
    pad_pa30: PadPa30,
    pad_pa31: PadPa31,
    pad_pa32: PadPa32,
    pad_pa33: PadPa33,
    pad_pa34: PadPa34,
    pad_pa35: PadPa35,
    pad_pa36: PadPa36,
    pad_pa37: PadPa37,
    pad_pa38: PadPa38,
    pad_pa39: PadPa39,
    pad_pa40: PadPa40,
    pad_pa41: PadPa41,
    pad_pa42: PadPa42,
    pad_pa43: PadPa43,
    pad_pa44: PadPa44,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn pad_sa00(&self) -> &PadSa00 {
        &self.pad_sa00
    }
    ///0x04 -
    #[inline(always)]
    pub const fn pad_sa01(&self) -> &PadSa01 {
        &self.pad_sa01
    }
    ///0x08 -
    #[inline(always)]
    pub const fn pad_sa02(&self) -> &PadSa02 {
        &self.pad_sa02
    }
    ///0x0c -
    #[inline(always)]
    pub const fn pad_sa03(&self) -> &PadSa03 {
        &self.pad_sa03
    }
    ///0x10 -
    #[inline(always)]
    pub const fn pad_sa04(&self) -> &PadSa04 {
        &self.pad_sa04
    }
    ///0x14 -
    #[inline(always)]
    pub const fn pad_sa05(&self) -> &PadSa05 {
        &self.pad_sa05
    }
    ///0x18 -
    #[inline(always)]
    pub const fn pad_sa06(&self) -> &PadSa06 {
        &self.pad_sa06
    }
    ///0x1c -
    #[inline(always)]
    pub const fn pad_sa07(&self) -> &PadSa07 {
        &self.pad_sa07
    }
    ///0x20 -
    #[inline(always)]
    pub const fn pad_sa08(&self) -> &PadSa08 {
        &self.pad_sa08
    }
    ///0x24 -
    #[inline(always)]
    pub const fn pad_sa09(&self) -> &PadSa09 {
        &self.pad_sa09
    }
    ///0x28 -
    #[inline(always)]
    pub const fn pad_sa10(&self) -> &PadSa10 {
        &self.pad_sa10
    }
    ///0x2c -
    #[inline(always)]
    pub const fn pad_sa11(&self) -> &PadSa11 {
        &self.pad_sa11
    }
    ///0x30 -
    #[inline(always)]
    pub const fn pad_sa12(&self) -> &PadSa12 {
        &self.pad_sa12
    }
    ///0x34 -
    #[inline(always)]
    pub const fn pad_pa00(&self) -> &PadPa00 {
        &self.pad_pa00
    }
    ///0x38 -
    #[inline(always)]
    pub const fn pad_pa01(&self) -> &PadPa01 {
        &self.pad_pa01
    }
    ///0x3c -
    #[inline(always)]
    pub const fn pad_pa02(&self) -> &PadPa02 {
        &self.pad_pa02
    }
    ///0x40 -
    #[inline(always)]
    pub const fn pad_pa03(&self) -> &PadPa03 {
        &self.pad_pa03
    }
    ///0x44 -
    #[inline(always)]
    pub const fn pad_pa04(&self) -> &PadPa04 {
        &self.pad_pa04
    }
    ///0x48 -
    #[inline(always)]
    pub const fn pad_pa05(&self) -> &PadPa05 {
        &self.pad_pa05
    }
    ///0x4c -
    #[inline(always)]
    pub const fn pad_pa06(&self) -> &PadPa06 {
        &self.pad_pa06
    }
    ///0x50 -
    #[inline(always)]
    pub const fn pad_pa07(&self) -> &PadPa07 {
        &self.pad_pa07
    }
    ///0x54 -
    #[inline(always)]
    pub const fn pad_pa08(&self) -> &PadPa08 {
        &self.pad_pa08
    }
    ///0x58 -
    #[inline(always)]
    pub const fn pad_pa09(&self) -> &PadPa09 {
        &self.pad_pa09
    }
    ///0x5c -
    #[inline(always)]
    pub const fn pad_pa10(&self) -> &PadPa10 {
        &self.pad_pa10
    }
    ///0x60 -
    #[inline(always)]
    pub const fn pad_pa11(&self) -> &PadPa11 {
        &self.pad_pa11
    }
    ///0x64 -
    #[inline(always)]
    pub const fn pad_pa12(&self) -> &PadPa12 {
        &self.pad_pa12
    }
    ///0x68 -
    #[inline(always)]
    pub const fn pad_pa13(&self) -> &PadPa13 {
        &self.pad_pa13
    }
    ///0x6c -
    #[inline(always)]
    pub const fn pad_pa14(&self) -> &PadPa14 {
        &self.pad_pa14
    }
    ///0x70 -
    #[inline(always)]
    pub const fn pad_pa15(&self) -> &PadPa15 {
        &self.pad_pa15
    }
    ///0x74 -
    #[inline(always)]
    pub const fn pad_pa16(&self) -> &PadPa16 {
        &self.pad_pa16
    }
    ///0x78 -
    #[inline(always)]
    pub const fn pad_pa17(&self) -> &PadPa17 {
        &self.pad_pa17
    }
    ///0x7c -
    #[inline(always)]
    pub const fn pad_pa18(&self) -> &PadPa18 {
        &self.pad_pa18
    }
    ///0x80 -
    #[inline(always)]
    pub const fn pad_pa19(&self) -> &PadPa19 {
        &self.pad_pa19
    }
    ///0x84 -
    #[inline(always)]
    pub const fn pad_pa20(&self) -> &PadPa20 {
        &self.pad_pa20
    }
    ///0x88 -
    #[inline(always)]
    pub const fn pad_pa21(&self) -> &PadPa21 {
        &self.pad_pa21
    }
    ///0x8c -
    #[inline(always)]
    pub const fn pad_pa22(&self) -> &PadPa22 {
        &self.pad_pa22
    }
    ///0x90 -
    #[inline(always)]
    pub const fn pad_pa23(&self) -> &PadPa23 {
        &self.pad_pa23
    }
    ///0x94 -
    #[inline(always)]
    pub const fn pad_pa24(&self) -> &PadPa24 {
        &self.pad_pa24
    }
    ///0x98 -
    #[inline(always)]
    pub const fn pad_pa25(&self) -> &PadPa25 {
        &self.pad_pa25
    }
    ///0x9c -
    #[inline(always)]
    pub const fn pad_pa26(&self) -> &PadPa26 {
        &self.pad_pa26
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn pad_pa27(&self) -> &PadPa27 {
        &self.pad_pa27
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn pad_pa28(&self) -> &PadPa28 {
        &self.pad_pa28
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn pad_pa29(&self) -> &PadPa29 {
        &self.pad_pa29
    }
    ///0xac -
    #[inline(always)]
    pub const fn pad_pa30(&self) -> &PadPa30 {
        &self.pad_pa30
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn pad_pa31(&self) -> &PadPa31 {
        &self.pad_pa31
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn pad_pa32(&self) -> &PadPa32 {
        &self.pad_pa32
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn pad_pa33(&self) -> &PadPa33 {
        &self.pad_pa33
    }
    ///0xbc -
    #[inline(always)]
    pub const fn pad_pa34(&self) -> &PadPa34 {
        &self.pad_pa34
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn pad_pa35(&self) -> &PadPa35 {
        &self.pad_pa35
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn pad_pa36(&self) -> &PadPa36 {
        &self.pad_pa36
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn pad_pa37(&self) -> &PadPa37 {
        &self.pad_pa37
    }
    ///0xcc -
    #[inline(always)]
    pub const fn pad_pa38(&self) -> &PadPa38 {
        &self.pad_pa38
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn pad_pa39(&self) -> &PadPa39 {
        &self.pad_pa39
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn pad_pa40(&self) -> &PadPa40 {
        &self.pad_pa40
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn pad_pa41(&self) -> &PadPa41 {
        &self.pad_pa41
    }
    ///0xdc -
    #[inline(always)]
    pub const fn pad_pa42(&self) -> &PadPa42 {
        &self.pad_pa42
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn pad_pa43(&self) -> &PadPa43 {
        &self.pad_pa43
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn pad_pa44(&self) -> &PadPa44 {
        &self.pad_pa44
    }
}
///PAD_SA00 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa00`]
///module
#[doc(alias = "PAD_SA00")]
pub type PadSa00 = crate::Reg<pad_sa00::PAD_SA00rs>;
///
pub mod pad_sa00;
///PAD_SA01 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa01`]
///module
#[doc(alias = "PAD_SA01")]
pub type PadSa01 = crate::Reg<pad_sa01::PAD_SA01rs>;
///
pub mod pad_sa01;
///PAD_SA02 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa02`]
///module
#[doc(alias = "PAD_SA02")]
pub type PadSa02 = crate::Reg<pad_sa02::PAD_SA02rs>;
///
pub mod pad_sa02;
///PAD_SA03 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa03`]
///module
#[doc(alias = "PAD_SA03")]
pub type PadSa03 = crate::Reg<pad_sa03::PAD_SA03rs>;
///
pub mod pad_sa03;
///PAD_SA04 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa04`]
///module
#[doc(alias = "PAD_SA04")]
pub type PadSa04 = crate::Reg<pad_sa04::PAD_SA04rs>;
///
pub mod pad_sa04;
///PAD_SA05 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa05`]
///module
#[doc(alias = "PAD_SA05")]
pub type PadSa05 = crate::Reg<pad_sa05::PAD_SA05rs>;
///
pub mod pad_sa05;
///PAD_SA06 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa06`]
///module
#[doc(alias = "PAD_SA06")]
pub type PadSa06 = crate::Reg<pad_sa06::PAD_SA06rs>;
///
pub mod pad_sa06;
///PAD_SA07 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa07`]
///module
#[doc(alias = "PAD_SA07")]
pub type PadSa07 = crate::Reg<pad_sa07::PAD_SA07rs>;
///
pub mod pad_sa07;
///PAD_SA08 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa08`]
///module
#[doc(alias = "PAD_SA08")]
pub type PadSa08 = crate::Reg<pad_sa08::PAD_SA08rs>;
///
pub mod pad_sa08;
///PAD_SA09 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa09::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa09::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa09`]
///module
#[doc(alias = "PAD_SA09")]
pub type PadSa09 = crate::Reg<pad_sa09::PAD_SA09rs>;
///
pub mod pad_sa09;
///PAD_SA10 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa10`]
///module
#[doc(alias = "PAD_SA10")]
pub type PadSa10 = crate::Reg<pad_sa10::PAD_SA10rs>;
///
pub mod pad_sa10;
///PAD_SA11 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa11`]
///module
#[doc(alias = "PAD_SA11")]
pub type PadSa11 = crate::Reg<pad_sa11::PAD_SA11rs>;
///
pub mod pad_sa11;
///PAD_SA12 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_sa12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_sa12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_sa12`]
///module
#[doc(alias = "PAD_SA12")]
pub type PadSa12 = crate::Reg<pad_sa12::PAD_SA12rs>;
///
pub mod pad_sa12;
///PAD_PA00 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa00`]
///module
#[doc(alias = "PAD_PA00")]
pub type PadPa00 = crate::Reg<pad_pa00::PAD_PA00rs>;
///
pub mod pad_pa00;
///PAD_PA01 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa01`]
///module
#[doc(alias = "PAD_PA01")]
pub type PadPa01 = crate::Reg<pad_pa01::PAD_PA01rs>;
///
pub mod pad_pa01;
///PAD_PA02 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa02`]
///module
#[doc(alias = "PAD_PA02")]
pub type PadPa02 = crate::Reg<pad_pa02::PAD_PA02rs>;
///
pub mod pad_pa02;
///PAD_PA03 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa03`]
///module
#[doc(alias = "PAD_PA03")]
pub type PadPa03 = crate::Reg<pad_pa03::PAD_PA03rs>;
///
pub mod pad_pa03;
///PAD_PA04 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa04`]
///module
#[doc(alias = "PAD_PA04")]
pub type PadPa04 = crate::Reg<pad_pa04::PAD_PA04rs>;
///
pub mod pad_pa04;
///PAD_PA05 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa05`]
///module
#[doc(alias = "PAD_PA05")]
pub type PadPa05 = crate::Reg<pad_pa05::PAD_PA05rs>;
///
pub mod pad_pa05;
///PAD_PA06 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa06`]
///module
#[doc(alias = "PAD_PA06")]
pub type PadPa06 = crate::Reg<pad_pa06::PAD_PA06rs>;
///
pub mod pad_pa06;
///PAD_PA07 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa07`]
///module
#[doc(alias = "PAD_PA07")]
pub type PadPa07 = crate::Reg<pad_pa07::PAD_PA07rs>;
///
pub mod pad_pa07;
///PAD_PA08 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa08`]
///module
#[doc(alias = "PAD_PA08")]
pub type PadPa08 = crate::Reg<pad_pa08::PAD_PA08rs>;
///
pub mod pad_pa08;
///PAD_PA09 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa09::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa09::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa09`]
///module
#[doc(alias = "PAD_PA09")]
pub type PadPa09 = crate::Reg<pad_pa09::PAD_PA09rs>;
///
pub mod pad_pa09;
///PAD_PA10 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa10`]
///module
#[doc(alias = "PAD_PA10")]
pub type PadPa10 = crate::Reg<pad_pa10::PAD_PA10rs>;
///
pub mod pad_pa10;
///PAD_PA11 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa11`]
///module
#[doc(alias = "PAD_PA11")]
pub type PadPa11 = crate::Reg<pad_pa11::PAD_PA11rs>;
///
pub mod pad_pa11;
///PAD_PA12 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa12`]
///module
#[doc(alias = "PAD_PA12")]
pub type PadPa12 = crate::Reg<pad_pa12::PAD_PA12rs>;
///
pub mod pad_pa12;
///PAD_PA13 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa13`]
///module
#[doc(alias = "PAD_PA13")]
pub type PadPa13 = crate::Reg<pad_pa13::PAD_PA13rs>;
///
pub mod pad_pa13;
///PAD_PA14 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa14`]
///module
#[doc(alias = "PAD_PA14")]
pub type PadPa14 = crate::Reg<pad_pa14::PAD_PA14rs>;
///
pub mod pad_pa14;
///PAD_PA15 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa15`]
///module
#[doc(alias = "PAD_PA15")]
pub type PadPa15 = crate::Reg<pad_pa15::PAD_PA15rs>;
///
pub mod pad_pa15;
///PAD_PA16 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa16`]
///module
#[doc(alias = "PAD_PA16")]
pub type PadPa16 = crate::Reg<pad_pa16::PAD_PA16rs>;
///
pub mod pad_pa16;
///PAD_PA17 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa17`]
///module
#[doc(alias = "PAD_PA17")]
pub type PadPa17 = crate::Reg<pad_pa17::PAD_PA17rs>;
///
pub mod pad_pa17;
///PAD_PA18 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa18`]
///module
#[doc(alias = "PAD_PA18")]
pub type PadPa18 = crate::Reg<pad_pa18::PAD_PA18rs>;
///
pub mod pad_pa18;
///PAD_PA19 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa19`]
///module
#[doc(alias = "PAD_PA19")]
pub type PadPa19 = crate::Reg<pad_pa19::PAD_PA19rs>;
///
pub mod pad_pa19;
///PAD_PA20 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa20`]
///module
#[doc(alias = "PAD_PA20")]
pub type PadPa20 = crate::Reg<pad_pa20::PAD_PA20rs>;
///
pub mod pad_pa20;
///PAD_PA21 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa21`]
///module
#[doc(alias = "PAD_PA21")]
pub type PadPa21 = crate::Reg<pad_pa21::PAD_PA21rs>;
///
pub mod pad_pa21;
///PAD_PA22 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa22`]
///module
#[doc(alias = "PAD_PA22")]
pub type PadPa22 = crate::Reg<pad_pa22::PAD_PA22rs>;
///
pub mod pad_pa22;
///PAD_PA23 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa23`]
///module
#[doc(alias = "PAD_PA23")]
pub type PadPa23 = crate::Reg<pad_pa23::PAD_PA23rs>;
///
pub mod pad_pa23;
///PAD_PA24 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa24`]
///module
#[doc(alias = "PAD_PA24")]
pub type PadPa24 = crate::Reg<pad_pa24::PAD_PA24rs>;
///
pub mod pad_pa24;
///PAD_PA25 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa25`]
///module
#[doc(alias = "PAD_PA25")]
pub type PadPa25 = crate::Reg<pad_pa25::PAD_PA25rs>;
///
pub mod pad_pa25;
///PAD_PA26 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa26`]
///module
#[doc(alias = "PAD_PA26")]
pub type PadPa26 = crate::Reg<pad_pa26::PAD_PA26rs>;
///
pub mod pad_pa26;
///PAD_PA27 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa27`]
///module
#[doc(alias = "PAD_PA27")]
pub type PadPa27 = crate::Reg<pad_pa27::PAD_PA27rs>;
///
pub mod pad_pa27;
///PAD_PA28 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa28`]
///module
#[doc(alias = "PAD_PA28")]
pub type PadPa28 = crate::Reg<pad_pa28::PAD_PA28rs>;
///
pub mod pad_pa28;
///PAD_PA29 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa29`]
///module
#[doc(alias = "PAD_PA29")]
pub type PadPa29 = crate::Reg<pad_pa29::PAD_PA29rs>;
///
pub mod pad_pa29;
///PAD_PA30 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa30`]
///module
#[doc(alias = "PAD_PA30")]
pub type PadPa30 = crate::Reg<pad_pa30::PAD_PA30rs>;
///
pub mod pad_pa30;
///PAD_PA31 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa31`]
///module
#[doc(alias = "PAD_PA31")]
pub type PadPa31 = crate::Reg<pad_pa31::PAD_PA31rs>;
///
pub mod pad_pa31;
///PAD_PA32 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa32`]
///module
#[doc(alias = "PAD_PA32")]
pub type PadPa32 = crate::Reg<pad_pa32::PAD_PA32rs>;
///
pub mod pad_pa32;
///PAD_PA33 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa33`]
///module
#[doc(alias = "PAD_PA33")]
pub type PadPa33 = crate::Reg<pad_pa33::PAD_PA33rs>;
///
pub mod pad_pa33;
///PAD_PA34 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa34`]
///module
#[doc(alias = "PAD_PA34")]
pub type PadPa34 = crate::Reg<pad_pa34::PAD_PA34rs>;
///
pub mod pad_pa34;
///PAD_PA35 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa35`]
///module
#[doc(alias = "PAD_PA35")]
pub type PadPa35 = crate::Reg<pad_pa35::PAD_PA35rs>;
///
pub mod pad_pa35;
///PAD_PA36 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa36`]
///module
#[doc(alias = "PAD_PA36")]
pub type PadPa36 = crate::Reg<pad_pa36::PAD_PA36rs>;
///
pub mod pad_pa36;
///PAD_PA37 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa37`]
///module
#[doc(alias = "PAD_PA37")]
pub type PadPa37 = crate::Reg<pad_pa37::PAD_PA37rs>;
///
pub mod pad_pa37;
///PAD_PA38 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa38`]
///module
#[doc(alias = "PAD_PA38")]
pub type PadPa38 = crate::Reg<pad_pa38::PAD_PA38rs>;
///
pub mod pad_pa38;
///PAD_PA39 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa39`]
///module
#[doc(alias = "PAD_PA39")]
pub type PadPa39 = crate::Reg<pad_pa39::PAD_PA39rs>;
///
pub mod pad_pa39;
///PAD_PA40 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa40`]
///module
#[doc(alias = "PAD_PA40")]
pub type PadPa40 = crate::Reg<pad_pa40::PAD_PA40rs>;
///
pub mod pad_pa40;
///PAD_PA41 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa41`]
///module
#[doc(alias = "PAD_PA41")]
pub type PadPa41 = crate::Reg<pad_pa41::PAD_PA41rs>;
///
pub mod pad_pa41;
///PAD_PA42 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa42`]
///module
#[doc(alias = "PAD_PA42")]
pub type PadPa42 = crate::Reg<pad_pa42::PAD_PA42rs>;
///
pub mod pad_pa42;
///PAD_PA43 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa43`]
///module
#[doc(alias = "PAD_PA43")]
pub type PadPa43 = crate::Reg<pad_pa43::PAD_PA43rs>;
///
pub mod pad_pa43;
///PAD_PA44 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pa44`]
///module
#[doc(alias = "PAD_PA44")]
pub type PadPa44 = crate::Reg<pad_pa44::PAD_PA44rs>;
///
pub mod pad_pa44;
