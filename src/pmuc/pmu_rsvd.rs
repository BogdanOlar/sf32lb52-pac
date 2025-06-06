///Register `PMU_RSVD` reader
pub type R = crate::R<PMU_RSVDrs>;
///Register `PMU_RSVD` writer
pub type W = crate::W<PMU_RSVDrs>;
///Field `RESERVE0` reader -
pub type Reserve0R = crate::FieldReader;
///Field `RESERVE0` writer -
pub type Reserve0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RESERVE1` reader -
pub type Reserve1R = crate::FieldReader;
///Field `RESERVE1` writer -
pub type Reserve1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RESERVE2` reader -
pub type Reserve2R = crate::FieldReader;
///Field `RESERVE2` writer -
pub type Reserve2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RESERVE3` reader -
pub type Reserve3R = crate::FieldReader;
///Field `RESERVE3` writer -
pub type Reserve3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn reserve0(&self) -> Reserve0R {
        Reserve0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn reserve1(&self) -> Reserve1R {
        Reserve1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn reserve2(&self) -> Reserve2R {
        Reserve2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn reserve3(&self) -> Reserve3R {
        Reserve3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_RSVD")
            .field("reserve3", &self.reserve3())
            .field("reserve2", &self.reserve2())
            .field("reserve1", &self.reserve1())
            .field("reserve0", &self.reserve0())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    pub fn reserve0(&mut self) -> Reserve0W<PMU_RSVDrs> {
        Reserve0W::new(self, 0)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn reserve1(&mut self) -> Reserve1W<PMU_RSVDrs> {
        Reserve1W::new(self, 8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn reserve2(&mut self) -> Reserve2W<PMU_RSVDrs> {
        Reserve2W::new(self, 16)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn reserve3(&mut self) -> Reserve3W<PMU_RSVDrs> {
        Reserve3W::new(self, 24)
    }
}
///PMU Reserved Register
///
///You can [`read`](crate::Reg::read) this register and get [`pmu_rsvd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_rsvd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PMU_RSVDrs;
impl crate::RegisterSpec for PMU_RSVDrs {
    type Ux = u32;
}
///`read()` method returns [`pmu_rsvd::R`](R) reader structure
impl crate::Readable for PMU_RSVDrs {}
///`write(|w| ..)` method takes [`pmu_rsvd::W`](W) writer structure
impl crate::Writable for PMU_RSVDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PMU_RSVD to value 0
impl crate::Resettable for PMU_RSVDrs {
    const RESET_VALUE: u32 = 0;
}
