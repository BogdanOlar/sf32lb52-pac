///Register `CAOFF` reader
pub type R = crate::R<CAOFFrs>;
///Register `CAOFF` writer
pub type W = crate::W<CAOFFrs>;
///Field `CACHE_OFFSET` reader - offset to map ahb address to sd address for ahb access
pub type CacheOffsetR = crate::FieldReader<u32>;
///Field `CACHE_OFFSET` writer - offset to map ahb address to sd address for ahb access
pub type CacheOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - offset to map ahb address to sd address for ahb access
    #[inline(always)]
    pub fn cache_offset(&self) -> CacheOffsetR {
        CacheOffsetR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAOFF")
            .field("cache_offset", &self.cache_offset())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - offset to map ahb address to sd address for ahb access
    #[inline(always)]
    pub fn cache_offset(&mut self) -> CacheOffsetW<CAOFFrs> {
        CacheOffsetW::new(self, 0)
    }
}
///cache offset register
///
///You can [`read`](crate::Reg::read) this register and get [`caoff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caoff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CAOFFrs;
impl crate::RegisterSpec for CAOFFrs {
    type Ux = u32;
}
///`read()` method returns [`caoff::R`](R) reader structure
impl crate::Readable for CAOFFrs {}
///`write(|w| ..)` method takes [`caoff::W`](W) writer structure
impl crate::Writable for CAOFFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAOFF to value 0
impl crate::Resettable for CAOFFrs {
    const RESET_VALUE: u32 = 0;
}
