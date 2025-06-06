///Register `DMA_MASK` reader
pub type R = crate::R<DMA_MASKrs>;
///Register `DMA_MASK` writer
pub type W = crate::W<DMA_MASKrs>;
///Field `RX_DMA_MASK` reader - RX DMA mask enable:1: mask0: do not mask
pub type RxDmaMaskR = crate::BitReader;
///Field `RX_DMA_MASK` writer - RX DMA mask enable:1: mask0: do not mask
pub type RxDmaMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_DMA_MASK` reader - TX DMA mask enable:1: mask0: do not mask
pub type TxDmaMaskR = crate::BitReader;
///Field `TX_DMA_MASK` writer - TX DMA mask enable:1: mask0: do not mask
pub type TxDmaMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - RX DMA mask enable:1: mask0: do not mask
    #[inline(always)]
    pub fn rx_dma_mask(&self) -> RxDmaMaskR {
        RxDmaMaskR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX DMA mask enable:1: mask0: do not mask
    #[inline(always)]
    pub fn tx_dma_mask(&self) -> TxDmaMaskR {
        TxDmaMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MASK")
            .field("rsvd", &self.rsvd())
            .field("tx_dma_mask", &self.tx_dma_mask())
            .field("rx_dma_mask", &self.rx_dma_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - RX DMA mask enable:1: mask0: do not mask
    #[inline(always)]
    pub fn rx_dma_mask(&mut self) -> RxDmaMaskW<DMA_MASKrs> {
        RxDmaMaskW::new(self, 0)
    }
    ///Bit 1 - TX DMA mask enable:1: mask0: do not mask
    #[inline(always)]
    pub fn tx_dma_mask(&mut self) -> TxDmaMaskW<DMA_MASKrs> {
        TxDmaMaskW::new(self, 1)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DMA_MASKrs> {
        RsvdW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dma_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DMA_MASKrs;
impl crate::RegisterSpec for DMA_MASKrs {
    type Ux = u32;
}
///`read()` method returns [`dma_mask::R`](R) reader structure
impl crate::Readable for DMA_MASKrs {}
///`write(|w| ..)` method takes [`dma_mask::W`](W) writer structure
impl crate::Writable for DMA_MASKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_MASK to value 0x03
impl crate::Resettable for DMA_MASKrs {
    const RESET_VALUE: u32 = 0x03;
}
