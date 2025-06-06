///Register `CBSR7` reader
pub type R = crate::R<CBSR7rs>;
///Register `CBSR7` writer
pub type W = crate::W<CBSR7rs>;
///Field `BS` reader - burst size in non memory-to-memory mode When BS>1, DMA will transfer for BS times for each request if left NDT is larger than BS, or else transfer for left NDT times. When BS=0 or 1, DMA will always do single transfer for each request. In memory-to-memory mode, BS is ignored.
pub type BsR = crate::FieldReader;
///Field `BS` writer - burst size in non memory-to-memory mode When BS>1, DMA will transfer for BS times for each request if left NDT is larger than BS, or else transfer for left NDT times. When BS=0 or 1, DMA will always do single transfer for each request. In memory-to-memory mode, BS is ignored.
pub type BsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - burst size in non memory-to-memory mode When BS>1, DMA will transfer for BS times for each request if left NDT is larger than BS, or else transfer for left NDT times. When BS=0 or 1, DMA will always do single transfer for each request. In memory-to-memory mode, BS is ignored.
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CBSR7").field("bs", &self.bs()).finish()
    }
}
impl W {
    ///Bits 0:7 - burst size in non memory-to-memory mode When BS>1, DMA will transfer for BS times for each request if left NDT is larger than BS, or else transfer for left NDT times. When BS=0 or 1, DMA will always do single transfer for each request. In memory-to-memory mode, BS is ignored.
    #[inline(always)]
    pub fn bs(&mut self) -> BsW<CBSR7rs> {
        BsW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cbsr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CBSR7rs;
impl crate::RegisterSpec for CBSR7rs {
    type Ux = u32;
}
///`read()` method returns [`cbsr7::R`](R) reader structure
impl crate::Readable for CBSR7rs {}
///`write(|w| ..)` method takes [`cbsr7::W`](W) writer structure
impl crate::Writable for CBSR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CBSR7 to value 0
impl crate::Resettable for CBSR7rs {
    const RESET_VALUE: u32 = 0;
}
