///Register `TX_CH2_CFG` reader
pub type R = crate::R<TX_CH2_CFGrs>;
///Register `TX_CH2_CFG` writer
pub type W = crate::W<TX_CH2_CFGrs>;
///Field `ENABLE` reader - tx channel 0 enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - tx channel 0 enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORMAT` reader - tx format 0: 16-bit mode 1: 24-bit mode
pub type FormatR = crate::BitReader;
///Field `FORMAT` writer - tx format 0: 16-bit mode 1: 24-bit mode
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 3 is not working, both left and right audio data comes from channel 2.
pub type ModeR = crate::BitReader;
///Field `MODE` writer - tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 3 is not working, both left and right audio data comes from channel 2.
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_MSK` reader - 1: mask the dma request for tx ch2
pub type DmaMskR = crate::BitReader;
///Field `DMA_MSK` writer - 1: mask the dma request for tx ch2
pub type DmaMskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_CNT` reader - tx fifo counter
pub type FifoCntR = crate::FieldReader;
///Field `FIFO_CNT` writer - tx fifo counter
pub type FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - tx channel 0 enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - tx format 0: 16-bit mode 1: 24-bit mode
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 3 is not working, both left and right audio data comes from channel 2.
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1: mask the dma request for tx ch2
    #[inline(always)]
    pub fn dma_msk(&self) -> DmaMskR {
        DmaMskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - tx fifo counter
    #[inline(always)]
    pub fn fifo_cnt(&self) -> FifoCntR {
        FifoCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH2_CFG")
            .field("fifo_cnt", &self.fifo_cnt())
            .field("dma_msk", &self.dma_msk())
            .field("mode", &self.mode())
            .field("format", &self.format())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - tx channel 0 enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<TX_CH2_CFGrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - tx format 0: 16-bit mode 1: 24-bit mode
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<TX_CH2_CFGrs> {
        FormatW::new(self, 1)
    }
    ///Bit 2 - tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 3 is not working, both left and right audio data comes from channel 2.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<TX_CH2_CFGrs> {
        ModeW::new(self, 2)
    }
    ///Bit 3 - 1: mask the dma request for tx ch2
    #[inline(always)]
    pub fn dma_msk(&mut self) -> DmaMskW<TX_CH2_CFGrs> {
        DmaMskW::new(self, 3)
    }
    ///Bits 4:7 - tx fifo counter
    #[inline(always)]
    pub fn fifo_cnt(&mut self) -> FifoCntW<TX_CH2_CFGrs> {
        FifoCntW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch2_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch2_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_CH2_CFGrs;
impl crate::RegisterSpec for TX_CH2_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`tx_ch2_cfg::R`](R) reader structure
impl crate::Readable for TX_CH2_CFGrs {}
///`write(|w| ..)` method takes [`tx_ch2_cfg::W`](W) writer structure
impl crate::Writable for TX_CH2_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CH2_CFG to value 0
impl crate::Resettable for TX_CH2_CFGrs {
    const RESET_VALUE: u32 = 0;
}
