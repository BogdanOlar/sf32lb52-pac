#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ezip_ctrl: EzipCtrl,
    src_addr: SrcAddr,
    dst_addr: DstAddr,
    ezip_para: EzipPara,
    cache_clr: CacheClr,
    start_point: StartPoint,
    end_point: EndPoint,
    row_sign: RowSign,
    int_en: IntEn,
    int_sta: IntSta,
    int_mask: IntMask,
    nap_para: NapPara,
    src_len: SrcLen,
    aezip_ctrl: AezipCtrl,
    frame_start: FrameStart,
    play_start: PlayStart,
    frame_num: FrameNum,
    play_num: PlayNum,
    seq_num: SeqNum,
    frame_area: FrameArea,
    frame_offset: FrameOffset,
    frame_delay: FrameDelay,
    frame_type: FrameType,
    frame_size: FrameSize,
    grey_para: GreyPara,
    db_sel: DbSel,
    db_data0: DbData0,
    db_data1: DbData1,
    db_data2: DbData2,
    db_data3: DbData3,
    db_data4: DbData4,
    db_data5: DbData5,
    db_data6: DbData6,
    db_data7: DbData7,
    db_data8: DbData8,
    db_data9: DbData9,
    db_data10: DbData10,
    db_data11: DbData11,
    db_data12: DbData12,
    db_data13: DbData13,
}
impl RegisterBlock {
    ///0x00 - ezip/aezip_frame decoder ctrl
    #[inline(always)]
    pub const fn ezip_ctrl(&self) -> &EzipCtrl {
        &self.ezip_ctrl
    }
    ///0x04 - ezip decoder source address
    #[inline(always)]
    pub const fn src_addr(&self) -> &SrcAddr {
        &self.src_addr
    }
    ///0x08 - ezip decoder destination address
    #[inline(always)]
    pub const fn dst_addr(&self) -> &DstAddr {
        &self.dst_addr
    }
    ///0x0c - ezip decoder parameter
    #[inline(always)]
    pub const fn ezip_para(&self) -> &EzipPara {
        &self.ezip_para
    }
    ///0x10 - ezip index cache clear
    #[inline(always)]
    pub const fn cache_clr(&self) -> &CacheClr {
        &self.cache_clr
    }
    ///0x14 - ezip decoder start point
    #[inline(always)]
    pub const fn start_point(&self) -> &StartPoint {
        &self.start_point
    }
    ///0x18 - ezip decoder end point
    #[inline(always)]
    pub const fn end_point(&self) -> &EndPoint {
        &self.end_point
    }
    ///0x1c - ezip decoder row sign
    #[inline(always)]
    pub const fn row_sign(&self) -> &RowSign {
        &self.row_sign
    }
    ///0x20 - ezip decoder _int_en
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    ///0x24 - ezip decoder _int_sta
    #[inline(always)]
    pub const fn int_sta(&self) -> &IntSta {
        &self.int_sta
    }
    ///0x28 - ezip decoder int mask state
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    ///0x2c - ezip decoder release bus parameter
    #[inline(always)]
    pub const fn nap_para(&self) -> &NapPara {
        &self.nap_para
    }
    ///0x30 - ezip source data length
    #[inline(always)]
    pub const fn src_len(&self) -> &SrcLen {
        &self.src_len
    }
    ///0x34 - AEZIP ctrl
    #[inline(always)]
    pub const fn aezip_ctrl(&self) -> &AezipCtrl {
        &self.aezip_ctrl
    }
    ///0x38 - Aezip start number of frames
    #[inline(always)]
    pub const fn frame_start(&self) -> &FrameStart {
        &self.frame_start
    }
    ///0x3c - Aezip start number of play
    #[inline(always)]
    pub const fn play_start(&self) -> &PlayStart {
        &self.play_start
    }
    ///0x40 - Aezip number of frames
    #[inline(always)]
    pub const fn frame_num(&self) -> &FrameNum {
        &self.frame_num
    }
    ///0x44 - Aezip number of times to loop this AEZIP
    #[inline(always)]
    pub const fn play_num(&self) -> &PlayNum {
        &self.play_num
    }
    ///0x48 - Aezip sequence number
    #[inline(always)]
    pub const fn seq_num(&self) -> &SeqNum {
        &self.seq_num
    }
    ///0x4c - Aezip frame area
    #[inline(always)]
    pub const fn frame_area(&self) -> &FrameArea {
        &self.frame_area
    }
    ///0x50 - Aezip frame area
    #[inline(always)]
    pub const fn frame_offset(&self) -> &FrameOffset {
        &self.frame_offset
    }
    ///0x54 - Aezip frame delay
    #[inline(always)]
    pub const fn frame_delay(&self) -> &FrameDelay {
        &self.frame_delay
    }
    ///0x58 -
    #[inline(always)]
    pub const fn frame_type(&self) -> &FrameType {
        &self.frame_type
    }
    ///0x5c - Aezip frame size
    #[inline(always)]
    pub const fn frame_size(&self) -> &FrameSize {
        &self.frame_size
    }
    ///0x60 -
    #[inline(always)]
    pub const fn grey_para(&self) -> &GreyPara {
        &self.grey_para
    }
    ///0x64 - ezip decoder debug sel
    #[inline(always)]
    pub const fn db_sel(&self) -> &DbSel {
        &self.db_sel
    }
    ///0x68 - ezip decoder debug data0
    #[inline(always)]
    pub const fn db_data0(&self) -> &DbData0 {
        &self.db_data0
    }
    ///0x6c - ezip decoder debug data1
    #[inline(always)]
    pub const fn db_data1(&self) -> &DbData1 {
        &self.db_data1
    }
    ///0x70 - ezip decoder debug data2
    #[inline(always)]
    pub const fn db_data2(&self) -> &DbData2 {
        &self.db_data2
    }
    ///0x74 - ezip decoder debug data3
    #[inline(always)]
    pub const fn db_data3(&self) -> &DbData3 {
        &self.db_data3
    }
    ///0x78 - ezip decoder debug data4
    #[inline(always)]
    pub const fn db_data4(&self) -> &DbData4 {
        &self.db_data4
    }
    ///0x7c - ezip decoder debug data5
    #[inline(always)]
    pub const fn db_data5(&self) -> &DbData5 {
        &self.db_data5
    }
    ///0x80 - ezip decoder debug data6
    #[inline(always)]
    pub const fn db_data6(&self) -> &DbData6 {
        &self.db_data6
    }
    ///0x84 - ezip decoder debug data7
    #[inline(always)]
    pub const fn db_data7(&self) -> &DbData7 {
        &self.db_data7
    }
    ///0x88 - ezip decoder debug data8
    #[inline(always)]
    pub const fn db_data8(&self) -> &DbData8 {
        &self.db_data8
    }
    ///0x8c - ezip decoder debug data9
    #[inline(always)]
    pub const fn db_data9(&self) -> &DbData9 {
        &self.db_data9
    }
    ///0x90 - ezip decoder debug data10
    #[inline(always)]
    pub const fn db_data10(&self) -> &DbData10 {
        &self.db_data10
    }
    ///0x94 - ezip decoder debug data11
    #[inline(always)]
    pub const fn db_data11(&self) -> &DbData11 {
        &self.db_data11
    }
    ///0x98 - ezip decoder debug data12
    #[inline(always)]
    pub const fn db_data12(&self) -> &DbData12 {
        &self.db_data12
    }
    ///0x9c - ezip decoder debug data13
    #[inline(always)]
    pub const fn db_data13(&self) -> &DbData13 {
        &self.db_data13
    }
}
///EZIP_CTRL (rw) register accessor: ezip/aezip_frame decoder ctrl
///
///You can [`read`](crate::Reg::read) this register and get [`ezip_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ezip_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ezip_ctrl`]
///module
#[doc(alias = "EZIP_CTRL")]
pub type EzipCtrl = crate::Reg<ezip_ctrl::EZIP_CTRLrs>;
///ezip/aezip_frame decoder ctrl
pub mod ezip_ctrl;
///SRC_ADDR (rw) register accessor: ezip decoder source address
///
///You can [`read`](crate::Reg::read) this register and get [`src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@src_addr`]
///module
#[doc(alias = "SRC_ADDR")]
pub type SrcAddr = crate::Reg<src_addr::SRC_ADDRrs>;
///ezip decoder source address
pub mod src_addr;
///DST_ADDR (rw) register accessor: ezip decoder destination address
///
///You can [`read`](crate::Reg::read) this register and get [`dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@dst_addr`]
///module
#[doc(alias = "DST_ADDR")]
pub type DstAddr = crate::Reg<dst_addr::DST_ADDRrs>;
///ezip decoder destination address
pub mod dst_addr;
///EZIP_PARA (rw) register accessor: ezip decoder parameter
///
///You can [`read`](crate::Reg::read) this register and get [`ezip_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ezip_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@ezip_para`]
///module
#[doc(alias = "EZIP_PARA")]
pub type EzipPara = crate::Reg<ezip_para::EZIP_PARArs>;
///ezip decoder parameter
pub mod ezip_para;
///CACHE_CLR (rw) register accessor: ezip index cache clear
///
///You can [`read`](crate::Reg::read) this register and get [`cache_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@cache_clr`]
///module
#[doc(alias = "CACHE_CLR")]
pub type CacheClr = crate::Reg<cache_clr::CACHE_CLRrs>;
///ezip index cache clear
pub mod cache_clr;
///START_POINT (rw) register accessor: ezip decoder start point
///
///You can [`read`](crate::Reg::read) this register and get [`start_point::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_point::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@start_point`]
///module
#[doc(alias = "START_POINT")]
pub type StartPoint = crate::Reg<start_point::START_POINTrs>;
///ezip decoder start point
pub mod start_point;
///END_POINT (rw) register accessor: ezip decoder end point
///
///You can [`read`](crate::Reg::read) this register and get [`end_point::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end_point::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@end_point`]
///module
#[doc(alias = "END_POINT")]
pub type EndPoint = crate::Reg<end_point::END_POINTrs>;
///ezip decoder end point
pub mod end_point;
///ROW_SIGN (rw) register accessor: ezip decoder row sign
///
///You can [`read`](crate::Reg::read) this register and get [`row_sign::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`row_sign::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@row_sign`]
///module
#[doc(alias = "ROW_SIGN")]
pub type RowSign = crate::Reg<row_sign::ROW_SIGNrs>;
///ezip decoder row sign
pub mod row_sign;
///INT_EN (rw) register accessor: ezip decoder _int_en
///
///You can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_en`]
///module
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::INT_ENrs>;
///ezip decoder _int_en
pub mod int_en;
///INT_STA (rw) register accessor: ezip decoder _int_sta
///
///You can [`read`](crate::Reg::read) this register and get [`int_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_sta`]
///module
#[doc(alias = "INT_STA")]
pub type IntSta = crate::Reg<int_sta::INT_STArs>;
///ezip decoder _int_sta
pub mod int_sta;
///INT_MASK (rw) register accessor: ezip decoder int mask state
///
///You can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@int_mask`]
///module
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::INT_MASKrs>;
///ezip decoder int mask state
pub mod int_mask;
///NAP_PARA (rw) register accessor: ezip decoder release bus parameter
///
///You can [`read`](crate::Reg::read) this register and get [`nap_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nap_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@nap_para`]
///module
#[doc(alias = "NAP_PARA")]
pub type NapPara = crate::Reg<nap_para::NAP_PARArs>;
///ezip decoder release bus parameter
pub mod nap_para;
///SRC_LEN (rw) register accessor: ezip source data length
///
///You can [`read`](crate::Reg::read) this register and get [`src_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@src_len`]
///module
#[doc(alias = "SRC_LEN")]
pub type SrcLen = crate::Reg<src_len::SRC_LENrs>;
///ezip source data length
pub mod src_len;
///AEZIP_CTRL (rw) register accessor: AEZIP ctrl
///
///You can [`read`](crate::Reg::read) this register and get [`aezip_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aezip_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@aezip_ctrl`]
///module
#[doc(alias = "AEZIP_CTRL")]
pub type AezipCtrl = crate::Reg<aezip_ctrl::AEZIP_CTRLrs>;
///AEZIP ctrl
pub mod aezip_ctrl;
///FRAME_START (rw) register accessor: Aezip start number of frames
///
///You can [`read`](crate::Reg::read) this register and get [`frame_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_start`]
///module
#[doc(alias = "FRAME_START")]
pub type FrameStart = crate::Reg<frame_start::FRAME_STARTrs>;
///Aezip start number of frames
pub mod frame_start;
///PLAY_START (rw) register accessor: Aezip start number of play
///
///You can [`read`](crate::Reg::read) this register and get [`play_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`play_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@play_start`]
///module
#[doc(alias = "PLAY_START")]
pub type PlayStart = crate::Reg<play_start::PLAY_STARTrs>;
///Aezip start number of play
pub mod play_start;
///FRAME_NUM (rw) register accessor: Aezip number of frames
///
///You can [`read`](crate::Reg::read) this register and get [`frame_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_num`]
///module
#[doc(alias = "FRAME_NUM")]
pub type FrameNum = crate::Reg<frame_num::FRAME_NUMrs>;
///Aezip number of frames
pub mod frame_num;
///PLAY_NUM (rw) register accessor: Aezip number of times to loop this AEZIP
///
///You can [`read`](crate::Reg::read) this register and get [`play_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`play_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@play_num`]
///module
#[doc(alias = "PLAY_NUM")]
pub type PlayNum = crate::Reg<play_num::PLAY_NUMrs>;
///Aezip number of times to loop this AEZIP
pub mod play_num;
///SEQ_NUM (rw) register accessor: Aezip sequence number
///
///You can [`read`](crate::Reg::read) this register and get [`seq_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@seq_num`]
///module
#[doc(alias = "SEQ_NUM")]
pub type SeqNum = crate::Reg<seq_num::SEQ_NUMrs>;
///Aezip sequence number
pub mod seq_num;
///FRAME_AREA (rw) register accessor: Aezip frame area
///
///You can [`read`](crate::Reg::read) this register and get [`frame_area::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_area::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_area`]
///module
#[doc(alias = "FRAME_AREA")]
pub type FrameArea = crate::Reg<frame_area::FRAME_AREArs>;
///Aezip frame area
pub mod frame_area;
///FRAME_OFFSET (rw) register accessor: Aezip frame area
///
///You can [`read`](crate::Reg::read) this register and get [`frame_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_offset`]
///module
#[doc(alias = "FRAME_OFFSET")]
pub type FrameOffset = crate::Reg<frame_offset::FRAME_OFFSETrs>;
///Aezip frame area
pub mod frame_offset;
///FRAME_DELAY (rw) register accessor: Aezip frame delay
///
///You can [`read`](crate::Reg::read) this register and get [`frame_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_delay`]
///module
#[doc(alias = "FRAME_DELAY")]
pub type FrameDelay = crate::Reg<frame_delay::FRAME_DELAYrs>;
///Aezip frame delay
pub mod frame_delay;
///FRAME_TYPE (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`frame_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_type`]
///module
#[doc(alias = "FRAME_TYPE")]
pub type FrameType = crate::Reg<frame_type::FRAME_TYPErs>;
///
pub mod frame_type;
///FRAME_SIZE (rw) register accessor: Aezip frame size
///
///You can [`read`](crate::Reg::read) this register and get [`frame_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@frame_size`]
///module
#[doc(alias = "FRAME_SIZE")]
pub type FrameSize = crate::Reg<frame_size::FRAME_SIZErs>;
///Aezip frame size
pub mod frame_size;
///GREY_PARA (rw) register accessor:
///
///You can [`read`](crate::Reg::read) this register and get [`grey_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grey_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@grey_para`]
///module
#[doc(alias = "GREY_PARA")]
pub type GreyPara = crate::Reg<grey_para::GREY_PARArs>;
///
pub mod grey_para;
///DB_SEL (rw) register accessor: ezip decoder debug sel
///
///You can [`read`](crate::Reg::read) this register and get [`db_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_sel`]
///module
#[doc(alias = "DB_SEL")]
pub type DbSel = crate::Reg<db_sel::DB_SELrs>;
///ezip decoder debug sel
pub mod db_sel;
///DB_DATA0 (rw) register accessor: ezip decoder debug data0
///
///You can [`read`](crate::Reg::read) this register and get [`db_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data0`]
///module
#[doc(alias = "DB_DATA0")]
pub type DbData0 = crate::Reg<db_data0::DB_DATA0rs>;
///ezip decoder debug data0
pub mod db_data0;
///DB_DATA1 (rw) register accessor: ezip decoder debug data1
///
///You can [`read`](crate::Reg::read) this register and get [`db_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data1`]
///module
#[doc(alias = "DB_DATA1")]
pub type DbData1 = crate::Reg<db_data1::DB_DATA1rs>;
///ezip decoder debug data1
pub mod db_data1;
///DB_DATA2 (rw) register accessor: ezip decoder debug data2
///
///You can [`read`](crate::Reg::read) this register and get [`db_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data2`]
///module
#[doc(alias = "DB_DATA2")]
pub type DbData2 = crate::Reg<db_data2::DB_DATA2rs>;
///ezip decoder debug data2
pub mod db_data2;
///DB_DATA3 (rw) register accessor: ezip decoder debug data3
///
///You can [`read`](crate::Reg::read) this register and get [`db_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data3`]
///module
#[doc(alias = "DB_DATA3")]
pub type DbData3 = crate::Reg<db_data3::DB_DATA3rs>;
///ezip decoder debug data3
pub mod db_data3;
///DB_DATA4 (rw) register accessor: ezip decoder debug data4
///
///You can [`read`](crate::Reg::read) this register and get [`db_data4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data4`]
///module
#[doc(alias = "DB_DATA4")]
pub type DbData4 = crate::Reg<db_data4::DB_DATA4rs>;
///ezip decoder debug data4
pub mod db_data4;
///DB_DATA5 (rw) register accessor: ezip decoder debug data5
///
///You can [`read`](crate::Reg::read) this register and get [`db_data5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data5`]
///module
#[doc(alias = "DB_DATA5")]
pub type DbData5 = crate::Reg<db_data5::DB_DATA5rs>;
///ezip decoder debug data5
pub mod db_data5;
///DB_DATA6 (rw) register accessor: ezip decoder debug data6
///
///You can [`read`](crate::Reg::read) this register and get [`db_data6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data6`]
///module
#[doc(alias = "DB_DATA6")]
pub type DbData6 = crate::Reg<db_data6::DB_DATA6rs>;
///ezip decoder debug data6
pub mod db_data6;
///DB_DATA7 (rw) register accessor: ezip decoder debug data7
///
///You can [`read`](crate::Reg::read) this register and get [`db_data7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data7`]
///module
#[doc(alias = "DB_DATA7")]
pub type DbData7 = crate::Reg<db_data7::DB_DATA7rs>;
///ezip decoder debug data7
pub mod db_data7;
///DB_DATA8 (rw) register accessor: ezip decoder debug data8
///
///You can [`read`](crate::Reg::read) this register and get [`db_data8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data8`]
///module
#[doc(alias = "DB_DATA8")]
pub type DbData8 = crate::Reg<db_data8::DB_DATA8rs>;
///ezip decoder debug data8
pub mod db_data8;
///DB_DATA9 (rw) register accessor: ezip decoder debug data9
///
///You can [`read`](crate::Reg::read) this register and get [`db_data9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data9`]
///module
#[doc(alias = "DB_DATA9")]
pub type DbData9 = crate::Reg<db_data9::DB_DATA9rs>;
///ezip decoder debug data9
pub mod db_data9;
///DB_DATA10 (rw) register accessor: ezip decoder debug data10
///
///You can [`read`](crate::Reg::read) this register and get [`db_data10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data10`]
///module
#[doc(alias = "DB_DATA10")]
pub type DbData10 = crate::Reg<db_data10::DB_DATA10rs>;
///ezip decoder debug data10
pub mod db_data10;
///DB_DATA11 (rw) register accessor: ezip decoder debug data11
///
///You can [`read`](crate::Reg::read) this register and get [`db_data11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data11`]
///module
#[doc(alias = "DB_DATA11")]
pub type DbData11 = crate::Reg<db_data11::DB_DATA11rs>;
///ezip decoder debug data11
pub mod db_data11;
///DB_DATA12 (rw) register accessor: ezip decoder debug data12
///
///You can [`read`](crate::Reg::read) this register and get [`db_data12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data12`]
///module
#[doc(alias = "DB_DATA12")]
pub type DbData12 = crate::Reg<db_data12::DB_DATA12rs>;
///ezip decoder debug data12
pub mod db_data12;
///DB_DATA13 (rw) register accessor: ezip decoder debug data13
///
///You can [`read`](crate::Reg::read) this register and get [`db_data13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [`mod@db_data13`]
///module
#[doc(alias = "DB_DATA13")]
pub type DbData13 = crate::Reg<db_data13::DB_DATA13rs>;
///ezip decoder debug data13
pub mod db_data13;
