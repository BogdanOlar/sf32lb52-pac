///Register `L2_SCALE_RATIO_V` reader
pub type R = crate::R<L2_SCALE_RATIO_Vrs>;
///Register `L2_SCALE_RATIO_V` writer
pub type W = crate::W<L2_SCALE_RATIO_Vrs>;
///Field `YPITCH` reader - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
pub type YpitchR = crate::FieldReader<u32>;
///Field `YPITCH` writer - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
pub type YpitchW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
    #[inline(always)]
    pub fn ypitch(&self) -> YpitchR {
        YpitchR::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_SCALE_RATIO_V")
            .field("ypitch", &self.ypitch())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)
    #[inline(always)]
    pub fn ypitch(&mut self) -> YpitchW<L2_SCALE_RATIO_Vrs> {
        YpitchW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_ratio_v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_ratio_v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_SCALE_RATIO_Vrs;
impl crate::RegisterSpec for L2_SCALE_RATIO_Vrs {
    type Ux = u32;
}
///`read()` method returns [`l2_scale_ratio_v::R`](R) reader structure
impl crate::Readable for L2_SCALE_RATIO_Vrs {}
///`write(|w| ..)` method takes [`l2_scale_ratio_v::W`](W) writer structure
impl crate::Writable for L2_SCALE_RATIO_Vrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_SCALE_RATIO_V to value 0
impl crate::Resettable for L2_SCALE_RATIO_Vrs {
    const RESET_VALUE: u32 = 0;
}
