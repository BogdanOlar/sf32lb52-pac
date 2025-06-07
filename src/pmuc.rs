#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: Cr,
    wer: Wer,
    wsr: Wsr,
    wcr: Wcr,
    vrtc_cr: VrtcCr,
    vret_cr: VretCr,
    lrc10_cr: Lrc10Cr,
    lrc32_cr: Lrc32Cr,
    lxt_cr: LxtCr,
    aon_bg: AonBg,
    aon_ldo: AonLdo,
    buck_cr1: BuckCr1,
    buck_cr2: BuckCr2,
    chg_cr1: ChgCr1,
    chg_cr2: ChgCr2,
    chg_cr3: ChgCr3,
    chg_cr4: ChgCr4,
    chg_cr5: ChgCr5,
    chg_sr: ChgSr,
    hpsys_ldo: HpsysLdo,
    lpsys_ldo: LpsysLdo,
    hpsys_swr: HpsysSwr,
    lpsys_swr: LpsysSwr,
    peri_ldo: PeriLdo,
    pmu_tr: PmuTr,
    _reserved25: [u8; 0x04],
    hxt_cr1: HxtCr1,
    hxt_cr2: HxtCr2,
    hxt_cr3: HxtCr3,
    hrc_cr: HrcCr,
    dbl96_cr: Dbl96Cr,
    dbl96_calr: Dbl96Calr,
    cau_bgr: CauBgr,
    cau_tr: CauTr,
    _reserved33: [u8; 0x04],
    wkup_cnt: WkupCnt,
    pwrkey_cnt: PwrkeyCnt,
    hpsys_vout: HpsysVout,
    lpsys_vout: LpsysVout,
    buck_vout: BuckVout,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x04 - Wakeup Enable register
    #[inline(always)]
    pub const fn wer(&self) -> &Wer {
        &self.wer
    }
    ///0x08 - Wakeup Status register
    #[inline(always)]
    pub const fn wsr(&self) -> &Wsr {
        &self.wsr
    }
    ///0x0c - Wakeup Clear register
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    ///0x10 - VRTC Control Register
    #[inline(always)]
    pub const fn vrtc_cr(&self) -> &VrtcCr {
        &self.vrtc_cr
    }
    ///0x14 - VRET Control Register
    #[inline(always)]
    pub const fn vret_cr(&self) -> &VretCr {
        &self.vret_cr
    }
    ///0x18 - RC10K Control Register
    #[inline(always)]
    pub const fn lrc10_cr(&self) -> &Lrc10Cr {
        &self.lrc10_cr
    }
    ///0x1c - RC32K Control Register
    #[inline(always)]
    pub const fn lrc32_cr(&self) -> &Lrc32Cr {
        &self.lrc32_cr
    }
    ///0x20 - XTAL32K Control Register
    #[inline(always)]
    pub const fn lxt_cr(&self) -> &LxtCr {
        &self.lxt_cr
    }
    ///0x24 - AON Bandgap Register
    #[inline(always)]
    pub const fn aon_bg(&self) -> &AonBg {
        &self.aon_bg
    }
    ///0x28 - AON LDO Register
    #[inline(always)]
    pub const fn aon_ldo(&self) -> &AonLdo {
        &self.aon_ldo
    }
    ///0x2c - BUCK Control Register 1
    #[inline(always)]
    pub const fn buck_cr1(&self) -> &BuckCr1 {
        &self.buck_cr1
    }
    ///0x30 - BUCK Control Register 2
    #[inline(always)]
    pub const fn buck_cr2(&self) -> &BuckCr2 {
        &self.buck_cr2
    }
    ///0x34 - Charger Control Register 1
    #[inline(always)]
    pub const fn chg_cr1(&self) -> &ChgCr1 {
        &self.chg_cr1
    }
    ///0x38 - Charger Control Register 2
    #[inline(always)]
    pub const fn chg_cr2(&self) -> &ChgCr2 {
        &self.chg_cr2
    }
    ///0x3c - Charger Control Register 3
    #[inline(always)]
    pub const fn chg_cr3(&self) -> &ChgCr3 {
        &self.chg_cr3
    }
    ///0x40 - Charger Control Register 4
    #[inline(always)]
    pub const fn chg_cr4(&self) -> &ChgCr4 {
        &self.chg_cr4
    }
    ///0x44 - Charger Control Register 5
    #[inline(always)]
    pub const fn chg_cr5(&self) -> &ChgCr5 {
        &self.chg_cr5
    }
    ///0x48 - Charger Status Register
    #[inline(always)]
    pub const fn chg_sr(&self) -> &ChgSr {
        &self.chg_sr
    }
    ///0x4c - HPSYS LDO Control Register
    #[inline(always)]
    pub const fn hpsys_ldo(&self) -> &HpsysLdo {
        &self.hpsys_ldo
    }
    ///0x50 - LPSYS LDO Control Register
    #[inline(always)]
    pub const fn lpsys_ldo(&self) -> &LpsysLdo {
        &self.lpsys_ldo
    }
    ///0x54 - HPSYS Switch Register
    #[inline(always)]
    pub const fn hpsys_swr(&self) -> &HpsysSwr {
        &self.hpsys_swr
    }
    ///0x58 - LPSYS Switch Register
    #[inline(always)]
    pub const fn lpsys_swr(&self) -> &LpsysSwr {
        &self.lpsys_swr
    }
    ///0x5c - Peripherals LDO
    #[inline(always)]
    pub const fn peri_ldo(&self) -> &PeriLdo {
        &self.peri_ldo
    }
    ///0x60 - PMU Test Register
    #[inline(always)]
    pub const fn pmu_tr(&self) -> &PmuTr {
        &self.pmu_tr
    }
    ///0x68 - HXT48 Control Register 1
    #[inline(always)]
    pub const fn hxt_cr1(&self) -> &HxtCr1 {
        &self.hxt_cr1
    }
    ///0x6c - HXT48 Control Register 2
    #[inline(always)]
    pub const fn hxt_cr2(&self) -> &HxtCr2 {
        &self.hxt_cr2
    }
    ///0x70 - HXT48 Control Register 3
    #[inline(always)]
    pub const fn hxt_cr3(&self) -> &HxtCr3 {
        &self.hxt_cr3
    }
    ///0x74 - HRC48 Control Register
    #[inline(always)]
    pub const fn hrc_cr(&self) -> &HrcCr {
        &self.hrc_cr
    }
    ///0x78 - DBL96 Control Register
    #[inline(always)]
    pub const fn dbl96_cr(&self) -> &Dbl96Cr {
        &self.dbl96_cr
    }
    ///0x7c - DBL96 Calibration Register
    #[inline(always)]
    pub const fn dbl96_calr(&self) -> &Dbl96Calr {
        &self.dbl96_calr
    }
    ///0x80 - CAU Bandgap Register
    #[inline(always)]
    pub const fn cau_bgr(&self) -> &CauBgr {
        &self.cau_bgr
    }
    ///0x84 - CAU Test Register
    #[inline(always)]
    pub const fn cau_tr(&self) -> &CauTr {
        &self.cau_tr
    }
    ///0x8c - Wakeup Count Register
    #[inline(always)]
    pub const fn wkup_cnt(&self) -> &WkupCnt {
        &self.wkup_cnt
    }
    ///0x90 - PowerKey Count Register
    #[inline(always)]
    pub const fn pwrkey_cnt(&self) -> &PwrkeyCnt {
        &self.pwrkey_cnt
    }
    ///0x94 -
    #[inline(always)]
    pub const fn hpsys_vout(&self) -> &HpsysVout {
        &self.hpsys_vout
    }
    ///0x98 -
    #[inline(always)]
    pub const fn lpsys_vout(&self) -> &LpsysVout {
        &self.lpsys_vout
    }
    ///0x9c -
    #[inline(always)]
    pub const fn buck_vout(&self) -> &BuckVout {
        &self.buck_vout
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
///WER (rw) register accessor: Wakeup Enable register
///
///You can [`read`](crate::Reg::read) this register and get [`wer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wer`]
///module
#[doc(alias = "WER")]
pub type Wer = crate::Reg<wer::WERrs>;
///Wakeup Enable register
pub mod wer;
///WSR (rw) register accessor: Wakeup Status register
///
///You can [`read`](crate::Reg::read) this register and get [`wsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wsr`]
///module
#[doc(alias = "WSR")]
pub type Wsr = crate::Reg<wsr::WSRrs>;
///Wakeup Status register
pub mod wsr;
///WCR (rw) register accessor: Wakeup Clear register
///
///You can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wcr`]
///module
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WCRrs>;
///Wakeup Clear register
pub mod wcr;
///VRTC_CR (rw) register accessor: VRTC Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`vrtc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrtc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vrtc_cr`]
///module
#[doc(alias = "VRTC_CR")]
pub type VrtcCr = crate::Reg<vrtc_cr::VRTC_CRrs>;
///VRTC Control Register
pub mod vrtc_cr;
///VRET_CR (rw) register accessor: VRET Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`vret_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vret_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vret_cr`]
///module
#[doc(alias = "VRET_CR")]
pub type VretCr = crate::Reg<vret_cr::VRET_CRrs>;
///VRET Control Register
pub mod vret_cr;
///LRC10_CR (rw) register accessor: RC10K Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lrc10_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrc10_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lrc10_cr`]
///module
#[doc(alias = "LRC10_CR")]
pub type Lrc10Cr = crate::Reg<lrc10_cr::LRC10_CRrs>;
///RC10K Control Register
pub mod lrc10_cr;
///LRC32_CR (rw) register accessor: RC32K Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lrc32_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrc32_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lrc32_cr`]
///module
#[doc(alias = "LRC32_CR")]
pub type Lrc32Cr = crate::Reg<lrc32_cr::LRC32_CRrs>;
///RC32K Control Register
pub mod lrc32_cr;
///LXT_CR (rw) register accessor: XTAL32K Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lxt_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lxt_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lxt_cr`]
///module
#[doc(alias = "LXT_CR")]
pub type LxtCr = crate::Reg<lxt_cr::LXT_CRrs>;
///XTAL32K Control Register
pub mod lxt_cr;
///AON_BG (rw) register accessor: AON Bandgap Register
///
///You can [`read`](crate::Reg::read) this register and get [`aon_bg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_bg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aon_bg`]
///module
#[doc(alias = "AON_BG")]
pub type AonBg = crate::Reg<aon_bg::AON_BGrs>;
///AON Bandgap Register
pub mod aon_bg;
///AON_LDO (rw) register accessor: AON LDO Register
///
///You can [`read`](crate::Reg::read) this register and get [`aon_ldo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_ldo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aon_ldo`]
///module
#[doc(alias = "AON_LDO")]
pub type AonLdo = crate::Reg<aon_ldo::AON_LDOrs>;
///AON LDO Register
pub mod aon_ldo;
///BUCK_CR1 (rw) register accessor: BUCK Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`buck_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@buck_cr1`]
///module
#[doc(alias = "BUCK_CR1")]
pub type BuckCr1 = crate::Reg<buck_cr1::BUCK_CR1rs>;
///BUCK Control Register 1
pub mod buck_cr1;
///BUCK_CR2 (rw) register accessor: BUCK Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`buck_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@buck_cr2`]
///module
#[doc(alias = "BUCK_CR2")]
pub type BuckCr2 = crate::Reg<buck_cr2::BUCK_CR2rs>;
///BUCK Control Register 2
pub mod buck_cr2;
///CHG_CR1 (rw) register accessor: Charger Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chg_cr1`]
///module
#[doc(alias = "CHG_CR1")]
pub type ChgCr1 = crate::Reg<chg_cr1::CHG_CR1rs>;
///Charger Control Register 1
pub mod chg_cr1;
///CHG_CR2 (rw) register accessor: Charger Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chg_cr2`]
///module
#[doc(alias = "CHG_CR2")]
pub type ChgCr2 = crate::Reg<chg_cr2::CHG_CR2rs>;
///Charger Control Register 2
pub mod chg_cr2;
///CHG_CR3 (rw) register accessor: Charger Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chg_cr3`]
///module
#[doc(alias = "CHG_CR3")]
pub type ChgCr3 = crate::Reg<chg_cr3::CHG_CR3rs>;
///Charger Control Register 3
pub mod chg_cr3;
///CHG_CR4 (rw) register accessor: Charger Control Register 4
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chg_cr4`]
///module
#[doc(alias = "CHG_CR4")]
pub type ChgCr4 = crate::Reg<chg_cr4::CHG_CR4rs>;
///Charger Control Register 4
pub mod chg_cr4;
///CHG_CR5 (rw) register accessor: Charger Control Register 5
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chg_cr5`]
///module
#[doc(alias = "CHG_CR5")]
pub type ChgCr5 = crate::Reg<chg_cr5::CHG_CR5rs>;
///Charger Control Register 5
pub mod chg_cr5;
///CHG_SR (rw) register accessor: Charger Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`chg_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@chg_sr`]
///module
#[doc(alias = "CHG_SR")]
pub type ChgSr = crate::Reg<chg_sr::CHG_SRrs>;
///Charger Status Register
pub mod chg_sr;
///HPSYS_LDO (rw) register accessor: HPSYS LDO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`hpsys_ldo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpsys_ldo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hpsys_ldo`]
///module
#[doc(alias = "HPSYS_LDO")]
pub type HpsysLdo = crate::Reg<hpsys_ldo::HPSYS_LDOrs>;
///HPSYS LDO Control Register
pub mod hpsys_ldo;
///LPSYS_LDO (rw) register accessor: LPSYS LDO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lpsys_ldo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsys_ldo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lpsys_ldo`]
///module
#[doc(alias = "LPSYS_LDO")]
pub type LpsysLdo = crate::Reg<lpsys_ldo::LPSYS_LDOrs>;
///LPSYS LDO Control Register
pub mod lpsys_ldo;
///HPSYS_SWR (rw) register accessor: HPSYS Switch Register
///
///You can [`read`](crate::Reg::read) this register and get [`hpsys_swr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpsys_swr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hpsys_swr`]
///module
#[doc(alias = "HPSYS_SWR")]
pub type HpsysSwr = crate::Reg<hpsys_swr::HPSYS_SWRrs>;
///HPSYS Switch Register
pub mod hpsys_swr;
///LPSYS_SWR (rw) register accessor: LPSYS Switch Register
///
///You can [`read`](crate::Reg::read) this register and get [`lpsys_swr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsys_swr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lpsys_swr`]
///module
#[doc(alias = "LPSYS_SWR")]
pub type LpsysSwr = crate::Reg<lpsys_swr::LPSYS_SWRrs>;
///LPSYS Switch Register
pub mod lpsys_swr;
///PERI_LDO (rw) register accessor: Peripherals LDO
///
///You can [`read`](crate::Reg::read) this register and get [`peri_ldo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_ldo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@peri_ldo`]
///module
#[doc(alias = "PERI_LDO")]
pub type PeriLdo = crate::Reg<peri_ldo::PERI_LDOrs>;
///Peripherals LDO
pub mod peri_ldo;
///PMU_TR (rw) register accessor: PMU Test Register
///
///You can [`read`](crate::Reg::read) this register and get [`pmu_tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pmu_tr`]
///module
#[doc(alias = "PMU_TR")]
pub type PmuTr = crate::Reg<pmu_tr::PMU_TRrs>;
///PMU Test Register
pub mod pmu_tr;
///HXT_CR1 (rw) register accessor: HXT48 Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`hxt_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hxt_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hxt_cr1`]
///module
#[doc(alias = "HXT_CR1")]
pub type HxtCr1 = crate::Reg<hxt_cr1::HXT_CR1rs>;
///HXT48 Control Register 1
pub mod hxt_cr1;
///HXT_CR2 (rw) register accessor: HXT48 Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`hxt_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hxt_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hxt_cr2`]
///module
#[doc(alias = "HXT_CR2")]
pub type HxtCr2 = crate::Reg<hxt_cr2::HXT_CR2rs>;
///HXT48 Control Register 2
pub mod hxt_cr2;
///HXT_CR3 (rw) register accessor: HXT48 Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`hxt_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hxt_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hxt_cr3`]
///module
#[doc(alias = "HXT_CR3")]
pub type HxtCr3 = crate::Reg<hxt_cr3::HXT_CR3rs>;
///HXT48 Control Register 3
pub mod hxt_cr3;
///HRC_CR (rw) register accessor: HRC48 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`hrc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hrc_cr`]
///module
#[doc(alias = "HRC_CR")]
pub type HrcCr = crate::Reg<hrc_cr::HRC_CRrs>;
///HRC48 Control Register
pub mod hrc_cr;
///DBL96_CR (rw) register accessor: DBL96 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbl96_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbl96_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbl96_cr`]
///module
#[doc(alias = "DBL96_CR")]
pub type Dbl96Cr = crate::Reg<dbl96_cr::DBL96_CRrs>;
///DBL96 Control Register
pub mod dbl96_cr;
///DBL96_CALR (rw) register accessor: DBL96 Calibration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbl96_calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbl96_calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dbl96_calr`]
///module
#[doc(alias = "DBL96_CALR")]
pub type Dbl96Calr = crate::Reg<dbl96_calr::DBL96_CALRrs>;
///DBL96 Calibration Register
pub mod dbl96_calr;
///CAU_BGR (rw) register accessor: CAU Bandgap Register
///
///You can [`read`](crate::Reg::read) this register and get [`cau_bgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau_bgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cau_bgr`]
///module
#[doc(alias = "CAU_BGR")]
pub type CauBgr = crate::Reg<cau_bgr::CAU_BGRrs>;
///CAU Bandgap Register
pub mod cau_bgr;
///CAU_TR (rw) register accessor: CAU Test Register
///
///You can [`read`](crate::Reg::read) this register and get [`cau_tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau_tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cau_tr`]
///module
#[doc(alias = "CAU_TR")]
pub type CauTr = crate::Reg<cau_tr::CAU_TRrs>;
///CAU Test Register
pub mod cau_tr;
///WKUP_CNT (rw) register accessor: Wakeup Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`wkup_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkup_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@wkup_cnt`]
///module
#[doc(alias = "WKUP_CNT")]
pub type WkupCnt = crate::Reg<wkup_cnt::WKUP_CNTrs>;
///Wakeup Count Register
pub mod wkup_cnt;
///PWRKEY_CNT (rw) register accessor: PowerKey Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrkey_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrkey_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pwrkey_cnt`]
///module
#[doc(alias = "PWRKEY_CNT")]
pub type PwrkeyCnt = crate::Reg<pwrkey_cnt::PWRKEY_CNTrs>;
///PowerKey Count Register
pub mod pwrkey_cnt;
///HPSYS_VOUT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hpsys_vout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpsys_vout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hpsys_vout`]
///module
#[doc(alias = "HPSYS_VOUT")]
pub type HpsysVout = crate::Reg<hpsys_vout::HPSYS_VOUTrs>;
///
pub mod hpsys_vout;
///LPSYS_VOUT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lpsys_vout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsys_vout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lpsys_vout`]
///module
#[doc(alias = "LPSYS_VOUT")]
pub type LpsysVout = crate::Reg<lpsys_vout::LPSYS_VOUTrs>;
///
pub mod lpsys_vout;
///BUCK_VOUT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`buck_vout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_vout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@buck_vout`]
///module
#[doc(alias = "BUCK_VOUT")]
pub type BuckVout = crate::Reg<buck_vout::BUCK_VOUTrs>;
///
pub mod buck_vout;
