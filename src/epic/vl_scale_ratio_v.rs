///Register `VL_SCALE_RATIO_V` reader
pub type R = crate::R<VL_SCALE_RATIO_Vrs>;
///Register `VL_SCALE_RATIO_V` writer
pub type W = crate::W<VL_SCALE_RATIO_Vrs>;
///Field `YPITCH` reader - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
pub type YpitchR = crate::FieldReader<u32>;
///Field `YPITCH` writer - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
pub type YpitchW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:25 - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
    #[inline(always)]
    pub fn ypitch(&self) -> YpitchR {
        YpitchR::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_SCALE_RATIO_V")
            .field("rsvd", &self.rsvd())
            .field("ypitch", &self.ypitch())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
    #[inline(always)]
    pub fn ypitch(&mut self) -> YpitchW<VL_SCALE_RATIO_Vrs> {
        YpitchW::new(self, 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<VL_SCALE_RATIO_Vrs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_ratio_v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_ratio_v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_SCALE_RATIO_Vrs;
impl crate::RegisterSpec for VL_SCALE_RATIO_Vrs {
    type Ux = u32;
}
///`read()` method returns [`vl_scale_ratio_v::R`](R) reader structure
impl crate::Readable for VL_SCALE_RATIO_Vrs {}
///`write(|w| ..)` method takes [`vl_scale_ratio_v::W`](W) writer structure
impl crate::Writable for VL_SCALE_RATIO_Vrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_SCALE_RATIO_V to value 0
impl crate::Resettable for VL_SCALE_RATIO_Vrs {
    const RESET_VALUE: u32 = 0;
}
