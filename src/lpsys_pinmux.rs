#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pad_pb00: PadPb00,
    pad_pb01: PadPb01,
    pad_pb02: PadPb02,
    pad_pb03: PadPb03,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn pad_pb00(&self) -> &PadPb00 {
        &self.pad_pb00
    }
    ///0x04 -
    #[inline(always)]
    pub const fn pad_pb01(&self) -> &PadPb01 {
        &self.pad_pb01
    }
    ///0x08 -
    #[inline(always)]
    pub const fn pad_pb02(&self) -> &PadPb02 {
        &self.pad_pb02
    }
    ///0x0c -
    #[inline(always)]
    pub const fn pad_pb03(&self) -> &PadPb03 {
        &self.pad_pb03
    }
}
///PAD_PB00 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pb00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pb00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pb00`]
///module
#[doc(alias = "PAD_PB00")]
pub type PadPb00 = crate::Reg<pad_pb00::PAD_PB00rs>;
///
pub mod pad_pb00;
///PAD_PB01 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pb01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pb01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pb01`]
///module
#[doc(alias = "PAD_PB01")]
pub type PadPb01 = crate::Reg<pad_pb01::PAD_PB01rs>;
///
pub mod pad_pb01;
///PAD_PB02 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pb02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pb02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pb02`]
///module
#[doc(alias = "PAD_PB02")]
pub type PadPb02 = crate::Reg<pad_pb02::PAD_PB02rs>;
///
pub mod pad_pb02;
///PAD_PB03 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pb03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pb03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pad_pb03`]
///module
#[doc(alias = "PAD_PB03")]
pub type PadPb03 = crate::Reg<pad_pb03::PAD_PB03rs>;
///
pub mod pad_pb03;
