///Register `RX_DMA_ENTRY` reader
pub type R = crate::R<RX_DMA_ENTRYrs>;
///Register `RX_DMA_ENTRY` writer
pub type W = crate::W<RX_DMA_ENTRYrs>;
///Field `RX_DMA_ENTRY` reader - RX DMA entry
pub type RxDmaEntryR = crate::FieldReader<u32>;
///Field `RX_DMA_ENTRY` writer - RX DMA entry
pub type RxDmaEntryW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - RX DMA entry
    #[inline(always)]
    pub fn rx_dma_entry(&self) -> RxDmaEntryR {
        RxDmaEntryR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_DMA_ENTRY")
            .field("rx_dma_entry", &self.rx_dma_entry())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - RX DMA entry
    #[inline(always)]
    pub fn rx_dma_entry(&mut self) -> RxDmaEntryW<RX_DMA_ENTRYrs> {
        RxDmaEntryW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_dma_entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_dma_entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_DMA_ENTRYrs;
impl crate::RegisterSpec for RX_DMA_ENTRYrs {
    type Ux = u32;
}
///`read()` method returns [`rx_dma_entry::R`](R) reader structure
impl crate::Readable for RX_DMA_ENTRYrs {}
///`write(|w| ..)` method takes [`rx_dma_entry::W`](W) writer structure
impl crate::Writable for RX_DMA_ENTRYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_DMA_ENTRY to value 0
impl crate::Resettable for RX_DMA_ENTRYrs {
    const RESET_VALUE: u32 = 0;
}
