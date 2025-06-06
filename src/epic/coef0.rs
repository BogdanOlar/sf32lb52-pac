///Register `COEF0` reader
pub type R = crate::R<COEF0rs>;
///Register `COEF0` writer
pub type W = crate::W<COEF0rs>;
///Field `FUB` reader - YUV Fub coef
pub type FubR = crate::FieldReader<u16>;
///Field `FUB` writer - YUV Fub coef
pub type FubW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `FUG` reader - YUV Fug coef
pub type FugR = crate::FieldReader<u16>;
///Field `FUG` writer - YUV Fug coef
pub type FugW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `FY` reader - YUV Fy coef
pub type FyR = crate::FieldReader<u16>;
///Field `FY` writer - YUV Fy coef
pub type FyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:9 - YUV Fub coef
    #[inline(always)]
    pub fn fub(&self) -> FubR {
        FubR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - YUV Fug coef
    #[inline(always)]
    pub fn fug(&self) -> FugR {
        FugR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - YUV Fy coef
    #[inline(always)]
    pub fn fy(&self) -> FyR {
        FyR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEF0")
            .field("rsvd", &self.rsvd())
            .field("fy", &self.fy())
            .field("fug", &self.fug())
            .field("fub", &self.fub())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - YUV Fub coef
    #[inline(always)]
    pub fn fub(&mut self) -> FubW<COEF0rs> {
        FubW::new(self, 0)
    }
    ///Bits 10:19 - YUV Fug coef
    #[inline(always)]
    pub fn fug(&mut self) -> FugW<COEF0rs> {
        FugW::new(self, 10)
    }
    ///Bits 20:29 - YUV Fy coef
    #[inline(always)]
    pub fn fy(&mut self) -> FyW<COEF0rs> {
        FyW::new(self, 20)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<COEF0rs> {
        RsvdW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`coef0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coef0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COEF0rs;
impl crate::RegisterSpec for COEF0rs {
    type Ux = u32;
}
///`read()` method returns [`coef0::R`](R) reader structure
impl crate::Readable for COEF0rs {}
///`write(|w| ..)` method takes [`coef0::W`](W) writer structure
impl crate::Writable for COEF0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COEF0 to value 0
impl crate::Resettable for COEF0rs {
    const RESET_VALUE: u32 = 0;
}
