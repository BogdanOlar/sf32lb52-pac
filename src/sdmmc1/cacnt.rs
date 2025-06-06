///Register `CACNT` reader
pub type R = crate::R<CACNTrs>;
///Register `CACNT` writer
pub type W = crate::W<CACNTrs>;
///Field `CACHE_NCC` reader - cmd-cmd interval counter in hclk cycles
pub type CacheNccR = crate::FieldReader;
///Field `CACHE_NCC` writer - cmd-cmd interval counter in hclk cycles
pub type CacheNccW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CACHE_NDC` reader - data-cmd interval counter in hclk cycles
pub type CacheNdcR = crate::FieldReader;
///Field `CACHE_NDC` writer - data-cmd interval counter in hclk cycles
pub type CacheNdcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CACHE_TOR` reader - timeout count register for ahb read
pub type CacheTorR = crate::FieldReader<u16>;
///Field `CACHE_TOR` writer - timeout count register for ahb read
pub type CacheTorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:7 - cmd-cmd interval counter in hclk cycles
    #[inline(always)]
    pub fn cache_ncc(&self) -> CacheNccR {
        CacheNccR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - data-cmd interval counter in hclk cycles
    #[inline(always)]
    pub fn cache_ndc(&self) -> CacheNdcR {
        CacheNdcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - timeout count register for ahb read
    #[inline(always)]
    pub fn cache_tor(&self) -> CacheTorR {
        CacheTorR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACNT")
            .field("cache_tor", &self.cache_tor())
            .field("cache_ndc", &self.cache_ndc())
            .field("cache_ncc", &self.cache_ncc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - cmd-cmd interval counter in hclk cycles
    #[inline(always)]
    pub fn cache_ncc(&mut self) -> CacheNccW<CACNTrs> {
        CacheNccW::new(self, 0)
    }
    ///Bits 8:15 - data-cmd interval counter in hclk cycles
    #[inline(always)]
    pub fn cache_ndc(&mut self) -> CacheNdcW<CACNTrs> {
        CacheNdcW::new(self, 8)
    }
    ///Bits 16:31 - timeout count register for ahb read
    #[inline(always)]
    pub fn cache_tor(&mut self) -> CacheTorW<CACNTrs> {
        CacheTorW::new(self, 16)
    }
}
///cache counter register
///
///You can [`read`](crate::Reg::read) this register and get [`cacnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CACNTrs;
impl crate::RegisterSpec for CACNTrs {
    type Ux = u32;
}
///`read()` method returns [`cacnt::R`](R) reader structure
impl crate::Readable for CACNTrs {}
///`write(|w| ..)` method takes [`cacnt::W`](W) writer structure
impl crate::Writable for CACNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACNT to value 0xffff_0020
impl crate::Resettable for CACNTrs {
    const RESET_VALUE: u32 = 0xffff_0020;
}
