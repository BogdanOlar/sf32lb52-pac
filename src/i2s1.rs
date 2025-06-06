#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    tx_pcm_format: TxPcmFormat,
    _reserved1: [u8; 0x0c],
    tx_pcm_sample_clk: TxPcmSampleClk,
    _reserved2: [u8; 0x0c],
    tx_rs_smooth: TxRsSmooth,
    _reserved3: [u8; 0x0c],
    tx_pcm_ch_sel: TxPcmChSel,
    _reserved4: [u8; 0x0c],
    tx_vol_ctrl: TxVolCtrl,
    _reserved5: [u8; 0x0c],
    tx_lr_bal_ctrl: TxLrBalCtrl,
    _reserved6: [u8; 0x0c],
    audio_tx_lrck_div: AudioTxLrckDiv,
    _reserved7: [u8; 0x0c],
    audio_tx_bclk_div: AudioTxBclkDiv,
    _reserved8: [u8; 0x0c],
    audio_tx_format: AudioTxFormat,
    _reserved9: [u8; 0x0c],
    audio_serial_timing: AudioSerialTiming,
    _reserved10: [u8; 0x0c],
    audio_tx_func_en: AudioTxFuncEn,
    _reserved11: [u8; 0x0c],
    audio_tx_pause: AudioTxPause,
    _reserved12: [u8; 0x04],
    audio_i2s_sl_merge: AudioI2sSlMerge,
    _reserved13: [u8; 0x34],
    audio_rx_func_en: AudioRxFuncEn,
    _reserved14: [u8; 0x0c],
    audio_rx_pause: AudioRxPause,
    _reserved15: [u8; 0x0c],
    audio_rx_serial_timing: AudioRxSerialTiming,
    _reserved16: [u8; 0x0c],
    audio_rx_pcm_dw: AudioRxPcmDw,
    _reserved17: [u8; 0x0c],
    audio_rx_lrck_div: AudioRxLrckDiv,
    _reserved18: [u8; 0x0c],
    audio_rx_bclk_div: AudioRxBclkDiv,
    _reserved19: [u8; 0x0c],
    record_data_sel: RecordDataSel,
    _reserved20: [u8; 0x0c],
    rx_re_sample_clk_div: RxReSampleClkDiv,
    _reserved21: [u8; 0x0c],
    rx_re_sample: RxReSample,
    _reserved22: [u8; 0x0c],
    record_format: RecordFormat,
    _reserved23: [u8; 0x0c],
    rx_ch_sel: RxChSel,
    _reserved24: [u8; 0x5c],
    bt_phone_ctrl: BtPhoneCtrl,
    _reserved25: [u8; 0x0c],
    bb_pcm_format: BbPcmFormat,
    _reserved26: [u8; 0x0c],
    bt_pcm_dw: BtPcmDw,
    _reserved27: [u8; 0x0c],
    bt_pcm_timing: BtPcmTiming,
    _reserved28: [u8; 0x0c],
    bt_pcm_clk_duty: BtPcmClkDuty,
    _reserved29: [u8; 0x0c],
    bt_pcm_sync_duty: BtPcmSyncDuty,
    _reserved30: [u8; 0x0c],
    bt_vol_ctrl: BtVolCtrl,
    _reserved31: [u8; 0x9c],
    int_mask: IntMask,
    _reserved32: [u8; 0x0c],
    int_status: IntStatus,
    _reserved33: [u8; 0xec],
    tx_dma_entry: TxDmaEntry,
    _reserved34: [u8; 0x3c],
    rx_dma_entry: RxDmaEntry,
    _reserved35: [u8; 0x3c],
    dma_mask: DmaMask,
    _reserved36: [u8; 0x7c],
    debug_loop: DebugLoop,
    _reserved37: [u8; 0xfc],
    fifo_status: FifoStatus,
    _reserved38: [u8; 0xfc],
    tx_equalizer_en: TxEqualizerEn,
    _reserved39: [u8; 0x0c],
    tx_equalizer_gain1: TxEqualizerGain1,
    _reserved40: [u8; 0x0c],
    tx_equalizer_gain2: TxEqualizerGain2,
}
impl RegisterBlock {
    ///0x10 -
    #[inline(always)]
    pub const fn tx_pcm_format(&self) -> &TxPcmFormat {
        &self.tx_pcm_format
    }
    ///0x20 -
    #[inline(always)]
    pub const fn tx_pcm_sample_clk(&self) -> &TxPcmSampleClk {
        &self.tx_pcm_sample_clk
    }
    ///0x30 -
    #[inline(always)]
    pub const fn tx_rs_smooth(&self) -> &TxRsSmooth {
        &self.tx_rs_smooth
    }
    ///0x40 -
    #[inline(always)]
    pub const fn tx_pcm_ch_sel(&self) -> &TxPcmChSel {
        &self.tx_pcm_ch_sel
    }
    ///0x50 -
    #[inline(always)]
    pub const fn tx_vol_ctrl(&self) -> &TxVolCtrl {
        &self.tx_vol_ctrl
    }
    ///0x60 -
    #[inline(always)]
    pub const fn tx_lr_bal_ctrl(&self) -> &TxLrBalCtrl {
        &self.tx_lr_bal_ctrl
    }
    ///0x70 -
    #[inline(always)]
    pub const fn audio_tx_lrck_div(&self) -> &AudioTxLrckDiv {
        &self.audio_tx_lrck_div
    }
    ///0x80 -
    #[inline(always)]
    pub const fn audio_tx_bclk_div(&self) -> &AudioTxBclkDiv {
        &self.audio_tx_bclk_div
    }
    ///0x90 -
    #[inline(always)]
    pub const fn audio_tx_format(&self) -> &AudioTxFormat {
        &self.audio_tx_format
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn audio_serial_timing(&self) -> &AudioSerialTiming {
        &self.audio_serial_timing
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn audio_tx_func_en(&self) -> &AudioTxFuncEn {
        &self.audio_tx_func_en
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn audio_tx_pause(&self) -> &AudioTxPause {
        &self.audio_tx_pause
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn audio_i2s_sl_merge(&self) -> &AudioI2sSlMerge {
        &self.audio_i2s_sl_merge
    }
    ///0x100 -
    #[inline(always)]
    pub const fn audio_rx_func_en(&self) -> &AudioRxFuncEn {
        &self.audio_rx_func_en
    }
    ///0x110 -
    #[inline(always)]
    pub const fn audio_rx_pause(&self) -> &AudioRxPause {
        &self.audio_rx_pause
    }
    ///0x120 -
    #[inline(always)]
    pub const fn audio_rx_serial_timing(&self) -> &AudioRxSerialTiming {
        &self.audio_rx_serial_timing
    }
    ///0x130 -
    #[inline(always)]
    pub const fn audio_rx_pcm_dw(&self) -> &AudioRxPcmDw {
        &self.audio_rx_pcm_dw
    }
    ///0x140 -
    #[inline(always)]
    pub const fn audio_rx_lrck_div(&self) -> &AudioRxLrckDiv {
        &self.audio_rx_lrck_div
    }
    ///0x150 -
    #[inline(always)]
    pub const fn audio_rx_bclk_div(&self) -> &AudioRxBclkDiv {
        &self.audio_rx_bclk_div
    }
    ///0x160 -
    #[inline(always)]
    pub const fn record_data_sel(&self) -> &RecordDataSel {
        &self.record_data_sel
    }
    ///0x170 -
    #[inline(always)]
    pub const fn rx_re_sample_clk_div(&self) -> &RxReSampleClkDiv {
        &self.rx_re_sample_clk_div
    }
    ///0x180 -
    #[inline(always)]
    pub const fn rx_re_sample(&self) -> &RxReSample {
        &self.rx_re_sample
    }
    ///0x190 -
    #[inline(always)]
    pub const fn record_format(&self) -> &RecordFormat {
        &self.record_format
    }
    ///0x1a0 -
    #[inline(always)]
    pub const fn rx_ch_sel(&self) -> &RxChSel {
        &self.rx_ch_sel
    }
    ///0x200 -
    #[inline(always)]
    pub const fn bt_phone_ctrl(&self) -> &BtPhoneCtrl {
        &self.bt_phone_ctrl
    }
    ///0x210 -
    #[inline(always)]
    pub const fn bb_pcm_format(&self) -> &BbPcmFormat {
        &self.bb_pcm_format
    }
    ///0x220 -
    #[inline(always)]
    pub const fn bt_pcm_dw(&self) -> &BtPcmDw {
        &self.bt_pcm_dw
    }
    ///0x230 -
    #[inline(always)]
    pub const fn bt_pcm_timing(&self) -> &BtPcmTiming {
        &self.bt_pcm_timing
    }
    ///0x240 -
    #[inline(always)]
    pub const fn bt_pcm_clk_duty(&self) -> &BtPcmClkDuty {
        &self.bt_pcm_clk_duty
    }
    ///0x250 -
    #[inline(always)]
    pub const fn bt_pcm_sync_duty(&self) -> &BtPcmSyncDuty {
        &self.bt_pcm_sync_duty
    }
    ///0x260 -
    #[inline(always)]
    pub const fn bt_vol_ctrl(&self) -> &BtVolCtrl {
        &self.bt_vol_ctrl
    }
    ///0x300 -
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    ///0x310 -
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    ///0x400 -
    #[inline(always)]
    pub const fn tx_dma_entry(&self) -> &TxDmaEntry {
        &self.tx_dma_entry
    }
    ///0x440 -
    #[inline(always)]
    pub const fn rx_dma_entry(&self) -> &RxDmaEntry {
        &self.rx_dma_entry
    }
    ///0x480 -
    #[inline(always)]
    pub const fn dma_mask(&self) -> &DmaMask {
        &self.dma_mask
    }
    ///0x500 -
    #[inline(always)]
    pub const fn debug_loop(&self) -> &DebugLoop {
        &self.debug_loop
    }
    ///0x600 -
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FifoStatus {
        &self.fifo_status
    }
    ///0x700 -
    #[inline(always)]
    pub const fn tx_equalizer_en(&self) -> &TxEqualizerEn {
        &self.tx_equalizer_en
    }
    ///0x710 -
    #[inline(always)]
    pub const fn tx_equalizer_gain1(&self) -> &TxEqualizerGain1 {
        &self.tx_equalizer_gain1
    }
    ///0x720 -
    #[inline(always)]
    pub const fn tx_equalizer_gain2(&self) -> &TxEqualizerGain2 {
        &self.tx_equalizer_gain2
    }
}
///TX_PCM_FORMAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_pcm_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_pcm_format`]
///module
#[doc(alias = "TX_PCM_FORMAT")]
pub type TxPcmFormat = crate::Reg<tx_pcm_format::TX_PCM_FORMATrs>;
///
pub mod tx_pcm_format;
///TX_PCM_SAMPLE_CLK (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_pcm_sample_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm_sample_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_pcm_sample_clk`]
///module
#[doc(alias = "TX_PCM_SAMPLE_CLK")]
pub type TxPcmSampleClk = crate::Reg<tx_pcm_sample_clk::TX_PCM_SAMPLE_CLKrs>;
///
pub mod tx_pcm_sample_clk;
///TX_RS_SMOOTH (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_rs_smooth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_rs_smooth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_rs_smooth`]
///module
#[doc(alias = "TX_RS_SMOOTH")]
pub type TxRsSmooth = crate::Reg<tx_rs_smooth::TX_RS_SMOOTHrs>;
///
pub mod tx_rs_smooth;
///TX_PCM_CH_SEL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_pcm_ch_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm_ch_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_pcm_ch_sel`]
///module
#[doc(alias = "TX_PCM_CH_SEL")]
pub type TxPcmChSel = crate::Reg<tx_pcm_ch_sel::TX_PCM_CH_SELrs>;
///
pub mod tx_pcm_ch_sel;
///TX_VOL_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_vol_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_vol_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_vol_ctrl`]
///module
#[doc(alias = "TX_VOL_CTRL")]
pub type TxVolCtrl = crate::Reg<tx_vol_ctrl::TX_VOL_CTRLrs>;
///
pub mod tx_vol_ctrl;
///TX_LR_BAL_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_lr_bal_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_lr_bal_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_lr_bal_ctrl`]
///module
#[doc(alias = "TX_LR_BAL_CTRL")]
pub type TxLrBalCtrl = crate::Reg<tx_lr_bal_ctrl::TX_LR_BAL_CTRLrs>;
///
pub mod tx_lr_bal_ctrl;
///AUDIO_TX_LRCK_DIV (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_lrck_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_lrck_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_tx_lrck_div`]
///module
#[doc(alias = "AUDIO_TX_LRCK_DIV")]
pub type AudioTxLrckDiv = crate::Reg<audio_tx_lrck_div::AUDIO_TX_LRCK_DIVrs>;
///
pub mod audio_tx_lrck_div;
///AUDIO_TX_BCLK_DIV (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_bclk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_bclk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_tx_bclk_div`]
///module
#[doc(alias = "AUDIO_TX_BCLK_DIV")]
pub type AudioTxBclkDiv = crate::Reg<audio_tx_bclk_div::AUDIO_TX_BCLK_DIVrs>;
///
pub mod audio_tx_bclk_div;
///AUDIO_TX_FORMAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_tx_format`]
///module
#[doc(alias = "AUDIO_TX_FORMAT")]
pub type AudioTxFormat = crate::Reg<audio_tx_format::AUDIO_TX_FORMATrs>;
///
pub mod audio_tx_format;
///AUDIO_SERIAL_TIMING (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_serial_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_serial_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_serial_timing`]
///module
#[doc(alias = "AUDIO_SERIAL_TIMING")]
pub type AudioSerialTiming = crate::Reg<audio_serial_timing::AUDIO_SERIAL_TIMINGrs>;
///
pub mod audio_serial_timing;
///AUDIO_TX_FUNC_EN (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_func_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_func_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_tx_func_en`]
///module
#[doc(alias = "AUDIO_TX_FUNC_EN")]
pub type AudioTxFuncEn = crate::Reg<audio_tx_func_en::AUDIO_TX_FUNC_ENrs>;
///
pub mod audio_tx_func_en;
///AUDIO_TX_PAUSE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_pause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_pause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_tx_pause`]
///module
#[doc(alias = "AUDIO_TX_PAUSE")]
pub type AudioTxPause = crate::Reg<audio_tx_pause::AUDIO_TX_PAUSErs>;
///
pub mod audio_tx_pause;
///AUDIO_I2S_SL_MERGE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_i2s_sl_merge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_i2s_sl_merge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_i2s_sl_merge`]
///module
#[doc(alias = "AUDIO_I2S_SL_MERGE")]
pub type AudioI2sSlMerge = crate::Reg<audio_i2s_sl_merge::AUDIO_I2S_SL_MERGErs>;
///
pub mod audio_i2s_sl_merge;
///AUDIO_RX_FUNC_EN (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_func_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_func_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_rx_func_en`]
///module
#[doc(alias = "AUDIO_RX_FUNC_EN")]
pub type AudioRxFuncEn = crate::Reg<audio_rx_func_en::AUDIO_RX_FUNC_ENrs>;
///
pub mod audio_rx_func_en;
///AUDIO_RX_PAUSE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_pause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_pause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_rx_pause`]
///module
#[doc(alias = "AUDIO_RX_PAUSE")]
pub type AudioRxPause = crate::Reg<audio_rx_pause::AUDIO_RX_PAUSErs>;
///
pub mod audio_rx_pause;
///AUDIO_RX_SERIAL_TIMING (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_serial_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_serial_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_rx_serial_timing`]
///module
#[doc(alias = "AUDIO_RX_SERIAL_TIMING")]
pub type AudioRxSerialTiming = crate::Reg<audio_rx_serial_timing::AUDIO_RX_SERIAL_TIMINGrs>;
///
pub mod audio_rx_serial_timing;
///AUDIO_RX_PCM_DW (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_pcm_dw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_pcm_dw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_rx_pcm_dw`]
///module
#[doc(alias = "AUDIO_RX_PCM_DW")]
pub type AudioRxPcmDw = crate::Reg<audio_rx_pcm_dw::AUDIO_RX_PCM_DWrs>;
///
pub mod audio_rx_pcm_dw;
///AUDIO_RX_LRCK_DIV (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_lrck_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_lrck_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_rx_lrck_div`]
///module
#[doc(alias = "AUDIO_RX_LRCK_DIV")]
pub type AudioRxLrckDiv = crate::Reg<audio_rx_lrck_div::AUDIO_RX_LRCK_DIVrs>;
///
pub mod audio_rx_lrck_div;
///AUDIO_RX_BCLK_DIV (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_bclk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_bclk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@audio_rx_bclk_div`]
///module
#[doc(alias = "AUDIO_RX_BCLK_DIV")]
pub type AudioRxBclkDiv = crate::Reg<audio_rx_bclk_div::AUDIO_RX_BCLK_DIVrs>;
///
pub mod audio_rx_bclk_div;
///RECORD_DATA_SEL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`record_data_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`record_data_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@record_data_sel`]
///module
#[doc(alias = "RECORD_DATA_SEL")]
pub type RecordDataSel = crate::Reg<record_data_sel::RECORD_DATA_SELrs>;
///
pub mod record_data_sel;
///RX_RE_SAMPLE_CLK_DIV (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_re_sample_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_re_sample_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_re_sample_clk_div`]
///module
#[doc(alias = "RX_RE_SAMPLE_CLK_DIV")]
pub type RxReSampleClkDiv = crate::Reg<rx_re_sample_clk_div::RX_RE_SAMPLE_CLK_DIVrs>;
///
pub mod rx_re_sample_clk_div;
///RX_RE_SAMPLE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_re_sample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_re_sample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_re_sample`]
///module
#[doc(alias = "RX_RE_SAMPLE")]
pub type RxReSample = crate::Reg<rx_re_sample::RX_RE_SAMPLErs>;
///
pub mod rx_re_sample;
///RECORD_FORMAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`record_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`record_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@record_format`]
///module
#[doc(alias = "RECORD_FORMAT")]
pub type RecordFormat = crate::Reg<record_format::RECORD_FORMATrs>;
///
pub mod record_format;
///RX_CH_SEL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_ch_sel`]
///module
#[doc(alias = "RX_CH_SEL")]
pub type RxChSel = crate::Reg<rx_ch_sel::RX_CH_SELrs>;
///
pub mod rx_ch_sel;
///BT_PHONE_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bt_phone_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_phone_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bt_phone_ctrl`]
///module
#[doc(alias = "BT_PHONE_CTRL")]
pub type BtPhoneCtrl = crate::Reg<bt_phone_ctrl::BT_PHONE_CTRLrs>;
///
pub mod bt_phone_ctrl;
///BB_PCM_FORMAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bb_pcm_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bb_pcm_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bb_pcm_format`]
///module
#[doc(alias = "BB_PCM_FORMAT")]
pub type BbPcmFormat = crate::Reg<bb_pcm_format::BB_PCM_FORMATrs>;
///
pub mod bb_pcm_format;
///BT_PCM_DW (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_dw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_dw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bt_pcm_dw`]
///module
#[doc(alias = "BT_PCM_DW")]
pub type BtPcmDw = crate::Reg<bt_pcm_dw::BT_PCM_DWrs>;
///
pub mod bt_pcm_dw;
///BT_PCM_TIMING (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bt_pcm_timing`]
///module
#[doc(alias = "BT_PCM_TIMING")]
pub type BtPcmTiming = crate::Reg<bt_pcm_timing::BT_PCM_TIMINGrs>;
///
pub mod bt_pcm_timing;
///BT_PCM_CLK_DUTY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_clk_duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_clk_duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bt_pcm_clk_duty`]
///module
#[doc(alias = "BT_PCM_CLK_DUTY")]
pub type BtPcmClkDuty = crate::Reg<bt_pcm_clk_duty::BT_PCM_CLK_DUTYrs>;
///
pub mod bt_pcm_clk_duty;
///BT_PCM_SYNC_DUTY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_sync_duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_sync_duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bt_pcm_sync_duty`]
///module
#[doc(alias = "BT_PCM_SYNC_DUTY")]
pub type BtPcmSyncDuty = crate::Reg<bt_pcm_sync_duty::BT_PCM_SYNC_DUTYrs>;
///
pub mod bt_pcm_sync_duty;
///BT_VOL_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`bt_vol_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_vol_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@bt_vol_ctrl`]
///module
#[doc(alias = "BT_VOL_CTRL")]
pub type BtVolCtrl = crate::Reg<bt_vol_ctrl::BT_VOL_CTRLrs>;
///
pub mod bt_vol_ctrl;
///INT_MASK (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_mask`]
///module
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::INT_MASKrs>;
///
pub mod int_mask;
///INT_STATUS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_status`]
///module
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::INT_STATUSrs>;
///
pub mod int_status;
///TX_DMA_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_dma_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_dma_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_dma_entry`]
///module
#[doc(alias = "TX_DMA_ENTRY")]
pub type TxDmaEntry = crate::Reg<tx_dma_entry::TX_DMA_ENTRYrs>;
///
pub mod tx_dma_entry;
///RX_DMA_ENTRY (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`rx_dma_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_dma_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@rx_dma_entry`]
///module
#[doc(alias = "RX_DMA_ENTRY")]
pub type RxDmaEntry = crate::Reg<rx_dma_entry::RX_DMA_ENTRYrs>;
///
pub mod rx_dma_entry;
///DMA_MASK (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dma_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dma_mask`]
///module
#[doc(alias = "DMA_MASK")]
pub type DmaMask = crate::Reg<dma_mask::DMA_MASKrs>;
///
pub mod dma_mask;
///DEBUG_LOOP (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`debug_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@debug_loop`]
///module
#[doc(alias = "DEBUG_LOOP")]
pub type DebugLoop = crate::Reg<debug_loop::DEBUG_LOOPrs>;
///
pub mod debug_loop;
///FIFO_STATUS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@fifo_status`]
///module
#[doc(alias = "FIFO_STATUS")]
pub type FifoStatus = crate::Reg<fifo_status::FIFO_STATUSrs>;
///
pub mod fifo_status;
///TX_EQUALIZER_EN (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_equalizer_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_equalizer_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_equalizer_en`]
///module
#[doc(alias = "TX_EQUALIZER_EN")]
pub type TxEqualizerEn = crate::Reg<tx_equalizer_en::TX_EQUALIZER_ENrs>;
///
pub mod tx_equalizer_en;
///TX_EQUALIZER_GAIN1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_equalizer_gain1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_equalizer_gain1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_equalizer_gain1`]
///module
#[doc(alias = "TX_EQUALIZER_GAIN1")]
pub type TxEqualizerGain1 = crate::Reg<tx_equalizer_gain1::TX_EQUALIZER_GAIN1rs>;
///
pub mod tx_equalizer_gain1;
///TX_EQUALIZER_GAIN2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`tx_equalizer_gain2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_equalizer_gain2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@tx_equalizer_gain2`]
///module
#[doc(alias = "TX_EQUALIZER_GAIN2")]
pub type TxEqualizerGain2 = crate::Reg<tx_equalizer_gain2::TX_EQUALIZER_GAIN2rs>;
///
pub mod tx_equalizer_gain2;
