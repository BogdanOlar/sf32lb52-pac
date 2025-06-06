///Register `DBL96_CALR` reader
pub type R = crate::R<DBL96_CALRrs>;
///Register `DBL96_CALR` writer
pub type W = crate::W<DBL96_CALRrs>;
///Field `CAL_EN` reader -
pub type CalEnR = crate::BitReader;
///Field `CAL_EN` writer -
pub type CalEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_CLOSE_EXT_EN` reader -
pub type CalCloseExtEnR = crate::BitReader;
///Field `CAL_CLOSE_EXT_EN` writer -
pub type CalCloseExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_OP` reader -
pub type CalOpR = crate::FieldReader<u16>;
///Field `CAL_OP` writer -
pub type CalOpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `CAL_LOCK` reader -
pub type CalLockR = crate::BitReader;
///Field `CAL_LOCK` writer -
pub type CalLockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn cal_en(&self) -> CalEnR {
        CalEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn cal_close_ext_en(&self) -> CalCloseExtEnR {
        CalCloseExtEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:12
    #[inline(always)]
    pub fn cal_op(&self) -> CalOpR {
        CalOpR::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    ///Bit 13
    #[inline(always)]
    pub fn cal_lock(&self) -> CalLockR {
        CalLockR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBL96_CALR")
            .field("rsvd", &self.rsvd())
            .field("cal_lock", &self.cal_lock())
            .field("cal_op", &self.cal_op())
            .field("cal_close_ext_en", &self.cal_close_ext_en())
            .field("cal_en", &self.cal_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn cal_en(&mut self) -> CalEnW<DBL96_CALRrs> {
        CalEnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn cal_close_ext_en(&mut self) -> CalCloseExtEnW<DBL96_CALRrs> {
        CalCloseExtEnW::new(self, 1)
    }
    ///Bits 2:12
    #[inline(always)]
    pub fn cal_op(&mut self) -> CalOpW<DBL96_CALRrs> {
        CalOpW::new(self, 2)
    }
    ///Bit 13
    #[inline(always)]
    pub fn cal_lock(&mut self) -> CalLockW<DBL96_CALRrs> {
        CalLockW::new(self, 13)
    }
    ///Bits 14:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DBL96_CALRrs> {
        RsvdW::new(self, 14)
    }
}
///DBL96 Calibration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbl96_calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbl96_calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBL96_CALRrs;
impl crate::RegisterSpec for DBL96_CALRrs {
    type Ux = u32;
}
///`read()` method returns [`dbl96_calr::R`](R) reader structure
impl crate::Readable for DBL96_CALRrs {}
///`write(|w| ..)` method takes [`dbl96_calr::W`](W) writer structure
impl crate::Writable for DBL96_CALRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBL96_CALR to value 0
impl crate::Resettable for DBL96_CALRrs {
    const RESET_VALUE: u32 = 0;
}
