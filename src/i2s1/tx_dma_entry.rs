///Register `TX_DMA_ENTRY` reader
pub type R = crate::R<TX_DMA_ENTRYrs>;
///Register `TX_DMA_ENTRY` writer
pub type W = crate::W<TX_DMA_ENTRYrs>;
///Field `TX_DMA_ENTRY` reader - TX DMA entry
pub type TxDmaEntryR = crate::FieldReader<u32>;
///Field `TX_DMA_ENTRY` writer - TX DMA entry
pub type TxDmaEntryW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TX DMA entry
    #[inline(always)]
    pub fn tx_dma_entry(&self) -> TxDmaEntryR {
        TxDmaEntryR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DMA_ENTRY")
            .field("tx_dma_entry", &self.tx_dma_entry())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TX DMA entry
    #[inline(always)]
    pub fn tx_dma_entry(&mut self) -> TxDmaEntryW<TX_DMA_ENTRYrs> {
        TxDmaEntryW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_dma_entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_dma_entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_DMA_ENTRYrs;
impl crate::RegisterSpec for TX_DMA_ENTRYrs {
    type Ux = u32;
}
///`read()` method returns [`tx_dma_entry::R`](R) reader structure
impl crate::Readable for TX_DMA_ENTRYrs {}
///`write(|w| ..)` method takes [`tx_dma_entry::W`](W) writer structure
impl crate::Writable for TX_DMA_ENTRYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_DMA_ENTRY to value 0
impl crate::Resettable for TX_DMA_ENTRYrs {
    const RESET_VALUE: u32 = 0;
}
