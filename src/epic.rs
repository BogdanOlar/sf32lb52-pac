#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    command: Command,
    status: Status,
    eof_irq: EofIrq,
    setting: Setting,
    canvas_tl_pos: CanvasTlPos,
    canvas_br_pos: CanvasBrPos,
    canvas_bg: CanvasBg,
    vl_cfg: VlCfg,
    vl_tl_pos: VlTlPos,
    vl_br_pos: VlBrPos,
    vl_extents: VlExtents,
    vl_filter: VlFilter,
    vl_src: VlSrc,
    vl_rot: VlRot,
    vl_rot_stat: VlRotStat,
    vl_scale_ratio_h: VlScaleRatioH,
    vl_scale_ratio_v: VlScaleRatioV,
    vl_fill: VlFill,
    vl_misc_cfg: VlMiscCfg,
    _reserved19: [u8; 0x04],
    l0_cfg: L0Cfg,
    l0_tl_pos: L0TlPos,
    l0_br_pos: L0BrPos,
    l0_filter: L0Filter,
    l0_src: L0Src,
    l0_fill: L0Fill,
    l0_misc_cfg: L0MiscCfg,
    _reserved26: [u8; 0x04],
    l1_cfg: L1Cfg,
    l1_tl_pos: L1TlPos,
    l1_br_pos: L1BrPos,
    l1_filter: L1Filter,
    l1_src: L1Src,
    l1_fill: L1Fill,
    l1_misc_cfg: L1MiscCfg,
    _reserved33: [u8; 0x04],
    l2_cfg: L2Cfg,
    l2_tl_pos: L2TlPos,
    l2_br_pos: L2BrPos,
    l2_extents: L2Extents,
    l2_filter: L2Filter,
    l2_src: L2Src,
    l2_rot: L2Rot,
    l2_rot_stat: L2RotStat,
    l2_scale_ratio_h: L2ScaleRatioH,
    l2_scale_ratio_v: L2ScaleRatioV,
    l2_fill: L2Fill,
    l2_misc_cfg: L2MiscCfg,
    mask_cfg: MaskCfg,
    mask_tl_pos: MaskTlPos,
    mask_br_pos: MaskBrPos,
    mask_src: MaskSrc,
    coeng_cfg: CoengCfg,
    yuv_eng_cfg0: YuvEngCfg0,
    yuv_eng_cfg1: YuvEngCfg1,
    y_src: YSrc,
    u_src: USrc,
    v_src: VSrc,
    coef0: Coef0,
    coef1: Coef1,
    dither_conf: DitherConf,
    dither_lfsr: DitherLfsr,
    ahb_ctrl: AhbCtrl,
    ahb_mem: AhbMem,
    ahb_stride: AhbStride,
    debug: Debug,
    vl_rot_m_cfg1: VlRotMCfg1,
    vl_rot_m_cfg2: VlRotMCfg2,
    vl_rot_m_cfg3: VlRotMCfg3,
    vl_scale_init_cfg1: VlScaleInitCfg1,
    vl_scale_init_cfg2: VlScaleInitCfg2,
    l2_rot_m_cfg1: L2RotMCfg1,
    l2_rot_m_cfg2: L2RotMCfg2,
    l2_rot_m_cfg3: L2RotMCfg3,
    l2_scale_init_cfg1: L2ScaleInitCfg1,
    l2_scale_init_cfg2: L2ScaleInitCfg2,
    perf_cnt: PerfCnt,
    _reserved74: [u8; 0x0c],
    canvas_stat: CanvasStat,
    ezip_stat: EzipStat,
    ol_stat: OlStat,
    ol2_stat: Ol2Stat,
    vl_stat: VlStat,
    ml_stat: MlStat,
    mem_if_stat: MemIfStat,
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
    pub const fn eof_irq(&self) -> &EofIrq {
        &self.eof_irq
    }
    ///0x0c -
    #[inline(always)]
    pub const fn setting(&self) -> &Setting {
        &self.setting
    }
    ///0x10 - Top-Left pixel coordinate
    #[inline(always)]
    pub const fn canvas_tl_pos(&self) -> &CanvasTlPos {
        &self.canvas_tl_pos
    }
    ///0x14 - Bottom-Right pixel coordinate
    #[inline(always)]
    pub const fn canvas_br_pos(&self) -> &CanvasBrPos {
        &self.canvas_br_pos
    }
    ///0x18 - Background color
    #[inline(always)]
    pub const fn canvas_bg(&self) -> &CanvasBg {
        &self.canvas_bg
    }
    ///0x1c -
    #[inline(always)]
    pub const fn vl_cfg(&self) -> &VlCfg {
        &self.vl_cfg
    }
    ///0x20 -
    #[inline(always)]
    pub const fn vl_tl_pos(&self) -> &VlTlPos {
        &self.vl_tl_pos
    }
    ///0x24 -
    #[inline(always)]
    pub const fn vl_br_pos(&self) -> &VlBrPos {
        &self.vl_br_pos
    }
    ///0x28 -
    #[inline(always)]
    pub const fn vl_extents(&self) -> &VlExtents {
        &self.vl_extents
    }
    ///0x2c -
    #[inline(always)]
    pub const fn vl_filter(&self) -> &VlFilter {
        &self.vl_filter
    }
    ///0x30 -
    #[inline(always)]
    pub const fn vl_src(&self) -> &VlSrc {
        &self.vl_src
    }
    ///0x34 -
    #[inline(always)]
    pub const fn vl_rot(&self) -> &VlRot {
        &self.vl_rot
    }
    ///0x38 -
    #[inline(always)]
    pub const fn vl_rot_stat(&self) -> &VlRotStat {
        &self.vl_rot_stat
    }
    ///0x3c -
    #[inline(always)]
    pub const fn vl_scale_ratio_h(&self) -> &VlScaleRatioH {
        &self.vl_scale_ratio_h
    }
    ///0x40 -
    #[inline(always)]
    pub const fn vl_scale_ratio_v(&self) -> &VlScaleRatioV {
        &self.vl_scale_ratio_v
    }
    ///0x44 -
    #[inline(always)]
    pub const fn vl_fill(&self) -> &VlFill {
        &self.vl_fill
    }
    ///0x48 -
    #[inline(always)]
    pub const fn vl_misc_cfg(&self) -> &VlMiscCfg {
        &self.vl_misc_cfg
    }
    ///0x50 -
    #[inline(always)]
    pub const fn l0_cfg(&self) -> &L0Cfg {
        &self.l0_cfg
    }
    ///0x54 -
    #[inline(always)]
    pub const fn l0_tl_pos(&self) -> &L0TlPos {
        &self.l0_tl_pos
    }
    ///0x58 -
    #[inline(always)]
    pub const fn l0_br_pos(&self) -> &L0BrPos {
        &self.l0_br_pos
    }
    ///0x5c -
    #[inline(always)]
    pub const fn l0_filter(&self) -> &L0Filter {
        &self.l0_filter
    }
    ///0x60 -
    #[inline(always)]
    pub const fn l0_src(&self) -> &L0Src {
        &self.l0_src
    }
    ///0x64 -
    #[inline(always)]
    pub const fn l0_fill(&self) -> &L0Fill {
        &self.l0_fill
    }
    ///0x68 -
    #[inline(always)]
    pub const fn l0_misc_cfg(&self) -> &L0MiscCfg {
        &self.l0_misc_cfg
    }
    ///0x70 -
    #[inline(always)]
    pub const fn l1_cfg(&self) -> &L1Cfg {
        &self.l1_cfg
    }
    ///0x74 -
    #[inline(always)]
    pub const fn l1_tl_pos(&self) -> &L1TlPos {
        &self.l1_tl_pos
    }
    ///0x78 -
    #[inline(always)]
    pub const fn l1_br_pos(&self) -> &L1BrPos {
        &self.l1_br_pos
    }
    ///0x7c -
    #[inline(always)]
    pub const fn l1_filter(&self) -> &L1Filter {
        &self.l1_filter
    }
    ///0x80 -
    #[inline(always)]
    pub const fn l1_src(&self) -> &L1Src {
        &self.l1_src
    }
    ///0x84 -
    #[inline(always)]
    pub const fn l1_fill(&self) -> &L1Fill {
        &self.l1_fill
    }
    ///0x88 -
    #[inline(always)]
    pub const fn l1_misc_cfg(&self) -> &L1MiscCfg {
        &self.l1_misc_cfg
    }
    ///0x90 -
    #[inline(always)]
    pub const fn l2_cfg(&self) -> &L2Cfg {
        &self.l2_cfg
    }
    ///0x94 -
    #[inline(always)]
    pub const fn l2_tl_pos(&self) -> &L2TlPos {
        &self.l2_tl_pos
    }
    ///0x98 -
    #[inline(always)]
    pub const fn l2_br_pos(&self) -> &L2BrPos {
        &self.l2_br_pos
    }
    ///0x9c -
    #[inline(always)]
    pub const fn l2_extents(&self) -> &L2Extents {
        &self.l2_extents
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn l2_filter(&self) -> &L2Filter {
        &self.l2_filter
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn l2_src(&self) -> &L2Src {
        &self.l2_src
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn l2_rot(&self) -> &L2Rot {
        &self.l2_rot
    }
    ///0xac -
    #[inline(always)]
    pub const fn l2_rot_stat(&self) -> &L2RotStat {
        &self.l2_rot_stat
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn l2_scale_ratio_h(&self) -> &L2ScaleRatioH {
        &self.l2_scale_ratio_h
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn l2_scale_ratio_v(&self) -> &L2ScaleRatioV {
        &self.l2_scale_ratio_v
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn l2_fill(&self) -> &L2Fill {
        &self.l2_fill
    }
    ///0xbc -
    #[inline(always)]
    pub const fn l2_misc_cfg(&self) -> &L2MiscCfg {
        &self.l2_misc_cfg
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn mask_cfg(&self) -> &MaskCfg {
        &self.mask_cfg
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn mask_tl_pos(&self) -> &MaskTlPos {
        &self.mask_tl_pos
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn mask_br_pos(&self) -> &MaskBrPos {
        &self.mask_br_pos
    }
    ///0xcc -
    #[inline(always)]
    pub const fn mask_src(&self) -> &MaskSrc {
        &self.mask_src
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn coeng_cfg(&self) -> &CoengCfg {
        &self.coeng_cfg
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn yuv_eng_cfg0(&self) -> &YuvEngCfg0 {
        &self.yuv_eng_cfg0
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn yuv_eng_cfg1(&self) -> &YuvEngCfg1 {
        &self.yuv_eng_cfg1
    }
    ///0xdc -
    #[inline(always)]
    pub const fn y_src(&self) -> &YSrc {
        &self.y_src
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn u_src(&self) -> &USrc {
        &self.u_src
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn v_src(&self) -> &VSrc {
        &self.v_src
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn coef0(&self) -> &Coef0 {
        &self.coef0
    }
    ///0xec -
    #[inline(always)]
    pub const fn coef1(&self) -> &Coef1 {
        &self.coef1
    }
    ///0xf0 -
    #[inline(always)]
    pub const fn dither_conf(&self) -> &DitherConf {
        &self.dither_conf
    }
    ///0xf4 -
    #[inline(always)]
    pub const fn dither_lfsr(&self) -> &DitherLfsr {
        &self.dither_lfsr
    }
    ///0xf8 -
    #[inline(always)]
    pub const fn ahb_ctrl(&self) -> &AhbCtrl {
        &self.ahb_ctrl
    }
    ///0xfc -
    #[inline(always)]
    pub const fn ahb_mem(&self) -> &AhbMem {
        &self.ahb_mem
    }
    ///0x100 -
    #[inline(always)]
    pub const fn ahb_stride(&self) -> &AhbStride {
        &self.ahb_stride
    }
    ///0x104 -
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    ///0x108 -
    #[inline(always)]
    pub const fn vl_rot_m_cfg1(&self) -> &VlRotMCfg1 {
        &self.vl_rot_m_cfg1
    }
    ///0x10c -
    #[inline(always)]
    pub const fn vl_rot_m_cfg2(&self) -> &VlRotMCfg2 {
        &self.vl_rot_m_cfg2
    }
    ///0x110 -
    #[inline(always)]
    pub const fn vl_rot_m_cfg3(&self) -> &VlRotMCfg3 {
        &self.vl_rot_m_cfg3
    }
    ///0x114 -
    #[inline(always)]
    pub const fn vl_scale_init_cfg1(&self) -> &VlScaleInitCfg1 {
        &self.vl_scale_init_cfg1
    }
    ///0x118 -
    #[inline(always)]
    pub const fn vl_scale_init_cfg2(&self) -> &VlScaleInitCfg2 {
        &self.vl_scale_init_cfg2
    }
    ///0x11c -
    #[inline(always)]
    pub const fn l2_rot_m_cfg1(&self) -> &L2RotMCfg1 {
        &self.l2_rot_m_cfg1
    }
    ///0x120 -
    #[inline(always)]
    pub const fn l2_rot_m_cfg2(&self) -> &L2RotMCfg2 {
        &self.l2_rot_m_cfg2
    }
    ///0x124 -
    #[inline(always)]
    pub const fn l2_rot_m_cfg3(&self) -> &L2RotMCfg3 {
        &self.l2_rot_m_cfg3
    }
    ///0x128 -
    #[inline(always)]
    pub const fn l2_scale_init_cfg1(&self) -> &L2ScaleInitCfg1 {
        &self.l2_scale_init_cfg1
    }
    ///0x12c -
    #[inline(always)]
    pub const fn l2_scale_init_cfg2(&self) -> &L2ScaleInitCfg2 {
        &self.l2_scale_init_cfg2
    }
    ///0x130 -
    #[inline(always)]
    pub const fn perf_cnt(&self) -> &PerfCnt {
        &self.perf_cnt
    }
    ///0x140 -
    #[inline(always)]
    pub const fn canvas_stat(&self) -> &CanvasStat {
        &self.canvas_stat
    }
    ///0x144 -
    #[inline(always)]
    pub const fn ezip_stat(&self) -> &EzipStat {
        &self.ezip_stat
    }
    ///0x148 -
    #[inline(always)]
    pub const fn ol_stat(&self) -> &OlStat {
        &self.ol_stat
    }
    ///0x14c -
    #[inline(always)]
    pub const fn ol2_stat(&self) -> &Ol2Stat {
        &self.ol2_stat
    }
    ///0x150 -
    #[inline(always)]
    pub const fn vl_stat(&self) -> &VlStat {
        &self.vl_stat
    }
    ///0x154 -
    #[inline(always)]
    pub const fn ml_stat(&self) -> &MlStat {
        &self.ml_stat
    }
    ///0x158 -
    #[inline(always)]
    pub const fn mem_if_stat(&self) -> &MemIfStat {
        &self.mem_if_stat
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
///EOF_IRQ (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`eof_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eof_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@eof_irq`]
///module
#[doc(alias = "EOF_IRQ")]
pub type EofIrq = crate::Reg<eof_irq::EOF_IRQrs>;
///
pub mod eof_irq;
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
///CANVAS_TL_POS (rw) register accessor: Top-Left pixel coordinate
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_tl_pos`]
///module
#[doc(alias = "CANVAS_TL_POS")]
pub type CanvasTlPos = crate::Reg<canvas_tl_pos::CANVAS_TL_POSrs>;
///Top-Left pixel coordinate
pub mod canvas_tl_pos;
///CANVAS_BR_POS (rw) register accessor: Bottom-Right pixel coordinate
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_br_pos`]
///module
#[doc(alias = "CANVAS_BR_POS")]
pub type CanvasBrPos = crate::Reg<canvas_br_pos::CANVAS_BR_POSrs>;
///Bottom-Right pixel coordinate
pub mod canvas_br_pos;
///CANVAS_BG (rw) register accessor: Background color
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_bg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_bg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_bg`]
///module
#[doc(alias = "CANVAS_BG")]
pub type CanvasBg = crate::Reg<canvas_bg::CANVAS_BGrs>;
///Background color
pub mod canvas_bg;
///VL_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_cfg`]
///module
#[doc(alias = "VL_CFG")]
pub type VlCfg = crate::Reg<vl_cfg::VL_CFGrs>;
///
pub mod vl_cfg;
///VL_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_tl_pos`]
///module
#[doc(alias = "VL_TL_POS")]
pub type VlTlPos = crate::Reg<vl_tl_pos::VL_TL_POSrs>;
///
pub mod vl_tl_pos;
///VL_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_br_pos`]
///module
#[doc(alias = "VL_BR_POS")]
pub type VlBrPos = crate::Reg<vl_br_pos::VL_BR_POSrs>;
///
pub mod vl_br_pos;
///VL_EXTENTS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_extents::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_extents::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_extents`]
///module
#[doc(alias = "VL_EXTENTS")]
pub type VlExtents = crate::Reg<vl_extents::VL_EXTENTSrs>;
///
pub mod vl_extents;
///VL_FILTER (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_filter`]
///module
#[doc(alias = "VL_FILTER")]
pub type VlFilter = crate::Reg<vl_filter::VL_FILTERrs>;
///
pub mod vl_filter;
///VL_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_src`]
///module
#[doc(alias = "VL_SRC")]
pub type VlSrc = crate::Reg<vl_src::VL_SRCrs>;
///
pub mod vl_src;
///VL_ROT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_rot`]
///module
#[doc(alias = "VL_ROT")]
pub type VlRot = crate::Reg<vl_rot::VL_ROTrs>;
///
pub mod vl_rot;
///VL_ROT_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_rot_stat`]
///module
#[doc(alias = "VL_ROT_STAT")]
pub type VlRotStat = crate::Reg<vl_rot_stat::VL_ROT_STATrs>;
///
pub mod vl_rot_stat;
///VL_SCALE_RATIO_H (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_ratio_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_ratio_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_scale_ratio_h`]
///module
#[doc(alias = "VL_SCALE_RATIO_H")]
pub type VlScaleRatioH = crate::Reg<vl_scale_ratio_h::VL_SCALE_RATIO_Hrs>;
///
pub mod vl_scale_ratio_h;
///VL_SCALE_RATIO_V (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_ratio_v::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_ratio_v::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_scale_ratio_v`]
///module
#[doc(alias = "VL_SCALE_RATIO_V")]
pub type VlScaleRatioV = crate::Reg<vl_scale_ratio_v::VL_SCALE_RATIO_Vrs>;
///
pub mod vl_scale_ratio_v;
///VL_FILL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_fill::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_fill::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_fill`]
///module
#[doc(alias = "VL_FILL")]
pub type VlFill = crate::Reg<vl_fill::VL_FILLrs>;
///
pub mod vl_fill;
///VL_MISC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_misc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_misc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_misc_cfg`]
///module
#[doc(alias = "VL_MISC_CFG")]
pub type VlMiscCfg = crate::Reg<vl_misc_cfg::VL_MISC_CFGrs>;
///
pub mod vl_misc_cfg;
///L0_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_cfg`]
///module
#[doc(alias = "L0_CFG")]
pub type L0Cfg = crate::Reg<l0_cfg::L0_CFGrs>;
///
pub mod l0_cfg;
///L0_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_tl_pos`]
///module
#[doc(alias = "L0_TL_POS")]
pub type L0TlPos = crate::Reg<l0_tl_pos::L0_TL_POSrs>;
///
pub mod l0_tl_pos;
///L0_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_br_pos`]
///module
#[doc(alias = "L0_BR_POS")]
pub type L0BrPos = crate::Reg<l0_br_pos::L0_BR_POSrs>;
///
pub mod l0_br_pos;
///L0_FILTER (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_filter`]
///module
#[doc(alias = "L0_FILTER")]
pub type L0Filter = crate::Reg<l0_filter::L0_FILTERrs>;
///
pub mod l0_filter;
///L0_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_src`]
///module
#[doc(alias = "L0_SRC")]
pub type L0Src = crate::Reg<l0_src::L0_SRCrs>;
///
pub mod l0_src;
///L0_FILL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_fill::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_fill::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_fill`]
///module
#[doc(alias = "L0_FILL")]
pub type L0Fill = crate::Reg<l0_fill::L0_FILLrs>;
///
pub mod l0_fill;
///L0_MISC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l0_misc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_misc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l0_misc_cfg`]
///module
#[doc(alias = "L0_MISC_CFG")]
pub type L0MiscCfg = crate::Reg<l0_misc_cfg::L0_MISC_CFGrs>;
///
pub mod l0_misc_cfg;
///L1_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_cfg`]
///module
#[doc(alias = "L1_CFG")]
pub type L1Cfg = crate::Reg<l1_cfg::L1_CFGrs>;
///
pub mod l1_cfg;
///L1_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_tl_pos`]
///module
#[doc(alias = "L1_TL_POS")]
pub type L1TlPos = crate::Reg<l1_tl_pos::L1_TL_POSrs>;
///
pub mod l1_tl_pos;
///L1_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_br_pos`]
///module
#[doc(alias = "L1_BR_POS")]
pub type L1BrPos = crate::Reg<l1_br_pos::L1_BR_POSrs>;
///
pub mod l1_br_pos;
///L1_FILTER (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_filter`]
///module
#[doc(alias = "L1_FILTER")]
pub type L1Filter = crate::Reg<l1_filter::L1_FILTERrs>;
///
pub mod l1_filter;
///L1_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_src`]
///module
#[doc(alias = "L1_SRC")]
pub type L1Src = crate::Reg<l1_src::L1_SRCrs>;
///
pub mod l1_src;
///L1_FILL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_fill::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_fill::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_fill`]
///module
#[doc(alias = "L1_FILL")]
pub type L1Fill = crate::Reg<l1_fill::L1_FILLrs>;
///
pub mod l1_fill;
///L1_MISC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l1_misc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_misc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l1_misc_cfg`]
///module
#[doc(alias = "L1_MISC_CFG")]
pub type L1MiscCfg = crate::Reg<l1_misc_cfg::L1_MISC_CFGrs>;
///
pub mod l1_misc_cfg;
///L2_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_cfg`]
///module
#[doc(alias = "L2_CFG")]
pub type L2Cfg = crate::Reg<l2_cfg::L2_CFGrs>;
///
pub mod l2_cfg;
///L2_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_tl_pos`]
///module
#[doc(alias = "L2_TL_POS")]
pub type L2TlPos = crate::Reg<l2_tl_pos::L2_TL_POSrs>;
///
pub mod l2_tl_pos;
///L2_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_br_pos`]
///module
#[doc(alias = "L2_BR_POS")]
pub type L2BrPos = crate::Reg<l2_br_pos::L2_BR_POSrs>;
///
pub mod l2_br_pos;
///L2_EXTENTS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_extents::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_extents::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_extents`]
///module
#[doc(alias = "L2_EXTENTS")]
pub type L2Extents = crate::Reg<l2_extents::L2_EXTENTSrs>;
///
pub mod l2_extents;
///L2_FILTER (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_filter`]
///module
#[doc(alias = "L2_FILTER")]
pub type L2Filter = crate::Reg<l2_filter::L2_FILTERrs>;
///
pub mod l2_filter;
///L2_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_src`]
///module
#[doc(alias = "L2_SRC")]
pub type L2Src = crate::Reg<l2_src::L2_SRCrs>;
///
pub mod l2_src;
///L2_ROT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_rot`]
///module
#[doc(alias = "L2_ROT")]
pub type L2Rot = crate::Reg<l2_rot::L2_ROTrs>;
///
pub mod l2_rot;
///L2_ROT_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_rot_stat`]
///module
#[doc(alias = "L2_ROT_STAT")]
pub type L2RotStat = crate::Reg<l2_rot_stat::L2_ROT_STATrs>;
///
pub mod l2_rot_stat;
///L2_SCALE_RATIO_H (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_ratio_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_ratio_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_scale_ratio_h`]
///module
#[doc(alias = "L2_SCALE_RATIO_H")]
pub type L2ScaleRatioH = crate::Reg<l2_scale_ratio_h::L2_SCALE_RATIO_Hrs>;
///
pub mod l2_scale_ratio_h;
///L2_SCALE_RATIO_V (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_ratio_v::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_ratio_v::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_scale_ratio_v`]
///module
#[doc(alias = "L2_SCALE_RATIO_V")]
pub type L2ScaleRatioV = crate::Reg<l2_scale_ratio_v::L2_SCALE_RATIO_Vrs>;
///
pub mod l2_scale_ratio_v;
///L2_FILL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_fill::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_fill::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_fill`]
///module
#[doc(alias = "L2_FILL")]
pub type L2Fill = crate::Reg<l2_fill::L2_FILLrs>;
///
pub mod l2_fill;
///L2_MISC_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_misc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_misc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_misc_cfg`]
///module
#[doc(alias = "L2_MISC_CFG")]
pub type L2MiscCfg = crate::Reg<l2_misc_cfg::L2_MISC_CFGrs>;
///
pub mod l2_misc_cfg;
///MASK_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`mask_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mask_cfg`]
///module
#[doc(alias = "MASK_CFG")]
pub type MaskCfg = crate::Reg<mask_cfg::MASK_CFGrs>;
///
pub mod mask_cfg;
///MASK_TL_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`mask_tl_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_tl_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mask_tl_pos`]
///module
#[doc(alias = "MASK_TL_POS")]
pub type MaskTlPos = crate::Reg<mask_tl_pos::MASK_TL_POSrs>;
///
pub mod mask_tl_pos;
///MASK_BR_POS (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`mask_br_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_br_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mask_br_pos`]
///module
#[doc(alias = "MASK_BR_POS")]
pub type MaskBrPos = crate::Reg<mask_br_pos::MASK_BR_POSrs>;
///
pub mod mask_br_pos;
///MASK_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`mask_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@mask_src`]
///module
#[doc(alias = "MASK_SRC")]
pub type MaskSrc = crate::Reg<mask_src::MASK_SRCrs>;
///
pub mod mask_src;
///COENG_CFG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`coeng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coeng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@coeng_cfg`]
///module
#[doc(alias = "COENG_CFG")]
pub type CoengCfg = crate::Reg<coeng_cfg::COENG_CFGrs>;
///
pub mod coeng_cfg;
///YUV_ENG_CFG0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`yuv_eng_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yuv_eng_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@yuv_eng_cfg0`]
///module
#[doc(alias = "YUV_ENG_CFG0")]
pub type YuvEngCfg0 = crate::Reg<yuv_eng_cfg0::YUV_ENG_CFG0rs>;
///
pub mod yuv_eng_cfg0;
///YUV_ENG_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`yuv_eng_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yuv_eng_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@yuv_eng_cfg1`]
///module
#[doc(alias = "YUV_ENG_CFG1")]
pub type YuvEngCfg1 = crate::Reg<yuv_eng_cfg1::YUV_ENG_CFG1rs>;
///
pub mod yuv_eng_cfg1;
///Y_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`y_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@y_src`]
///module
#[doc(alias = "Y_SRC")]
pub type YSrc = crate::Reg<y_src::Y_SRCrs>;
///
pub mod y_src;
///U_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`u_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@u_src`]
///module
#[doc(alias = "U_SRC")]
pub type USrc = crate::Reg<u_src::U_SRCrs>;
///
pub mod u_src;
///V_SRC (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`v_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`v_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@v_src`]
///module
#[doc(alias = "V_SRC")]
pub type VSrc = crate::Reg<v_src::V_SRCrs>;
///
pub mod v_src;
///COEF0 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`coef0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coef0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@coef0`]
///module
#[doc(alias = "COEF0")]
pub type Coef0 = crate::Reg<coef0::COEF0rs>;
///
pub mod coef0;
///COEF1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`coef1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coef1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@coef1`]
///module
#[doc(alias = "COEF1")]
pub type Coef1 = crate::Reg<coef1::COEF1rs>;
///
pub mod coef1;
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
///AHB_CTRL (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ahb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ahb_ctrl`]
///module
#[doc(alias = "AHB_CTRL")]
pub type AhbCtrl = crate::Reg<ahb_ctrl::AHB_CTRLrs>;
///
pub mod ahb_ctrl;
///AHB_MEM (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ahb_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ahb_mem`]
///module
#[doc(alias = "AHB_MEM")]
pub type AhbMem = crate::Reg<ahb_mem::AHB_MEMrs>;
///
pub mod ahb_mem;
///AHB_STRIDE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ahb_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ahb_stride`]
///module
#[doc(alias = "AHB_STRIDE")]
pub type AhbStride = crate::Reg<ahb_stride::AHB_STRIDErs>;
///
pub mod ahb_stride;
///DEBUG (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@debug`]
///module
#[doc(alias = "DEBUG")]
pub type Debug = crate::Reg<debug::DEBUGrs>;
///
pub mod debug;
///VL_ROT_M_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot_m_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot_m_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_rot_m_cfg1`]
///module
#[doc(alias = "VL_ROT_M_CFG1")]
pub type VlRotMCfg1 = crate::Reg<vl_rot_m_cfg1::VL_ROT_M_CFG1rs>;
///
pub mod vl_rot_m_cfg1;
///VL_ROT_M_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot_m_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot_m_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_rot_m_cfg2`]
///module
#[doc(alias = "VL_ROT_M_CFG2")]
pub type VlRotMCfg2 = crate::Reg<vl_rot_m_cfg2::VL_ROT_M_CFG2rs>;
///
pub mod vl_rot_m_cfg2;
///VL_ROT_M_CFG3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot_m_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot_m_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_rot_m_cfg3`]
///module
#[doc(alias = "VL_ROT_M_CFG3")]
pub type VlRotMCfg3 = crate::Reg<vl_rot_m_cfg3::VL_ROT_M_CFG3rs>;
///
pub mod vl_rot_m_cfg3;
///VL_SCALE_INIT_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_init_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_init_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_scale_init_cfg1`]
///module
#[doc(alias = "VL_SCALE_INIT_CFG1")]
pub type VlScaleInitCfg1 = crate::Reg<vl_scale_init_cfg1::VL_SCALE_INIT_CFG1rs>;
///
pub mod vl_scale_init_cfg1;
///VL_SCALE_INIT_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_init_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_init_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_scale_init_cfg2`]
///module
#[doc(alias = "VL_SCALE_INIT_CFG2")]
pub type VlScaleInitCfg2 = crate::Reg<vl_scale_init_cfg2::VL_SCALE_INIT_CFG2rs>;
///
pub mod vl_scale_init_cfg2;
///L2_ROT_M_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_m_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_m_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_rot_m_cfg1`]
///module
#[doc(alias = "L2_ROT_M_CFG1")]
pub type L2RotMCfg1 = crate::Reg<l2_rot_m_cfg1::L2_ROT_M_CFG1rs>;
///
pub mod l2_rot_m_cfg1;
///L2_ROT_M_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_m_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_m_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_rot_m_cfg2`]
///module
#[doc(alias = "L2_ROT_M_CFG2")]
pub type L2RotMCfg2 = crate::Reg<l2_rot_m_cfg2::L2_ROT_M_CFG2rs>;
///
pub mod l2_rot_m_cfg2;
///L2_ROT_M_CFG3 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_m_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_m_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_rot_m_cfg3`]
///module
#[doc(alias = "L2_ROT_M_CFG3")]
pub type L2RotMCfg3 = crate::Reg<l2_rot_m_cfg3::L2_ROT_M_CFG3rs>;
///
pub mod l2_rot_m_cfg3;
///L2_SCALE_INIT_CFG1 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_init_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_init_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_scale_init_cfg1`]
///module
#[doc(alias = "L2_SCALE_INIT_CFG1")]
pub type L2ScaleInitCfg1 = crate::Reg<l2_scale_init_cfg1::L2_SCALE_INIT_CFG1rs>;
///
pub mod l2_scale_init_cfg1;
///L2_SCALE_INIT_CFG2 (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_init_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_init_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@l2_scale_init_cfg2`]
///module
#[doc(alias = "L2_SCALE_INIT_CFG2")]
pub type L2ScaleInitCfg2 = crate::Reg<l2_scale_init_cfg2::L2_SCALE_INIT_CFG2rs>;
///
pub mod l2_scale_init_cfg2;
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
///CANVAS_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@canvas_stat`]
///module
#[doc(alias = "CANVAS_STAT")]
pub type CanvasStat = crate::Reg<canvas_stat::CANVAS_STATrs>;
///
pub mod canvas_stat;
///EZIP_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ezip_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ezip_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ezip_stat`]
///module
#[doc(alias = "EZIP_STAT")]
pub type EzipStat = crate::Reg<ezip_stat::EZIP_STATrs>;
///
pub mod ezip_stat;
///OL_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ol_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ol_stat`]
///module
#[doc(alias = "OL_STAT")]
pub type OlStat = crate::Reg<ol_stat::OL_STATrs>;
///
pub mod ol_stat;
///OL2_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ol2_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol2_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ol2_stat`]
///module
#[doc(alias = "OL2_STAT")]
pub type Ol2Stat = crate::Reg<ol2_stat::OL2_STATrs>;
///
pub mod ol2_stat;
///VL_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`vl_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@vl_stat`]
///module
#[doc(alias = "VL_STAT")]
pub type VlStat = crate::Reg<vl_stat::VL_STATrs>;
///
pub mod vl_stat;
///ML_STAT (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`ml_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ml_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ml_stat`]
///module
#[doc(alias = "ML_STAT")]
pub type MlStat = crate::Reg<ml_stat::ML_STATrs>;
///
pub mod ml_stat;
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
