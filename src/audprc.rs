#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    id: Id,
    cfg: Cfg,
    stb: Stb,
    irq: Irq,
    tx_ch0_cfg: TxCh0Cfg,
    tx_ch0_entry: TxCh0Entry,
    tx_ch1_cfg: TxCh1Cfg,
    tx_ch1_entry: TxCh1Entry,
    tx_ch2_cfg: TxCh2Cfg,
    tx_ch2_entry: TxCh2Entry,
    tx_ch3_cfg: TxCh3Cfg,
    tx_ch3_entry: TxCh3Entry,
    rx_ch0_cfg: RxCh0Cfg,
    rx_ch0_entry: RxCh0Entry,
    rx_ch1_cfg: RxCh1Cfg,
    rx_ch1_entry: RxCh1Entry,
    tx_out_ch0_cfg: TxOutCh0Cfg,
    tx_out_ch0_entry: TxOutCh0Entry,
    tx_out_ch1_cfg: TxOutCh1Cfg,
    tx_out_ch1_entry: TxOutCh1Entry,
    dac_path_cfg0: DacPathCfg0,
    dac_path_cfg1: DacPathCfg1,
    dac_path_cfg2: DacPathCfg2,
    dac_path_cfg3: DacPathCfg3,
    adc_path_cfg0: AdcPathCfg0,
    _reserved25: [u8; 0x0c],
    dac_eq_cfg0: DacEqCfg0,
    dac_eq_cfg1: DacEqCfg1,
    dac_eq_cfg2: DacEqCfg2,
    dac_eq_cfg3: DacEqCfg3,
    dac_eq_cfg4: DacEqCfg4,
    dac_eq_cfg5: DacEqCfg5,
    dac_eq_cfg6: DacEqCfg6,
    dac_eq_cfg7: DacEqCfg7,
    dac_eq_cfg8: DacEqCfg8,
    dac_eq_cfg9: DacEqCfg9,
    dac_eq_cfg10: DacEqCfg10,
    dac_eq_cfg11: DacEqCfg11,
    dac_eq_cfg12: DacEqCfg12,
    dac_eq_cfg13: DacEqCfg13,
    dac_eq_cfg14: DacEqCfg14,
    dac_eq_cfg15: DacEqCfg15,
    dac_eq_cfg16: DacEqCfg16,
    dac_eq_cfg17: DacEqCfg17,
    dac_eq_cfg18: DacEqCfg18,
    dac_eq_cfg19: DacEqCfg19,
    dac_eq_cfg20: DacEqCfg20,
    dac_eq_cfg21: DacEqCfg21,
    dac_eq_cfg22: DacEqCfg22,
    dac_eq_cfg23: DacEqCfg23,
    dac_eq_cfg24: DacEqCfg24,
    dac_eq_cfg25: DacEqCfg25,
    dac_eq_cfg26: DacEqCfg26,
    dac_eq_cfg27: DacEqCfg27,
    dac_eq_cfg28: DacEqCfg28,
    dac_eq_cfg29: DacEqCfg29,
    dac_eq_cfg30: DacEqCfg30,
    dac_eq_cfg31: DacEqCfg31,
    dac_eq_cfg32: DacEqCfg32,
    dac_eq_cfg33: DacEqCfg33,
    dac_eq_cfg34: DacEqCfg34,
    dac_eq_cfg35: DacEqCfg35,
    dac_eq_cfg36: DacEqCfg36,
    dac_eq_cfg37: DacEqCfg37,
    dac_eq_cfg38: DacEqCfg38,
    dac_eq_cfg39: DacEqCfg39,
    dac_eq_cfg40: DacEqCfg40,
    dac_eq_cfg41: DacEqCfg41,
    dac_eq_cfg42: DacEqCfg42,
    dac_eq_cfg43: DacEqCfg43,
    dac_eq_cfg44: DacEqCfg44,
    dac_eq_cfg45: DacEqCfg45,
    dac_eq_cfg46: DacEqCfg46,
    dac_eq_cfg47: DacEqCfg47,
    dac_eq_cfg48: DacEqCfg48,
    dac_eq_cfg49: DacEqCfg49,
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
    pub const fn stb(&self) -> &Stb {
        &self.stb
    }
    ///0x0c -
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    ///0x10 -
    #[inline(always)]
    pub const fn tx_ch0_cfg(&self) -> &TxCh0Cfg {
        &self.tx_ch0_cfg
    }
    ///0x14 -
    #[inline(always)]
    pub const fn tx_ch0_entry(&self) -> &TxCh0Entry {
        &self.tx_ch0_entry
    }
    ///0x18 -
    #[inline(always)]
    pub const fn tx_ch1_cfg(&self) -> &TxCh1Cfg {
        &self.tx_ch1_cfg
    }
    ///0x1c -
    #[inline(always)]
    pub const fn tx_ch1_entry(&self) -> &TxCh1Entry {
        &self.tx_ch1_entry
    }
    ///0x20 -
    #[inline(always)]
    pub const fn tx_ch2_cfg(&self) -> &TxCh2Cfg {
        &self.tx_ch2_cfg
    }
    ///0x24 -
    #[inline(always)]
    pub const fn tx_ch2_entry(&self) -> &TxCh2Entry {
        &self.tx_ch2_entry
    }
    ///0x28 -
    #[inline(always)]
    pub const fn tx_ch3_cfg(&self) -> &TxCh3Cfg {
        &self.tx_ch3_cfg
    }
    ///0x2c -
    #[inline(always)]
    pub const fn tx_ch3_entry(&self) -> &TxCh3Entry {
        &self.tx_ch3_entry
    }
    ///0x30 -
    #[inline(always)]
    pub const fn rx_ch0_cfg(&self) -> &RxCh0Cfg {
        &self.rx_ch0_cfg
    }
    ///0x34 -
    #[inline(always)]
    pub const fn rx_ch0_entry(&self) -> &RxCh0Entry {
        &self.rx_ch0_entry
    }
    ///0x38 -
    #[inline(always)]
    pub const fn rx_ch1_cfg(&self) -> &RxCh1Cfg {
        &self.rx_ch1_cfg
    }
    ///0x3c -
    #[inline(always)]
    pub const fn rx_ch1_entry(&self) -> &RxCh1Entry {
        &self.rx_ch1_entry
    }
    ///0x40 -
    #[inline(always)]
    pub const fn tx_out_ch0_cfg(&self) -> &TxOutCh0Cfg {
        &self.tx_out_ch0_cfg
    }
    ///0x44 -
    #[inline(always)]
    pub const fn tx_out_ch0_entry(&self) -> &TxOutCh0Entry {
        &self.tx_out_ch0_entry
    }
    ///0x48 -
    #[inline(always)]
    pub const fn tx_out_ch1_cfg(&self) -> &TxOutCh1Cfg {
        &self.tx_out_ch1_cfg
    }
    ///0x4c -
    #[inline(always)]
    pub const fn tx_out_ch1_entry(&self) -> &TxOutCh1Entry {
        &self.tx_out_ch1_entry
    }
    ///0x50 -
    #[inline(always)]
    pub const fn dac_path_cfg0(&self) -> &DacPathCfg0 {
        &self.dac_path_cfg0
    }
    ///0x54 -
    #[inline(always)]
    pub const fn dac_path_cfg1(&self) -> &DacPathCfg1 {
        &self.dac_path_cfg1
    }
    ///0x58 -
    #[inline(always)]
    pub const fn dac_path_cfg2(&self) -> &DacPathCfg2 {
        &self.dac_path_cfg2
    }
    ///0x5c -
    #[inline(always)]
    pub const fn dac_path_cfg3(&self) -> &DacPathCfg3 {
        &self.dac_path_cfg3
    }
    ///0x60 -
    #[inline(always)]
    pub const fn adc_path_cfg0(&self) -> &AdcPathCfg0 {
        &self.adc_path_cfg0
    }
    ///0x70 -
    #[inline(always)]
    pub const fn dac_eq_cfg0(&self) -> &DacEqCfg0 {
        &self.dac_eq_cfg0
    }
    ///0x74 -
    #[inline(always)]
    pub const fn dac_eq_cfg1(&self) -> &DacEqCfg1 {
        &self.dac_eq_cfg1
    }
    ///0x78 -
    #[inline(always)]
    pub const fn dac_eq_cfg2(&self) -> &DacEqCfg2 {
        &self.dac_eq_cfg2
    }
    ///0x7c -
    #[inline(always)]
    pub const fn dac_eq_cfg3(&self) -> &DacEqCfg3 {
        &self.dac_eq_cfg3
    }
    ///0x80 -
    #[inline(always)]
    pub const fn dac_eq_cfg4(&self) -> &DacEqCfg4 {
        &self.dac_eq_cfg4
    }
    ///0x84 -
    #[inline(always)]
    pub const fn dac_eq_cfg5(&self) -> &DacEqCfg5 {
        &self.dac_eq_cfg5
    }
    ///0x88 -
    #[inline(always)]
    pub const fn dac_eq_cfg6(&self) -> &DacEqCfg6 {
        &self.dac_eq_cfg6
    }
    ///0x8c -
    #[inline(always)]
    pub const fn dac_eq_cfg7(&self) -> &DacEqCfg7 {
        &self.dac_eq_cfg7
    }
    ///0x90 -
    #[inline(always)]
    pub const fn dac_eq_cfg8(&self) -> &DacEqCfg8 {
        &self.dac_eq_cfg8
    }
    ///0x94 -
    #[inline(always)]
    pub const fn dac_eq_cfg9(&self) -> &DacEqCfg9 {
        &self.dac_eq_cfg9
    }
    ///0x98 -
    #[inline(always)]
    pub const fn dac_eq_cfg10(&self) -> &DacEqCfg10 {
        &self.dac_eq_cfg10
    }
    ///0x9c -
    #[inline(always)]
    pub const fn dac_eq_cfg11(&self) -> &DacEqCfg11 {
        &self.dac_eq_cfg11
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn dac_eq_cfg12(&self) -> &DacEqCfg12 {
        &self.dac_eq_cfg12
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn dac_eq_cfg13(&self) -> &DacEqCfg13 {
        &self.dac_eq_cfg13
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn dac_eq_cfg14(&self) -> &DacEqCfg14 {
        &self.dac_eq_cfg14
    }
    ///0xac -
    #[inline(always)]
    pub const fn dac_eq_cfg15(&self) -> &DacEqCfg15 {
        &self.dac_eq_cfg15
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn dac_eq_cfg16(&self) -> &DacEqCfg16 {
        &self.dac_eq_cfg16
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn dac_eq_cfg17(&self) -> &DacEqCfg17 {
        &self.dac_eq_cfg17
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn dac_eq_cfg18(&self) -> &DacEqCfg18 {
        &self.dac_eq_cfg18
    }
    ///0xbc -
    #[inline(always)]
    pub const fn dac_eq_cfg19(&self) -> &DacEqCfg19 {
        &self.dac_eq_cfg19
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn dac_eq_cfg20(&self) -> &DacEqCfg20 {
        &self.dac_eq_cfg20
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn dac_eq_cfg21(&self) -> &DacEqCfg21 {
        &self.dac_eq_cfg21
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn dac_eq_cfg22(&self) -> &DacEqCfg22 {
        &self.dac_eq_cfg22
    }
    ///0xcc -
    #[inline(always)]
    pub const fn dac_eq_cfg23(&self) -> &DacEqCfg23 {
        &self.dac_eq_cfg23
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn dac_eq_cfg24(&self) -> &DacEqCfg24 {
        &self.dac_eq_cfg24
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn dac_eq_cfg25(&self) -> &DacEqCfg25 {
        &self.dac_eq_cfg25
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn dac_eq_cfg26(&self) -> &DacEqCfg26 {
        &self.dac_eq_cfg26
    }
    ///0xdc -
    #[inline(always)]
    pub const fn dac_eq_cfg27(&self) -> &DacEqCfg27 {
        &self.dac_eq_cfg27
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn dac_eq_cfg28(&self) -> &DacEqCfg28 {
        &self.dac_eq_cfg28
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn dac_eq_cfg29(&self) -> &DacEqCfg29 {
        &self.dac_eq_cfg29
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn dac_eq_cfg30(&self) -> &DacEqCfg30 {
        &self.dac_eq_cfg30
    }
    ///0xec -
    #[inline(always)]
    pub const fn dac_eq_cfg31(&self) -> &DacEqCfg31 {
        &self.dac_eq_cfg31
    }
    ///0xf0 -
    #[inline(always)]
    pub const fn dac_eq_cfg32(&self) -> &DacEqCfg32 {
        &self.dac_eq_cfg32
    }
    ///0xf4 -
    #[inline(always)]
    pub const fn dac_eq_cfg33(&self) -> &DacEqCfg33 {
        &self.dac_eq_cfg33
    }
    ///0xf8 -
    #[inline(always)]
    pub const fn dac_eq_cfg34(&self) -> &DacEqCfg34 {
        &self.dac_eq_cfg34
    }
    ///0xfc -
    #[inline(always)]
    pub const fn dac_eq_cfg35(&self) -> &DacEqCfg35 {
        &self.dac_eq_cfg35
    }
    ///0x100 -
    #[inline(always)]
    pub const fn dac_eq_cfg36(&self) -> &DacEqCfg36 {
        &self.dac_eq_cfg36
    }
    ///0x104 -
    #[inline(always)]
    pub const fn dac_eq_cfg37(&self) -> &DacEqCfg37 {
        &self.dac_eq_cfg37
    }
    ///0x108 -
    #[inline(always)]
    pub const fn dac_eq_cfg38(&self) -> &DacEqCfg38 {
        &self.dac_eq_cfg38
    }
    ///0x10c -
    #[inline(always)]
    pub const fn dac_eq_cfg39(&self) -> &DacEqCfg39 {
        &self.dac_eq_cfg39
    }
    ///0x110 -
    #[inline(always)]
    pub const fn dac_eq_cfg40(&self) -> &DacEqCfg40 {
        &self.dac_eq_cfg40
    }
    ///0x114 -
    #[inline(always)]
    pub const fn dac_eq_cfg41(&self) -> &DacEqCfg41 {
        &self.dac_eq_cfg41
    }
    ///0x118 -
    #[inline(always)]
    pub const fn dac_eq_cfg42(&self) -> &DacEqCfg42 {
        &self.dac_eq_cfg42
    }
    ///0x11c -
    #[inline(always)]
    pub const fn dac_eq_cfg43(&self) -> &DacEqCfg43 {
        &self.dac_eq_cfg43
    }
    ///0x120 -
    #[inline(always)]
    pub const fn dac_eq_cfg44(&self) -> &DacEqCfg44 {
        &self.dac_eq_cfg44
    }
    ///0x124 -
    #[inline(always)]
    pub const fn dac_eq_cfg45(&self) -> &DacEqCfg45 {
        &self.dac_eq_cfg45
    }
    ///0x128 -
    #[inline(always)]
    pub const fn dac_eq_cfg46(&self) -> &DacEqCfg46 {
        &self.dac_eq_cfg46
    }
    ///0x12c -
    #[inline(always)]
    pub const fn dac_eq_cfg47(&self) -> &DacEqCfg47 {
        &self.dac_eq_cfg47
    }
    ///0x130 -
    #[inline(always)]
    pub const fn dac_eq_cfg48(&self) -> &DacEqCfg48 {
        &self.dac_eq_cfg48
    }
    ///0x134 -
    #[inline(always)]
    pub const fn dac_eq_cfg49(&self) -> &DacEqCfg49 {
        &self.dac_eq_cfg49
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
///STB (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`stb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@stb`]
///module
#[doc(alias = "STB")]
pub type Stb = crate::Reg<stb::STBrs>;
///
pub mod stb;
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
///TX_CH0_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch0_cfg`]
///module
#[doc(alias = "TX_CH0_CFG")]
pub type TxCh0Cfg = crate::Reg<tx_ch0_cfg::TX_CH0_CFGrs>;
///
pub mod tx_ch0_cfg;
///TX_CH0_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch0_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch0_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch0_entry`]
///module
#[doc(alias = "TX_CH0_ENTRY")]
pub type TxCh0Entry = crate::Reg<tx_ch0_entry::TX_CH0_ENTRYrs>;
///
pub mod tx_ch0_entry;
///TX_CH1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch1_cfg`]
///module
#[doc(alias = "TX_CH1_CFG")]
pub type TxCh1Cfg = crate::Reg<tx_ch1_cfg::TX_CH1_CFGrs>;
///
pub mod tx_ch1_cfg;
///TX_CH1_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch1_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch1_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch1_entry`]
///module
#[doc(alias = "TX_CH1_ENTRY")]
pub type TxCh1Entry = crate::Reg<tx_ch1_entry::TX_CH1_ENTRYrs>;
///
pub mod tx_ch1_entry;
///TX_CH2_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch2_cfg`]
///module
#[doc(alias = "TX_CH2_CFG")]
pub type TxCh2Cfg = crate::Reg<tx_ch2_cfg::TX_CH2_CFGrs>;
///
pub mod tx_ch2_cfg;
///TX_CH2_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch2_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch2_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch2_entry`]
///module
#[doc(alias = "TX_CH2_ENTRY")]
pub type TxCh2Entry = crate::Reg<tx_ch2_entry::TX_CH2_ENTRYrs>;
///
pub mod tx_ch2_entry;
///TX_CH3_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch3_cfg`]
///module
#[doc(alias = "TX_CH3_CFG")]
pub type TxCh3Cfg = crate::Reg<tx_ch3_cfg::TX_CH3_CFGrs>;
///
pub mod tx_ch3_cfg;
///TX_CH3_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch3_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch3_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_ch3_entry`]
///module
#[doc(alias = "TX_CH3_ENTRY")]
pub type TxCh3Entry = crate::Reg<tx_ch3_entry::TX_CH3_ENTRYrs>;
///
pub mod tx_ch3_entry;
///RX_CH0_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_ch0_cfg`]
///module
#[doc(alias = "RX_CH0_CFG")]
pub type RxCh0Cfg = crate::Reg<rx_ch0_cfg::RX_CH0_CFGrs>;
///
pub mod rx_ch0_cfg;
///RX_CH0_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch0_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch0_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_ch0_entry`]
///module
#[doc(alias = "RX_CH0_ENTRY")]
pub type RxCh0Entry = crate::Reg<rx_ch0_entry::RX_CH0_ENTRYrs>;
///
pub mod rx_ch0_entry;
///RX_CH1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_ch1_cfg`]
///module
#[doc(alias = "RX_CH1_CFG")]
pub type RxCh1Cfg = crate::Reg<rx_ch1_cfg::RX_CH1_CFGrs>;
///
pub mod rx_ch1_cfg;
///RX_CH1_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch1_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch1_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_ch1_entry`]
///module
#[doc(alias = "RX_CH1_ENTRY")]
pub type RxCh1Entry = crate::Reg<rx_ch1_entry::RX_CH1_ENTRYrs>;
///
pub mod rx_ch1_entry;
///TX_OUT_CH0_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_out_ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_out_ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_out_ch0_cfg`]
///module
#[doc(alias = "TX_OUT_CH0_CFG")]
pub type TxOutCh0Cfg = crate::Reg<tx_out_ch0_cfg::TX_OUT_CH0_CFGrs>;
///
pub mod tx_out_ch0_cfg;
///TX_OUT_CH0_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_out_ch0_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_out_ch0_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_out_ch0_entry`]
///module
#[doc(alias = "TX_OUT_CH0_ENTRY")]
pub type TxOutCh0Entry = crate::Reg<tx_out_ch0_entry::TX_OUT_CH0_ENTRYrs>;
///
pub mod tx_out_ch0_entry;
///TX_OUT_CH1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_out_ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_out_ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_out_ch1_cfg`]
///module
#[doc(alias = "TX_OUT_CH1_CFG")]
pub type TxOutCh1Cfg = crate::Reg<tx_out_ch1_cfg::TX_OUT_CH1_CFGrs>;
///
pub mod tx_out_ch1_cfg;
///TX_OUT_CH1_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_out_ch1_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_out_ch1_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_out_ch1_entry`]
///module
#[doc(alias = "TX_OUT_CH1_ENTRY")]
pub type TxOutCh1Entry = crate::Reg<tx_out_ch1_entry::TX_OUT_CH1_ENTRYrs>;
///
pub mod tx_out_ch1_entry;
///DAC_PATH_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_path_cfg0`]
///module
#[doc(alias = "DAC_PATH_CFG0")]
pub type DacPathCfg0 = crate::Reg<dac_path_cfg0::DAC_PATH_CFG0rs>;
///
pub mod dac_path_cfg0;
///DAC_PATH_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_path_cfg1`]
///module
#[doc(alias = "DAC_PATH_CFG1")]
pub type DacPathCfg1 = crate::Reg<dac_path_cfg1::DAC_PATH_CFG1rs>;
///
pub mod dac_path_cfg1;
///DAC_PATH_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_path_cfg2`]
///module
#[doc(alias = "DAC_PATH_CFG2")]
pub type DacPathCfg2 = crate::Reg<dac_path_cfg2::DAC_PATH_CFG2rs>;
///
pub mod dac_path_cfg2;
///DAC_PATH_CFG3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_path_cfg3`]
///module
#[doc(alias = "DAC_PATH_CFG3")]
pub type DacPathCfg3 = crate::Reg<dac_path_cfg3::DAC_PATH_CFG3rs>;
///
pub mod dac_path_cfg3;
///ADC_PATH_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`adc_path_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_path_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@adc_path_cfg0`]
///module
#[doc(alias = "ADC_PATH_CFG0")]
pub type AdcPathCfg0 = crate::Reg<adc_path_cfg0::ADC_PATH_CFG0rs>;
///
pub mod adc_path_cfg0;
///DAC_EQ_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg0`]
///module
#[doc(alias = "DAC_EQ_CFG0")]
pub type DacEqCfg0 = crate::Reg<dac_eq_cfg0::DAC_EQ_CFG0rs>;
///
pub mod dac_eq_cfg0;
///DAC_EQ_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg1`]
///module
#[doc(alias = "DAC_EQ_CFG1")]
pub type DacEqCfg1 = crate::Reg<dac_eq_cfg1::DAC_EQ_CFG1rs>;
///
pub mod dac_eq_cfg1;
///DAC_EQ_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg2`]
///module
#[doc(alias = "DAC_EQ_CFG2")]
pub type DacEqCfg2 = crate::Reg<dac_eq_cfg2::DAC_EQ_CFG2rs>;
///
pub mod dac_eq_cfg2;
///DAC_EQ_CFG3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg3`]
///module
#[doc(alias = "DAC_EQ_CFG3")]
pub type DacEqCfg3 = crate::Reg<dac_eq_cfg3::DAC_EQ_CFG3rs>;
///
pub mod dac_eq_cfg3;
///DAC_EQ_CFG4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg4`]
///module
#[doc(alias = "DAC_EQ_CFG4")]
pub type DacEqCfg4 = crate::Reg<dac_eq_cfg4::DAC_EQ_CFG4rs>;
///
pub mod dac_eq_cfg4;
///DAC_EQ_CFG5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg5`]
///module
#[doc(alias = "DAC_EQ_CFG5")]
pub type DacEqCfg5 = crate::Reg<dac_eq_cfg5::DAC_EQ_CFG5rs>;
///
pub mod dac_eq_cfg5;
///DAC_EQ_CFG6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg6`]
///module
#[doc(alias = "DAC_EQ_CFG6")]
pub type DacEqCfg6 = crate::Reg<dac_eq_cfg6::DAC_EQ_CFG6rs>;
///
pub mod dac_eq_cfg6;
///DAC_EQ_CFG7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg7`]
///module
#[doc(alias = "DAC_EQ_CFG7")]
pub type DacEqCfg7 = crate::Reg<dac_eq_cfg7::DAC_EQ_CFG7rs>;
///
pub mod dac_eq_cfg7;
///DAC_EQ_CFG8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg8`]
///module
#[doc(alias = "DAC_EQ_CFG8")]
pub type DacEqCfg8 = crate::Reg<dac_eq_cfg8::DAC_EQ_CFG8rs>;
///
pub mod dac_eq_cfg8;
///DAC_EQ_CFG9 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg9`]
///module
#[doc(alias = "DAC_EQ_CFG9")]
pub type DacEqCfg9 = crate::Reg<dac_eq_cfg9::DAC_EQ_CFG9rs>;
///
pub mod dac_eq_cfg9;
///DAC_EQ_CFG10 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg10`]
///module
#[doc(alias = "DAC_EQ_CFG10")]
pub type DacEqCfg10 = crate::Reg<dac_eq_cfg10::DAC_EQ_CFG10rs>;
///
pub mod dac_eq_cfg10;
///DAC_EQ_CFG11 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg11`]
///module
#[doc(alias = "DAC_EQ_CFG11")]
pub type DacEqCfg11 = crate::Reg<dac_eq_cfg11::DAC_EQ_CFG11rs>;
///
pub mod dac_eq_cfg11;
///DAC_EQ_CFG12 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg12`]
///module
#[doc(alias = "DAC_EQ_CFG12")]
pub type DacEqCfg12 = crate::Reg<dac_eq_cfg12::DAC_EQ_CFG12rs>;
///
pub mod dac_eq_cfg12;
///DAC_EQ_CFG13 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg13`]
///module
#[doc(alias = "DAC_EQ_CFG13")]
pub type DacEqCfg13 = crate::Reg<dac_eq_cfg13::DAC_EQ_CFG13rs>;
///
pub mod dac_eq_cfg13;
///DAC_EQ_CFG14 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg14`]
///module
#[doc(alias = "DAC_EQ_CFG14")]
pub type DacEqCfg14 = crate::Reg<dac_eq_cfg14::DAC_EQ_CFG14rs>;
///
pub mod dac_eq_cfg14;
///DAC_EQ_CFG15 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg15`]
///module
#[doc(alias = "DAC_EQ_CFG15")]
pub type DacEqCfg15 = crate::Reg<dac_eq_cfg15::DAC_EQ_CFG15rs>;
///
pub mod dac_eq_cfg15;
///DAC_EQ_CFG16 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg16`]
///module
#[doc(alias = "DAC_EQ_CFG16")]
pub type DacEqCfg16 = crate::Reg<dac_eq_cfg16::DAC_EQ_CFG16rs>;
///
pub mod dac_eq_cfg16;
///DAC_EQ_CFG17 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg17`]
///module
#[doc(alias = "DAC_EQ_CFG17")]
pub type DacEqCfg17 = crate::Reg<dac_eq_cfg17::DAC_EQ_CFG17rs>;
///
pub mod dac_eq_cfg17;
///DAC_EQ_CFG18 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg18`]
///module
#[doc(alias = "DAC_EQ_CFG18")]
pub type DacEqCfg18 = crate::Reg<dac_eq_cfg18::DAC_EQ_CFG18rs>;
///
pub mod dac_eq_cfg18;
///DAC_EQ_CFG19 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg19`]
///module
#[doc(alias = "DAC_EQ_CFG19")]
pub type DacEqCfg19 = crate::Reg<dac_eq_cfg19::DAC_EQ_CFG19rs>;
///
pub mod dac_eq_cfg19;
///DAC_EQ_CFG20 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg20`]
///module
#[doc(alias = "DAC_EQ_CFG20")]
pub type DacEqCfg20 = crate::Reg<dac_eq_cfg20::DAC_EQ_CFG20rs>;
///
pub mod dac_eq_cfg20;
///DAC_EQ_CFG21 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg21`]
///module
#[doc(alias = "DAC_EQ_CFG21")]
pub type DacEqCfg21 = crate::Reg<dac_eq_cfg21::DAC_EQ_CFG21rs>;
///
pub mod dac_eq_cfg21;
///DAC_EQ_CFG22 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg22`]
///module
#[doc(alias = "DAC_EQ_CFG22")]
pub type DacEqCfg22 = crate::Reg<dac_eq_cfg22::DAC_EQ_CFG22rs>;
///
pub mod dac_eq_cfg22;
///DAC_EQ_CFG23 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg23`]
///module
#[doc(alias = "DAC_EQ_CFG23")]
pub type DacEqCfg23 = crate::Reg<dac_eq_cfg23::DAC_EQ_CFG23rs>;
///
pub mod dac_eq_cfg23;
///DAC_EQ_CFG24 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg24`]
///module
#[doc(alias = "DAC_EQ_CFG24")]
pub type DacEqCfg24 = crate::Reg<dac_eq_cfg24::DAC_EQ_CFG24rs>;
///
pub mod dac_eq_cfg24;
///DAC_EQ_CFG25 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg25`]
///module
#[doc(alias = "DAC_EQ_CFG25")]
pub type DacEqCfg25 = crate::Reg<dac_eq_cfg25::DAC_EQ_CFG25rs>;
///
pub mod dac_eq_cfg25;
///DAC_EQ_CFG26 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg26`]
///module
#[doc(alias = "DAC_EQ_CFG26")]
pub type DacEqCfg26 = crate::Reg<dac_eq_cfg26::DAC_EQ_CFG26rs>;
///
pub mod dac_eq_cfg26;
///DAC_EQ_CFG27 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg27`]
///module
#[doc(alias = "DAC_EQ_CFG27")]
pub type DacEqCfg27 = crate::Reg<dac_eq_cfg27::DAC_EQ_CFG27rs>;
///
pub mod dac_eq_cfg27;
///DAC_EQ_CFG28 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg28`]
///module
#[doc(alias = "DAC_EQ_CFG28")]
pub type DacEqCfg28 = crate::Reg<dac_eq_cfg28::DAC_EQ_CFG28rs>;
///
pub mod dac_eq_cfg28;
///DAC_EQ_CFG29 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg29`]
///module
#[doc(alias = "DAC_EQ_CFG29")]
pub type DacEqCfg29 = crate::Reg<dac_eq_cfg29::DAC_EQ_CFG29rs>;
///
pub mod dac_eq_cfg29;
///DAC_EQ_CFG30 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg30`]
///module
#[doc(alias = "DAC_EQ_CFG30")]
pub type DacEqCfg30 = crate::Reg<dac_eq_cfg30::DAC_EQ_CFG30rs>;
///
pub mod dac_eq_cfg30;
///DAC_EQ_CFG31 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg31`]
///module
#[doc(alias = "DAC_EQ_CFG31")]
pub type DacEqCfg31 = crate::Reg<dac_eq_cfg31::DAC_EQ_CFG31rs>;
///
pub mod dac_eq_cfg31;
///DAC_EQ_CFG32 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg32`]
///module
#[doc(alias = "DAC_EQ_CFG32")]
pub type DacEqCfg32 = crate::Reg<dac_eq_cfg32::DAC_EQ_CFG32rs>;
///
pub mod dac_eq_cfg32;
///DAC_EQ_CFG33 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg33`]
///module
#[doc(alias = "DAC_EQ_CFG33")]
pub type DacEqCfg33 = crate::Reg<dac_eq_cfg33::DAC_EQ_CFG33rs>;
///
pub mod dac_eq_cfg33;
///DAC_EQ_CFG34 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg34`]
///module
#[doc(alias = "DAC_EQ_CFG34")]
pub type DacEqCfg34 = crate::Reg<dac_eq_cfg34::DAC_EQ_CFG34rs>;
///
pub mod dac_eq_cfg34;
///DAC_EQ_CFG35 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg35`]
///module
#[doc(alias = "DAC_EQ_CFG35")]
pub type DacEqCfg35 = crate::Reg<dac_eq_cfg35::DAC_EQ_CFG35rs>;
///
pub mod dac_eq_cfg35;
///DAC_EQ_CFG36 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg36`]
///module
#[doc(alias = "DAC_EQ_CFG36")]
pub type DacEqCfg36 = crate::Reg<dac_eq_cfg36::DAC_EQ_CFG36rs>;
///
pub mod dac_eq_cfg36;
///DAC_EQ_CFG37 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg37`]
///module
#[doc(alias = "DAC_EQ_CFG37")]
pub type DacEqCfg37 = crate::Reg<dac_eq_cfg37::DAC_EQ_CFG37rs>;
///
pub mod dac_eq_cfg37;
///DAC_EQ_CFG38 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg38`]
///module
#[doc(alias = "DAC_EQ_CFG38")]
pub type DacEqCfg38 = crate::Reg<dac_eq_cfg38::DAC_EQ_CFG38rs>;
///
pub mod dac_eq_cfg38;
///DAC_EQ_CFG39 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg39`]
///module
#[doc(alias = "DAC_EQ_CFG39")]
pub type DacEqCfg39 = crate::Reg<dac_eq_cfg39::DAC_EQ_CFG39rs>;
///
pub mod dac_eq_cfg39;
///DAC_EQ_CFG40 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg40`]
///module
#[doc(alias = "DAC_EQ_CFG40")]
pub type DacEqCfg40 = crate::Reg<dac_eq_cfg40::DAC_EQ_CFG40rs>;
///
pub mod dac_eq_cfg40;
///DAC_EQ_CFG41 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg41`]
///module
#[doc(alias = "DAC_EQ_CFG41")]
pub type DacEqCfg41 = crate::Reg<dac_eq_cfg41::DAC_EQ_CFG41rs>;
///
pub mod dac_eq_cfg41;
///DAC_EQ_CFG42 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg42`]
///module
#[doc(alias = "DAC_EQ_CFG42")]
pub type DacEqCfg42 = crate::Reg<dac_eq_cfg42::DAC_EQ_CFG42rs>;
///
pub mod dac_eq_cfg42;
///DAC_EQ_CFG43 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg43`]
///module
#[doc(alias = "DAC_EQ_CFG43")]
pub type DacEqCfg43 = crate::Reg<dac_eq_cfg43::DAC_EQ_CFG43rs>;
///
pub mod dac_eq_cfg43;
///DAC_EQ_CFG44 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg44`]
///module
#[doc(alias = "DAC_EQ_CFG44")]
pub type DacEqCfg44 = crate::Reg<dac_eq_cfg44::DAC_EQ_CFG44rs>;
///
pub mod dac_eq_cfg44;
///DAC_EQ_CFG45 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg45`]
///module
#[doc(alias = "DAC_EQ_CFG45")]
pub type DacEqCfg45 = crate::Reg<dac_eq_cfg45::DAC_EQ_CFG45rs>;
///
pub mod dac_eq_cfg45;
///DAC_EQ_CFG46 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg46`]
///module
#[doc(alias = "DAC_EQ_CFG46")]
pub type DacEqCfg46 = crate::Reg<dac_eq_cfg46::DAC_EQ_CFG46rs>;
///
pub mod dac_eq_cfg46;
///DAC_EQ_CFG47 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg47`]
///module
#[doc(alias = "DAC_EQ_CFG47")]
pub type DacEqCfg47 = crate::Reg<dac_eq_cfg47::DAC_EQ_CFG47rs>;
///
pub mod dac_eq_cfg47;
///DAC_EQ_CFG48 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg48`]
///module
#[doc(alias = "DAC_EQ_CFG48")]
pub type DacEqCfg48 = crate::Reg<dac_eq_cfg48::DAC_EQ_CFG48rs>;
///
pub mod dac_eq_cfg48;
///DAC_EQ_CFG49 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dac_eq_cfg49`]
///module
#[doc(alias = "DAC_EQ_CFG49")]
pub type DacEqCfg49 = crate::Reg<dac_eq_cfg49::DAC_EQ_CFG49rs>;
///
pub mod dac_eq_cfg49;
