///Register `CHG_CR4` reader
pub type R = crate::R<CHG_CR4rs>;
///Register `CHG_CR4` writer
pub type W = crate::W<CHG_CR4rs>;
///Field `IE_VBUS_RDY` reader -
pub type IeVbusRdyR = crate::BitReader;
///Field `IE_VBUS_RDY` writer -
pub type IeVbusRdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_VBAT_HIGH` reader -
pub type IeVbatHighR = crate::BitReader;
///Field `IE_VBAT_HIGH` writer -
pub type IeVbatHighW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_ABOVE_REP` reader -
pub type IeAboveRepR = crate::BitReader;
///Field `IE_ABOVE_REP` writer -
pub type IeAboveRepW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_ABOVE_CC` reader -
pub type IeAboveCcR = crate::BitReader;
///Field `IE_ABOVE_CC` writer -
pub type IeAboveCcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_CC_MODE` reader -
pub type IeCcModeR = crate::BitReader;
///Field `IE_CC_MODE` writer -
pub type IeCcModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_CV_MODE` reader -
pub type IeCvModeR = crate::BitReader;
///Field `IE_CV_MODE` writer -
pub type IeCvModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_EOC_MODE` reader -
pub type IeEocModeR = crate::BitReader;
///Field `IE_EOC_MODE` writer -
pub type IeEocModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE_EOC` reader -
pub type IeEocR = crate::BitReader;
///Field `IE_EOC` writer -
pub type IeEocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM_VBUS_RDY` reader - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, others - both edge
pub type ImVbusRdyR = crate::FieldReader;
///Field `IM_VBUS_RDY` writer - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, others - both edge
pub type ImVbusRdyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IM_VBAT_HIGH` reader -
pub type ImVbatHighR = crate::FieldReader;
///Field `IM_VBAT_HIGH` writer -
pub type ImVbatHighW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IM_ABOVE_REP` reader -
pub type ImAboveRepR = crate::FieldReader;
///Field `IM_ABOVE_REP` writer -
pub type ImAboveRepW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IM_ABOVE_CC` reader -
pub type ImAboveCcR = crate::FieldReader;
///Field `IM_ABOVE_CC` writer -
pub type ImAboveCcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IM_CC_MODE` reader -
pub type ImCcModeR = crate::FieldReader;
///Field `IM_CC_MODE` writer -
pub type ImCcModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IM_CV_MODE` reader -
pub type ImCvModeR = crate::FieldReader;
///Field `IM_CV_MODE` writer -
pub type ImCvModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IM_EOC_MODE` reader -
pub type ImEocModeR = crate::FieldReader;
///Field `IM_EOC_MODE` writer -
pub type ImEocModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn ie_vbus_rdy(&self) -> IeVbusRdyR {
        IeVbusRdyR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn ie_vbat_high(&self) -> IeVbatHighR {
        IeVbatHighR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn ie_above_rep(&self) -> IeAboveRepR {
        IeAboveRepR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn ie_above_cc(&self) -> IeAboveCcR {
        IeAboveCcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ie_cc_mode(&self) -> IeCcModeR {
        IeCcModeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ie_cv_mode(&self) -> IeCvModeR {
        IeCvModeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn ie_eoc_mode(&self) -> IeEocModeR {
        IeEocModeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn ie_eoc(&self) -> IeEocR {
        IeEocR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 11:13 - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, others - both edge
    #[inline(always)]
    pub fn im_vbus_rdy(&self) -> ImVbusRdyR {
        ImVbusRdyR::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:16
    #[inline(always)]
    pub fn im_vbat_high(&self) -> ImVbatHighR {
        ImVbatHighR::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn im_above_rep(&self) -> ImAboveRepR {
        ImAboveRepR::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22
    #[inline(always)]
    pub fn im_above_cc(&self) -> ImAboveCcR {
        ImAboveCcR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25
    #[inline(always)]
    pub fn im_cc_mode(&self) -> ImCcModeR {
        ImCcModeR::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:28
    #[inline(always)]
    pub fn im_cv_mode(&self) -> ImCvModeR {
        ImCvModeR::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31
    #[inline(always)]
    pub fn im_eoc_mode(&self) -> ImEocModeR {
        ImEocModeR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHG_CR4")
            .field("im_eoc_mode", &self.im_eoc_mode())
            .field("im_cv_mode", &self.im_cv_mode())
            .field("im_cc_mode", &self.im_cc_mode())
            .field("im_above_cc", &self.im_above_cc())
            .field("im_above_rep", &self.im_above_rep())
            .field("im_vbat_high", &self.im_vbat_high())
            .field("im_vbus_rdy", &self.im_vbus_rdy())
            .field("ie_eoc", &self.ie_eoc())
            .field("ie_eoc_mode", &self.ie_eoc_mode())
            .field("ie_cv_mode", &self.ie_cv_mode())
            .field("ie_cc_mode", &self.ie_cc_mode())
            .field("ie_above_cc", &self.ie_above_cc())
            .field("ie_above_rep", &self.ie_above_rep())
            .field("ie_vbat_high", &self.ie_vbat_high())
            .field("ie_vbus_rdy", &self.ie_vbus_rdy())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn ie_vbus_rdy(&mut self) -> IeVbusRdyW<CHG_CR4rs> {
        IeVbusRdyW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn ie_vbat_high(&mut self) -> IeVbatHighW<CHG_CR4rs> {
        IeVbatHighW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn ie_above_rep(&mut self) -> IeAboveRepW<CHG_CR4rs> {
        IeAboveRepW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn ie_above_cc(&mut self) -> IeAboveCcW<CHG_CR4rs> {
        IeAboveCcW::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ie_cc_mode(&mut self) -> IeCcModeW<CHG_CR4rs> {
        IeCcModeW::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ie_cv_mode(&mut self) -> IeCvModeW<CHG_CR4rs> {
        IeCvModeW::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn ie_eoc_mode(&mut self) -> IeEocModeW<CHG_CR4rs> {
        IeEocModeW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn ie_eoc(&mut self) -> IeEocW<CHG_CR4rs> {
        IeEocW::new(self, 7)
    }
    ///Bits 11:13 - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, others - both edge
    #[inline(always)]
    pub fn im_vbus_rdy(&mut self) -> ImVbusRdyW<CHG_CR4rs> {
        ImVbusRdyW::new(self, 11)
    }
    ///Bits 14:16
    #[inline(always)]
    pub fn im_vbat_high(&mut self) -> ImVbatHighW<CHG_CR4rs> {
        ImVbatHighW::new(self, 14)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn im_above_rep(&mut self) -> ImAboveRepW<CHG_CR4rs> {
        ImAboveRepW::new(self, 17)
    }
    ///Bits 20:22
    #[inline(always)]
    pub fn im_above_cc(&mut self) -> ImAboveCcW<CHG_CR4rs> {
        ImAboveCcW::new(self, 20)
    }
    ///Bits 23:25
    #[inline(always)]
    pub fn im_cc_mode(&mut self) -> ImCcModeW<CHG_CR4rs> {
        ImCcModeW::new(self, 23)
    }
    ///Bits 26:28
    #[inline(always)]
    pub fn im_cv_mode(&mut self) -> ImCvModeW<CHG_CR4rs> {
        ImCvModeW::new(self, 26)
    }
    ///Bits 29:31
    #[inline(always)]
    pub fn im_eoc_mode(&mut self) -> ImEocModeW<CHG_CR4rs> {
        ImEocModeW::new(self, 29)
    }
}
///Charger Control Register 4
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHG_CR4rs;
impl crate::RegisterSpec for CHG_CR4rs {
    type Ux = u32;
}
///`read()` method returns [`chg_cr4::R`](R) reader structure
impl crate::Readable for CHG_CR4rs {}
///`write(|w| ..)` method takes [`chg_cr4::W`](W) writer structure
impl crate::Writable for CHG_CR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHG_CR4 to value 0
impl crate::Resettable for CHG_CR4rs {
    const RESET_VALUE: u32 = 0;
}
