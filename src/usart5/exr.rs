///Register `EXR` reader
pub type R = crate::R<EXRrs>;
///Register `EXR` writer
pub type W = crate::W<EXRrs>;
///Field `BUSY` reader -
pub type BusyR = crate::BitReader;
///Field `BUSY` writer -
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ID` reader -
pub type IdR = crate::BitReader;
///Field `ID` writer -
pub type IdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXR")
            .field("rsvd", &self.rsvd())
            .field("id", &self.id())
            .field("rsvd2", &self.rsvd2())
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
    ///Bits 1:3
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<EXRrs> {
        Rsvd2W::new(self, 1)
    }
    ///Bit 4
    #[inline(always)]
    pub fn id(&mut self) -> IdW<EXRrs> {
        IdW::new(self, 4)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<EXRrs> {
        RsvdW::new(self, 5)
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
