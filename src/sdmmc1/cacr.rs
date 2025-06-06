///Register `CACR` reader
pub type R = crate::R<CACRrs>;
///Register `CACR` writer
pub type W = crate::W<CACRrs>;
///Field `READ_INDEX` reader - Command index for cache read. CMD18 by default
pub type ReadIndexR = crate::FieldReader;
///Field `READ_INDEX` writer - Command index for cache read. CMD18 by default
pub type ReadIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `READ_HAS_RSP` reader - Read command have a response
pub type ReadHasRspR = crate::BitReader;
///Field `READ_HAS_RSP` writer - Read command have a response
pub type ReadHasRspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READ_LONG_RSP` reader - Read response is 136-bit, long response
pub type ReadLongRspR = crate::BitReader;
///Field `READ_LONG_RSP` writer - Read response is 136-bit, long response
pub type ReadLongRspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_INDEX` reader - Command index for stop. CMD12 by default
pub type StopIndexR = crate::FieldReader;
///Field `STOP_INDEX` writer - Command index for stop. CMD12 by default
pub type StopIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `STOP_HAS_RSP` reader - Stop command have a response
pub type StopHasRspR = crate::BitReader;
///Field `STOP_HAS_RSP` writer - Stop command have a response
pub type StopHasRspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_LONG_RSP` reader - Stop response is 136-bit, long response
pub type StopLongRspR = crate::BitReader;
///Field `STOP_LONG_RSP` writer - Stop response is 136-bit, long response
pub type StopLongRspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_BLOCK` reader - cache depth is cache_block blocks
pub type CacheBlockR = crate::FieldReader;
///Field `CACHE_BLOCK` writer - cache depth is cache_block blocks
pub type CacheBlockW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_PREF_BLOCK` reader - cache prefetch depth is cache_pref_block blocks. Should be no less than cache_block
pub type CachePrefBlockR = crate::FieldReader;
///Field `CACHE_PREF_BLOCK` writer - cache prefetch depth is cache_pref_block blocks. Should be no less than cache_block
pub type CachePrefBlockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CACHE_HRESP` reader - 1: generate ahb error response when error occur 0: no ahb error response generated. Could check cache_err interrupt
pub type CacheHrespR = crate::BitReader;
///Field `CACHE_HRESP` writer - 1: generate ahb error response when error occur 0: no ahb error response generated. Could check cache_err interrupt
pub type CacheHrespW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_NOCRC` reader - 1: return ahb data without crc check 0: return ahb data after block crc pass
pub type CacheNocrcR = crate::BitReader;
///Field `CACHE_NOCRC` writer - 1: return ahb data without crc check 0: return ahb data after block crc pass
pub type CacheNocrcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_SDSC` reader - select card version 1: card size (=2GB, address of cmd18 is in byte 0: card size >2GB, address of cmd18 is in block
pub type CacheSdscR = crate::BitReader;
///Field `CACHE_SDSC` writer - select card version 1: card size (=2GB, address of cmd18 is in byte 0: card size >2GB, address of cmd18 is in block
pub type CacheSdscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_FORCE_READ` reader - force cache read done 1: start new fetch for miss access only after cache read done 0: start new fetch for miss access even when cache is still filling (read will be breaked by cmd12)
pub type CacheForceReadR = crate::BitReader;
///Field `CACHE_FORCE_READ` writer - force cache read done 1: start new fetch for miss access only after cache read done 0: start new fetch for miss access even when cache is still filling (read will be breaked by cmd12)
pub type CacheForceReadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_TO_EN` reader - enable ahb read timeout recover
pub type CacheToEnR = crate::BitReader;
///Field `CACHE_TO_EN` writer - enable ahb read timeout recover
pub type CacheToEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_EN` reader - enable cache 1: ahb read will return cached data 0: ahb read always return dummy data with no error response
pub type CacheEnR = crate::BitReader;
///Field `CACHE_EN` writer - enable cache 1: ahb read will return cached data 0: ahb read always return dummy data with no error response
pub type CacheEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Command index for cache read. CMD18 by default
    #[inline(always)]
    pub fn read_index(&self) -> ReadIndexR {
        ReadIndexR::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Read command have a response
    #[inline(always)]
    pub fn read_has_rsp(&self) -> ReadHasRspR {
        ReadHasRspR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Read response is 136-bit, long response
    #[inline(always)]
    pub fn read_long_rsp(&self) -> ReadLongRspR {
        ReadLongRspR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - Command index for stop. CMD12 by default
    #[inline(always)]
    pub fn stop_index(&self) -> StopIndexR {
        StopIndexR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 14 - Stop command have a response
    #[inline(always)]
    pub fn stop_has_rsp(&self) -> StopHasRspR {
        StopHasRspR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Stop response is 136-bit, long response
    #[inline(always)]
    pub fn stop_long_rsp(&self) -> StopLongRspR {
        StopLongRspR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - cache depth is cache_block blocks
    #[inline(always)]
    pub fn cache_block(&self) -> CacheBlockR {
        CacheBlockR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - cache prefetch depth is cache_pref_block blocks. Should be no less than cache_block
    #[inline(always)]
    pub fn cache_pref_block(&self) -> CachePrefBlockR {
        CachePrefBlockR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - 1: generate ahb error response when error occur 0: no ahb error response generated. Could check cache_err interrupt
    #[inline(always)]
    pub fn cache_hresp(&self) -> CacheHrespR {
        CacheHrespR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 1: return ahb data without crc check 0: return ahb data after block crc pass
    #[inline(always)]
    pub fn cache_nocrc(&self) -> CacheNocrcR {
        CacheNocrcR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - select card version 1: card size (=2GB, address of cmd18 is in byte 0: card size >2GB, address of cmd18 is in block
    #[inline(always)]
    pub fn cache_sdsc(&self) -> CacheSdscR {
        CacheSdscR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - force cache read done 1: start new fetch for miss access only after cache read done 0: start new fetch for miss access even when cache is still filling (read will be breaked by cmd12)
    #[inline(always)]
    pub fn cache_force_read(&self) -> CacheForceReadR {
        CacheForceReadR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable ahb read timeout recover
    #[inline(always)]
    pub fn cache_to_en(&self) -> CacheToEnR {
        CacheToEnR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - enable cache 1: ahb read will return cached data 0: ahb read always return dummy data with no error response
    #[inline(always)]
    pub fn cache_en(&self) -> CacheEnR {
        CacheEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR")
            .field("cache_en", &self.cache_en())
            .field("cache_to_en", &self.cache_to_en())
            .field("cache_force_read", &self.cache_force_read())
            .field("cache_sdsc", &self.cache_sdsc())
            .field("cache_nocrc", &self.cache_nocrc())
            .field("cache_hresp", &self.cache_hresp())
            .field("rsvd", &self.rsvd())
            .field("cache_pref_block", &self.cache_pref_block())
            .field("rsvd2", &self.rsvd2())
            .field("cache_block", &self.cache_block())
            .field("stop_long_rsp", &self.stop_long_rsp())
            .field("stop_has_rsp", &self.stop_has_rsp())
            .field("stop_index", &self.stop_index())
            .field("read_long_rsp", &self.read_long_rsp())
            .field("read_has_rsp", &self.read_has_rsp())
            .field("read_index", &self.read_index())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Command index for cache read. CMD18 by default
    #[inline(always)]
    pub fn read_index(&mut self) -> ReadIndexW<CACRrs> {
        ReadIndexW::new(self, 0)
    }
    ///Bit 6 - Read command have a response
    #[inline(always)]
    pub fn read_has_rsp(&mut self) -> ReadHasRspW<CACRrs> {
        ReadHasRspW::new(self, 6)
    }
    ///Bit 7 - Read response is 136-bit, long response
    #[inline(always)]
    pub fn read_long_rsp(&mut self) -> ReadLongRspW<CACRrs> {
        ReadLongRspW::new(self, 7)
    }
    ///Bits 8:13 - Command index for stop. CMD12 by default
    #[inline(always)]
    pub fn stop_index(&mut self) -> StopIndexW<CACRrs> {
        StopIndexW::new(self, 8)
    }
    ///Bit 14 - Stop command have a response
    #[inline(always)]
    pub fn stop_has_rsp(&mut self) -> StopHasRspW<CACRrs> {
        StopHasRspW::new(self, 14)
    }
    ///Bit 15 - Stop response is 136-bit, long response
    #[inline(always)]
    pub fn stop_long_rsp(&mut self) -> StopLongRspW<CACRrs> {
        StopLongRspW::new(self, 15)
    }
    ///Bits 16:18 - cache depth is cache_block blocks
    #[inline(always)]
    pub fn cache_block(&mut self) -> CacheBlockW<CACRrs> {
        CacheBlockW::new(self, 16)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CACRrs> {
        Rsvd2W::new(self, 19)
    }
    ///Bits 20:23 - cache prefetch depth is cache_pref_block blocks. Should be no less than cache_block
    #[inline(always)]
    pub fn cache_pref_block(&mut self) -> CachePrefBlockW<CACRrs> {
        CachePrefBlockW::new(self, 20)
    }
    ///Bits 24:25
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CACRrs> {
        RsvdW::new(self, 24)
    }
    ///Bit 26 - 1: generate ahb error response when error occur 0: no ahb error response generated. Could check cache_err interrupt
    #[inline(always)]
    pub fn cache_hresp(&mut self) -> CacheHrespW<CACRrs> {
        CacheHrespW::new(self, 26)
    }
    ///Bit 27 - 1: return ahb data without crc check 0: return ahb data after block crc pass
    #[inline(always)]
    pub fn cache_nocrc(&mut self) -> CacheNocrcW<CACRrs> {
        CacheNocrcW::new(self, 27)
    }
    ///Bit 28 - select card version 1: card size (=2GB, address of cmd18 is in byte 0: card size >2GB, address of cmd18 is in block
    #[inline(always)]
    pub fn cache_sdsc(&mut self) -> CacheSdscW<CACRrs> {
        CacheSdscW::new(self, 28)
    }
    ///Bit 29 - force cache read done 1: start new fetch for miss access only after cache read done 0: start new fetch for miss access even when cache is still filling (read will be breaked by cmd12)
    #[inline(always)]
    pub fn cache_force_read(&mut self) -> CacheForceReadW<CACRrs> {
        CacheForceReadW::new(self, 29)
    }
    ///Bit 30 - enable ahb read timeout recover
    #[inline(always)]
    pub fn cache_to_en(&mut self) -> CacheToEnW<CACRrs> {
        CacheToEnW::new(self, 30)
    }
    ///Bit 31 - enable cache 1: ahb read will return cached data 0: ahb read always return dummy data with no error response
    #[inline(always)]
    pub fn cache_en(&mut self) -> CacheEnW<CACRrs> {
        CacheEnW::new(self, 31)
    }
}
///cache control register
///
///You can [`read`](crate::Reg::read) this register and get [`cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CACRrs;
impl crate::RegisterSpec for CACRrs {
    type Ux = u32;
}
///`read()` method returns [`cacr::R`](R) reader structure
impl crate::Readable for CACRrs {}
///`write(|w| ..)` method takes [`cacr::W`](W) writer structure
impl crate::Writable for CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACR to value 0xd084_4c52
impl crate::Resettable for CACRrs {
    const RESET_VALUE: u32 = 0xd084_4c52;
}
