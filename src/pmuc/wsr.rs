///Register `WSR` reader
pub type R = crate::R<WSRrs>;
///Register `WSR` writer
pub type W = crate::W<WSRrs>;
///Field `RTC` reader - Indicates the wakeup status from RTC. Note: the status is masked by WER
pub type RtcR = crate::BitReader;
///Field `RTC` writer - Indicates the wakeup status from RTC. Note: the status is masked by WER
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT1` reader - Indicates reboot by WDT1
pub type Wdt1R = crate::BitReader;
///Field `WDT1` writer - Indicates reboot by WDT1
pub type Wdt1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT2` reader - Indicates reboot by WDT2
pub type Wdt2R = crate::BitReader;
///Field `WDT2` writer - Indicates reboot by WDT2
pub type Wdt2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN0` reader -
pub type Pin0R = crate::BitReader;
///Field `PIN0` writer -
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN1` reader -
pub type Pin1R = crate::BitReader;
///Field `PIN1` writer -
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDT` reader -
pub type IwdtR = crate::BitReader;
///Field `IWDT` writer -
pub type IwdtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRKEY` reader -
pub type PwrkeyR = crate::BitReader;
///Field `PWRKEY` writer -
pub type PwrkeyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOWBAT` reader - Indicates auto reboot due to battery low
pub type LowbatR = crate::BitReader;
///Field `LOWBAT` writer - Indicates auto reboot due to battery low
pub type LowbatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHG` reader -
pub type ChgR = crate::BitReader;
///Field `CHG` writer -
pub type ChgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bit 0 - Indicates the wakeup status from RTC. Note: the status is masked by WER
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indicates reboot by WDT1
    #[inline(always)]
    pub fn wdt1(&self) -> Wdt1R {
        Wdt1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Indicates reboot by WDT2
    #[inline(always)]
    pub fn wdt2(&self) -> Wdt2R {
        Wdt2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn iwdt(&self) -> IwdtR {
        IwdtR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn pwrkey(&self) -> PwrkeyR {
        PwrkeyR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Indicates auto reboot due to battery low
    #[inline(always)]
    pub fn lowbat(&self) -> LowbatR {
        LowbatR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn chg(&self) -> ChgR {
        ChgR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WSR")
            .field("rsvd", &self.rsvd())
            .field("chg", &self.chg())
            .field("lowbat", &self.lowbat())
            .field("pwrkey", &self.pwrkey())
            .field("iwdt", &self.iwdt())
            .field("pin1", &self.pin1())
            .field("pin0", &self.pin0())
            .field("wdt2", &self.wdt2())
            .field("wdt1", &self.wdt1())
            .field("rtc", &self.rtc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Indicates the wakeup status from RTC. Note: the status is masked by WER
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<WSRrs> {
        RtcW::new(self, 0)
    }
    ///Bit 1 - Indicates reboot by WDT1
    #[inline(always)]
    pub fn wdt1(&mut self) -> Wdt1W<WSRrs> {
        Wdt1W::new(self, 1)
    }
    ///Bit 2 - Indicates reboot by WDT2
    #[inline(always)]
    pub fn wdt2(&mut self) -> Wdt2W<WSRrs> {
        Wdt2W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<WSRrs> {
        Pin0W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<WSRrs> {
        Pin1W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn iwdt(&mut self) -> IwdtW<WSRrs> {
        IwdtW::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn pwrkey(&mut self) -> PwrkeyW<WSRrs> {
        PwrkeyW::new(self, 6)
    }
    ///Bit 7 - Indicates auto reboot due to battery low
    #[inline(always)]
    pub fn lowbat(&mut self) -> LowbatW<WSRrs> {
        LowbatW::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn chg(&mut self) -> ChgW<WSRrs> {
        ChgW::new(self, 8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WSRrs> {
        RsvdW::new(self, 9)
    }
}
///Wakeup Status register
///
///You can [`read`](crate::Reg::read) this register and get [`wsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WSRrs;
impl crate::RegisterSpec for WSRrs {
    type Ux = u32;
}
///`read()` method returns [`wsr::R`](R) reader structure
impl crate::Readable for WSRrs {}
///`write(|w| ..)` method takes [`wsr::W`](W) writer structure
impl crate::Writable for WSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WSR to value 0
impl crate::Resettable for WSRrs {
    const RESET_VALUE: u32 = 0;
}
