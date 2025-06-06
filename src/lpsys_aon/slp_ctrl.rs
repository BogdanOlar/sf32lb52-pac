///Register `SLP_CTRL` reader
pub type R = crate::R<SLP_CTRLrs>;
///Register `SLP_CTRL` writer
pub type W = crate::W<SLP_CTRLrs>;
///Field `SLEEP_REQ` reader - bt sleep request. Will be cleared automatically
pub type SleepReqR = crate::BitReader;
///Field `SLEEP_REQ` writer - bt sleep request. Will be cleared automatically
pub type SleepReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUP_REQ` reader - software request to wakeup bt. Will be cleared automatically
pub type WkupReqR = crate::BitReader;
///Field `WKUP_REQ` writer - software request to wakeup bt. Will be cleared automatically
pub type WkupReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SLEEP_STATUS` reader - bt sleep status. 1 means bt is sleeping and sleep_cnt is counting up
pub type SleepStatusR = crate::BitReader;
///Field `SLEEP_STATUS` writer - bt sleep status. 1 means bt is sleeping and sleep_cnt is counting up
pub type SleepStatusW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTAL_REQ` reader - xtal request status. 1 means bt is requiring xtal.
pub type XtalReqR = crate::BitReader;
///Field `XTAL_REQ` writer - xtal request status. 1 means bt is requiring xtal.
pub type XtalReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_WKUP` reader - bt wakeup source. 1 means bt has not enter sleep or has enter wakeup procedure
pub type BtWkupR = crate::BitReader;
///Field `BT_WKUP` writer - bt wakeup source. 1 means bt has not enter sleep or has enter wakeup procedure
pub type BtWkupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bit 0 - bt sleep request. Will be cleared automatically
    #[inline(always)]
    pub fn sleep_req(&self) -> SleepReqR {
        SleepReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - software request to wakeup bt. Will be cleared automatically
    #[inline(always)]
    pub fn wkup_req(&self) -> WkupReqR {
        WkupReqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - bt sleep status. 1 means bt is sleeping and sleep_cnt is counting up
    #[inline(always)]
    pub fn sleep_status(&self) -> SleepStatusR {
        SleepStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - xtal request status. 1 means bt is requiring xtal.
    #[inline(always)]
    pub fn xtal_req(&self) -> XtalReqR {
        XtalReqR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - bt wakeup source. 1 means bt has not enter sleep or has enter wakeup procedure
    #[inline(always)]
    pub fn bt_wkup(&self) -> BtWkupR {
        BtWkupR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_CTRL")
            .field("rsvd", &self.rsvd())
            .field("bt_wkup", &self.bt_wkup())
            .field("xtal_req", &self.xtal_req())
            .field("sleep_status", &self.sleep_status())
            .field("rsvd2", &self.rsvd2())
            .field("wkup_req", &self.wkup_req())
            .field("sleep_req", &self.sleep_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - bt sleep request. Will be cleared automatically
    #[inline(always)]
    pub fn sleep_req(&mut self) -> SleepReqW<SLP_CTRLrs> {
        SleepReqW::new(self, 0)
    }
    ///Bit 1 - software request to wakeup bt. Will be cleared automatically
    #[inline(always)]
    pub fn wkup_req(&mut self) -> WkupReqW<SLP_CTRLrs> {
        WkupReqW::new(self, 1)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<SLP_CTRLrs> {
        Rsvd2W::new(self, 2)
    }
    ///Bit 4 - bt sleep status. 1 means bt is sleeping and sleep_cnt is counting up
    #[inline(always)]
    pub fn sleep_status(&mut self) -> SleepStatusW<SLP_CTRLrs> {
        SleepStatusW::new(self, 4)
    }
    ///Bit 5 - xtal request status. 1 means bt is requiring xtal.
    #[inline(always)]
    pub fn xtal_req(&mut self) -> XtalReqW<SLP_CTRLrs> {
        XtalReqW::new(self, 5)
    }
    ///Bit 6 - bt wakeup source. 1 means bt has not enter sleep or has enter wakeup procedure
    #[inline(always)]
    pub fn bt_wkup(&mut self) -> BtWkupW<SLP_CTRLrs> {
        BtWkupW::new(self, 6)
    }
    ///Bits 7:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SLP_CTRLrs> {
        RsvdW::new(self, 7)
    }
}
///BT sleep control
///
///You can [`read`](crate::Reg::read) this register and get [`slp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SLP_CTRLrs;
impl crate::RegisterSpec for SLP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`slp_ctrl::R`](R) reader structure
impl crate::Readable for SLP_CTRLrs {}
///`write(|w| ..)` method takes [`slp_ctrl::W`](W) writer structure
impl crate::Writable for SLP_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_CTRL to value 0
impl crate::Resettable for SLP_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
