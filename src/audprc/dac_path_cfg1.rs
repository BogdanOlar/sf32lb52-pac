///Register `DAC_PATH_CFG1` reader
pub type R = crate::R<DAC_PATH_CFG1rs>;
///Register `DAC_PATH_CFG1` writer
pub type W = crate::W<DAC_PATH_CFG1rs>;
///Field `MUXLSRC0` reader - dac mux left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxlsrc0R = crate::FieldReader;
///Field `MUXLSRC0` writer - dac mux left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxlsrc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MUXLSRC1` reader - dac mux left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxlsrc1R = crate::FieldReader;
///Field `MUXLSRC1` writer - dac mux left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxlsrc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MUXRSRC0` reader - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxrsrc0R = crate::FieldReader;
///Field `MUXRSRC0` writer - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxrsrc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MUXRSRC1` reader - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxrsrc1R = crate::FieldReader;
///Field `MUXRSRC1` writer - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
pub type Muxrsrc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EQ_CH_EN` reader - equalizer channel enable 2'b11: enable both channel 2'b10: enable right chanel only 2'b01: enable left channel only 2'b00: bypass equalizer
pub type EqChEnR = crate::FieldReader;
///Field `EQ_CH_EN` writer - equalizer channel enable 2'b11: enable both channel 2'b10: enable right chanel only 2'b01: enable left channel only 2'b00: bypass equalizer
pub type EqChEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EQ_STAGE` reader - set equalizer stage, max is 10.
pub type EqStageR = crate::FieldReader;
///Field `EQ_STAGE` writer - set equalizer stage, max is 10.
pub type EqStageW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EQ_CLR_DONE` reader - equalizer clear done flag
pub type EqClrDoneR = crate::BitReader;
///Field `EQ_CLR_DONE` writer - equalizer clear done flag
pub type EqClrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EQ_CLR` reader - equalizer clear request
pub type EqClrR = crate::BitReader;
///Field `EQ_CLR` writer - equalizer clear request
pub type EqClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_CH_EN` reader - source rate converter channel enable
pub type SrcChEnR = crate::FieldReader;
///Field `SRC_CH_EN` writer - source rate converter channel enable
pub type SrcChEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRC_HBF1_EN` reader - 1st stage hbf enable
pub type SrcHbf1EnR = crate::BitReader;
///Field `SRC_HBF1_EN` writer - 1st stage hbf enable
pub type SrcHbf1EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_HBF1_MODE` reader - 1st stage hbf mode: 0: upsampling 1: downsampling
pub type SrcHbf1ModeR = crate::BitReader;
///Field `SRC_HBF1_MODE` writer - 1st stage hbf mode: 0: upsampling 1: downsampling
pub type SrcHbf1ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_HBF2_EN` reader - 2nd stage hbf enable
pub type SrcHbf2EnR = crate::BitReader;
///Field `SRC_HBF2_EN` writer - 2nd stage hbf enable
pub type SrcHbf2EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_HBF2_MODE` reader - 2nd stage hbf mode: 0: upsampling 1: downsampling
pub type SrcHbf2ModeR = crate::BitReader;
///Field `SRC_HBF2_MODE` writer - 2nd stage hbf mode: 0: upsampling 1: downsampling
pub type SrcHbf2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_HBF3_EN` reader - 3rd stage hbf enable
pub type SrcHbf3EnR = crate::BitReader;
///Field `SRC_HBF3_EN` writer - 3rd stage hbf enable
pub type SrcHbf3EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_HBF3_MODE` reader - 3rd stage hbf mode: 0: upsampling 1: downsampling
pub type SrcHbf3ModeR = crate::BitReader;
///Field `SRC_HBF3_MODE` writer - 3rd stage hbf mode: 0: upsampling 1: downsampling
pub type SrcHbf3ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRC_CH_CLR_DONE` reader - src channel internal data clear done
pub type SrcChClrDoneR = crate::FieldReader;
///Field `SRC_CH_CLR_DONE` writer - src channel internal data clear done
pub type SrcChClrDoneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRC_CH_CLR` reader - clear src channal internal data
pub type SrcChClrR = crate::FieldReader;
///Field `SRC_CH_CLR` writer - clear src channal internal data
pub type SrcChClrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - dac mux left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxlsrc0(&self) -> Muxlsrc0R {
        Muxlsrc0R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - dac mux left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxlsrc1(&self) -> Muxlsrc1R {
        Muxlsrc1R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxrsrc0(&self) -> Muxrsrc0R {
        Muxrsrc0R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxrsrc1(&self) -> Muxrsrc1R {
        Muxrsrc1R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:13 - equalizer channel enable 2'b11: enable both channel 2'b10: enable right chanel only 2'b01: enable left channel only 2'b00: bypass equalizer
    #[inline(always)]
    pub fn eq_ch_en(&self) -> EqChEnR {
        EqChEnR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - set equalizer stage, max is 10.
    #[inline(always)]
    pub fn eq_stage(&self) -> EqStageR {
        EqStageR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - equalizer clear done flag
    #[inline(always)]
    pub fn eq_clr_done(&self) -> EqClrDoneR {
        EqClrDoneR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - equalizer clear request
    #[inline(always)]
    pub fn eq_clr(&self) -> EqClrR {
        EqClrR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - source rate converter channel enable
    #[inline(always)]
    pub fn src_ch_en(&self) -> SrcChEnR {
        SrcChEnR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - 1st stage hbf enable
    #[inline(always)]
    pub fn src_hbf1_en(&self) -> SrcHbf1EnR {
        SrcHbf1EnR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - 1st stage hbf mode: 0: upsampling 1: downsampling
    #[inline(always)]
    pub fn src_hbf1_mode(&self) -> SrcHbf1ModeR {
        SrcHbf1ModeR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - 2nd stage hbf enable
    #[inline(always)]
    pub fn src_hbf2_en(&self) -> SrcHbf2EnR {
        SrcHbf2EnR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - 2nd stage hbf mode: 0: upsampling 1: downsampling
    #[inline(always)]
    pub fn src_hbf2_mode(&self) -> SrcHbf2ModeR {
        SrcHbf2ModeR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - 3rd stage hbf enable
    #[inline(always)]
    pub fn src_hbf3_en(&self) -> SrcHbf3EnR {
        SrcHbf3EnR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 3rd stage hbf mode: 0: upsampling 1: downsampling
    #[inline(always)]
    pub fn src_hbf3_mode(&self) -> SrcHbf3ModeR {
        SrcHbf3ModeR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - src channel internal data clear done
    #[inline(always)]
    pub fn src_ch_clr_done(&self) -> SrcChClrDoneR {
        SrcChClrDoneR::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - clear src channal internal data
    #[inline(always)]
    pub fn src_ch_clr(&self) -> SrcChClrR {
        SrcChClrR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_PATH_CFG1")
            .field("src_ch_clr", &self.src_ch_clr())
            .field("src_ch_clr_done", &self.src_ch_clr_done())
            .field("src_hbf3_mode", &self.src_hbf3_mode())
            .field("src_hbf3_en", &self.src_hbf3_en())
            .field("src_hbf2_mode", &self.src_hbf2_mode())
            .field("src_hbf2_en", &self.src_hbf2_en())
            .field("src_hbf1_mode", &self.src_hbf1_mode())
            .field("src_hbf1_en", &self.src_hbf1_en())
            .field("src_ch_en", &self.src_ch_en())
            .field("eq_clr", &self.eq_clr())
            .field("eq_clr_done", &self.eq_clr_done())
            .field("eq_stage", &self.eq_stage())
            .field("eq_ch_en", &self.eq_ch_en())
            .field("muxrsrc1", &self.muxrsrc1())
            .field("muxrsrc0", &self.muxrsrc0())
            .field("muxlsrc1", &self.muxlsrc1())
            .field("muxlsrc0", &self.muxlsrc0())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - dac mux left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxlsrc0(&mut self) -> Muxlsrc0W<DAC_PATH_CFG1rs> {
        Muxlsrc0W::new(self, 0)
    }
    ///Bits 3:5 - dac mux left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxlsrc1(&mut self) -> Muxlsrc1W<DAC_PATH_CFG1rs> {
        Muxlsrc1W::new(self, 3)
    }
    ///Bits 6:8 - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxrsrc0(&mut self) -> Muxrsrc0W<DAC_PATH_CFG1rs> {
        Muxrsrc0W::new(self, 6)
    }
    ///Bits 9:11 - dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute
    #[inline(always)]
    pub fn muxrsrc1(&mut self) -> Muxrsrc1W<DAC_PATH_CFG1rs> {
        Muxrsrc1W::new(self, 9)
    }
    ///Bits 12:13 - equalizer channel enable 2'b11: enable both channel 2'b10: enable right chanel only 2'b01: enable left channel only 2'b00: bypass equalizer
    #[inline(always)]
    pub fn eq_ch_en(&mut self) -> EqChEnW<DAC_PATH_CFG1rs> {
        EqChEnW::new(self, 12)
    }
    ///Bits 14:17 - set equalizer stage, max is 10.
    #[inline(always)]
    pub fn eq_stage(&mut self) -> EqStageW<DAC_PATH_CFG1rs> {
        EqStageW::new(self, 14)
    }
    ///Bit 18 - equalizer clear done flag
    #[inline(always)]
    pub fn eq_clr_done(&mut self) -> EqClrDoneW<DAC_PATH_CFG1rs> {
        EqClrDoneW::new(self, 18)
    }
    ///Bit 19 - equalizer clear request
    #[inline(always)]
    pub fn eq_clr(&mut self) -> EqClrW<DAC_PATH_CFG1rs> {
        EqClrW::new(self, 19)
    }
    ///Bits 20:21 - source rate converter channel enable
    #[inline(always)]
    pub fn src_ch_en(&mut self) -> SrcChEnW<DAC_PATH_CFG1rs> {
        SrcChEnW::new(self, 20)
    }
    ///Bit 22 - 1st stage hbf enable
    #[inline(always)]
    pub fn src_hbf1_en(&mut self) -> SrcHbf1EnW<DAC_PATH_CFG1rs> {
        SrcHbf1EnW::new(self, 22)
    }
    ///Bit 23 - 1st stage hbf mode: 0: upsampling 1: downsampling
    #[inline(always)]
    pub fn src_hbf1_mode(&mut self) -> SrcHbf1ModeW<DAC_PATH_CFG1rs> {
        SrcHbf1ModeW::new(self, 23)
    }
    ///Bit 24 - 2nd stage hbf enable
    #[inline(always)]
    pub fn src_hbf2_en(&mut self) -> SrcHbf2EnW<DAC_PATH_CFG1rs> {
        SrcHbf2EnW::new(self, 24)
    }
    ///Bit 25 - 2nd stage hbf mode: 0: upsampling 1: downsampling
    #[inline(always)]
    pub fn src_hbf2_mode(&mut self) -> SrcHbf2ModeW<DAC_PATH_CFG1rs> {
        SrcHbf2ModeW::new(self, 25)
    }
    ///Bit 26 - 3rd stage hbf enable
    #[inline(always)]
    pub fn src_hbf3_en(&mut self) -> SrcHbf3EnW<DAC_PATH_CFG1rs> {
        SrcHbf3EnW::new(self, 26)
    }
    ///Bit 27 - 3rd stage hbf mode: 0: upsampling 1: downsampling
    #[inline(always)]
    pub fn src_hbf3_mode(&mut self) -> SrcHbf3ModeW<DAC_PATH_CFG1rs> {
        SrcHbf3ModeW::new(self, 27)
    }
    ///Bits 28:29 - src channel internal data clear done
    #[inline(always)]
    pub fn src_ch_clr_done(&mut self) -> SrcChClrDoneW<DAC_PATH_CFG1rs> {
        SrcChClrDoneW::new(self, 28)
    }
    ///Bits 30:31 - clear src channal internal data
    #[inline(always)]
    pub fn src_ch_clr(&mut self) -> SrcChClrW<DAC_PATH_CFG1rs> {
        SrcChClrW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_PATH_CFG1rs;
impl crate::RegisterSpec for DAC_PATH_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`dac_path_cfg1::R`](R) reader structure
impl crate::Readable for DAC_PATH_CFG1rs {}
///`write(|w| ..)` method takes [`dac_path_cfg1::W`](W) writer structure
impl crate::Writable for DAC_PATH_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_PATH_CFG1 to value 0
impl crate::Resettable for DAC_PATH_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
