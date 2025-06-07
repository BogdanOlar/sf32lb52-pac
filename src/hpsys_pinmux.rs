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
pub use pad_sa00 as pad_sa01;
pub use pad_sa00 as pad_sa02;
pub use pad_sa00 as pad_sa03;
pub use pad_sa00 as pad_sa04;
pub use pad_sa00 as pad_sa05;
pub use pad_sa00 as pad_sa06;
pub use pad_sa00 as pad_sa07;
pub use pad_sa00 as pad_sa08;
pub use pad_sa00 as pad_sa09;
pub use pad_sa00 as pad_sa10;
pub use pad_sa00 as pad_sa11;
pub use pad_sa00 as pad_sa12;
pub use pad_sa00 as pad_pa00;
pub use pad_sa00 as pad_pa01;
pub use pad_sa00 as pad_pa02;
pub use pad_sa00 as pad_pa03;
pub use pad_sa00 as pad_pa04;
pub use pad_sa00 as pad_pa05;
pub use pad_sa00 as pad_pa06;
pub use pad_sa00 as pad_pa07;
pub use pad_sa00 as pad_pa08;
pub use pad_sa00 as pad_pa09;
pub use pad_sa00 as pad_pa10;
pub use pad_sa00 as pad_pa11;
pub use pad_sa00 as pad_pa12;
pub use pad_sa00 as pad_pa13;
pub use pad_sa00 as pad_pa14;
pub use pad_sa00 as pad_pa15;
pub use pad_sa00 as pad_pa16;
pub use pad_sa00 as pad_pa17;
pub use pad_sa00 as pad_pa18;
pub use pad_sa00 as pad_pa19;
pub use pad_sa00 as pad_pa20;
pub use pad_sa00 as pad_pa21;
pub use pad_sa00 as pad_pa22;
pub use pad_sa00 as pad_pa23;
pub use pad_sa00 as pad_pa24;
pub use pad_sa00 as pad_pa25;
pub use pad_sa00 as pad_pa26;
pub use pad_sa00 as pad_pa27;
pub use pad_sa00 as pad_pa28;
pub use pad_sa00 as pad_pa29;
pub use pad_sa00 as pad_pa30;
pub use pad_sa00 as pad_pa31;
pub use pad_sa00 as pad_pa32;
pub use pad_sa00 as pad_pa33;
pub use pad_sa00 as pad_pa34;
pub use pad_sa00 as pad_pa35;
pub use pad_sa00 as pad_pa36;
pub use pad_sa00 as pad_pa37;
pub use pad_sa00 as pad_pa38;
pub use PadSa00 as PadSa01;
pub use PadSa00 as PadSa02;
pub use PadSa00 as PadSa03;
pub use PadSa00 as PadSa04;
pub use PadSa00 as PadSa05;
pub use PadSa00 as PadSa06;
pub use PadSa00 as PadSa07;
pub use PadSa00 as PadSa08;
pub use PadSa00 as PadSa09;
pub use PadSa00 as PadSa10;
pub use PadSa00 as PadSa11;
pub use PadSa00 as PadSa12;
pub use PadSa00 as PadPa00;
pub use PadSa00 as PadPa01;
pub use PadSa00 as PadPa02;
pub use PadSa00 as PadPa03;
pub use PadSa00 as PadPa04;
pub use PadSa00 as PadPa05;
pub use PadSa00 as PadPa06;
pub use PadSa00 as PadPa07;
pub use PadSa00 as PadPa08;
pub use PadSa00 as PadPa09;
pub use PadSa00 as PadPa10;
pub use PadSa00 as PadPa11;
pub use PadSa00 as PadPa12;
pub use PadSa00 as PadPa13;
pub use PadSa00 as PadPa14;
pub use PadSa00 as PadPa15;
pub use PadSa00 as PadPa16;
pub use PadSa00 as PadPa17;
pub use PadSa00 as PadPa18;
pub use PadSa00 as PadPa19;
pub use PadSa00 as PadPa20;
pub use PadSa00 as PadPa21;
pub use PadSa00 as PadPa22;
pub use PadSa00 as PadPa23;
pub use PadSa00 as PadPa24;
pub use PadSa00 as PadPa25;
pub use PadSa00 as PadPa26;
pub use PadSa00 as PadPa27;
pub use PadSa00 as PadPa28;
pub use PadSa00 as PadPa29;
pub use PadSa00 as PadPa30;
pub use PadSa00 as PadPa31;
pub use PadSa00 as PadPa32;
pub use PadSa00 as PadPa33;
pub use PadSa00 as PadPa34;
pub use PadSa00 as PadPa35;
pub use PadSa00 as PadPa36;
pub use PadSa00 as PadPa37;
pub use PadSa00 as PadPa38;
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
pub use pad_pa39 as pad_pa40;
pub use pad_pa39 as pad_pa41;
pub use pad_pa39 as pad_pa42;
pub use pad_sa00 as pad_pa43;
pub use pad_sa00 as pad_pa44;
pub use PadPa39 as PadPa40;
pub use PadPa39 as PadPa41;
pub use PadPa39 as PadPa42;
pub use PadSa00 as PadPa43;
pub use PadSa00 as PadPa44;
