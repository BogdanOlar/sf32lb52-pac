///Register `BUCK_CR1` reader
pub type R = crate::R<BUCK_CR1rs>;
///Register `BUCK_CR1` writer
pub type W = crate::W<BUCK_CR1rs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRL` reader -
pub type CtrlR = crate::BitReader;
///Field `CTRL` writer -
pub type CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MOT_CTUNE` reader -
pub type MotCtuneR = crate::FieldReader;
///Field `MOT_CTUNE` writer -
pub type MotCtuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COT_CTUNE` reader -
pub type CotCtuneR = crate::FieldReader;
///Field `COT_CTUNE` writer -
pub type CotCtuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMP_BM_AHI` reader -
pub type CompBmAhiR = crate::BitReader;
///Field `COMP_BM_AHI` writer -
pub type CompBmAhiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_IQ_TUNE` reader -
pub type CompIqTuneR = crate::FieldReader;
///Field `COMP_IQ_TUNE` writer -
pub type CompIqTuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP_IDYN_TUNE` reader -
pub type CompIdynTuneR = crate::FieldReader;
///Field `COMP_IDYN_TUNE` writer -
pub type CompIdynTuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOCP_TUNE` reader -
pub type IocpTuneR = crate::FieldReader;
///Field `IOCP_TUNE` writer -
pub type IocpTuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SEL_IOCP_HI` reader -
pub type SelIocpHiR = crate::BitReader;
///Field `SEL_IOCP_HI` writer -
pub type SelIocpHiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_LX22` reader -
pub type SelLx22R = crate::BitReader;
///Field `SEL_LX22` writer -
pub type SelLx22W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCP_AON` reader -
pub type OcpAonR = crate::BitReader;
///Field `OCP_AON` writer -
pub type OcpAonW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZCD_AON` reader -
pub type ZcdAonR = crate::BitReader;
///Field `ZCD_AON` writer -
pub type ZcdAonW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UVLO_X_BIAS` reader -
pub type UvloXBiasR = crate::BitReader;
///Field `UVLO_X_BIAS` writer -
pub type UvloXBiasW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG_BUF_VOS_TRIM` reader -
pub type BgBufVosTrimR = crate::FieldReader;
///Field `BG_BUF_VOS_TRIM` writer -
pub type BgBufVosTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BG_BUF_VOS_STEP` reader -
pub type BgBufVosStepR = crate::FieldReader;
///Field `BG_BUF_VOS_STEP` writer -
pub type BgBufVosStepW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BG_BUF_VOS_POLAR` reader -
pub type BgBufVosPolarR = crate::BitReader;
///Field `BG_BUF_VOS_POLAR` writer -
pub type BgBufVosPolarW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SS_DONE` reader -
pub type SsDoneR = crate::BitReader;
///Field `SS_DONE` writer -
pub type SsDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:8
    #[inline(always)]
    pub fn mot_ctune(&self) -> MotCtuneR {
        MotCtuneR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11
    #[inline(always)]
    pub fn cot_ctune(&self) -> CotCtuneR {
        CotCtuneR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12
    #[inline(always)]
    pub fn comp_bm_ahi(&self) -> CompBmAhiR {
        CompBmAhiR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14
    #[inline(always)]
    pub fn comp_iq_tune(&self) -> CompIqTuneR {
        CompIqTuneR::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bits 15:16
    #[inline(always)]
    pub fn comp_idyn_tune(&self) -> CompIdynTuneR {
        CompIdynTuneR::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn iocp_tune(&self) -> IocpTuneR {
        IocpTuneR::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20
    #[inline(always)]
    pub fn sel_iocp_hi(&self) -> SelIocpHiR {
        SelIocpHiR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn sel_lx22(&self) -> SelLx22R {
        SelLx22R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn ocp_aon(&self) -> OcpAonR {
        OcpAonR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23
    #[inline(always)]
    pub fn zcd_aon(&self) -> ZcdAonR {
        ZcdAonR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn uvlo_x_bias(&self) -> UvloXBiasR {
        UvloXBiasR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27
    #[inline(always)]
    pub fn bg_buf_vos_trim(&self) -> BgBufVosTrimR {
        BgBufVosTrimR::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:29
    #[inline(always)]
    pub fn bg_buf_vos_step(&self) -> BgBufVosStepR {
        BgBufVosStepR::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30
    #[inline(always)]
    pub fn bg_buf_vos_polar(&self) -> BgBufVosPolarR {
        BgBufVosPolarR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn ss_done(&self) -> SsDoneR {
        SsDoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUCK_CR1")
            .field("ss_done", &self.ss_done())
            .field("bg_buf_vos_polar", &self.bg_buf_vos_polar())
            .field("bg_buf_vos_step", &self.bg_buf_vos_step())
            .field("bg_buf_vos_trim", &self.bg_buf_vos_trim())
            .field("uvlo_x_bias", &self.uvlo_x_bias())
            .field("zcd_aon", &self.zcd_aon())
            .field("ocp_aon", &self.ocp_aon())
            .field("sel_lx22", &self.sel_lx22())
            .field("sel_iocp_hi", &self.sel_iocp_hi())
            .field("iocp_tune", &self.iocp_tune())
            .field("comp_idyn_tune", &self.comp_idyn_tune())
            .field("comp_iq_tune", &self.comp_iq_tune())
            .field("comp_bm_ahi", &self.comp_bm_ahi())
            .field("cot_ctune", &self.cot_ctune())
            .field("mot_ctune", &self.mot_ctune())
            .field("rsvd", &self.rsvd())
            .field("ctrl", &self.ctrl())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<BUCK_CR1rs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn ctrl(&mut self) -> CtrlW<BUCK_CR1rs> {
        CtrlW::new(self, 1)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<BUCK_CR1rs> {
        RsvdW::new(self, 2)
    }
    ///Bits 6:8
    #[inline(always)]
    pub fn mot_ctune(&mut self) -> MotCtuneW<BUCK_CR1rs> {
        MotCtuneW::new(self, 6)
    }
    ///Bits 9:11
    #[inline(always)]
    pub fn cot_ctune(&mut self) -> CotCtuneW<BUCK_CR1rs> {
        CotCtuneW::new(self, 9)
    }
    ///Bit 12
    #[inline(always)]
    pub fn comp_bm_ahi(&mut self) -> CompBmAhiW<BUCK_CR1rs> {
        CompBmAhiW::new(self, 12)
    }
    ///Bits 13:14
    #[inline(always)]
    pub fn comp_iq_tune(&mut self) -> CompIqTuneW<BUCK_CR1rs> {
        CompIqTuneW::new(self, 13)
    }
    ///Bits 15:16
    #[inline(always)]
    pub fn comp_idyn_tune(&mut self) -> CompIdynTuneW<BUCK_CR1rs> {
        CompIdynTuneW::new(self, 15)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn iocp_tune(&mut self) -> IocpTuneW<BUCK_CR1rs> {
        IocpTuneW::new(self, 17)
    }
    ///Bit 20
    #[inline(always)]
    pub fn sel_iocp_hi(&mut self) -> SelIocpHiW<BUCK_CR1rs> {
        SelIocpHiW::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn sel_lx22(&mut self) -> SelLx22W<BUCK_CR1rs> {
        SelLx22W::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    pub fn ocp_aon(&mut self) -> OcpAonW<BUCK_CR1rs> {
        OcpAonW::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    pub fn zcd_aon(&mut self) -> ZcdAonW<BUCK_CR1rs> {
        ZcdAonW::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    pub fn uvlo_x_bias(&mut self) -> UvloXBiasW<BUCK_CR1rs> {
        UvloXBiasW::new(self, 24)
    }
    ///Bits 25:27
    #[inline(always)]
    pub fn bg_buf_vos_trim(&mut self) -> BgBufVosTrimW<BUCK_CR1rs> {
        BgBufVosTrimW::new(self, 25)
    }
    ///Bits 28:29
    #[inline(always)]
    pub fn bg_buf_vos_step(&mut self) -> BgBufVosStepW<BUCK_CR1rs> {
        BgBufVosStepW::new(self, 28)
    }
    ///Bit 30
    #[inline(always)]
    pub fn bg_buf_vos_polar(&mut self) -> BgBufVosPolarW<BUCK_CR1rs> {
        BgBufVosPolarW::new(self, 30)
    }
    ///Bit 31
    #[inline(always)]
    pub fn ss_done(&mut self) -> SsDoneW<BUCK_CR1rs> {
        SsDoneW::new(self, 31)
    }
}
///BUCK Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`buck_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BUCK_CR1rs;
impl crate::RegisterSpec for BUCK_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`buck_cr1::R`](R) reader structure
impl crate::Readable for BUCK_CR1rs {}
///`write(|w| ..)` method takes [`buck_cr1::W`](W) writer structure
impl crate::Writable for BUCK_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BUCK_CR1 to value 0
impl crate::Resettable for BUCK_CR1rs {
    const RESET_VALUE: u32 = 0;
}
