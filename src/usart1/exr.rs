///Register `EXR` reader
pub type R = crate::R<EXRrs>;
///Register `EXR` writer
pub type W = crate::W<EXRrs>;
///Field `BUSY` reader -
pub type BusyR = crate::BitReader;
///Field `BUSY` writer -
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ID` reader -
pub type IdR = crate::BitReader;
///Field `ID` writer -
pub type IdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXR")
            .field("id", &self.id())
            .field("busy", &self.busy())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<EXRrs> {
        BusyW::new(self, 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn id(&mut self) -> IdW<EXRrs> {
        IdW::new(self, 4)
    }
}
///Mutual Exclusive Register
///
///You can [`read`](crate::Reg::read) this register and get [`exr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EXRrs;
impl crate::RegisterSpec for EXRrs {
    type Ux = u32;
}
///`read()` method returns [`exr::R`](R) reader structure
impl crate::Readable for EXRrs {}
///`write(|w| ..)` method takes [`exr::W`](W) writer structure
impl crate::Writable for EXRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXR to value 0x01
impl crate::Resettable for EXRrs {
    const RESET_VALUE: u32 = 0x01;
}
