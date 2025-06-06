///Register `CHG_SR` reader
pub type R = crate::R<CHG_SRrs>;
///Register `CHG_SR` writer
pub type W = crate::W<CHG_SRrs>;
///Field `VBUS_RDY_OUT` reader -
pub type VbusRdyOutR = crate::BitReader;
///Field `VBUS_RDY_OUT` writer -
pub type VbusRdyOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBAT_HIGH_OUT` reader -
pub type VbatHighOutR = crate::BitReader;
///Field `VBAT_HIGH_OUT` writer -
pub type VbatHighOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBAT_ABOVE_REP_OUT` reader -
pub type VbatAboveRepOutR = crate::BitReader;
///Field `VBAT_ABOVE_REP_OUT` writer -
pub type VbatAboveRepOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBAT_ABOVE_CC_OUT` reader -
pub type VbatAboveCcOutR = crate::BitReader;
///Field `VBAT_ABOVE_CC_OUT` writer -
pub type VbatAboveCcOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC_MODE` reader -
pub type CcModeR = crate::BitReader;
///Field `CC_MODE` writer -
pub type CcModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CV_MODE` reader -
pub type CvModeR = crate::BitReader;
///Field `CV_MODE` writer -
pub type CvModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOC_MODE` reader -
pub type EocModeR = crate::BitReader;
///Field `EOC_MODE` writer -
pub type EocModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHG_STATE` reader - Charger finite state machine
pub type ChgStateR = crate::FieldReader;
///Field `CHG_STATE` writer - Charger finite state machine
pub type ChgStateW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn vbus_rdy_out(&self) -> VbusRdyOutR {
        VbusRdyOutR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn vbat_high_out(&self) -> VbatHighOutR {
        VbatHighOutR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn vbat_above_rep_out(&self) -> VbatAboveRepOutR {
        VbatAboveRepOutR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn vbat_above_cc_out(&self) -> VbatAboveCcOutR {
        VbatAboveCcOutR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn cc_mode(&self) -> CcModeR {
        CcModeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn cv_mode(&self) -> CvModeR {
        CvModeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn eoc_mode(&self) -> EocModeR {
        EocModeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:14 - Charger finite state machine
    #[inline(always)]
    pub fn chg_state(&self) -> ChgStateR {
        ChgStateR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 15:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHG_SR")
            .field("rsvd", &self.rsvd())
            .field("chg_state", &self.chg_state())
            .field("rsvd2", &self.rsvd2())
            .field("eoc_mode", &self.eoc_mode())
            .field("cv_mode", &self.cv_mode())
            .field("cc_mode", &self.cc_mode())
            .field("vbat_above_cc_out", &self.vbat_above_cc_out())
            .field("vbat_above_rep_out", &self.vbat_above_rep_out())
            .field("vbat_high_out", &self.vbat_high_out())
            .field("vbus_rdy_out", &self.vbus_rdy_out())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn vbus_rdy_out(&mut self) -> VbusRdyOutW<CHG_SRrs> {
        VbusRdyOutW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn vbat_high_out(&mut self) -> VbatHighOutW<CHG_SRrs> {
        VbatHighOutW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn vbat_above_rep_out(&mut self) -> VbatAboveRepOutW<CHG_SRrs> {
        VbatAboveRepOutW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn vbat_above_cc_out(&mut self) -> VbatAboveCcOutW<CHG_SRrs> {
        VbatAboveCcOutW::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn cc_mode(&mut self) -> CcModeW<CHG_SRrs> {
        CcModeW::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn cv_mode(&mut self) -> CvModeW<CHG_SRrs> {
        CvModeW::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn eoc_mode(&mut self) -> EocModeW<CHG_SRrs> {
        EocModeW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CHG_SRrs> {
        Rsvd2W::new(self, 7)
    }
    ///Bits 8:14 - Charger finite state machine
    #[inline(always)]
    pub fn chg_state(&mut self) -> ChgStateW<CHG_SRrs> {
        ChgStateW::new(self, 8)
    }
    ///Bits 15:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CHG_SRrs> {
        RsvdW::new(self, 15)
    }
}
///Charger Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`chg_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHG_SRrs;
impl crate::RegisterSpec for CHG_SRrs {
    type Ux = u32;
}
///`read()` method returns [`chg_sr::R`](R) reader structure
impl crate::Readable for CHG_SRrs {}
///`write(|w| ..)` method takes [`chg_sr::W`](W) writer structure
impl crate::Writable for CHG_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHG_SR to value 0
impl crate::Resettable for CHG_SRrs {
    const RESET_VALUE: u32 = 0;
}
