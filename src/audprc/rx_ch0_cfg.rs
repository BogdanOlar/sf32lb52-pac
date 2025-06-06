///Register `RX_CH0_CFG` reader
pub type R = crate::R<RX_CH0_CFGrs>;
///Register `RX_CH0_CFG` writer
pub type W = crate::W<RX_CH0_CFGrs>;
///Field `ENABLE` reader - rx channel 0 enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - rx channel 0 enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORMAT` reader - rx format 0: 16-bit mode 1: 24-bit mode
pub type FormatR = crate::BitReader;
///Field `FORMAT` writer - rx format 0: 16-bit mode 1: 24-bit mode
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - rx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0.
pub type ModeR = crate::BitReader;
///Field `MODE` writer - rx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0.
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_MSK` reader - 1: mask the dma request for rx ch0
pub type DmaMskR = crate::BitReader;
///Field `DMA_MSK` writer - 1: mask the dma request for rx ch0
pub type DmaMskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_CNT` reader - rx fifo counter
pub type FifoCntR = crate::FieldReader;
///Field `FIFO_CNT` writer - rx fifo counter
pub type FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bit 0 - rx channel 0 enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - rx format 0: 16-bit mode 1: 24-bit mode
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - rx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0.
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1: mask the dma request for rx ch0
    #[inline(always)]
    pub fn dma_msk(&self) -> DmaMskR {
        DmaMskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - rx fifo counter
    #[inline(always)]
    pub fn fifo_cnt(&self) -> FifoCntR {
        FifoCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH0_CFG")
            .field("rsvd", &self.rsvd())
            .field("fifo_cnt", &self.fifo_cnt())
            .field("dma_msk", &self.dma_msk())
            .field("mode", &self.mode())
            .field("format", &self.format())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - rx channel 0 enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<RX_CH0_CFGrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - rx format 0: 16-bit mode 1: 24-bit mode
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<RX_CH0_CFGrs> {
        FormatW::new(self, 1)
    }
    ///Bit 2 - rx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<RX_CH0_CFGrs> {
        ModeW::new(self, 2)
    }
    ///Bit 3 - 1: mask the dma request for rx ch0
    #[inline(always)]
    pub fn dma_msk(&mut self) -> DmaMskW<RX_CH0_CFGrs> {
        DmaMskW::new(self, 3)
    }
    ///Bits 4:7 - rx fifo counter
    #[inline(always)]
    pub fn fifo_cnt(&mut self) -> FifoCntW<RX_CH0_CFGrs> {
        FifoCntW::new(self, 4)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RX_CH0_CFGrs> {
        RsvdW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_CH0_CFGrs;
impl crate::RegisterSpec for RX_CH0_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ch0_cfg::R`](R) reader structure
impl crate::Readable for RX_CH0_CFGrs {}
///`write(|w| ..)` method takes [`rx_ch0_cfg::W`](W) writer structure
impl crate::Writable for RX_CH0_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_CH0_CFG to value 0
impl crate::Resettable for RX_CH0_CFGrs {
    const RESET_VALUE: u32 = 0;
}
