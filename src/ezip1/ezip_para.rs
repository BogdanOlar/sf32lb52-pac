///Register `EZIP_PARA` reader
pub type R = crate::R<EZIP_PARArs>;
///Register `EZIP_PARA` writer
pub type W = crate::W<EZIP_PARArs>;
///Field `OUT_SEL` reader - only used in ezip decoder mode. must select ahb in gzip/lz4 decoder mode. 0:epic 1:ahb
pub type OutSelR = crate::BitReader;
///Field `OUT_SEL` writer - only used in ezip decoder mode. must select ahb in gzip/lz4 decoder mode. 0:epic 1:ahb
pub type OutSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOD_SEL` reader - 0:ezip or aezip 1:gzip 2:Lz4
pub type ModSelR = crate::FieldReader;
///Field `MOD_SEL` writer - 0:ezip or aezip 1:gzip 2:Lz4
pub type ModSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CACHE_EN` reader - no used
pub type CacheEnR = crate::BitReader;
///Field `CACHE_EN` writer - no used
pub type CacheEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_SEL` reader - don't support ezip type2\type4. 0:ahb 1:fifo
pub type InSelR = crate::BitReader;
///Field `IN_SEL` writer - don't support ezip type2\type4. 0:ahb 1:fifo
pub type InSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SEL` reader - 0:QSPI4 1:QSPI3
pub type SpiSelR = crate::BitReader;
///Field `SPI_SEL` writer - 0:QSPI4 1:QSPI3
pub type SpiSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - only used in ezip decoder mode. must select ahb in gzip/lz4 decoder mode. 0:epic 1:ahb
    #[inline(always)]
    pub fn out_sel(&self) -> OutSelR {
        OutSelR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - 0:ezip or aezip 1:gzip 2:Lz4
    #[inline(always)]
    pub fn mod_sel(&self) -> ModSelR {
        ModSelR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - no used
    #[inline(always)]
    pub fn cache_en(&self) -> CacheEnR {
        CacheEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - don't support ezip type2\type4. 0:ahb 1:fifo
    #[inline(always)]
    pub fn in_sel(&self) -> InSelR {
        InSelR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 0:QSPI4 1:QSPI3
    #[inline(always)]
    pub fn spi_sel(&self) -> SpiSelR {
        SpiSelR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EZIP_PARA")
            .field("spi_sel", &self.spi_sel())
            .field("in_sel", &self.in_sel())
            .field("cache_en", &self.cache_en())
            .field("mod_sel", &self.mod_sel())
            .field("out_sel", &self.out_sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - only used in ezip decoder mode. must select ahb in gzip/lz4 decoder mode. 0:epic 1:ahb
    #[inline(always)]
    pub fn out_sel(&mut self) -> OutSelW<EZIP_PARArs> {
        OutSelW::new(self, 0)
    }
    ///Bits 1:2 - 0:ezip or aezip 1:gzip 2:Lz4
    #[inline(always)]
    pub fn mod_sel(&mut self) -> ModSelW<EZIP_PARArs> {
        ModSelW::new(self, 1)
    }
    ///Bit 3 - no used
    #[inline(always)]
    pub fn cache_en(&mut self) -> CacheEnW<EZIP_PARArs> {
        CacheEnW::new(self, 3)
    }
    ///Bit 4 - don't support ezip type2\type4. 0:ahb 1:fifo
    #[inline(always)]
    pub fn in_sel(&mut self) -> InSelW<EZIP_PARArs> {
        InSelW::new(self, 4)
    }
    ///Bit 5 - 0:QSPI4 1:QSPI3
    #[inline(always)]
    pub fn spi_sel(&mut self) -> SpiSelW<EZIP_PARArs> {
        SpiSelW::new(self, 5)
    }
}
///ezip decoder parameter
///
///You can [`read`](crate::Reg::read) this register and get [`ezip_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ezip_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EZIP_PARArs;
impl crate::RegisterSpec for EZIP_PARArs {
    type Ux = u32;
}
///`read()` method returns [`ezip_para::R`](R) reader structure
impl crate::Readable for EZIP_PARArs {}
///`write(|w| ..)` method takes [`ezip_para::W`](W) writer structure
impl crate::Writable for EZIP_PARArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EZIP_PARA to value 0
impl crate::Resettable for EZIP_PARArs {
    const RESET_VALUE: u32 = 0;
}
