///Register `DMA_DATA` reader
pub type R = crate::R<DMA_DATArs>;
///Register `DMA_DATA` writer
pub type W = crate::W<DMA_DATArs>;
///Field `SIZE` reader - AES_ACC data block size, AES_ACC only support block aligned transaction. Each block contains 16 bytes.
pub type SizeR = crate::FieldReader<u32>;
///Field `SIZE` writer - AES_ACC data block size, AES_ACC only support block aligned transaction. Each block contains 16 bytes.
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:27 - AES_ACC data block size, AES_ACC only support block aligned transaction. Each block contains 16 bytes.
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits & 0x0fff_ffff)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_DATA")
            .field("rsvd", &self.rsvd())
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - AES_ACC data block size, AES_ACC only support block aligned transaction. Each block contains 16 bytes.
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<DMA_DATArs> {
        SizeW::new(self, 0)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DMA_DATArs> {
        RsvdW::new(self, 28)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dma_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DMA_DATArs;
impl crate::RegisterSpec for DMA_DATArs {
    type Ux = u32;
}
///`read()` method returns [`dma_data::R`](R) reader structure
impl crate::Readable for DMA_DATArs {}
///`write(|w| ..)` method takes [`dma_data::W`](W) writer structure
impl crate::Writable for DMA_DATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_DATA to value 0
impl crate::Resettable for DMA_DATArs {
    const RESET_VALUE: u32 = 0;
}
