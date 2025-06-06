///Register `VL_ROT` reader
pub type R = crate::R<VL_ROTrs>;
///Register `VL_ROT` writer
pub type W = crate::W<VL_ROTrs>;
///Field `CALC_REQ` reader - rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation.
pub type CalcReqR = crate::BitReader;
///Field `CALC_REQ` writer - rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation.
pub type CalcReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALC_CLR` reader - rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result.
pub type CalcClrR = crate::BitReader;
///Field `CALC_CLR` writer - rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result.
pub type CalcClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROT_DEG` reader - rotation degree, rotation is clockwise.
pub type RotDegR = crate::FieldReader<u16>;
///Field `ROT_DEG` writer - rotation degree, rotation is clockwise.
pub type RotDegW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CALC_DONE` reader - calculation done indicator
pub type CalcDoneR = crate::BitReader;
///Field `CALC_DONE` writer - calculation done indicator
pub type CalcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bit 0 - rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation.
    #[inline(always)]
    pub fn calc_req(&self) -> CalcReqR {
        CalcReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result.
    #[inline(always)]
    pub fn calc_clr(&self) -> CalcClrR {
        CalcClrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:10 - rotation degree, rotation is clockwise.
    #[inline(always)]
    pub fn rot_deg(&self) -> RotDegR {
        RotDegR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bit 11 - calculation done indicator
    #[inline(always)]
    pub fn calc_done(&self) -> CalcDoneR {
        CalcDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_ROT")
            .field("rsvd", &self.rsvd())
            .field("calc_done", &self.calc_done())
            .field("rot_deg", &self.rot_deg())
            .field("calc_clr", &self.calc_clr())
            .field("calc_req", &self.calc_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation.
    #[inline(always)]
    pub fn calc_req(&mut self) -> CalcReqW<VL_ROTrs> {
        CalcReqW::new(self, 0)
    }
    ///Bit 1 - rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result.
    #[inline(always)]
    pub fn calc_clr(&mut self) -> CalcClrW<VL_ROTrs> {
        CalcClrW::new(self, 1)
    }
    ///Bits 2:10 - rotation degree, rotation is clockwise.
    #[inline(always)]
    pub fn rot_deg(&mut self) -> RotDegW<VL_ROTrs> {
        RotDegW::new(self, 2)
    }
    ///Bit 11 - calculation done indicator
    #[inline(always)]
    pub fn calc_done(&mut self) -> CalcDoneW<VL_ROTrs> {
        CalcDoneW::new(self, 11)
    }
    ///Bits 12:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<VL_ROTrs> {
        RsvdW::new(self, 12)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_ROTrs;
impl crate::RegisterSpec for VL_ROTrs {
    type Ux = u32;
}
///`read()` method returns [`vl_rot::R`](R) reader structure
impl crate::Readable for VL_ROTrs {}
///`write(|w| ..)` method takes [`vl_rot::W`](W) writer structure
impl crate::Writable for VL_ROTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_ROT to value 0
impl crate::Resettable for VL_ROTrs {
    const RESET_VALUE: u32 = 0;
}
