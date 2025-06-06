///Register `CHG_CR1` reader
pub type R = crate::R<CHG_CR1rs>;
///Register `CHG_CR1` writer
pub type W = crate::W<CHG_CR1rs>;
///Field `EN` reader - only available when CR3 FORCE_CTRL bit is set
pub type EnR = crate::BitReader;
///Field `EN` writer - only available when CR3 FORCE_CTRL bit is set
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOP_EN` reader - only available when CR3 FORCE_CTRL bit is set
pub type LoopEnR = crate::BitReader;
///Field `LOOP_EN` writer - only available when CR3 FORCE_CTRL bit is set
pub type LoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC_ICTRL` reader -
pub type CcIctrlR = crate::FieldReader;
///Field `CC_ICTRL` writer -
pub type CcIctrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CC_VCTRL` reader -
pub type CcVctrlR = crate::FieldReader;
///Field `CC_VCTRL` writer -
pub type CcVctrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CC_MP` reader -
pub type CcMpR = crate::FieldReader;
///Field `CC_MP` writer -
pub type CcMpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CC_MN` reader -
pub type CcMnR = crate::FieldReader;
///Field `CC_MN` writer -
pub type CcMnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CC_RANGE` reader -
pub type CcRangeR = crate::FieldReader;
///Field `CC_RANGE` writer -
pub type CcRangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CV_VCTRL` reader -
pub type CvVctrlR = crate::FieldReader;
///Field `CV_VCTRL` writer -
pub type CvVctrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - only available when CR3 FORCE_CTRL bit is set
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - only available when CR3 FORCE_CTRL bit is set
    #[inline(always)]
    pub fn loop_en(&self) -> LoopEnR {
        LoopEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn cc_ictrl(&self) -> CcIctrlR {
        CcIctrlR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    ///Bits 8:13
    #[inline(always)]
    pub fn cc_vctrl(&self) -> CcVctrlR {
        CcVctrlR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:18
    #[inline(always)]
    pub fn cc_mp(&self) -> CcMpR {
        CcMpR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    ///Bits 19:23
    #[inline(always)]
    pub fn cc_mn(&self) -> CcMnR {
        CcMnR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:25
    #[inline(always)]
    pub fn cc_range(&self) -> CcRangeR {
        CcRangeR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn cv_vctrl(&self) -> CvVctrlR {
        CvVctrlR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHG_CR1")
            .field("cv_vctrl", &self.cv_vctrl())
            .field("cc_range", &self.cc_range())
            .field("cc_mn", &self.cc_mn())
            .field("cc_mp", &self.cc_mp())
            .field("cc_vctrl", &self.cc_vctrl())
            .field("cc_ictrl", &self.cc_ictrl())
            .field("loop_en", &self.loop_en())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - only available when CR3 FORCE_CTRL bit is set
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CHG_CR1rs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - only available when CR3 FORCE_CTRL bit is set
    #[inline(always)]
    pub fn loop_en(&mut self) -> LoopEnW<CHG_CR1rs> {
        LoopEnW::new(self, 1)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn cc_ictrl(&mut self) -> CcIctrlW<CHG_CR1rs> {
        CcIctrlW::new(self, 2)
    }
    ///Bits 8:13
    #[inline(always)]
    pub fn cc_vctrl(&mut self) -> CcVctrlW<CHG_CR1rs> {
        CcVctrlW::new(self, 8)
    }
    ///Bits 14:18
    #[inline(always)]
    pub fn cc_mp(&mut self) -> CcMpW<CHG_CR1rs> {
        CcMpW::new(self, 14)
    }
    ///Bits 19:23
    #[inline(always)]
    pub fn cc_mn(&mut self) -> CcMnW<CHG_CR1rs> {
        CcMnW::new(self, 19)
    }
    ///Bits 24:25
    #[inline(always)]
    pub fn cc_range(&mut self) -> CcRangeW<CHG_CR1rs> {
        CcRangeW::new(self, 24)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn cv_vctrl(&mut self) -> CvVctrlW<CHG_CR1rs> {
        CvVctrlW::new(self, 26)
    }
}
///Charger Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHG_CR1rs;
impl crate::RegisterSpec for CHG_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`chg_cr1::R`](R) reader structure
impl crate::Readable for CHG_CR1rs {}
///`write(|w| ..)` method takes [`chg_cr1::W`](W) writer structure
impl crate::Writable for CHG_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHG_CR1 to value 0
impl crate::Resettable for CHG_CR1rs {
    const RESET_VALUE: u32 = 0;
}
