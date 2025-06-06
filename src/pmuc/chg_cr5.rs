///Register `CHG_CR5` reader
pub type R = crate::R<CHG_CR5rs>;
///Register `CHG_CR5` writer
pub type W = crate::W<CHG_CR5rs>;
///Field `IC_VBUS_RDY` reader -
pub type IcVbusRdyR = crate::BitReader;
///Field `IC_VBUS_RDY` writer -
pub type IcVbusRdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_VBAT_HIGH` reader -
pub type IcVbatHighR = crate::BitReader;
///Field `IC_VBAT_HIGH` writer -
pub type IcVbatHighW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_ABOVE_REP` reader -
pub type IcAboveRepR = crate::BitReader;
///Field `IC_ABOVE_REP` writer -
pub type IcAboveRepW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_ABOVE_CC` reader -
pub type IcAboveCcR = crate::BitReader;
///Field `IC_ABOVE_CC` writer -
pub type IcAboveCcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_CC_MODE` reader -
pub type IcCcModeR = crate::BitReader;
///Field `IC_CC_MODE` writer -
pub type IcCcModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_CV_MODE` reader -
pub type IcCvModeR = crate::BitReader;
///Field `IC_CV_MODE` writer -
pub type IcCvModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_EOC_MODE` reader -
pub type IcEocModeR = crate::BitReader;
///Field `IC_EOC_MODE` writer -
pub type IcEocModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_EOC` reader -
pub type IcEocR = crate::BitReader;
///Field `IC_EOC` writer -
pub type IcEocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_VBUS_RDY` reader -
pub type IsVbusRdyR = crate::BitReader;
///Field `IS_VBUS_RDY` writer -
pub type IsVbusRdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_VBAT_HIGH` reader -
pub type IsVbatHighR = crate::BitReader;
///Field `IS_VBAT_HIGH` writer -
pub type IsVbatHighW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_ABOVE_REP` reader -
pub type IsAboveRepR = crate::BitReader;
///Field `IS_ABOVE_REP` writer -
pub type IsAboveRepW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_ABOVE_CC` reader -
pub type IsAboveCcR = crate::BitReader;
///Field `IS_ABOVE_CC` writer -
pub type IsAboveCcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_CC_MODE` reader -
pub type IsCcModeR = crate::BitReader;
///Field `IS_CC_MODE` writer -
pub type IsCcModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_CV_MODE` reader -
pub type IsCvModeR = crate::BitReader;
///Field `IS_CV_MODE` writer -
pub type IsCvModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_EOC_MODE` reader -
pub type IsEocModeR = crate::BitReader;
///Field `IS_EOC_MODE` writer -
pub type IsEocModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS_EOC` reader -
pub type IsEocR = crate::BitReader;
///Field `IS_EOC` writer -
pub type IsEocW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn ic_vbus_rdy(&self) -> IcVbusRdyR {
        IcVbusRdyR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn ic_vbat_high(&self) -> IcVbatHighR {
        IcVbatHighR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn ic_above_rep(&self) -> IcAboveRepR {
        IcAboveRepR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn ic_above_cc(&self) -> IcAboveCcR {
        IcAboveCcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ic_cc_mode(&self) -> IcCcModeR {
        IcCcModeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ic_cv_mode(&self) -> IcCvModeR {
        IcCvModeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn ic_eoc_mode(&self) -> IcEocModeR {
        IcEocModeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn ic_eoc(&self) -> IcEocR {
        IcEocR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn is_vbus_rdy(&self) -> IsVbusRdyR {
        IsVbusRdyR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn is_vbat_high(&self) -> IsVbatHighR {
        IsVbatHighR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn is_above_rep(&self) -> IsAboveRepR {
        IsAboveRepR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn is_above_cc(&self) -> IsAboveCcR {
        IsAboveCcR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn is_cc_mode(&self) -> IsCcModeR {
        IsCcModeR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn is_cv_mode(&self) -> IsCvModeR {
        IsCvModeR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn is_eoc_mode(&self) -> IsEocModeR {
        IsEocModeR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23
    #[inline(always)]
    pub fn is_eoc(&self) -> IsEocR {
        IsEocR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHG_CR5")
            .field("is_eoc", &self.is_eoc())
            .field("is_eoc_mode", &self.is_eoc_mode())
            .field("is_cv_mode", &self.is_cv_mode())
            .field("is_cc_mode", &self.is_cc_mode())
            .field("is_above_cc", &self.is_above_cc())
            .field("is_above_rep", &self.is_above_rep())
            .field("is_vbat_high", &self.is_vbat_high())
            .field("is_vbus_rdy", &self.is_vbus_rdy())
            .field("ic_eoc", &self.ic_eoc())
            .field("ic_eoc_mode", &self.ic_eoc_mode())
            .field("ic_cv_mode", &self.ic_cv_mode())
            .field("ic_cc_mode", &self.ic_cc_mode())
            .field("ic_above_cc", &self.ic_above_cc())
            .field("ic_above_rep", &self.ic_above_rep())
            .field("ic_vbat_high", &self.ic_vbat_high())
            .field("ic_vbus_rdy", &self.ic_vbus_rdy())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn ic_vbus_rdy(&mut self) -> IcVbusRdyW<CHG_CR5rs> {
        IcVbusRdyW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn ic_vbat_high(&mut self) -> IcVbatHighW<CHG_CR5rs> {
        IcVbatHighW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn ic_above_rep(&mut self) -> IcAboveRepW<CHG_CR5rs> {
        IcAboveRepW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn ic_above_cc(&mut self) -> IcAboveCcW<CHG_CR5rs> {
        IcAboveCcW::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ic_cc_mode(&mut self) -> IcCcModeW<CHG_CR5rs> {
        IcCcModeW::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ic_cv_mode(&mut self) -> IcCvModeW<CHG_CR5rs> {
        IcCvModeW::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn ic_eoc_mode(&mut self) -> IcEocModeW<CHG_CR5rs> {
        IcEocModeW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn ic_eoc(&mut self) -> IcEocW<CHG_CR5rs> {
        IcEocW::new(self, 7)
    }
    ///Bit 16
    #[inline(always)]
    pub fn is_vbus_rdy(&mut self) -> IsVbusRdyW<CHG_CR5rs> {
        IsVbusRdyW::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn is_vbat_high(&mut self) -> IsVbatHighW<CHG_CR5rs> {
        IsVbatHighW::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn is_above_rep(&mut self) -> IsAboveRepW<CHG_CR5rs> {
        IsAboveRepW::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn is_above_cc(&mut self) -> IsAboveCcW<CHG_CR5rs> {
        IsAboveCcW::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn is_cc_mode(&mut self) -> IsCcModeW<CHG_CR5rs> {
        IsCcModeW::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn is_cv_mode(&mut self) -> IsCvModeW<CHG_CR5rs> {
        IsCvModeW::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    pub fn is_eoc_mode(&mut self) -> IsEocModeW<CHG_CR5rs> {
        IsEocModeW::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    pub fn is_eoc(&mut self) -> IsEocW<CHG_CR5rs> {
        IsEocW::new(self, 23)
    }
}
///Charger Control Register 5
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHG_CR5rs;
impl crate::RegisterSpec for CHG_CR5rs {
    type Ux = u32;
}
///`read()` method returns [`chg_cr5::R`](R) reader structure
impl crate::Readable for CHG_CR5rs {}
///`write(|w| ..)` method takes [`chg_cr5::W`](W) writer structure
impl crate::Writable for CHG_CR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHG_CR5 to value 0
impl crate::Resettable for CHG_CR5rs {
    const RESET_VALUE: u32 = 0;
}
