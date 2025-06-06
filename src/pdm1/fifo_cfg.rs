///Register `FIFO_CFG` reader
pub type R = crate::R<FIFO_CFGrs>;
///Register `FIFO_CFG` writer
pub type W = crate::W<FIFO_CFGrs>;
///Field `BYTE_CON` reader - 1: combine left channel and right channel; 0: not combine left channel and right channel
pub type ByteConR = crate::BitReader;
///Field `BYTE_CON` writer - 1: combine left channel and right channel; 0: not combine left channel and right channel
pub type ByteConW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTE_TRUNC` reader - 1: 16bits output ; 0: 24bits output ;2: 8bits output ; 3: 32bits output
pub type ByteTruncR = crate::FieldReader;
///Field `BYTE_TRUNC` writer - 1: 16bits output ; 0: 24bits output ;2: 8bits output ; 3: 32bits output
pub type ByteTruncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PDM_SHIFT` reader - the number of data left shift for higher data accuracy
pub type PdmShiftR = crate::FieldReader;
///Field `PDM_SHIFT` writer - the number of data left shift for higher data accuracy
pub type PdmShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RX_DMA_MSK_R` reader - 1:disable right channel dma request; 0: enable right channel dma request
pub type RxDmaMskRR = crate::BitReader;
///Field `RX_DMA_MSK_R` writer - 1:disable right channel dma request; 0: enable right channel dma request
pub type RxDmaMskRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_DMA_MSK_L` reader - 1:disable left channel dma request; 0: enable left channel dma request
pub type RxDmaMskLR = crate::BitReader;
///Field `RX_DMA_MSK_L` writer - 1:disable left channel dma request; 0: enable left channel dma request
pub type RxDmaMskLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LR_CHG` reader - 1:exchange storage location of left and right channel; 0: don't exchange storage location of left and right channel
pub type LrChgR = crate::BitReader;
///Field `LR_CHG` writer - 1:exchange storage location of left and right channel; 0: don't exchange storage location of left and right channel
pub type LrChgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1: combine left channel and right channel; 0: not combine left channel and right channel
    #[inline(always)]
    pub fn byte_con(&self) -> ByteConR {
        ByteConR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - 1: 16bits output ; 0: 24bits output ;2: 8bits output ; 3: 32bits output
    #[inline(always)]
    pub fn byte_trunc(&self) -> ByteTruncR {
        ByteTruncR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:5 - the number of data left shift for higher data accuracy
    #[inline(always)]
    pub fn pdm_shift(&self) -> PdmShiftR {
        PdmShiftR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - 1:disable right channel dma request; 0: enable right channel dma request
    #[inline(always)]
    pub fn rx_dma_msk_r(&self) -> RxDmaMskRR {
        RxDmaMskRR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1:disable left channel dma request; 0: enable left channel dma request
    #[inline(always)]
    pub fn rx_dma_msk_l(&self) -> RxDmaMskLR {
        RxDmaMskLR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1:exchange storage location of left and right channel; 0: don't exchange storage location of left and right channel
    #[inline(always)]
    pub fn lr_chg(&self) -> LrChgR {
        LrChgR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CFG")
            .field("lr_chg", &self.lr_chg())
            .field("rx_dma_msk_l", &self.rx_dma_msk_l())
            .field("rx_dma_msk_r", &self.rx_dma_msk_r())
            .field("pdm_shift", &self.pdm_shift())
            .field("byte_trunc", &self.byte_trunc())
            .field("byte_con", &self.byte_con())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1: combine left channel and right channel; 0: not combine left channel and right channel
    #[inline(always)]
    pub fn byte_con(&mut self) -> ByteConW<FIFO_CFGrs> {
        ByteConW::new(self, 0)
    }
    ///Bits 1:2 - 1: 16bits output ; 0: 24bits output ;2: 8bits output ; 3: 32bits output
    #[inline(always)]
    pub fn byte_trunc(&mut self) -> ByteTruncW<FIFO_CFGrs> {
        ByteTruncW::new(self, 1)
    }
    ///Bits 3:5 - the number of data left shift for higher data accuracy
    #[inline(always)]
    pub fn pdm_shift(&mut self) -> PdmShiftW<FIFO_CFGrs> {
        PdmShiftW::new(self, 3)
    }
    ///Bit 6 - 1:disable right channel dma request; 0: enable right channel dma request
    #[inline(always)]
    pub fn rx_dma_msk_r(&mut self) -> RxDmaMskRW<FIFO_CFGrs> {
        RxDmaMskRW::new(self, 6)
    }
    ///Bit 7 - 1:disable left channel dma request; 0: enable left channel dma request
    #[inline(always)]
    pub fn rx_dma_msk_l(&mut self) -> RxDmaMskLW<FIFO_CFGrs> {
        RxDmaMskLW::new(self, 7)
    }
    ///Bit 8 - 1:exchange storage location of left and right channel; 0: don't exchange storage location of left and right channel
    #[inline(always)]
    pub fn lr_chg(&mut self) -> LrChgW<FIFO_CFGrs> {
        LrChgW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FIFO_CFGrs;
impl crate::RegisterSpec for FIFO_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`fifo_cfg::R`](R) reader structure
impl crate::Readable for FIFO_CFGrs {}
///`write(|w| ..)` method takes [`fifo_cfg::W`](W) writer structure
impl crate::Writable for FIFO_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_CFG to value 0
impl crate::Resettable for FIFO_CFGrs {
    const RESET_VALUE: u32 = 0;
}
