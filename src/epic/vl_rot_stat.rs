///Register `VL_ROT_STAT` reader
pub type R = crate::R<VL_ROT_STATrs>;
///Register `VL_ROT_STAT` writer
pub type W = crate::W<VL_ROT_STATrs>;
///Field `ROT_MAX_LINE` reader - max line of rotated image
pub type RotMaxLineR = crate::FieldReader<u16>;
///Field `ROT_MAX_LINE` writer - max line of rotated image
pub type RotMaxLineW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ROT_MAX_COL` reader - max column of rotated image
pub type RotMaxColR = crate::FieldReader<u16>;
///Field `ROT_MAX_COL` writer - max column of rotated image
pub type RotMaxColW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:10 - max line of rotated image
    #[inline(always)]
    pub fn rot_max_line(&self) -> RotMaxLineR {
        RotMaxLineR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 16:26 - max column of rotated image
    #[inline(always)]
    pub fn rot_max_col(&self) -> RotMaxColR {
        RotMaxColR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bits 27:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_ROT_STAT")
            .field("rsvd", &self.rsvd())
            .field("rot_max_col", &self.rot_max_col())
            .field("rsvd2", &self.rsvd2())
            .field("rot_max_line", &self.rot_max_line())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - max line of rotated image
    #[inline(always)]
    pub fn rot_max_line(&mut self) -> RotMaxLineW<VL_ROT_STATrs> {
        RotMaxLineW::new(self, 0)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<VL_ROT_STATrs> {
        Rsvd2W::new(self, 11)
    }
    ///Bits 16:26 - max column of rotated image
    #[inline(always)]
    pub fn rot_max_col(&mut self) -> RotMaxColW<VL_ROT_STATrs> {
        RotMaxColW::new(self, 16)
    }
    ///Bits 27:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<VL_ROT_STATrs> {
        RsvdW::new(self, 27)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_ROT_STATrs;
impl crate::RegisterSpec for VL_ROT_STATrs {
    type Ux = u32;
}
///`read()` method returns [`vl_rot_stat::R`](R) reader structure
impl crate::Readable for VL_ROT_STATrs {}
///`write(|w| ..)` method takes [`vl_rot_stat::W`](W) writer structure
impl crate::Writable for VL_ROT_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_ROT_STAT to value 0
impl crate::Resettable for VL_ROT_STATrs {
    const RESET_VALUE: u32 = 0;
}
