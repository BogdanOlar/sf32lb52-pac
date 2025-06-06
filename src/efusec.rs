#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: Cr,
    timr: Timr,
    sr: Sr,
    _reserved3: [u8; 0x04],
    pgm_data0: PgmData0,
    pgm_data1: PgmData1,
    pgm_data2: PgmData2,
    pgm_data3: PgmData3,
    pgm_data4: PgmData4,
    pgm_data5: PgmData5,
    pgm_data6: PgmData6,
    pgm_data7: PgmData7,
    bank0_data0: Bank0Data0,
    bank0_data1: Bank0Data1,
    bank0_data2: Bank0Data2,
    bank0_data3: Bank0Data3,
    bank0_data4: Bank0Data4,
    bank0_data5: Bank0Data5,
    bank0_data6: Bank0Data6,
    bank0_data7: Bank0Data7,
    bank1_data0: Bank1Data0,
    bank1_data1: Bank1Data1,
    bank1_data2: Bank1Data2,
    bank1_data3: Bank1Data3,
    bank1_data4: Bank1Data4,
    bank1_data5: Bank1Data5,
    bank1_data6: Bank1Data6,
    bank1_data7: Bank1Data7,
    bank2_data0: Bank2Data0,
    bank2_data1: Bank2Data1,
    bank2_data2: Bank2Data2,
    bank2_data3: Bank2Data3,
    bank2_data4: Bank2Data4,
    bank2_data5: Bank2Data5,
    bank2_data6: Bank2Data6,
    bank2_data7: Bank2Data7,
    bank3_data0: Bank3Data0,
    bank3_data1: Bank3Data1,
    bank3_data2: Bank3Data2,
    bank3_data3: Bank3Data3,
    bank3_data4: Bank3Data4,
    bank3_data5: Bank3Data5,
    bank3_data6: Bank3Data6,
    bank3_data7: Bank3Data7,
    anacr: Anacr,
    db_sel: DbSel,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x04 - Timer Register
    #[inline(always)]
    pub const fn timr(&self) -> &Timr {
        &self.timr
    }
    ///0x08 - Status Register
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    ///0x10 - Program Data0
    #[inline(always)]
    pub const fn pgm_data0(&self) -> &PgmData0 {
        &self.pgm_data0
    }
    ///0x14 - Program Data1
    #[inline(always)]
    pub const fn pgm_data1(&self) -> &PgmData1 {
        &self.pgm_data1
    }
    ///0x18 - Program Data2
    #[inline(always)]
    pub const fn pgm_data2(&self) -> &PgmData2 {
        &self.pgm_data2
    }
    ///0x1c - Program Data3
    #[inline(always)]
    pub const fn pgm_data3(&self) -> &PgmData3 {
        &self.pgm_data3
    }
    ///0x20 - Program Data4
    #[inline(always)]
    pub const fn pgm_data4(&self) -> &PgmData4 {
        &self.pgm_data4
    }
    ///0x24 - Program Data5
    #[inline(always)]
    pub const fn pgm_data5(&self) -> &PgmData5 {
        &self.pgm_data5
    }
    ///0x28 - Program Data6
    #[inline(always)]
    pub const fn pgm_data6(&self) -> &PgmData6 {
        &self.pgm_data6
    }
    ///0x2c - Program Data7
    #[inline(always)]
    pub const fn pgm_data7(&self) -> &PgmData7 {
        &self.pgm_data7
    }
    ///0x30 - Bank0 Data0
    #[inline(always)]
    pub const fn bank0_data0(&self) -> &Bank0Data0 {
        &self.bank0_data0
    }
    ///0x34 - Bank0 Data1
    #[inline(always)]
    pub const fn bank0_data1(&self) -> &Bank0Data1 {
        &self.bank0_data1
    }
    ///0x38 - Bank0 Data2
    #[inline(always)]
    pub const fn bank0_data2(&self) -> &Bank0Data2 {
        &self.bank0_data2
    }
    ///0x3c - Bank0 Data3
    #[inline(always)]
    pub const fn bank0_data3(&self) -> &Bank0Data3 {
        &self.bank0_data3
    }
    ///0x40 - Bank0 Data4
    #[inline(always)]
    pub const fn bank0_data4(&self) -> &Bank0Data4 {
        &self.bank0_data4
    }
    ///0x44 - Bank0 Data5
    #[inline(always)]
    pub const fn bank0_data5(&self) -> &Bank0Data5 {
        &self.bank0_data5
    }
    ///0x48 - Bank0 Data6
    #[inline(always)]
    pub const fn bank0_data6(&self) -> &Bank0Data6 {
        &self.bank0_data6
    }
    ///0x4c - Bank0 Data7
    #[inline(always)]
    pub const fn bank0_data7(&self) -> &Bank0Data7 {
        &self.bank0_data7
    }
    ///0x50 - Bank1 Data0
    #[inline(always)]
    pub const fn bank1_data0(&self) -> &Bank1Data0 {
        &self.bank1_data0
    }
    ///0x54 - Bank1 Data1
    #[inline(always)]
    pub const fn bank1_data1(&self) -> &Bank1Data1 {
        &self.bank1_data1
    }
    ///0x58 - Bank1 Data2
    #[inline(always)]
    pub const fn bank1_data2(&self) -> &Bank1Data2 {
        &self.bank1_data2
    }
    ///0x5c - Bank1 Data3
    #[inline(always)]
    pub const fn bank1_data3(&self) -> &Bank1Data3 {
        &self.bank1_data3
    }
    ///0x60 - Bank1 Data4
    #[inline(always)]
    pub const fn bank1_data4(&self) -> &Bank1Data4 {
        &self.bank1_data4
    }
    ///0x64 - Bank1 Data5
    #[inline(always)]
    pub const fn bank1_data5(&self) -> &Bank1Data5 {
        &self.bank1_data5
    }
    ///0x68 - Bank1 Data6
    #[inline(always)]
    pub const fn bank1_data6(&self) -> &Bank1Data6 {
        &self.bank1_data6
    }
    ///0x6c - Bank1 Data7
    #[inline(always)]
    pub const fn bank1_data7(&self) -> &Bank1Data7 {
        &self.bank1_data7
    }
    ///0x70 - Bank2 Data0
    #[inline(always)]
    pub const fn bank2_data0(&self) -> &Bank2Data0 {
        &self.bank2_data0
    }
    ///0x74 - Bank2 Data1
    #[inline(always)]
    pub const fn bank2_data1(&self) -> &Bank2Data1 {
        &self.bank2_data1
    }
    ///0x78 - Bank2 Data2
    #[inline(always)]
    pub const fn bank2_data2(&self) -> &Bank2Data2 {
        &self.bank2_data2
    }
    ///0x7c - Bank2 Data3
    #[inline(always)]
    pub const fn bank2_data3(&self) -> &Bank2Data3 {
        &self.bank2_data3
    }
    ///0x80 - Bank2 Data4
    #[inline(always)]
    pub const fn bank2_data4(&self) -> &Bank2Data4 {
        &self.bank2_data4
    }
    ///0x84 - Bank2 Data5
    #[inline(always)]
    pub const fn bank2_data5(&self) -> &Bank2Data5 {
        &self.bank2_data5
    }
    ///0x88 - Bank2 Data6
    #[inline(always)]
    pub const fn bank2_data6(&self) -> &Bank2Data6 {
        &self.bank2_data6
    }
    ///0x8c - Bank2 Data7
    #[inline(always)]
    pub const fn bank2_data7(&self) -> &Bank2Data7 {
        &self.bank2_data7
    }
    ///0x90 - Bank3 Data0
    #[inline(always)]
    pub const fn bank3_data0(&self) -> &Bank3Data0 {
        &self.bank3_data0
    }
    ///0x94 - Bank3 Data1
    #[inline(always)]
    pub const fn bank3_data1(&self) -> &Bank3Data1 {
        &self.bank3_data1
    }
    ///0x98 - Bank3 Data2
    #[inline(always)]
    pub const fn bank3_data2(&self) -> &Bank3Data2 {
        &self.bank3_data2
    }
    ///0x9c - Bank3 Data3
    #[inline(always)]
    pub const fn bank3_data3(&self) -> &Bank3Data3 {
        &self.bank3_data3
    }
    ///0xa0 - Bank3 Data4
    #[inline(always)]
    pub const fn bank3_data4(&self) -> &Bank3Data4 {
        &self.bank3_data4
    }
    ///0xa4 - Bank3 Data5
    #[inline(always)]
    pub const fn bank3_data5(&self) -> &Bank3Data5 {
        &self.bank3_data5
    }
    ///0xa8 - Bank3 Data6
    #[inline(always)]
    pub const fn bank3_data6(&self) -> &Bank3Data6 {
        &self.bank3_data6
    }
    ///0xac - Bank3 Data7
    #[inline(always)]
    pub const fn bank3_data7(&self) -> &Bank3Data7 {
        &self.bank3_data7
    }
    ///0xb0 - Bank3 Data7
    #[inline(always)]
    pub const fn anacr(&self) -> &Anacr {
        &self.anacr
    }
    ///0xb4 - debug signal select
    #[inline(always)]
    pub const fn db_sel(&self) -> &DbSel {
        &self.db_sel
    }
}
///CR (rw) register accessor: Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cr`]
///module
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CRrs>;
///Control Register
pub mod cr;
///TIMR (rw) register accessor: Timer Register
///
///You can [`read`](crate::Reg::read) this register and get [`timr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@timr`]
///module
#[doc(alias = "TIMR")]
pub type Timr = crate::Reg<timr::TIMRrs>;
///Timer Register
pub mod timr;
///SR (rw) register accessor: Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@sr`]
///module
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SRrs>;
///Status Register
pub mod sr;
///PGM_DATA0 (rw) register accessor: Program Data0
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data0`]
///module
#[doc(alias = "PGM_DATA0")]
pub type PgmData0 = crate::Reg<pgm_data0::PGM_DATA0rs>;
///Program Data0
pub mod pgm_data0;
///PGM_DATA1 (rw) register accessor: Program Data1
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data1`]
///module
#[doc(alias = "PGM_DATA1")]
pub type PgmData1 = crate::Reg<pgm_data1::PGM_DATA1rs>;
///Program Data1
pub mod pgm_data1;
///PGM_DATA2 (rw) register accessor: Program Data2
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data2`]
///module
#[doc(alias = "PGM_DATA2")]
pub type PgmData2 = crate::Reg<pgm_data2::PGM_DATA2rs>;
///Program Data2
pub mod pgm_data2;
///PGM_DATA3 (rw) register accessor: Program Data3
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data3`]
///module
#[doc(alias = "PGM_DATA3")]
pub type PgmData3 = crate::Reg<pgm_data3::PGM_DATA3rs>;
///Program Data3
pub mod pgm_data3;
///PGM_DATA4 (rw) register accessor: Program Data4
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data4`]
///module
#[doc(alias = "PGM_DATA4")]
pub type PgmData4 = crate::Reg<pgm_data4::PGM_DATA4rs>;
///Program Data4
pub mod pgm_data4;
///PGM_DATA5 (rw) register accessor: Program Data5
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data5`]
///module
#[doc(alias = "PGM_DATA5")]
pub type PgmData5 = crate::Reg<pgm_data5::PGM_DATA5rs>;
///Program Data5
pub mod pgm_data5;
///PGM_DATA6 (rw) register accessor: Program Data6
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data6`]
///module
#[doc(alias = "PGM_DATA6")]
pub type PgmData6 = crate::Reg<pgm_data6::PGM_DATA6rs>;
///Program Data6
pub mod pgm_data6;
///PGM_DATA7 (rw) register accessor: Program Data7
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pgm_data7`]
///module
#[doc(alias = "PGM_DATA7")]
pub type PgmData7 = crate::Reg<pgm_data7::PGM_DATA7rs>;
///Program Data7
pub mod pgm_data7;
///BANK0_DATA0 (rw) register accessor: Bank0 Data0
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data0`]
///module
#[doc(alias = "BANK0_DATA0")]
pub type Bank0Data0 = crate::Reg<bank0_data0::BANK0_DATA0rs>;
///Bank0 Data0
pub mod bank0_data0;
///BANK0_DATA1 (rw) register accessor: Bank0 Data1
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data1`]
///module
#[doc(alias = "BANK0_DATA1")]
pub type Bank0Data1 = crate::Reg<bank0_data1::BANK0_DATA1rs>;
///Bank0 Data1
pub mod bank0_data1;
///BANK0_DATA2 (rw) register accessor: Bank0 Data2
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data2`]
///module
#[doc(alias = "BANK0_DATA2")]
pub type Bank0Data2 = crate::Reg<bank0_data2::BANK0_DATA2rs>;
///Bank0 Data2
pub mod bank0_data2;
///BANK0_DATA3 (rw) register accessor: Bank0 Data3
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data3`]
///module
#[doc(alias = "BANK0_DATA3")]
pub type Bank0Data3 = crate::Reg<bank0_data3::BANK0_DATA3rs>;
///Bank0 Data3
pub mod bank0_data3;
///BANK0_DATA4 (rw) register accessor: Bank0 Data4
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data4`]
///module
#[doc(alias = "BANK0_DATA4")]
pub type Bank0Data4 = crate::Reg<bank0_data4::BANK0_DATA4rs>;
///Bank0 Data4
pub mod bank0_data4;
///BANK0_DATA5 (rw) register accessor: Bank0 Data5
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data5`]
///module
#[doc(alias = "BANK0_DATA5")]
pub type Bank0Data5 = crate::Reg<bank0_data5::BANK0_DATA5rs>;
///Bank0 Data5
pub mod bank0_data5;
///BANK0_DATA6 (rw) register accessor: Bank0 Data6
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data6`]
///module
#[doc(alias = "BANK0_DATA6")]
pub type Bank0Data6 = crate::Reg<bank0_data6::BANK0_DATA6rs>;
///Bank0 Data6
pub mod bank0_data6;
///BANK0_DATA7 (rw) register accessor: Bank0 Data7
///
///You can [`read`](crate::Reg::read) this register and get [`bank0_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank0_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank0_data7`]
///module
#[doc(alias = "BANK0_DATA7")]
pub type Bank0Data7 = crate::Reg<bank0_data7::BANK0_DATA7rs>;
///Bank0 Data7
pub mod bank0_data7;
///BANK1_DATA0 (rw) register accessor: Bank1 Data0
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data0`]
///module
#[doc(alias = "BANK1_DATA0")]
pub type Bank1Data0 = crate::Reg<bank1_data0::BANK1_DATA0rs>;
///Bank1 Data0
pub mod bank1_data0;
///BANK1_DATA1 (rw) register accessor: Bank1 Data1
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data1`]
///module
#[doc(alias = "BANK1_DATA1")]
pub type Bank1Data1 = crate::Reg<bank1_data1::BANK1_DATA1rs>;
///Bank1 Data1
pub mod bank1_data1;
///BANK1_DATA2 (rw) register accessor: Bank1 Data2
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data2`]
///module
#[doc(alias = "BANK1_DATA2")]
pub type Bank1Data2 = crate::Reg<bank1_data2::BANK1_DATA2rs>;
///Bank1 Data2
pub mod bank1_data2;
///BANK1_DATA3 (rw) register accessor: Bank1 Data3
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data3`]
///module
#[doc(alias = "BANK1_DATA3")]
pub type Bank1Data3 = crate::Reg<bank1_data3::BANK1_DATA3rs>;
///Bank1 Data3
pub mod bank1_data3;
///BANK1_DATA4 (rw) register accessor: Bank1 Data4
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data4`]
///module
#[doc(alias = "BANK1_DATA4")]
pub type Bank1Data4 = crate::Reg<bank1_data4::BANK1_DATA4rs>;
///Bank1 Data4
pub mod bank1_data4;
///BANK1_DATA5 (rw) register accessor: Bank1 Data5
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data5`]
///module
#[doc(alias = "BANK1_DATA5")]
pub type Bank1Data5 = crate::Reg<bank1_data5::BANK1_DATA5rs>;
///Bank1 Data5
pub mod bank1_data5;
///BANK1_DATA6 (rw) register accessor: Bank1 Data6
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data6`]
///module
#[doc(alias = "BANK1_DATA6")]
pub type Bank1Data6 = crate::Reg<bank1_data6::BANK1_DATA6rs>;
///Bank1 Data6
pub mod bank1_data6;
///BANK1_DATA7 (rw) register accessor: Bank1 Data7
///
///You can [`read`](crate::Reg::read) this register and get [`bank1_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank1_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank1_data7`]
///module
#[doc(alias = "BANK1_DATA7")]
pub type Bank1Data7 = crate::Reg<bank1_data7::BANK1_DATA7rs>;
///Bank1 Data7
pub mod bank1_data7;
///BANK2_DATA0 (rw) register accessor: Bank2 Data0
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data0`]
///module
#[doc(alias = "BANK2_DATA0")]
pub type Bank2Data0 = crate::Reg<bank2_data0::BANK2_DATA0rs>;
///Bank2 Data0
pub mod bank2_data0;
///BANK2_DATA1 (rw) register accessor: Bank2 Data1
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data1`]
///module
#[doc(alias = "BANK2_DATA1")]
pub type Bank2Data1 = crate::Reg<bank2_data1::BANK2_DATA1rs>;
///Bank2 Data1
pub mod bank2_data1;
///BANK2_DATA2 (rw) register accessor: Bank2 Data2
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data2`]
///module
#[doc(alias = "BANK2_DATA2")]
pub type Bank2Data2 = crate::Reg<bank2_data2::BANK2_DATA2rs>;
///Bank2 Data2
pub mod bank2_data2;
///BANK2_DATA3 (rw) register accessor: Bank2 Data3
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data3`]
///module
#[doc(alias = "BANK2_DATA3")]
pub type Bank2Data3 = crate::Reg<bank2_data3::BANK2_DATA3rs>;
///Bank2 Data3
pub mod bank2_data3;
///BANK2_DATA4 (rw) register accessor: Bank2 Data4
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data4`]
///module
#[doc(alias = "BANK2_DATA4")]
pub type Bank2Data4 = crate::Reg<bank2_data4::BANK2_DATA4rs>;
///Bank2 Data4
pub mod bank2_data4;
///BANK2_DATA5 (rw) register accessor: Bank2 Data5
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data5`]
///module
#[doc(alias = "BANK2_DATA5")]
pub type Bank2Data5 = crate::Reg<bank2_data5::BANK2_DATA5rs>;
///Bank2 Data5
pub mod bank2_data5;
///BANK2_DATA6 (rw) register accessor: Bank2 Data6
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data6`]
///module
#[doc(alias = "BANK2_DATA6")]
pub type Bank2Data6 = crate::Reg<bank2_data6::BANK2_DATA6rs>;
///Bank2 Data6
pub mod bank2_data6;
///BANK2_DATA7 (rw) register accessor: Bank2 Data7
///
///You can [`read`](crate::Reg::read) this register and get [`bank2_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank2_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank2_data7`]
///module
#[doc(alias = "BANK2_DATA7")]
pub type Bank2Data7 = crate::Reg<bank2_data7::BANK2_DATA7rs>;
///Bank2 Data7
pub mod bank2_data7;
///BANK3_DATA0 (rw) register accessor: Bank3 Data0
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data0`]
///module
#[doc(alias = "BANK3_DATA0")]
pub type Bank3Data0 = crate::Reg<bank3_data0::BANK3_DATA0rs>;
///Bank3 Data0
pub mod bank3_data0;
///BANK3_DATA1 (rw) register accessor: Bank3 Data1
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data1`]
///module
#[doc(alias = "BANK3_DATA1")]
pub type Bank3Data1 = crate::Reg<bank3_data1::BANK3_DATA1rs>;
///Bank3 Data1
pub mod bank3_data1;
///BANK3_DATA2 (rw) register accessor: Bank3 Data2
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data2`]
///module
#[doc(alias = "BANK3_DATA2")]
pub type Bank3Data2 = crate::Reg<bank3_data2::BANK3_DATA2rs>;
///Bank3 Data2
pub mod bank3_data2;
///BANK3_DATA3 (rw) register accessor: Bank3 Data3
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data3`]
///module
#[doc(alias = "BANK3_DATA3")]
pub type Bank3Data3 = crate::Reg<bank3_data3::BANK3_DATA3rs>;
///Bank3 Data3
pub mod bank3_data3;
///BANK3_DATA4 (rw) register accessor: Bank3 Data4
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data4`]
///module
#[doc(alias = "BANK3_DATA4")]
pub type Bank3Data4 = crate::Reg<bank3_data4::BANK3_DATA4rs>;
///Bank3 Data4
pub mod bank3_data4;
///BANK3_DATA5 (rw) register accessor: Bank3 Data5
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data5`]
///module
#[doc(alias = "BANK3_DATA5")]
pub type Bank3Data5 = crate::Reg<bank3_data5::BANK3_DATA5rs>;
///Bank3 Data5
pub mod bank3_data5;
///BANK3_DATA6 (rw) register accessor: Bank3 Data6
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data6`]
///module
#[doc(alias = "BANK3_DATA6")]
pub type Bank3Data6 = crate::Reg<bank3_data6::BANK3_DATA6rs>;
///Bank3 Data6
pub mod bank3_data6;
///BANK3_DATA7 (rw) register accessor: Bank3 Data7
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bank3_data7`]
///module
#[doc(alias = "BANK3_DATA7")]
pub type Bank3Data7 = crate::Reg<bank3_data7::BANK3_DATA7rs>;
///Bank3 Data7
pub mod bank3_data7;
///ANACR (rw) register accessor: Bank3 Data7
///
///You can [`read`](crate::Reg::read) this register and get [`anacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@anacr`]
///module
#[doc(alias = "ANACR")]
pub type Anacr = crate::Reg<anacr::ANACRrs>;
///Bank3 Data7
pub mod anacr;
///DB_SEL (rw) register accessor: debug signal select
///
///You can [`read`](crate::Reg::read) this register and get [`db_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_sel`]
///module
#[doc(alias = "DB_SEL")]
pub type DbSel = crate::Reg<db_sel::DB_SELrs>;
///debug signal select
pub mod db_sel;
