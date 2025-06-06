///Register `HRCCAL1` reader
pub type R = crate::R<HRCCAL1rs>;
///Register `HRCCAL1` writer
pub type W = crate::W<HRCCAL1rs>;
///Field `CAL_LENGTH` reader - Target clk_hxt48 cycles during calibration
pub type CalLengthR = crate::FieldReader<u16>;
///Field `CAL_LENGTH` writer - Target clk_hxt48 cycles during calibration
pub type CalLengthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `CAL_EN` reader - Calibration enble. Set to 0 to clear result, then set to 1 to start a new calibration
pub type CalEnR = crate::BitReader;
///Field `CAL_EN` writer - Calibration enble. Set to 0 to clear result, then set to 1 to start a new calibration
pub type CalEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_DONE` reader - Calibration done. After a new calibration started, results should be processed only when cal_done asserted.
pub type CalDoneR = crate::BitReader;
///Field `CAL_DONE` writer - Calibration done. After a new calibration started, results should be processed only when cal_done asserted.
pub type CalDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Target clk_hxt48 cycles during calibration
    #[inline(always)]
    pub fn cal_length(&self) -> CalLengthR {
        CalLengthR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:29
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    ///Bit 30 - Calibration enble. Set to 0 to clear result, then set to 1 to start a new calibration
    #[inline(always)]
    pub fn cal_en(&self) -> CalEnR {
        CalEnR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Calibration done. After a new calibration started, results should be processed only when cal_done asserted.
    #[inline(always)]
    pub fn cal_done(&self) -> CalDoneR {
        CalDoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRCCAL1")
            .field("cal_done", &self.cal_done())
            .field("cal_en", &self.cal_en())
            .field("rsvd", &self.rsvd())
            .field("cal_length", &self.cal_length())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Target clk_hxt48 cycles during calibration
    #[inline(always)]
    pub fn cal_length(&mut self) -> CalLengthW<HRCCAL1rs> {
        CalLengthW::new(self, 0)
    }
    ///Bits 16:29
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<HRCCAL1rs> {
        RsvdW::new(self, 16)
    }
    ///Bit 30 - Calibration enble. Set to 0 to clear result, then set to 1 to start a new calibration
    #[inline(always)]
    pub fn cal_en(&mut self) -> CalEnW<HRCCAL1rs> {
        CalEnW::new(self, 30)
    }
    ///Bit 31 - Calibration done. After a new calibration started, results should be processed only when cal_done asserted.
    #[inline(always)]
    pub fn cal_done(&mut self) -> CalDoneW<HRCCAL1rs> {
        CalDoneW::new(self, 31)
    }
}
///HRC Calibration Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`hrccal1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccal1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HRCCAL1rs;
impl crate::RegisterSpec for HRCCAL1rs {
    type Ux = u32;
}
///`read()` method returns [`hrccal1::R`](R) reader structure
impl crate::Readable for HRCCAL1rs {}
///`write(|w| ..)` method takes [`hrccal1::W`](W) writer structure
impl crate::Writable for HRCCAL1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRCCAL1 to value 0
impl crate::Resettable for HRCCAL1rs {
    const RESET_VALUE: u32 = 0;
}
