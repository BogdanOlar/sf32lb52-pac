///Register `DMA_OUT` reader
pub type R = crate::R<DMA_OUTrs>;
///Register `DMA_OUT` writer
pub type W = crate::W<DMA_OUTrs>;
///Field `ADDR` reader - AES_ACC output data address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - AES_ACC output data address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES_ACC output data address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES_ACC output data address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<DMA_OUTrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dma_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DMA_OUTrs;
impl crate::RegisterSpec for DMA_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`dma_out::R`](R) reader structure
impl crate::Readable for DMA_OUTrs {}
///`write(|w| ..)` method takes [`dma_out::W`](W) writer structure
impl crate::Writable for DMA_OUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_OUT to value 0
impl crate::Resettable for DMA_OUTrs {
    const RESET_VALUE: u32 = 0;
}
