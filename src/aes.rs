#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    command: Command,
    status: Status,
    irq: Irq,
    setting: Setting,
    aes_setting: AesSetting,
    dma_in: DmaIn,
    dma_out: DmaOut,
    dma_data: DmaData,
    iv_w0: IvW0,
    iv_w1: IvW1,
    iv_w2: IvW2,
    iv_w3: IvW3,
    ext_key_w0: ExtKeyW0,
    ext_key_w1: ExtKeyW1,
    ext_key_w2: ExtKeyW2,
    ext_key_w3: ExtKeyW3,
    ext_key_w4: ExtKeyW4,
    ext_key_w5: ExtKeyW5,
    ext_key_w6: ExtKeyW6,
    ext_key_w7: ExtKeyW7,
    hash_setting: HashSetting,
    hash_dma_in: HashDmaIn,
    hash_dma_data: HashDmaData,
    hash_iv_h0: HashIvH0,
    hash_iv_h1: HashIvH1,
    hash_iv_h2: HashIvH2,
    hash_iv_h3: HashIvH3,
    hash_iv_h4: HashIvH4,
    hash_iv_h5: HashIvH5,
    hash_iv_h6: HashIvH6,
    hash_iv_h7: HashIvH7,
    hash_result_h0: HashResultH0,
    hash_result_h1: HashResultH1,
    hash_result_h2: HashResultH2,
    hash_result_h3: HashResultH3,
    hash_result_h4: HashResultH4,
    hash_result_h5: HashResultH5,
    hash_result_h6: HashResultH6,
    hash_result_h7: HashResultH7,
    hash_len_l: HashLenL,
    hash_len_h: HashLenH,
    hash_result_len_l: HashResultLenL,
    hash_result_len_h: HashResultLenH,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    ///0x04 -
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    ///0x08 -
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    ///0x0c -
    #[inline(always)]
    pub const fn setting(&self) -> &Setting {
        &self.setting
    }
    ///0x10 -
    #[inline(always)]
    pub const fn aes_setting(&self) -> &AesSetting {
        &self.aes_setting
    }
    ///0x14 -
    #[inline(always)]
    pub const fn dma_in(&self) -> &DmaIn {
        &self.dma_in
    }
    ///0x18 -
    #[inline(always)]
    pub const fn dma_out(&self) -> &DmaOut {
        &self.dma_out
    }
    ///0x1c -
    #[inline(always)]
    pub const fn dma_data(&self) -> &DmaData {
        &self.dma_data
    }
    ///0x20 -
    #[inline(always)]
    pub const fn iv_w0(&self) -> &IvW0 {
        &self.iv_w0
    }
    ///0x24 -
    #[inline(always)]
    pub const fn iv_w1(&self) -> &IvW1 {
        &self.iv_w1
    }
    ///0x28 -
    #[inline(always)]
    pub const fn iv_w2(&self) -> &IvW2 {
        &self.iv_w2
    }
    ///0x2c -
    #[inline(always)]
    pub const fn iv_w3(&self) -> &IvW3 {
        &self.iv_w3
    }
    ///0x30 -
    #[inline(always)]
    pub const fn ext_key_w0(&self) -> &ExtKeyW0 {
        &self.ext_key_w0
    }
    ///0x34 -
    #[inline(always)]
    pub const fn ext_key_w1(&self) -> &ExtKeyW1 {
        &self.ext_key_w1
    }
    ///0x38 -
    #[inline(always)]
    pub const fn ext_key_w2(&self) -> &ExtKeyW2 {
        &self.ext_key_w2
    }
    ///0x3c -
    #[inline(always)]
    pub const fn ext_key_w3(&self) -> &ExtKeyW3 {
        &self.ext_key_w3
    }
    ///0x40 -
    #[inline(always)]
    pub const fn ext_key_w4(&self) -> &ExtKeyW4 {
        &self.ext_key_w4
    }
    ///0x44 -
    #[inline(always)]
    pub const fn ext_key_w5(&self) -> &ExtKeyW5 {
        &self.ext_key_w5
    }
    ///0x48 -
    #[inline(always)]
    pub const fn ext_key_w6(&self) -> &ExtKeyW6 {
        &self.ext_key_w6
    }
    ///0x4c -
    #[inline(always)]
    pub const fn ext_key_w7(&self) -> &ExtKeyW7 {
        &self.ext_key_w7
    }
    ///0x50 -
    #[inline(always)]
    pub const fn hash_setting(&self) -> &HashSetting {
        &self.hash_setting
    }
    ///0x54 -
    #[inline(always)]
    pub const fn hash_dma_in(&self) -> &HashDmaIn {
        &self.hash_dma_in
    }
    ///0x58 -
    #[inline(always)]
    pub const fn hash_dma_data(&self) -> &HashDmaData {
        &self.hash_dma_data
    }
    ///0x5c -
    #[inline(always)]
    pub const fn hash_iv_h0(&self) -> &HashIvH0 {
        &self.hash_iv_h0
    }
    ///0x60 -
    #[inline(always)]
    pub const fn hash_iv_h1(&self) -> &HashIvH1 {
        &self.hash_iv_h1
    }
    ///0x64 -
    #[inline(always)]
    pub const fn hash_iv_h2(&self) -> &HashIvH2 {
        &self.hash_iv_h2
    }
    ///0x68 -
    #[inline(always)]
    pub const fn hash_iv_h3(&self) -> &HashIvH3 {
        &self.hash_iv_h3
    }
    ///0x6c -
    #[inline(always)]
    pub const fn hash_iv_h4(&self) -> &HashIvH4 {
        &self.hash_iv_h4
    }
    ///0x70 -
    #[inline(always)]
    pub const fn hash_iv_h5(&self) -> &HashIvH5 {
        &self.hash_iv_h5
    }
    ///0x74 -
    #[inline(always)]
    pub const fn hash_iv_h6(&self) -> &HashIvH6 {
        &self.hash_iv_h6
    }
    ///0x78 -
    #[inline(always)]
    pub const fn hash_iv_h7(&self) -> &HashIvH7 {
        &self.hash_iv_h7
    }
    ///0x7c -
    #[inline(always)]
    pub const fn hash_result_h0(&self) -> &HashResultH0 {
        &self.hash_result_h0
    }
    ///0x80 -
    #[inline(always)]
    pub const fn hash_result_h1(&self) -> &HashResultH1 {
        &self.hash_result_h1
    }
    ///0x84 -
    #[inline(always)]
    pub const fn hash_result_h2(&self) -> &HashResultH2 {
        &self.hash_result_h2
    }
    ///0x88 -
    #[inline(always)]
    pub const fn hash_result_h3(&self) -> &HashResultH3 {
        &self.hash_result_h3
    }
    ///0x8c -
    #[inline(always)]
    pub const fn hash_result_h4(&self) -> &HashResultH4 {
        &self.hash_result_h4
    }
    ///0x90 -
    #[inline(always)]
    pub const fn hash_result_h5(&self) -> &HashResultH5 {
        &self.hash_result_h5
    }
    ///0x94 -
    #[inline(always)]
    pub const fn hash_result_h6(&self) -> &HashResultH6 {
        &self.hash_result_h6
    }
    ///0x98 -
    #[inline(always)]
    pub const fn hash_result_h7(&self) -> &HashResultH7 {
        &self.hash_result_h7
    }
    ///0x9c -
    #[inline(always)]
    pub const fn hash_len_l(&self) -> &HashLenL {
        &self.hash_len_l
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn hash_len_h(&self) -> &HashLenH {
        &self.hash_len_h
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn hash_result_len_l(&self) -> &HashResultLenL {
        &self.hash_result_len_l
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn hash_result_len_h(&self) -> &HashResultLenH {
        &self.hash_result_len_h
    }
}
///COMMAND (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@command`]
///module
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::COMMANDrs>;
///
pub mod command;
///STATUS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@status`]
///module
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::STATUSrs>;
///
pub mod status;
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
///SETTING (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@setting`]
///module
#[doc(alias = "SETTING")]
pub type Setting = crate::Reg<setting::SETTINGrs>;
///
pub mod setting;
///AES_SETTING (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`aes_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aes_setting`]
///module
#[doc(alias = "AES_SETTING")]
pub type AesSetting = crate::Reg<aes_setting::AES_SETTINGrs>;
///
pub mod aes_setting;
///DMA_IN (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dma_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dma_in`]
///module
#[doc(alias = "DMA_IN")]
pub type DmaIn = crate::Reg<dma_in::DMA_INrs>;
///
pub mod dma_in;
///DMA_OUT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dma_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dma_out`]
///module
#[doc(alias = "DMA_OUT")]
pub type DmaOut = crate::Reg<dma_out::DMA_OUTrs>;
///
pub mod dma_out;
///DMA_DATA (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dma_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dma_data`]
///module
#[doc(alias = "DMA_DATA")]
pub type DmaData = crate::Reg<dma_data::DMA_DATArs>;
///
pub mod dma_data;
///IV_W0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`iv_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iv_w0`]
///module
#[doc(alias = "IV_W0")]
pub type IvW0 = crate::Reg<iv_w0::IV_W0rs>;
///
pub mod iv_w0;
///IV_W1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`iv_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iv_w1`]
///module
#[doc(alias = "IV_W1")]
pub type IvW1 = crate::Reg<iv_w1::IV_W1rs>;
///
pub mod iv_w1;
///IV_W2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`iv_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iv_w2`]
///module
#[doc(alias = "IV_W2")]
pub type IvW2 = crate::Reg<iv_w2::IV_W2rs>;
///
pub mod iv_w2;
///IV_W3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`iv_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@iv_w3`]
///module
#[doc(alias = "IV_W3")]
pub type IvW3 = crate::Reg<iv_w3::IV_W3rs>;
///
pub mod iv_w3;
///EXT_KEY_W0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w0`]
///module
#[doc(alias = "EXT_KEY_W0")]
pub type ExtKeyW0 = crate::Reg<ext_key_w0::EXT_KEY_W0rs>;
///
pub mod ext_key_w0;
///EXT_KEY_W1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w1`]
///module
#[doc(alias = "EXT_KEY_W1")]
pub type ExtKeyW1 = crate::Reg<ext_key_w1::EXT_KEY_W1rs>;
///
pub mod ext_key_w1;
///EXT_KEY_W2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w2`]
///module
#[doc(alias = "EXT_KEY_W2")]
pub type ExtKeyW2 = crate::Reg<ext_key_w2::EXT_KEY_W2rs>;
///
pub mod ext_key_w2;
///EXT_KEY_W3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w3`]
///module
#[doc(alias = "EXT_KEY_W3")]
pub type ExtKeyW3 = crate::Reg<ext_key_w3::EXT_KEY_W3rs>;
///
pub mod ext_key_w3;
///EXT_KEY_W4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w4`]
///module
#[doc(alias = "EXT_KEY_W4")]
pub type ExtKeyW4 = crate::Reg<ext_key_w4::EXT_KEY_W4rs>;
///
pub mod ext_key_w4;
///EXT_KEY_W5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w5`]
///module
#[doc(alias = "EXT_KEY_W5")]
pub type ExtKeyW5 = crate::Reg<ext_key_w5::EXT_KEY_W5rs>;
///
pub mod ext_key_w5;
///EXT_KEY_W6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w6`]
///module
#[doc(alias = "EXT_KEY_W6")]
pub type ExtKeyW6 = crate::Reg<ext_key_w6::EXT_KEY_W6rs>;
///
pub mod ext_key_w6;
///EXT_KEY_W7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ext_key_w7`]
///module
#[doc(alias = "EXT_KEY_W7")]
pub type ExtKeyW7 = crate::Reg<ext_key_w7::EXT_KEY_W7rs>;
///
pub mod ext_key_w7;
///HASH_SETTING (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_setting`]
///module
#[doc(alias = "HASH_SETTING")]
pub type HashSetting = crate::Reg<hash_setting::HASH_SETTINGrs>;
///
pub mod hash_setting;
///HASH_DMA_IN (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_dma_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_dma_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_dma_in`]
///module
#[doc(alias = "HASH_DMA_IN")]
pub type HashDmaIn = crate::Reg<hash_dma_in::HASH_DMA_INrs>;
///
pub mod hash_dma_in;
///HASH_DMA_DATA (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_dma_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_dma_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_dma_data`]
///module
#[doc(alias = "HASH_DMA_DATA")]
pub type HashDmaData = crate::Reg<hash_dma_data::HASH_DMA_DATArs>;
///
pub mod hash_dma_data;
///HASH_IV_H0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h0`]
///module
#[doc(alias = "HASH_IV_H0")]
pub type HashIvH0 = crate::Reg<hash_iv_h0::HASH_IV_H0rs>;
///
pub mod hash_iv_h0;
///HASH_IV_H1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h1`]
///module
#[doc(alias = "HASH_IV_H1")]
pub type HashIvH1 = crate::Reg<hash_iv_h1::HASH_IV_H1rs>;
///
pub mod hash_iv_h1;
///HASH_IV_H2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h2`]
///module
#[doc(alias = "HASH_IV_H2")]
pub type HashIvH2 = crate::Reg<hash_iv_h2::HASH_IV_H2rs>;
///
pub mod hash_iv_h2;
///HASH_IV_H3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h3`]
///module
#[doc(alias = "HASH_IV_H3")]
pub type HashIvH3 = crate::Reg<hash_iv_h3::HASH_IV_H3rs>;
///
pub mod hash_iv_h3;
///HASH_IV_H4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h4`]
///module
#[doc(alias = "HASH_IV_H4")]
pub type HashIvH4 = crate::Reg<hash_iv_h4::HASH_IV_H4rs>;
///
pub mod hash_iv_h4;
///HASH_IV_H5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h5`]
///module
#[doc(alias = "HASH_IV_H5")]
pub type HashIvH5 = crate::Reg<hash_iv_h5::HASH_IV_H5rs>;
///
pub mod hash_iv_h5;
///HASH_IV_H6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h6`]
///module
#[doc(alias = "HASH_IV_H6")]
pub type HashIvH6 = crate::Reg<hash_iv_h6::HASH_IV_H6rs>;
///
pub mod hash_iv_h6;
///HASH_IV_H7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_iv_h7`]
///module
#[doc(alias = "HASH_IV_H7")]
pub type HashIvH7 = crate::Reg<hash_iv_h7::HASH_IV_H7rs>;
///
pub mod hash_iv_h7;
///HASH_RESULT_H0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h0`]
///module
#[doc(alias = "HASH_RESULT_H0")]
pub type HashResultH0 = crate::Reg<hash_result_h0::HASH_RESULT_H0rs>;
///
pub mod hash_result_h0;
///HASH_RESULT_H1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h1`]
///module
#[doc(alias = "HASH_RESULT_H1")]
pub type HashResultH1 = crate::Reg<hash_result_h1::HASH_RESULT_H1rs>;
///
pub mod hash_result_h1;
///HASH_RESULT_H2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h2`]
///module
#[doc(alias = "HASH_RESULT_H2")]
pub type HashResultH2 = crate::Reg<hash_result_h2::HASH_RESULT_H2rs>;
///
pub mod hash_result_h2;
///HASH_RESULT_H3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h3`]
///module
#[doc(alias = "HASH_RESULT_H3")]
pub type HashResultH3 = crate::Reg<hash_result_h3::HASH_RESULT_H3rs>;
///
pub mod hash_result_h3;
///HASH_RESULT_H4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h4`]
///module
#[doc(alias = "HASH_RESULT_H4")]
pub type HashResultH4 = crate::Reg<hash_result_h4::HASH_RESULT_H4rs>;
///
pub mod hash_result_h4;
///HASH_RESULT_H5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h5`]
///module
#[doc(alias = "HASH_RESULT_H5")]
pub type HashResultH5 = crate::Reg<hash_result_h5::HASH_RESULT_H5rs>;
///
pub mod hash_result_h5;
///HASH_RESULT_H6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h6`]
///module
#[doc(alias = "HASH_RESULT_H6")]
pub type HashResultH6 = crate::Reg<hash_result_h6::HASH_RESULT_H6rs>;
///
pub mod hash_result_h6;
///HASH_RESULT_H7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_h7`]
///module
#[doc(alias = "HASH_RESULT_H7")]
pub type HashResultH7 = crate::Reg<hash_result_h7::HASH_RESULT_H7rs>;
///
pub mod hash_result_h7;
///HASH_LEN_L (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_len_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_len_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_len_l`]
///module
#[doc(alias = "HASH_LEN_L")]
pub type HashLenL = crate::Reg<hash_len_l::HASH_LEN_Lrs>;
///
pub mod hash_len_l;
///HASH_LEN_H (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_len_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_len_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_len_h`]
///module
#[doc(alias = "HASH_LEN_H")]
pub type HashLenH = crate::Reg<hash_len_h::HASH_LEN_Hrs>;
///
pub mod hash_len_h;
///HASH_RESULT_LEN_L (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_len_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_len_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_len_l`]
///module
#[doc(alias = "HASH_RESULT_LEN_L")]
pub type HashResultLenL = crate::Reg<hash_result_len_l::HASH_RESULT_LEN_Lrs>;
///
pub mod hash_result_len_l;
///HASH_RESULT_LEN_H (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_len_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_len_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@hash_result_len_h`]
///module
#[doc(alias = "HASH_RESULT_LEN_H")]
pub type HashResultLenH = crate::Reg<hash_result_len_h::HASH_RESULT_LEN_Hrs>;
///
pub mod hash_result_len_h;
