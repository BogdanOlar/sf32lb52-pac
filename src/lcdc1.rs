#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    command: Command,
    status: Status,
    irq: Irq,
    setting: Setting,
    canvas_tl_pos: CanvasTlPos,
    canvas_br_pos: CanvasBrPos,
    canvas_bg: CanvasBg,
    layer0_config: Layer0Config,
    layer0_tl_pos: Layer0TlPos,
    layer0_br_pos: Layer0BrPos,
    layer0_filter: Layer0Filter,
    layer0_src: Layer0Src,
    layer0_fill: Layer0Fill,
    layer0_decomp: Layer0Decomp,
    layer0_decomp_cfg0: Layer0DecompCfg0,
    layer0_decomp_cfg1: Layer0DecompCfg1,
    layer0_decomp_stat: Layer0DecompStat,
    _reserved17: [u8; 0x1c],
    layer1_config: Layer1Config,
    layer1_tl_pos: Layer1TlPos,
    layer1_br_pos: Layer1BrPos,
    layer1_filter: Layer1Filter,
    layer1_src: Layer1Src,
    layer1_fill: Layer1Fill,
    dither_conf: DitherConf,
    dither_lfsr: DitherLfsr,
    lcd_conf: LcdConf,
    lcd_if_conf: LcdIfConf,
    lcd_mem: LcdMem,
    lcd_o_width: LcdOWidth,
    lcd_single: LcdSingle,
    lcd_wr: LcdWr,
    lcd_rd: LcdRd,
    spi_if_conf: SpiIfConf,
    te_conf: TeConf,
    te_conf2: TeConf2,
    dpi_if_conf1: DpiIfConf1,
    dpi_if_conf2: DpiIfConf2,
    dpi_if_conf3: DpiIfConf3,
    dpi_if_conf4: DpiIfConf4,
    dpi_if_conf5: DpiIfConf5,
    dpi_ctrl: DpiCtrl,
    dpi_stat: DpiStat,
    jdi_ser_conf1: JdiSerConf1,
    jdi_ser_conf2: JdiSerConf2,
    jdi_ser_ctrl: JdiSerCtrl,
    jdi_par_conf1: JdiParConf1,
    jdi_par_conf2: JdiParConf2,
    jdi_par_conf3: JdiParConf3,
    jdi_par_conf4: JdiParConf4,
    jdi_par_conf5: JdiParConf5,
    jdi_par_conf6: JdiParConf6,
    jdi_par_conf7: JdiParConf7,
    jdi_par_ctrl: JdiParCtrl,
    jdi_par_stat: JdiParStat,
    jdi_par_ex_ctrl: JdiParExCtrl,
    jdi_par_conf8: JdiParConf8,
    jdi_par_conf9: JdiParConf9,
    jdi_par_conf10: JdiParConf10,
    _reserved58: [u8; 0x0c],
    canvas_stat0: CanvasStat0,
    canvas_stat1: CanvasStat1,
    ol0_stat: Ol0Stat,
    ol1_stat: Ol1Stat,
    mem_if_stat: MemIfStat,
    perf_cnt: PerfCnt,
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
    pub const fn canvas_tl_pos(&self) -> &CanvasTlPos {
        &self.canvas_tl_pos
    }
    ///0x14 -
    #[inline(always)]
    pub const fn canvas_br_pos(&self) -> &CanvasBrPos {
        &self.canvas_br_pos
    }
    ///0x18 -
    #[inline(always)]
    pub const fn canvas_bg(&self) -> &CanvasBg {
        &self.canvas_bg
    }
    ///0x1c -
    #[inline(always)]
    pub const fn layer0_config(&self) -> &Layer0Config {
        &self.layer0_config
    }
    ///0x20 -
    #[inline(always)]
    pub const fn layer0_tl_pos(&self) -> &Layer0TlPos {
        &self.layer0_tl_pos
    }
    ///0x24 -
    #[inline(always)]
    pub const fn layer0_br_pos(&self) -> &Layer0BrPos {
        &self.layer0_br_pos
    }
    ///0x28 -
    #[inline(always)]
    pub const fn layer0_filter(&self) -> &Layer0Filter {
        &self.layer0_filter
    }
    ///0x2c -
    #[inline(always)]
    pub const fn layer0_src(&self) -> &Layer0Src {
        &self.layer0_src
    }
    ///0x30 -
    #[inline(always)]
    pub const fn layer0_fill(&self) -> &Layer0Fill {
        &self.layer0_fill
    }
    ///0x34 -
    #[inline(always)]
    pub const fn layer0_decomp(&self) -> &Layer0Decomp {
        &self.layer0_decomp
    }
    ///0x38 -
    #[inline(always)]
    pub const fn layer0_decomp_cfg0(&self) -> &Layer0DecompCfg0 {
        &self.layer0_decomp_cfg0
    }
    ///0x3c -
    #[inline(always)]
    pub const fn layer0_decomp_cfg1(&self) -> &Layer0DecompCfg1 {
        &self.layer0_decomp_cfg1
    }
    ///0x40 -
    #[inline(always)]
    pub const fn layer0_decomp_stat(&self) -> &Layer0DecompStat {
        &self.layer0_decomp_stat
    }
    ///0x60 -
    #[inline(always)]
    pub const fn layer1_config(&self) -> &Layer1Config {
        &self.layer1_config
    }
    ///0x64 -
    #[inline(always)]
    pub const fn layer1_tl_pos(&self) -> &Layer1TlPos {
        &self.layer1_tl_pos
    }
    ///0x68 -
    #[inline(always)]
    pub const fn layer1_br_pos(&self) -> &Layer1BrPos {
        &self.layer1_br_pos
    }
    ///0x6c -
    #[inline(always)]
    pub const fn layer1_filter(&self) -> &Layer1Filter {
        &self.layer1_filter
    }
    ///0x70 -
    #[inline(always)]
    pub const fn layer1_src(&self) -> &Layer1Src {
        &self.layer1_src
    }
    ///0x74 -
    #[inline(always)]
    pub const fn layer1_fill(&self) -> &Layer1Fill {
        &self.layer1_fill
    }
    ///0x78 -
    #[inline(always)]
    pub const fn dither_conf(&self) -> &DitherConf {
        &self.dither_conf
    }
    ///0x7c -
    #[inline(always)]
    pub const fn dither_lfsr(&self) -> &DitherLfsr {
        &self.dither_lfsr
    }
    ///0x80 -
    #[inline(always)]
    pub const fn lcd_conf(&self) -> &LcdConf {
        &self.lcd_conf
    }
    ///0x84 -
    #[inline(always)]
    pub const fn lcd_if_conf(&self) -> &LcdIfConf {
        &self.lcd_if_conf
    }
    ///0x88 -
    #[inline(always)]
    pub const fn lcd_mem(&self) -> &LcdMem {
        &self.lcd_mem
    }
    ///0x8c -
    #[inline(always)]
    pub const fn lcd_o_width(&self) -> &LcdOWidth {
        &self.lcd_o_width
    }
    ///0x90 -
    #[inline(always)]
    pub const fn lcd_single(&self) -> &LcdSingle {
        &self.lcd_single
    }
    ///0x94 -
    #[inline(always)]
    pub const fn lcd_wr(&self) -> &LcdWr {
        &self.lcd_wr
    }
    ///0x98 -
    #[inline(always)]
    pub const fn lcd_rd(&self) -> &LcdRd {
        &self.lcd_rd
    }
    ///0x9c -
    #[inline(always)]
    pub const fn spi_if_conf(&self) -> &SpiIfConf {
        &self.spi_if_conf
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn te_conf(&self) -> &TeConf {
        &self.te_conf
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn te_conf2(&self) -> &TeConf2 {
        &self.te_conf2
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn dpi_if_conf1(&self) -> &DpiIfConf1 {
        &self.dpi_if_conf1
    }
    ///0xac -
    #[inline(always)]
    pub const fn dpi_if_conf2(&self) -> &DpiIfConf2 {
        &self.dpi_if_conf2
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn dpi_if_conf3(&self) -> &DpiIfConf3 {
        &self.dpi_if_conf3
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn dpi_if_conf4(&self) -> &DpiIfConf4 {
        &self.dpi_if_conf4
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn dpi_if_conf5(&self) -> &DpiIfConf5 {
        &self.dpi_if_conf5
    }
    ///0xbc -
    #[inline(always)]
    pub const fn dpi_ctrl(&self) -> &DpiCtrl {
        &self.dpi_ctrl
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn dpi_stat(&self) -> &DpiStat {
        &self.dpi_stat
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn jdi_ser_conf1(&self) -> &JdiSerConf1 {
        &self.jdi_ser_conf1
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn jdi_ser_conf2(&self) -> &JdiSerConf2 {
        &self.jdi_ser_conf2
    }
    ///0xcc -
    #[inline(always)]
    pub const fn jdi_ser_ctrl(&self) -> &JdiSerCtrl {
        &self.jdi_ser_ctrl
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn jdi_par_conf1(&self) -> &JdiParConf1 {
        &self.jdi_par_conf1
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn jdi_par_conf2(&self) -> &JdiParConf2 {
        &self.jdi_par_conf2
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn jdi_par_conf3(&self) -> &JdiParConf3 {
        &self.jdi_par_conf3
    }
    ///0xdc -
    #[inline(always)]
    pub const fn jdi_par_conf4(&self) -> &JdiParConf4 {
        &self.jdi_par_conf4
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn jdi_par_conf5(&self) -> &JdiParConf5 {
        &self.jdi_par_conf5
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn jdi_par_conf6(&self) -> &JdiParConf6 {
        &self.jdi_par_conf6
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn jdi_par_conf7(&self) -> &JdiParConf7 {
        &self.jdi_par_conf7
    }
    ///0xec -
    #[inline(always)]
    pub const fn jdi_par_ctrl(&self) -> &JdiParCtrl {
        &self.jdi_par_ctrl
    }
    ///0xf0 -
    #[inline(always)]
    pub const fn jdi_par_stat(&self) -> &JdiParStat {
        &self.jdi_par_stat
    }
    ///0xf4 -
    #[inline(always)]
    pub const fn jdi_par_ex_ctrl(&self) -> &JdiParExCtrl {
        &self.jdi_par_ex_ctrl
    }
    ///0xf8 -
    #[inline(always)]
    pub const fn jdi_par_conf8(&self) -> &JdiParConf8 {
        &self.jdi_par_conf8
    }
    ///0xfc -
    #[inline(always)]
    pub const fn jdi_par_conf9(&self) -> &JdiParConf9 {
        &self.jdi_par_conf9
    }
    ///0x100 -
    #[inline(always)]
    pub const fn jdi_par_conf10(&self) -> &JdiParConf10 {
        &self.jdi_par_conf10
    }
    ///0x110 -
    #[inline(always)]
    pub const fn canvas_stat0(&self) -> &CanvasStat0 {
        &self.canvas_stat0
    }
    ///0x114 -
    #[inline(always)]
    pub const fn canvas_stat1(&self) -> &CanvasStat1 {
        &self.canvas_stat1
    }
    ///0x118 -
    #[inline(always)]
    pub const fn ol0_stat(&self) -> &Ol0Stat {
        &self.ol0_stat
    }
    ///0x11c -
    #[inline(always)]
    pub const fn ol1_stat(&self) -> &Ol1Stat {
        &self.ol1_stat
    }
    ///0x120 -
    #[inline(always)]
    pub const fn mem_if_stat(&self) -> &MemIfStat {
        &self.mem_if_stat
    }
    ///0x124 -
    #[inline(always)]
    pub const fn perf_cnt(&self) -> &PerfCnt {
        &self.perf_cnt
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
///CANVAS_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_tl_pos`]
///module
#[doc(alias = "CANVAS_TL_POS")]
pub type CanvasTlPos = crate::Reg<canvas_tl_pos::CANVAS_TL_POSrs>;
///
pub mod canvas_tl_pos;
///CANVAS_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_br_pos`]
///module
#[doc(alias = "CANVAS_BR_POS")]
pub type CanvasBrPos = crate::Reg<canvas_br_pos::CANVAS_BR_POSrs>;
///
pub mod canvas_br_pos;
///CANVAS_BG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_bg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_bg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_bg`]
///module
#[doc(alias = "CANVAS_BG")]
pub type CanvasBg = crate::Reg<canvas_bg::CANVAS_BGrs>;
///
pub mod canvas_bg;
///LAYER0_CONFIG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_config`]
///module
#[doc(alias = "LAYER0_CONFIG")]
pub type Layer0Config = crate::Reg<layer0_config::LAYER0_CONFIGrs>;
///
pub mod layer0_config;
///LAYER0_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_tl_pos`]
///module
#[doc(alias = "LAYER0_TL_POS")]
pub type Layer0TlPos = crate::Reg<layer0_tl_pos::LAYER0_TL_POSrs>;
///
pub mod layer0_tl_pos;
///LAYER0_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_br_pos`]
///module
#[doc(alias = "LAYER0_BR_POS")]
pub type Layer0BrPos = crate::Reg<layer0_br_pos::LAYER0_BR_POSrs>;
///
pub mod layer0_br_pos;
///LAYER0_FILTER (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_filter`]
///module
#[doc(alias = "LAYER0_FILTER")]
pub type Layer0Filter = crate::Reg<layer0_filter::LAYER0_FILTERrs>;
///
pub mod layer0_filter;
///LAYER0_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_src`]
///module
#[doc(alias = "LAYER0_SRC")]
pub type Layer0Src = crate::Reg<layer0_src::LAYER0_SRCrs>;
///
pub mod layer0_src;
///LAYER0_FILL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_fill::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_fill::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_fill`]
///module
#[doc(alias = "LAYER0_FILL")]
pub type Layer0Fill = crate::Reg<layer0_fill::LAYER0_FILLrs>;
///
pub mod layer0_fill;
///LAYER0_DECOMP (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_decomp`]
///module
#[doc(alias = "LAYER0_DECOMP")]
pub type Layer0Decomp = crate::Reg<layer0_decomp::LAYER0_DECOMPrs>;
///
pub mod layer0_decomp;
///LAYER0_DECOMP_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_decomp_cfg0`]
///module
#[doc(alias = "LAYER0_DECOMP_CFG0")]
pub type Layer0DecompCfg0 = crate::Reg<layer0_decomp_cfg0::LAYER0_DECOMP_CFG0rs>;
///
pub mod layer0_decomp_cfg0;
///LAYER0_DECOMP_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_decomp_cfg1`]
///module
#[doc(alias = "LAYER0_DECOMP_CFG1")]
pub type Layer0DecompCfg1 = crate::Reg<layer0_decomp_cfg1::LAYER0_DECOMP_CFG1rs>;
///
pub mod layer0_decomp_cfg1;
///LAYER0_DECOMP_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer0_decomp_stat`]
///module
#[doc(alias = "LAYER0_DECOMP_STAT")]
pub type Layer0DecompStat = crate::Reg<layer0_decomp_stat::LAYER0_DECOMP_STATrs>;
///
pub mod layer0_decomp_stat;
///LAYER1_CONFIG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer1_config`]
///module
#[doc(alias = "LAYER1_CONFIG")]
pub type Layer1Config = crate::Reg<layer1_config::LAYER1_CONFIGrs>;
///
pub mod layer1_config;
///LAYER1_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer1_tl_pos`]
///module
#[doc(alias = "LAYER1_TL_POS")]
pub type Layer1TlPos = crate::Reg<layer1_tl_pos::LAYER1_TL_POSrs>;
///
pub mod layer1_tl_pos;
///LAYER1_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer1_br_pos`]
///module
#[doc(alias = "LAYER1_BR_POS")]
pub type Layer1BrPos = crate::Reg<layer1_br_pos::LAYER1_BR_POSrs>;
///
pub mod layer1_br_pos;
///LAYER1_FILTER (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer1_filter`]
///module
#[doc(alias = "LAYER1_FILTER")]
pub type Layer1Filter = crate::Reg<layer1_filter::LAYER1_FILTERrs>;
///
pub mod layer1_filter;
///LAYER1_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer1_src`]
///module
#[doc(alias = "LAYER1_SRC")]
pub type Layer1Src = crate::Reg<layer1_src::LAYER1_SRCrs>;
///
pub mod layer1_src;
///LAYER1_FILL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_fill::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_fill::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@layer1_fill`]
///module
#[doc(alias = "LAYER1_FILL")]
pub type Layer1Fill = crate::Reg<layer1_fill::LAYER1_FILLrs>;
///
pub mod layer1_fill;
///DITHER_CONF (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dither_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dither_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dither_conf`]
///module
#[doc(alias = "DITHER_CONF")]
pub type DitherConf = crate::Reg<dither_conf::DITHER_CONFrs>;
///
pub mod dither_conf;
///DITHER_LFSR (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dither_lfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dither_lfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dither_lfsr`]
///module
#[doc(alias = "DITHER_LFSR")]
pub type DitherLfsr = crate::Reg<dither_lfsr::DITHER_LFSRrs>;
///
pub mod dither_lfsr;
///LCD_CONF (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_conf`]
///module
#[doc(alias = "LCD_CONF")]
pub type LcdConf = crate::Reg<lcd_conf::LCD_CONFrs>;
///
pub mod lcd_conf;
///LCD_IF_CONF (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_if_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_if_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_if_conf`]
///module
#[doc(alias = "LCD_IF_CONF")]
pub type LcdIfConf = crate::Reg<lcd_if_conf::LCD_IF_CONFrs>;
///
pub mod lcd_if_conf;
///LCD_MEM (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_mem`]
///module
#[doc(alias = "LCD_MEM")]
pub type LcdMem = crate::Reg<lcd_mem::LCD_MEMrs>;
///
pub mod lcd_mem;
///LCD_O_WIDTH (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_o_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_o_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_o_width`]
///module
#[doc(alias = "LCD_O_WIDTH")]
pub type LcdOWidth = crate::Reg<lcd_o_width::LCD_O_WIDTHrs>;
///
pub mod lcd_o_width;
///LCD_SINGLE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_single::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_single::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_single`]
///module
#[doc(alias = "LCD_SINGLE")]
pub type LcdSingle = crate::Reg<lcd_single::LCD_SINGLErs>;
///
pub mod lcd_single;
///LCD_WR (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_wr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_wr`]
///module
#[doc(alias = "LCD_WR")]
pub type LcdWr = crate::Reg<lcd_wr::LCD_WRrs>;
///
pub mod lcd_wr;
///LCD_RD (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@lcd_rd`]
///module
#[doc(alias = "LCD_RD")]
pub type LcdRd = crate::Reg<lcd_rd::LCD_RDrs>;
///
pub mod lcd_rd;
///SPI_IF_CONF (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`spi_if_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_if_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@spi_if_conf`]
///module
#[doc(alias = "SPI_IF_CONF")]
pub type SpiIfConf = crate::Reg<spi_if_conf::SPI_IF_CONFrs>;
///
pub mod spi_if_conf;
///TE_CONF (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`te_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`te_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@te_conf`]
///module
#[doc(alias = "TE_CONF")]
pub type TeConf = crate::Reg<te_conf::TE_CONFrs>;
///
pub mod te_conf;
///TE_CONF2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`te_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`te_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@te_conf2`]
///module
#[doc(alias = "TE_CONF2")]
pub type TeConf2 = crate::Reg<te_conf2::TE_CONF2rs>;
///
pub mod te_conf2;
///DPI_IF_CONF1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_if_conf1`]
///module
#[doc(alias = "DPI_IF_CONF1")]
pub type DpiIfConf1 = crate::Reg<dpi_if_conf1::DPI_IF_CONF1rs>;
///
pub mod dpi_if_conf1;
///DPI_IF_CONF2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_if_conf2`]
///module
#[doc(alias = "DPI_IF_CONF2")]
pub type DpiIfConf2 = crate::Reg<dpi_if_conf2::DPI_IF_CONF2rs>;
///
pub mod dpi_if_conf2;
///DPI_IF_CONF3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_if_conf3`]
///module
#[doc(alias = "DPI_IF_CONF3")]
pub type DpiIfConf3 = crate::Reg<dpi_if_conf3::DPI_IF_CONF3rs>;
///
pub mod dpi_if_conf3;
///DPI_IF_CONF4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_if_conf4`]
///module
#[doc(alias = "DPI_IF_CONF4")]
pub type DpiIfConf4 = crate::Reg<dpi_if_conf4::DPI_IF_CONF4rs>;
///
pub mod dpi_if_conf4;
///DPI_IF_CONF5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_if_conf5`]
///module
#[doc(alias = "DPI_IF_CONF5")]
pub type DpiIfConf5 = crate::Reg<dpi_if_conf5::DPI_IF_CONF5rs>;
///
pub mod dpi_if_conf5;
///DPI_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_ctrl`]
///module
#[doc(alias = "DPI_CTRL")]
pub type DpiCtrl = crate::Reg<dpi_ctrl::DPI_CTRLrs>;
///
pub mod dpi_ctrl;
///DPI_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dpi_stat`]
///module
#[doc(alias = "DPI_STAT")]
pub type DpiStat = crate::Reg<dpi_stat::DPI_STATrs>;
///
pub mod dpi_stat;
///JDI_SER_CONF1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_ser_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_ser_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_ser_conf1`]
///module
#[doc(alias = "JDI_SER_CONF1")]
pub type JdiSerConf1 = crate::Reg<jdi_ser_conf1::JDI_SER_CONF1rs>;
///
pub mod jdi_ser_conf1;
///JDI_SER_CONF2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_ser_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_ser_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_ser_conf2`]
///module
#[doc(alias = "JDI_SER_CONF2")]
pub type JdiSerConf2 = crate::Reg<jdi_ser_conf2::JDI_SER_CONF2rs>;
///
pub mod jdi_ser_conf2;
///JDI_SER_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_ser_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_ser_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_ser_ctrl`]
///module
#[doc(alias = "JDI_SER_CTRL")]
pub type JdiSerCtrl = crate::Reg<jdi_ser_ctrl::JDI_SER_CTRLrs>;
///
pub mod jdi_ser_ctrl;
///JDI_PAR_CONF1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf1`]
///module
#[doc(alias = "JDI_PAR_CONF1")]
pub type JdiParConf1 = crate::Reg<jdi_par_conf1::JDI_PAR_CONF1rs>;
///
pub mod jdi_par_conf1;
///JDI_PAR_CONF2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf2`]
///module
#[doc(alias = "JDI_PAR_CONF2")]
pub type JdiParConf2 = crate::Reg<jdi_par_conf2::JDI_PAR_CONF2rs>;
///
pub mod jdi_par_conf2;
///JDI_PAR_CONF3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf3`]
///module
#[doc(alias = "JDI_PAR_CONF3")]
pub type JdiParConf3 = crate::Reg<jdi_par_conf3::JDI_PAR_CONF3rs>;
///
pub mod jdi_par_conf3;
///JDI_PAR_CONF4 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf4`]
///module
#[doc(alias = "JDI_PAR_CONF4")]
pub type JdiParConf4 = crate::Reg<jdi_par_conf4::JDI_PAR_CONF4rs>;
///
pub mod jdi_par_conf4;
///JDI_PAR_CONF5 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf5`]
///module
#[doc(alias = "JDI_PAR_CONF5")]
pub type JdiParConf5 = crate::Reg<jdi_par_conf5::JDI_PAR_CONF5rs>;
///
pub mod jdi_par_conf5;
///JDI_PAR_CONF6 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf6`]
///module
#[doc(alias = "JDI_PAR_CONF6")]
pub type JdiParConf6 = crate::Reg<jdi_par_conf6::JDI_PAR_CONF6rs>;
///
pub mod jdi_par_conf6;
///JDI_PAR_CONF7 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf7`]
///module
#[doc(alias = "JDI_PAR_CONF7")]
pub type JdiParConf7 = crate::Reg<jdi_par_conf7::JDI_PAR_CONF7rs>;
///
pub mod jdi_par_conf7;
///JDI_PAR_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_ctrl`]
///module
#[doc(alias = "JDI_PAR_CTRL")]
pub type JdiParCtrl = crate::Reg<jdi_par_ctrl::JDI_PAR_CTRLrs>;
///
pub mod jdi_par_ctrl;
///JDI_PAR_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_stat`]
///module
#[doc(alias = "JDI_PAR_STAT")]
pub type JdiParStat = crate::Reg<jdi_par_stat::JDI_PAR_STATrs>;
///
pub mod jdi_par_stat;
///JDI_PAR_EX_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_ex_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_ex_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_ex_ctrl`]
///module
#[doc(alias = "JDI_PAR_EX_CTRL")]
pub type JdiParExCtrl = crate::Reg<jdi_par_ex_ctrl::JDI_PAR_EX_CTRLrs>;
///
pub mod jdi_par_ex_ctrl;
///JDI_PAR_CONF8 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf8`]
///module
#[doc(alias = "JDI_PAR_CONF8")]
pub type JdiParConf8 = crate::Reg<jdi_par_conf8::JDI_PAR_CONF8rs>;
///
pub mod jdi_par_conf8;
///JDI_PAR_CONF9 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf9`]
///module
#[doc(alias = "JDI_PAR_CONF9")]
pub type JdiParConf9 = crate::Reg<jdi_par_conf9::JDI_PAR_CONF9rs>;
///
pub mod jdi_par_conf9;
///JDI_PAR_CONF10 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@jdi_par_conf10`]
///module
#[doc(alias = "JDI_PAR_CONF10")]
pub type JdiParConf10 = crate::Reg<jdi_par_conf10::JDI_PAR_CONF10rs>;
///
pub mod jdi_par_conf10;
///CANVAS_STAT0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_stat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_stat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_stat0`]
///module
#[doc(alias = "CANVAS_STAT0")]
pub type CanvasStat0 = crate::Reg<canvas_stat0::CANVAS_STAT0rs>;
///
pub mod canvas_stat0;
///CANVAS_STAT1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_stat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_stat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_stat1`]
///module
#[doc(alias = "CANVAS_STAT1")]
pub type CanvasStat1 = crate::Reg<canvas_stat1::CANVAS_STAT1rs>;
///
pub mod canvas_stat1;
///OL0_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ol0_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol0_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ol0_stat`]
///module
#[doc(alias = "OL0_STAT")]
pub type Ol0Stat = crate::Reg<ol0_stat::OL0_STATrs>;
///
pub mod ol0_stat;
///OL1_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ol1_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol1_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ol1_stat`]
///module
#[doc(alias = "OL1_STAT")]
pub type Ol1Stat = crate::Reg<ol1_stat::OL1_STATrs>;
///
pub mod ol1_stat;
///MEM_IF_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`mem_if_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_if_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mem_if_stat`]
///module
#[doc(alias = "MEM_IF_STAT")]
pub type MemIfStat = crate::Reg<mem_if_stat::MEM_IF_STATrs>;
///
pub mod mem_if_stat;
///PERF_CNT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`perf_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perf_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@perf_cnt`]
///module
#[doc(alias = "PERF_CNT")]
pub type PerfCnt = crate::Reg<perf_cnt::PERF_CNTrs>;
///
pub mod perf_cnt;
