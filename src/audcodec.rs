#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    id: Id,
    cfg: Cfg,
    irq: Irq,
    irq_msk: IrqMsk,
    dac_cfg: DacCfg,
    adc_cfg: AdcCfg,
    apb_stat: ApbStat,
    _reserved7: [u8; 0x04],
    adc_ch0_cfg: AdcCh0Cfg,
    adc_ch1_cfg: AdcCh1Cfg,
    _reserved9: [u8; 0x08],
    dac_ch0_cfg: DacCh0Cfg,
    dac_ch0_cfg_ext: DacCh0CfgExt,
    dac_ch1_cfg: DacCh1Cfg,
    dac_ch1_cfg_ext: DacCh1CfgExt,
    adc_ch0_entry: AdcCh0Entry,
    adc_ch1_entry: AdcCh1Entry,
    _reserved15: [u8; 0x08],
    dac_ch0_entry: DacCh0Entry,
    dac_ch1_entry: DacCh1Entry,
    dac_ch0_debug: DacCh0Debug,
    dac_ch1_debug: DacCh1Debug,
    dac_ch0_dc: DacCh0Dc,
    dac_ch1_dc: DacCh1Dc,
    _reserved21: [u8; 0x08],
    common_cfg: CommonCfg,
    bg_cfg0: BgCfg0,
    bg_cfg1: BgCfg1,
    bg_cfg2: BgCfg2,
    refgen_cfg: RefgenCfg,
    pll_cfg0: PllCfg0,
    pll_cfg1: PllCfg1,
    pll_cfg2: PllCfg2,
    pll_cfg3: PllCfg3,
    pll_cfg4: PllCfg4,
    pll_cfg5: PllCfg5,
    pll_cfg6: PllCfg6,
    pll_stat: PllStat,
    pll_cal_cfg: PllCalCfg,
    pll_cal_result: PllCalResult,
    adc_ana_cfg: AdcAnaCfg,
    adc1_cfg1: Adc1Cfg1,
    adc1_cfg2: Adc1Cfg2,
    adc2_cfg1: Adc2Cfg1,
    adc2_cfg2: Adc2Cfg2,
    dac1_cfg: Dac1Cfg,
    dac2_cfg: Dac2Cfg,
    _reserved43: [u8; 0x08],
    reserved_in0: ReservedIn0,
    reserved_in1: ReservedIn1,
    reserved_out: ReservedOut,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    ///0x04 -
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    ///0x08 -
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    ///0x0c -
    #[inline(always)]
    pub const fn irq_msk(&self) -> &IrqMsk {
        &self.irq_msk
    }
    ///0x10 -
    #[inline(always)]
    pub const fn dac_cfg(&self) -> &DacCfg {
        &self.dac_cfg
    }
    ///0x14 -
    #[inline(always)]
    pub const fn adc_cfg(&self) -> &AdcCfg {
        &self.adc_cfg
    }
    ///0x18 -
    #[inline(always)]
    pub const fn apb_stat(&self) -> &ApbStat {
        &self.apb_stat
    }
    ///0x20 -
    #[inline(always)]
    pub const fn adc_ch0_cfg(&self) -> &AdcCh0Cfg {
        &self.adc_ch0_cfg
    }
    ///0x24 -
    #[inline(always)]
    pub const fn adc_ch1_cfg(&self) -> &AdcCh1Cfg {
        &self.adc_ch1_cfg
    }
    ///0x30 -
    #[inline(always)]
    pub const fn dac_ch0_cfg(&self) -> &DacCh0Cfg {
        &self.dac_ch0_cfg
    }
    ///0x34 -
    #[inline(always)]
    pub const fn dac_ch0_cfg_ext(&self) -> &DacCh0CfgExt {
        &self.dac_ch0_cfg_ext
    }
    ///0x38 -
    #[inline(always)]
    pub const fn dac_ch1_cfg(&self) -> &DacCh1Cfg {
        &self.dac_ch1_cfg
    }
    ///0x3c -
    #[inline(always)]
    pub const fn dac_ch1_cfg_ext(&self) -> &DacCh1CfgExt {
        &self.dac_ch1_cfg_ext
    }
    ///0x40 -
    #[inline(always)]
    pub const fn adc_ch0_entry(&self) -> &AdcCh0Entry {
        &self.adc_ch0_entry
    }
    ///0x44 -
    #[inline(always)]
    pub const fn adc_ch1_entry(&self) -> &AdcCh1Entry {
        &self.adc_ch1_entry
    }
    ///0x50 -
    #[inline(always)]
    pub const fn dac_ch0_entry(&self) -> &DacCh0Entry {
        &self.dac_ch0_entry
    }
    ///0x54 -
    #[inline(always)]
    pub const fn dac_ch1_entry(&self) -> &DacCh1Entry {
        &self.dac_ch1_entry
    }
    ///0x58 -
    #[inline(always)]
    pub const fn dac_ch0_debug(&self) -> &DacCh0Debug {
        &self.dac_ch0_debug
    }
    ///0x5c -
    #[inline(always)]
    pub const fn dac_ch1_debug(&self) -> &DacCh1Debug {
        &self.dac_ch1_debug
    }
    ///0x60 -
    #[inline(always)]
    pub const fn dac_ch0_dc(&self) -> &DacCh0Dc {
        &self.dac_ch0_dc
    }
    ///0x64 -
    #[inline(always)]
    pub const fn dac_ch1_dc(&self) -> &DacCh1Dc {
        &self.dac_ch1_dc
    }
    ///0x70 -
    #[inline(always)]
    pub const fn common_cfg(&self) -> &CommonCfg {
        &self.common_cfg
    }
    ///0x74 -
    #[inline(always)]
    pub const fn bg_cfg0(&self) -> &BgCfg0 {
        &self.bg_cfg0
    }
    ///0x78 -
    #[inline(always)]
    pub const fn bg_cfg1(&self) -> &BgCfg1 {
        &self.bg_cfg1
    }
    ///0x7c -
    #[inline(always)]
    pub const fn bg_cfg2(&self) -> &BgCfg2 {
        &self.bg_cfg2
    }
    ///0x80 -
    #[inline(always)]
    pub const fn refgen_cfg(&self) -> &RefgenCfg {
        &self.refgen_cfg
    }
    ///0x84 -
    #[inline(always)]
    pub const fn pll_cfg0(&self) -> &PllCfg0 {
        &self.pll_cfg0
    }
    ///0x88 -
    #[inline(always)]
    pub const fn pll_cfg1(&self) -> &PllCfg1 {
        &self.pll_cfg1
    }
    ///0x8c -
    #[inline(always)]
    pub const fn pll_cfg2(&self) -> &PllCfg2 {
        &self.pll_cfg2
    }
    ///0x90 -
    #[inline(always)]
    pub const fn pll_cfg3(&self) -> &PllCfg3 {
        &self.pll_cfg3
    }
    ///0x94 -
    #[inline(always)]
    pub const fn pll_cfg4(&self) -> &PllCfg4 {
        &self.pll_cfg4
    }
    ///0x98 -
    #[inline(always)]
    pub const fn pll_cfg5(&self) -> &PllCfg5 {
        &self.pll_cfg5
    }
    ///0x9c -
    #[inline(always)]
    pub const fn pll_cfg6(&self) -> &PllCfg6 {
        &self.pll_cfg6
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn pll_stat(&self) -> &PllStat {
        &self.pll_stat
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn pll_cal_cfg(&self) -> &PllCalCfg {
        &self.pll_cal_cfg
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn pll_cal_result(&self) -> &PllCalResult {
        &self.pll_cal_result
    }
    ///0xac -
    #[inline(always)]
    pub const fn adc_ana_cfg(&self) -> &AdcAnaCfg {
        &self.adc_ana_cfg
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn adc1_cfg1(&self) -> &Adc1Cfg1 {
        &self.adc1_cfg1
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn adc1_cfg2(&self) -> &Adc1Cfg2 {
        &self.adc1_cfg2
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn adc2_cfg1(&self) -> &Adc2Cfg1 {
        &self.adc2_cfg1
    }
    ///0xbc -
    #[inline(always)]
    pub const fn adc2_cfg2(&self) -> &Adc2Cfg2 {
        &self.adc2_cfg2
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn dac1_cfg(&self) -> &Dac1Cfg {
        &self.dac1_cfg
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn dac2_cfg(&self) -> &Dac2Cfg {
        &self.dac2_cfg
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn reserved_in0(&self) -> &ReservedIn0 {
        &self.reserved_in0
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn reserved_in1(&self) -> &ReservedIn1 {
        &self.reserved_in1
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn reserved_out(&self) -> &ReservedOut {
        &self.reserved_out
    }
}
///ID (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@id`]
///module
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IDrs>;
///
pub mod id;
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
///IRQ_MSK (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`irq_msk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_msk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@irq_msk`]
///module
#[doc(alias = "IRQ_MSK")]
pub type IrqMsk = crate::Reg<irq_msk::IRQ_MSKrs>;
///
pub mod irq_msk;
///DAC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_cfg`]
///module
#[doc(alias = "DAC_CFG")]
pub type DacCfg = crate::Reg<dac_cfg::DAC_CFGrs>;
///
pub mod dac_cfg;
///ADC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_cfg`]
///module
#[doc(alias = "ADC_CFG")]
pub type AdcCfg = crate::Reg<adc_cfg::ADC_CFGrs>;
///
pub mod adc_cfg;
///APB_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`apb_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@apb_stat`]
///module
#[doc(alias = "APB_STAT")]
pub type ApbStat = crate::Reg<apb_stat::APB_STATrs>;
///
pub mod apb_stat;
///ADC_CH0_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ch0_cfg`]
///module
#[doc(alias = "ADC_CH0_CFG")]
pub type AdcCh0Cfg = crate::Reg<adc_ch0_cfg::ADC_CH0_CFGrs>;
///
pub mod adc_ch0_cfg;
///ADC_CH1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ch1_cfg`]
///module
#[doc(alias = "ADC_CH1_CFG")]
pub type AdcCh1Cfg = crate::Reg<adc_ch1_cfg::ADC_CH1_CFGrs>;
///
pub mod adc_ch1_cfg;
///DAC_CH0_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch0_cfg`]
///module
#[doc(alias = "DAC_CH0_CFG")]
pub type DacCh0Cfg = crate::Reg<dac_ch0_cfg::DAC_CH0_CFGrs>;
///
pub mod dac_ch0_cfg;
///DAC_CH0_CFG_EXT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_cfg_ext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_cfg_ext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch0_cfg_ext`]
///module
#[doc(alias = "DAC_CH0_CFG_EXT")]
pub type DacCh0CfgExt = crate::Reg<dac_ch0_cfg_ext::DAC_CH0_CFG_EXTrs>;
///
pub mod dac_ch0_cfg_ext;
///DAC_CH1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch1_cfg`]
///module
#[doc(alias = "DAC_CH1_CFG")]
pub type DacCh1Cfg = crate::Reg<dac_ch1_cfg::DAC_CH1_CFGrs>;
///
pub mod dac_ch1_cfg;
///DAC_CH1_CFG_EXT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_cfg_ext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_cfg_ext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch1_cfg_ext`]
///module
#[doc(alias = "DAC_CH1_CFG_EXT")]
pub type DacCh1CfgExt = crate::Reg<dac_ch1_cfg_ext::DAC_CH1_CFG_EXTrs>;
///
pub mod dac_ch1_cfg_ext;
///ADC_CH0_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ch0_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ch0_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ch0_entry`]
///module
#[doc(alias = "ADC_CH0_ENTRY")]
pub type AdcCh0Entry = crate::Reg<adc_ch0_entry::ADC_CH0_ENTRYrs>;
///
pub mod adc_ch0_entry;
///ADC_CH1_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ch1_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ch1_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ch1_entry`]
///module
#[doc(alias = "ADC_CH1_ENTRY")]
pub type AdcCh1Entry = crate::Reg<adc_ch1_entry::ADC_CH1_ENTRYrs>;
///
pub mod adc_ch1_entry;
///DAC_CH0_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch0_entry`]
///module
#[doc(alias = "DAC_CH0_ENTRY")]
pub type DacCh0Entry = crate::Reg<dac_ch0_entry::DAC_CH0_ENTRYrs>;
///
pub mod dac_ch0_entry;
///DAC_CH1_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch1_entry`]
///module
#[doc(alias = "DAC_CH1_ENTRY")]
pub type DacCh1Entry = crate::Reg<dac_ch1_entry::DAC_CH1_ENTRYrs>;
///
pub mod dac_ch1_entry;
///DAC_CH0_DEBUG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch0_debug`]
///module
#[doc(alias = "DAC_CH0_DEBUG")]
pub type DacCh0Debug = crate::Reg<dac_ch0_debug::DAC_CH0_DEBUGrs>;
///
pub mod dac_ch0_debug;
///DAC_CH1_DEBUG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch1_debug`]
///module
#[doc(alias = "DAC_CH1_DEBUG")]
pub type DacCh1Debug = crate::Reg<dac_ch1_debug::DAC_CH1_DEBUGrs>;
///
pub mod dac_ch1_debug;
///DAC_CH0_DC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch0_dc`]
///module
#[doc(alias = "DAC_CH0_DC")]
pub type DacCh0Dc = crate::Reg<dac_ch0_dc::DAC_CH0_DCrs>;
///
pub mod dac_ch0_dc;
///DAC_CH1_DC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_ch1_dc`]
///module
#[doc(alias = "DAC_CH1_DC")]
pub type DacCh1Dc = crate::Reg<dac_ch1_dc::DAC_CH1_DCrs>;
///
pub mod dac_ch1_dc;
///COMMON_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`common_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`common_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@common_cfg`]
///module
#[doc(alias = "COMMON_CFG")]
pub type CommonCfg = crate::Reg<common_cfg::COMMON_CFGrs>;
///
pub mod common_cfg;
///BG_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bg_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bg_cfg0`]
///module
#[doc(alias = "BG_CFG0")]
pub type BgCfg0 = crate::Reg<bg_cfg0::BG_CFG0rs>;
///
pub mod bg_cfg0;
///BG_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bg_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bg_cfg1`]
///module
#[doc(alias = "BG_CFG1")]
pub type BgCfg1 = crate::Reg<bg_cfg1::BG_CFG1rs>;
///
pub mod bg_cfg1;
///BG_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bg_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bg_cfg2`]
///module
#[doc(alias = "BG_CFG2")]
pub type BgCfg2 = crate::Reg<bg_cfg2::BG_CFG2rs>;
///
pub mod bg_cfg2;
///REFGEN_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`refgen_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refgen_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@refgen_cfg`]
///module
#[doc(alias = "REFGEN_CFG")]
pub type RefgenCfg = crate::Reg<refgen_cfg::REFGEN_CFGrs>;
///
pub mod refgen_cfg;
///PLL_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg0`]
///module
#[doc(alias = "PLL_CFG0")]
pub type PllCfg0 = crate::Reg<pll_cfg0::PLL_CFG0rs>;
///
pub mod pll_cfg0;
///PLL_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg1`]
///module
#[doc(alias = "PLL_CFG1")]
pub type PllCfg1 = crate::Reg<pll_cfg1::PLL_CFG1rs>;
///
pub mod pll_cfg1;
///PLL_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg2`]
///module
#[doc(alias = "PLL_CFG2")]
pub type PllCfg2 = crate::Reg<pll_cfg2::PLL_CFG2rs>;
///
pub mod pll_cfg2;
///PLL_CFG3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg3`]
///module
#[doc(alias = "PLL_CFG3")]
pub type PllCfg3 = crate::Reg<pll_cfg3::PLL_CFG3rs>;
///
pub mod pll_cfg3;
///PLL_CFG4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg4`]
///module
#[doc(alias = "PLL_CFG4")]
pub type PllCfg4 = crate::Reg<pll_cfg4::PLL_CFG4rs>;
///
pub mod pll_cfg4;
///PLL_CFG5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg5`]
///module
#[doc(alias = "PLL_CFG5")]
pub type PllCfg5 = crate::Reg<pll_cfg5::PLL_CFG5rs>;
///
pub mod pll_cfg5;
///PLL_CFG6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cfg6`]
///module
#[doc(alias = "PLL_CFG6")]
pub type PllCfg6 = crate::Reg<pll_cfg6::PLL_CFG6rs>;
///
pub mod pll_cfg6;
///PLL_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_stat`]
///module
#[doc(alias = "PLL_STAT")]
pub type PllStat = crate::Reg<pll_stat::PLL_STATrs>;
///
pub mod pll_stat;
///PLL_CAL_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cal_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cal_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cal_cfg`]
///module
#[doc(alias = "PLL_CAL_CFG")]
pub type PllCalCfg = crate::Reg<pll_cal_cfg::PLL_CAL_CFGrs>;
///
pub mod pll_cal_cfg;
///PLL_CAL_RESULT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cal_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cal_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@pll_cal_result`]
///module
#[doc(alias = "PLL_CAL_RESULT")]
pub type PllCalResult = crate::Reg<pll_cal_result::PLL_CAL_RESULTrs>;
///
pub mod pll_cal_result;
///ADC_ANA_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ana_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ana_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_ana_cfg`]
///module
#[doc(alias = "ADC_ANA_CFG")]
pub type AdcAnaCfg = crate::Reg<adc_ana_cfg::ADC_ANA_CFGrs>;
///
pub mod adc_ana_cfg;
///ADC1_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc1_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc1_cfg1`]
///module
#[doc(alias = "ADC1_CFG1")]
pub type Adc1Cfg1 = crate::Reg<adc1_cfg1::ADC1_CFG1rs>;
///
pub mod adc1_cfg1;
///ADC1_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc1_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc1_cfg2`]
///module
#[doc(alias = "ADC1_CFG2")]
pub type Adc1Cfg2 = crate::Reg<adc1_cfg2::ADC1_CFG2rs>;
///
pub mod adc1_cfg2;
///ADC2_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc2_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc2_cfg1`]
///module
#[doc(alias = "ADC2_CFG1")]
pub type Adc2Cfg1 = crate::Reg<adc2_cfg1::ADC2_CFG1rs>;
///
pub mod adc2_cfg1;
///ADC2_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc2_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc2_cfg2`]
///module
#[doc(alias = "ADC2_CFG2")]
pub type Adc2Cfg2 = crate::Reg<adc2_cfg2::ADC2_CFG2rs>;
///
pub mod adc2_cfg2;
///DAC1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac1_cfg`]
///module
#[doc(alias = "DAC1_CFG")]
pub type Dac1Cfg = crate::Reg<dac1_cfg::DAC1_CFGrs>;
///
pub mod dac1_cfg;
///DAC2_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac2_cfg`]
///module
#[doc(alias = "DAC2_CFG")]
pub type Dac2Cfg = crate::Reg<dac2_cfg::DAC2_CFGrs>;
///
pub mod dac2_cfg;
///RESERVED_IN0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_in0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_in0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reserved_in0`]
///module
#[doc(alias = "RESERVED_IN0")]
pub type ReservedIn0 = crate::Reg<reserved_in0::RESERVED_IN0rs>;
///
pub mod reserved_in0;
///RESERVED_IN1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_in1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_in1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reserved_in1`]
///module
#[doc(alias = "RESERVED_IN1")]
pub type ReservedIn1 = crate::Reg<reserved_in1::RESERVED_IN1rs>;
///
pub mod reserved_in1;
///RESERVED_OUT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@reserved_out`]
///module
#[doc(alias = "RESERVED_OUT")]
pub type ReservedOut = crate::Reg<reserved_out::RESERVED_OUTrs>;
///
pub mod reserved_out;
