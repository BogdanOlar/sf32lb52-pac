///Register `RX_CH1_CFG` reader
pub type R = crate::R<RX_CH1_CFGrs>;
///Register `RX_CH1_CFG` writer
pub type W = crate::W<RX_CH1_CFGrs>;
///Field `ENABLE` reader - rx channel 1 enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - rx channel 1 enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORMAT` reader - rx format 0: 16-bit mode 1: 24-bit mode
pub type FormatR = crate::BitReader;
///Field `FORMAT` writer - rx format 0: 16-bit mode 1: 24-bit mode
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_MSK` reader - 1: mask the dma request for rx ch1
pub type DmaMskR = crate::BitReader;
///Field `DMA_MSK` writer - 1: mask the dma request for rx ch1
pub type DmaMskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_CNT` reader - rx fifo counter
pub type FifoCntR = crate::FieldReader;
///Field `FIFO_CNT` writer - rx fifo counter
pub type FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - rx channel 1 enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - rx format 0: 16-bit mode 1: 24-bit mode
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - 1: mask the dma request for rx ch1
    #[inline(always)]
    pub fn dma_msk(&self) -> DmaMskR {
        DmaMskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - rx fifo counter
    #[inline(always)]
    pub fn fifo_cnt(&self) -> FifoCntR {
        FifoCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH1_CFG")
            .field("fifo_cnt", &self.fifo_cnt())
            .field("dma_msk", &self.dma_msk())
            .field("format", &self.format())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - rx channel 1 enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<RX_CH1_CFGrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - rx format 0: 16-bit mode 1: 24-bit mode
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<RX_CH1_CFGrs> {
        FormatW::new(self, 1)
    }
    ///Bit 3 - 1: mask the dma request for rx ch1
    #[inline(always)]
    pub fn dma_msk(&mut self) -> DmaMskW<RX_CH1_CFGrs> {
        DmaMskW::new(self, 3)
    }
    ///Bits 4:7 - rx fifo counter
    #[inline(always)]
    pub fn fifo_cnt(&mut self) -> FifoCntW<RX_CH1_CFGrs> {
        FifoCntW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_CH1_CFGrs;
impl crate::RegisterSpec for RX_CH1_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ch1_cfg::R`](R) reader structure
impl crate::Readable for RX_CH1_CFGrs {}
///`write(|w| ..)` method takes [`rx_ch1_cfg::W`](W) writer structure
impl crate::Writable for RX_CH1_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_CH1_CFG to value 0
impl crate::Resettable for RX_CH1_CFGrs {
    const RESET_VALUE: u32 = 0;
}
