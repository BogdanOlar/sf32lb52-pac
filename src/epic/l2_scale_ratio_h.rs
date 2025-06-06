///Register `L2_SCALE_RATIO_H` reader
pub type R = crate::R<L2_SCALE_RATIO_Hrs>;
///Register `L2_SCALE_RATIO_H` writer
pub type W = crate::W<L2_SCALE_RATIO_Hrs>;
///Field `XPITCH` reader - x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)
pub type XpitchR = crate::FieldReader<u32>;
///Field `XPITCH` writer - x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)
pub type XpitchW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)
    #[inline(always)]
    pub fn xpitch(&self) -> XpitchR {
        XpitchR::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_SCALE_RATIO_H")
            .field("xpitch", &self.xpitch())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)
    #[inline(always)]
    pub fn xpitch(&mut self) -> XpitchW<L2_SCALE_RATIO_Hrs> {
        XpitchW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_ratio_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_ratio_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_SCALE_RATIO_Hrs;
impl crate::RegisterSpec for L2_SCALE_RATIO_Hrs {
    type Ux = u32;
}
///`read()` method returns [`l2_scale_ratio_h::R`](R) reader structure
impl crate::Readable for L2_SCALE_RATIO_Hrs {}
///`write(|w| ..)` method takes [`l2_scale_ratio_h::W`](W) writer structure
impl crate::Writable for L2_SCALE_RATIO_Hrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_SCALE_RATIO_H to value 0
impl crate::Resettable for L2_SCALE_RATIO_Hrs {
    const RESET_VALUE: u32 = 0;
}
