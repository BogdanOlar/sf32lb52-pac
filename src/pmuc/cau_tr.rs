///Register `CAU_TR` reader
pub type R = crate::R<CAU_TRrs>;
///Register `CAU_TR` writer
pub type W = crate::W<CAU_TRrs>;
///Field `CAU_DC_TR` reader -
pub type CauDcTrR = crate::FieldReader;
///Field `CAU_DC_TR` writer -
pub type CauDcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CAU_DC_BR` reader -
pub type CauDcBrR = crate::FieldReader;
///Field `CAU_DC_BR` writer -
pub type CauDcBrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CAU_DC_MR` reader -
pub type CauDcMrR = crate::FieldReader;
///Field `CAU_DC_MR` writer -
pub type CauDcMrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn cau_dc_tr(&self) -> CauDcTrR {
        CauDcTrR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn cau_dc_br(&self) -> CauDcBrR {
        CauDcBrR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8
    #[inline(always)]
    pub fn cau_dc_mr(&self) -> CauDcMrR {
        CauDcMrR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAU_TR")
            .field("rsvd", &self.rsvd())
            .field("cau_dc_mr", &self.cau_dc_mr())
            .field("cau_dc_br", &self.cau_dc_br())
            .field("cau_dc_tr", &self.cau_dc_tr())
            .finish()
    }
}
impl W {
    ///Bits 0:2
    #[inline(always)]
    pub fn cau_dc_tr(&mut self) -> CauDcTrW<CAU_TRrs> {
        CauDcTrW::new(self, 0)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn cau_dc_br(&mut self) -> CauDcBrW<CAU_TRrs> {
        CauDcBrW::new(self, 3)
    }
    ///Bits 6:8
    #[inline(always)]
    pub fn cau_dc_mr(&mut self) -> CauDcMrW<CAU_TRrs> {
        CauDcMrW::new(self, 6)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CAU_TRrs> {
        RsvdW::new(self, 9)
    }
}
///CAU Test Register
///
///You can [`read`](crate::Reg::read) this register and get [`cau_tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau_tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CAU_TRrs;
impl crate::RegisterSpec for CAU_TRrs {
    type Ux = u32;
}
///`read()` method returns [`cau_tr::R`](R) reader structure
impl crate::Readable for CAU_TRrs {}
///`write(|w| ..)` method takes [`cau_tr::W`](W) writer structure
impl crate::Writable for CAU_TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAU_TR to value 0
impl crate::Resettable for CAU_TRrs {
    const RESET_VALUE: u32 = 0;
}
