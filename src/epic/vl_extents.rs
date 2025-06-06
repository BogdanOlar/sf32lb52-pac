///Register `VL_EXTENTS` reader
pub type R = crate::R<VL_EXTENTSrs>;
///Register `VL_EXTENTS` writer
pub type W = crate::W<VL_EXTENTSrs>;
///Field `MAX_LINE` reader - number of pixels of each column of source image(not including padding)
pub type MaxLineR = crate::FieldReader<u16>;
///Field `MAX_LINE` writer - number of pixels of each column of source image(not including padding)
pub type MaxLineW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `MAX_COL` reader - number of pixels of each line of source image(not including padding)
pub type MaxColR = crate::FieldReader<u16>;
///Field `MAX_COL` writer - number of pixels of each line of source image(not including padding)
pub type MaxColW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:9 - number of pixels of each column of source image(not including padding)
    #[inline(always)]
    pub fn max_line(&self) -> MaxLineR {
        MaxLineR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 16:25 - number of pixels of each line of source image(not including padding)
    #[inline(always)]
    pub fn max_col(&self) -> MaxColR {
        MaxColR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_EXTENTS")
            .field("rsvd", &self.rsvd())
            .field("max_col", &self.max_col())
            .field("rsvd2", &self.rsvd2())
            .field("max_line", &self.max_line())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - number of pixels of each column of source image(not including padding)
    #[inline(always)]
    pub fn max_line(&mut self) -> MaxLineW<VL_EXTENTSrs> {
        MaxLineW::new(self, 0)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<VL_EXTENTSrs> {
        Rsvd2W::new(self, 10)
    }
    ///Bits 16:25 - number of pixels of each line of source image(not including padding)
    #[inline(always)]
    pub fn max_col(&mut self) -> MaxColW<VL_EXTENTSrs> {
        MaxColW::new(self, 16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<VL_EXTENTSrs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_extents::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_extents::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_EXTENTSrs;
impl crate::RegisterSpec for VL_EXTENTSrs {
    type Ux = u32;
}
///`read()` method returns [`vl_extents::R`](R) reader structure
impl crate::Readable for VL_EXTENTSrs {}
///`write(|w| ..)` method takes [`vl_extents::W`](W) writer structure
impl crate::Writable for VL_EXTENTSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_EXTENTS to value 0
impl crate::Resettable for VL_EXTENTSrs {
    const RESET_VALUE: u32 = 0;
}
