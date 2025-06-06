///Register `CACHE_CLR` reader
pub type R = crate::R<CACHE_CLRrs>;
///Register `CACHE_CLR` writer
pub type W = crate::W<CACHE_CLRrs>;
///Field `CACHE_CLR` reader - no used
pub type CacheClrR = crate::BitReader;
///Field `CACHE_CLR` writer - no used
pub type CacheClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - no used
    #[inline(always)]
    pub fn cache_clr(&self) -> CacheClrR {
        CacheClrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CLR")
            .field("cache_clr", &self.cache_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - no used
    #[inline(always)]
    pub fn cache_clr(&mut self) -> CacheClrW<CACHE_CLRrs> {
        CacheClrW::new(self, 0)
    }
}
///ezip index cache clear
///
///You can [`read`](crate::Reg::read) this register and get [`cache_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CACHE_CLRrs;
impl crate::RegisterSpec for CACHE_CLRrs {
    type Ux = u32;
}
///`read()` method returns [`cache_clr::R`](R) reader structure
impl crate::Readable for CACHE_CLRrs {}
///`write(|w| ..)` method takes [`cache_clr::W`](W) writer structure
impl crate::Writable for CACHE_CLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_CLR to value 0
impl crate::Resettable for CACHE_CLRrs {
    const RESET_VALUE: u32 = 0;
}
