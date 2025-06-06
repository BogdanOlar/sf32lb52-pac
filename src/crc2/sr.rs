///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DONE` reader - Done flag. When DR written, done flag will be cleared automatically. The flag will assert after CRC operation of current DR finished.
pub type DoneR = crate::BitReader;
///Field `DONE` writer - Done flag. When DR written, done flag will be cleared automatically. The flag will assert after CRC operation of current DR finished.
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVERFLOW` reader - Overflow when new data arrive while last calculation not done yet
pub type OverflowR = crate::BitReader;
///Field `OVERFLOW` writer - Overflow when new data arrive while last calculation not done yet
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Done flag. When DR written, done flag will be cleared automatically. The flag will assert after CRC operation of current DR finished.
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overflow when new data arrive while last calculation not done yet
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("overflow", &self.overflow())
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    ///Bit 0 - Done flag. When DR written, done flag will be cleared automatically. The flag will assert after CRC operation of current DR finished.
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<SRrs> {
        DoneW::new(self, 0)
    }
    ///Bit 1 - Overflow when new data arrive while last calculation not done yet
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<SRrs> {
        OverflowW::new(self, 1)
    }
}
///Status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
