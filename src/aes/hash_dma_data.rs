///Register `HASH_DMA_DATA` reader
pub type R = crate::R<HASH_DMA_DATArs>;
///Register `HASH_DMA_DATA` writer
pub type W = crate::W<HASH_DMA_DATArs>;
///Field `SIZE` reader - HASH input data byte size.
pub type SizeR = crate::FieldReader<u32>;
///Field `SIZE` writer - HASH input data byte size.
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HASH input data byte size.
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_DMA_DATA")
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HASH input data byte size.
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<HASH_DMA_DATArs> {
        SizeW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hash_dma_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_dma_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_DMA_DATArs;
impl crate::RegisterSpec for HASH_DMA_DATArs {
    type Ux = u32;
}
///`read()` method returns [`hash_dma_data::R`](R) reader structure
impl crate::Readable for HASH_DMA_DATArs {}
///`write(|w| ..)` method takes [`hash_dma_data::W`](W) writer structure
impl crate::Writable for HASH_DMA_DATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_DMA_DATA to value 0
impl crate::Resettable for HASH_DMA_DATArs {
    const RESET_VALUE: u32 = 0;
}
