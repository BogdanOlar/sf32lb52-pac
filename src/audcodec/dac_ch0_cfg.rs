///Register `DAC_CH0_CFG` reader
pub type R = crate::R<DAC_CH0_CFGrs>;
///Register `DAC_CH0_CFG` writer
pub type W = crate::W<DAC_CH0_CFGrs>;
///Field `ENABLE` reader - dac channel enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - dac channel enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOUT_MUTE` reader - dac output mute, set 1 to mute the output
pub type DoutMuteR = crate::BitReader;
///Field `DOUT_MUTE` writer - dac output mute, set 1 to mute the output
pub type DoutMuteW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEM_MODE` reader - dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved
pub type DemModeR = crate::FieldReader;
///Field `DEM_MODE` writer - dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved
pub type DemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STB_FIFO_CNT` reader - dac input stb fifo cnt
pub type StbFifoCntR = crate::FieldReader;
///Field `STB_FIFO_CNT` writer - dac input stb fifo cnt
pub type StbFifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMA_EN` reader - dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface
pub type DmaEnR = crate::BitReader;
///Field `DMA_EN` writer - dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROUGH_VOL` reader - dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolR = crate::FieldReader;
///Field `ROUGH_VOL` writer - dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FINE_VOL` reader - dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolR = crate::FieldReader;
///Field `FINE_VOL` writer - dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATA_FORMAT` reader - dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
pub type DataFormatR = crate::BitReader;
///Field `DATA_FORMAT` writer - dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
pub type DataFormatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINC_GAIN` reader - dac sinc filter gain
pub type SincGainR = crate::FieldReader<u16>;
///Field `SINC_GAIN` writer - dac sinc filter gain
pub type SincGainW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DITHER_GAIN` reader - sdm dither gain
pub type DitherGainR = crate::FieldReader;
///Field `DITHER_GAIN` writer - sdm dither gain
pub type DitherGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DITHER_EN` reader - sdm dither enable
pub type DitherEnR = crate::BitReader;
///Field `DITHER_EN` writer - sdm dither enable
pub type DitherEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_ANA_POL` reader - analog dac clock polarity
pub type ClkAnaPolR = crate::BitReader;
///Field `CLK_ANA_POL` writer - analog dac clock polarity
pub type ClkAnaPolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - dac channel enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - dac output mute, set 1 to mute the output
    #[inline(always)]
    pub fn dout_mute(&self) -> DoutMuteR {
        DoutMuteR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved
    #[inline(always)]
    pub fn dem_mode(&self) -> DemModeR {
        DemModeR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - dac input stb fifo cnt
    #[inline(always)]
    pub fn stb_fifo_cnt(&self) -> StbFifoCntR {
        StbFifoCntR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol(&self) -> RoughVolR {
        RoughVolR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol(&self) -> FineVolR {
        FineVolR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
    ///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
    #[inline(always)]
    pub fn data_format(&self) -> DataFormatR {
        DataFormatR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:25 - dac sinc filter gain
    #[inline(always)]
    pub fn sinc_gain(&self) -> SincGainR {
        SincGainR::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    ///Bits 26:28 - sdm dither gain
    #[inline(always)]
    pub fn dither_gain(&self) -> DitherGainR {
        DitherGainR::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bit 29 - sdm dither enable
    #[inline(always)]
    pub fn dither_en(&self) -> DitherEnR {
        DitherEnR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - analog dac clock polarity
    #[inline(always)]
    pub fn clk_ana_pol(&self) -> ClkAnaPolR {
        ClkAnaPolR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH0_CFG")
            .field("clk_ana_pol", &self.clk_ana_pol())
            .field("dither_en", &self.dither_en())
            .field("dither_gain", &self.dither_gain())
            .field("sinc_gain", &self.sinc_gain())
            .field("data_format", &self.data_format())
            .field("fine_vol", &self.fine_vol())
            .field("rough_vol", &self.rough_vol())
            .field("dma_en", &self.dma_en())
            .field("stb_fifo_cnt", &self.stb_fifo_cnt())
            .field("dem_mode", &self.dem_mode())
            .field("dout_mute", &self.dout_mute())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - dac channel enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<DAC_CH0_CFGrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - dac output mute, set 1 to mute the output
    #[inline(always)]
    pub fn dout_mute(&mut self) -> DoutMuteW<DAC_CH0_CFGrs> {
        DoutMuteW::new(self, 1)
    }
    ///Bits 2:3 - dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved
    #[inline(always)]
    pub fn dem_mode(&mut self) -> DemModeW<DAC_CH0_CFGrs> {
        DemModeW::new(self, 2)
    }
    ///Bits 4:6 - dac input stb fifo cnt
    #[inline(always)]
    pub fn stb_fifo_cnt(&mut self) -> StbFifoCntW<DAC_CH0_CFGrs> {
        StbFifoCntW::new(self, 4)
    }
    ///Bit 7 - dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<DAC_CH0_CFGrs> {
        DmaEnW::new(self, 7)
    }
    ///Bits 8:11 - dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol(&mut self) -> RoughVolW<DAC_CH0_CFGrs> {
        RoughVolW::new(self, 8)
    }
    ///Bits 12:15 - dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol(&mut self) -> FineVolW<DAC_CH0_CFGrs> {
        FineVolW::new(self, 12)
    }
    ///Bit 16 - dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
    ///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
    #[inline(always)]
    pub fn data_format(&mut self) -> DataFormatW<DAC_CH0_CFGrs> {
        DataFormatW::new(self, 16)
    }
    ///Bits 17:25 - dac sinc filter gain
    #[inline(always)]
    pub fn sinc_gain(&mut self) -> SincGainW<DAC_CH0_CFGrs> {
        SincGainW::new(self, 17)
    }
    ///Bits 26:28 - sdm dither gain
    #[inline(always)]
    pub fn dither_gain(&mut self) -> DitherGainW<DAC_CH0_CFGrs> {
        DitherGainW::new(self, 26)
    }
    ///Bit 29 - sdm dither enable
    #[inline(always)]
    pub fn dither_en(&mut self) -> DitherEnW<DAC_CH0_CFGrs> {
        DitherEnW::new(self, 29)
    }
    ///Bit 30 - analog dac clock polarity
    #[inline(always)]
    pub fn clk_ana_pol(&mut self) -> ClkAnaPolW<DAC_CH0_CFGrs> {
        ClkAnaPolW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH0_CFGrs;
impl crate::RegisterSpec for DAC_CH0_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch0_cfg::R`](R) reader structure
impl crate::Readable for DAC_CH0_CFGrs {}
///`write(|w| ..)` method takes [`dac_ch0_cfg::W`](W) writer structure
impl crate::Writable for DAC_CH0_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH0_CFG to value 0
impl crate::Resettable for DAC_CH0_CFGrs {
    const RESET_VALUE: u32 = 0;
}
