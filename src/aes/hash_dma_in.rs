///Register `HASH_DMA_IN` reader
pub type R = crate::R<HASH_DMA_INrs>;
///Register `HASH_DMA_IN` writer
pub type W = crate::W<HASH_DMA_INrs>;
///Field `ADDR` reader - input data address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - input data address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - input data address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_DMA_IN")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - input data address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<HASH_DMA_INrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hash_dma_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_dma_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_DMA_INrs;
impl crate::RegisterSpec for HASH_DMA_INrs {
    type Ux = u32;
}
///`read()` method returns [`hash_dma_in::R`](R) reader structure
impl crate::Readable for HASH_DMA_INrs {}
///`write(|w| ..)` method takes [`hash_dma_in::W`](W) writer structure
impl crate::Writable for HASH_DMA_INrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_DMA_IN to value 0
impl crate::Resettable for HASH_DMA_INrs {
    const RESET_VALUE: u32 = 0;
}
