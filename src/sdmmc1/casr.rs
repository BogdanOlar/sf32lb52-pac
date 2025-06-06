///Register `CASR` reader
pub type R = crate::R<CASRrs>;
///Register `CASR` writer
pub type W = crate::W<CASRrs>;
///Field `SD_REQ` reader - Set 1 to request sd normal access. sd_req will be cleared automatically after sd_busy asserted
pub type SdReqR = crate::BitReader;
///Field `SD_REQ` writer - Set 1 to request sd normal access. sd_req will be cleared automatically after sd_busy asserted
pub type SdReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD_BUSY` reader - Read 1 indicates sd is ready for normal access. Ahb access will be hold during sd_busy asserted. After sd normal access done, write 1 to clear, and ahb access will continue
pub type SdBusyR = crate::BitReader;
///Field `SD_BUSY` writer - Read 1 indicates sd is ready for normal access. Ahb access will be hold during sd_busy asserted. After sd normal access done, write 1 to clear, and ahb access will continue
pub type SdBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_BUSY` reader - Indicates cache is working
pub type CacheBusyR = crate::BitReader;
///Field `CACHE_BUSY` writer - Indicates cache is working
pub type CacheBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_FLUSH` reader - Set 1 to flush cache. Should set when cache not busy.
pub type CacheFlushR = crate::BitReader;
///Field `CACHE_FLUSH` writer - Set 1 to flush cache. Should set when cache not busy.
pub type CacheFlushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to request sd normal access. sd_req will be cleared automatically after sd_busy asserted
    #[inline(always)]
    pub fn sd_req(&self) -> SdReqR {
        SdReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read 1 indicates sd is ready for normal access. Ahb access will be hold during sd_busy asserted. After sd normal access done, write 1 to clear, and ahb access will continue
    #[inline(always)]
    pub fn sd_busy(&self) -> SdBusyR {
        SdBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Indicates cache is working
    #[inline(always)]
    pub fn cache_busy(&self) -> CacheBusyR {
        CacheBusyR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to flush cache. Should set when cache not busy.
    #[inline(always)]
    pub fn cache_flush(&self) -> CacheFlushR {
        CacheFlushR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CASR")
            .field("cache_flush", &self.cache_flush())
            .field("cache_busy", &self.cache_busy())
            .field("sd_busy", &self.sd_busy())
            .field("sd_req", &self.sd_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to request sd normal access. sd_req will be cleared automatically after sd_busy asserted
    #[inline(always)]
    pub fn sd_req(&mut self) -> SdReqW<CASRrs> {
        SdReqW::new(self, 0)
    }
    ///Bit 1 - Read 1 indicates sd is ready for normal access. Ahb access will be hold during sd_busy asserted. After sd normal access done, write 1 to clear, and ahb access will continue
    #[inline(always)]
    pub fn sd_busy(&mut self) -> SdBusyW<CASRrs> {
        SdBusyW::new(self, 1)
    }
    ///Bit 2 - Indicates cache is working
    #[inline(always)]
    pub fn cache_busy(&mut self) -> CacheBusyW<CASRrs> {
        CacheBusyW::new(self, 2)
    }
    ///Bit 3 - Set 1 to flush cache. Should set when cache not busy.
    #[inline(always)]
    pub fn cache_flush(&mut self) -> CacheFlushW<CASRrs> {
        CacheFlushW::new(self, 3)
    }
}
///cache status register
///
///You can [`read`](crate::Reg::read) this register and get [`casr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`casr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CASRrs;
impl crate::RegisterSpec for CASRrs {
    type Ux = u32;
}
///`read()` method returns [`casr::R`](R) reader structure
impl crate::Readable for CASRrs {}
///`write(|w| ..)` method takes [`casr::W`](W) writer structure
impl crate::Writable for CASRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CASR to value 0
impl crate::Resettable for CASRrs {
    const RESET_VALUE: u32 = 0;
}
