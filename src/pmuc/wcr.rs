///Register `WCR` reader
pub type R = crate::R<WCRrs>;
///Register `WCR` writer
pub type W = crate::W<WCRrs>;
///Field `RSVD3` reader - Clear status in RTC
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer - Clear status in RTC
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT1` reader - Write 1 to clear WDT1 reboot flag
pub type Wdt1R = crate::BitReader;
///Field `WDT1` writer - Write 1 to clear WDT1 reboot flag
pub type Wdt1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT2` reader - Write 1 to clear WDT2 reboot flag
pub type Wdt2R = crate::BitReader;
///Field `WDT2` writer - Write 1 to clear WDT2 reboot flag
pub type Wdt2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN0` reader - Write 1 to clear PIN0 wakeup flag. Only valid if PIN wakeup is configured as edge trigger
pub type Pin0R = crate::BitReader;
///Field `PIN0` writer - Write 1 to clear PIN0 wakeup flag. Only valid if PIN wakeup is configured as edge trigger
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN1` reader - Write 1 to clear PIN1 wakeup flag.
pub type Pin1R = crate::BitReader;
///Field `PIN1` writer - Write 1 to clear PIN1 wakeup flag.
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader - Clear status in IWDT
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer - Clear status in IWDT
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRKEY` reader - Write 1 to clear PWRKEY reset flag
pub type PwrkeyR = crate::BitReader;
///Field `PWRKEY` writer - Write 1 to clear PWRKEY reset flag
pub type PwrkeyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOWBAT` reader - Write 1 to clear LOWBAT flag
pub type LowbatR = crate::BitReader;
///Field `LOWBAT` writer - Write 1 to clear LOWBAT flag
pub type LowbatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
///Field `AON` reader - Write 1 to clear the AON wakeup IRQ status
pub type AonR = crate::BitReader;
///Field `AON` writer - Write 1 to clear the AON wakeup IRQ status
pub type AonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear status in RTC
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write 1 to clear WDT1 reboot flag
    #[inline(always)]
    pub fn wdt1(&self) -> Wdt1R {
        Wdt1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write 1 to clear WDT2 reboot flag
    #[inline(always)]
    pub fn wdt2(&self) -> Wdt2R {
        Wdt2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Write 1 to clear PIN0 wakeup flag. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Write 1 to clear PIN1 wakeup flag.
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear status in IWDT
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Write 1 to clear PWRKEY reset flag
    #[inline(always)]
    pub fn pwrkey(&self) -> PwrkeyR {
        PwrkeyR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Write 1 to clear LOWBAT flag
    #[inline(always)]
    pub fn lowbat(&self) -> LowbatR {
        LowbatR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x007f_ffff)
    }
    ///Bit 31 - Write 1 to clear the AON wakeup IRQ status
    #[inline(always)]
    pub fn aon(&self) -> AonR {
        AonR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCR")
            .field("aon", &self.aon())
            .field("rsvd", &self.rsvd())
            .field("lowbat", &self.lowbat())
            .field("pwrkey", &self.pwrkey())
            .field("rsvd2", &self.rsvd2())
            .field("pin1", &self.pin1())
            .field("pin0", &self.pin0())
            .field("wdt2", &self.wdt2())
            .field("wdt1", &self.wdt1())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear status in RTC
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<WCRrs> {
        Rsvd3W::new(self, 0)
    }
    ///Bit 1 - Write 1 to clear WDT1 reboot flag
    #[inline(always)]
    pub fn wdt1(&mut self) -> Wdt1W<WCRrs> {
        Wdt1W::new(self, 1)
    }
    ///Bit 2 - Write 1 to clear WDT2 reboot flag
    #[inline(always)]
    pub fn wdt2(&mut self) -> Wdt2W<WCRrs> {
        Wdt2W::new(self, 2)
    }
    ///Bit 3 - Write 1 to clear PIN0 wakeup flag. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<WCRrs> {
        Pin0W::new(self, 3)
    }
    ///Bit 4 - Write 1 to clear PIN1 wakeup flag.
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<WCRrs> {
        Pin1W::new(self, 4)
    }
    ///Bit 5 - Clear status in IWDT
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<WCRrs> {
        Rsvd2W::new(self, 5)
    }
    ///Bit 6 - Write 1 to clear PWRKEY reset flag
    #[inline(always)]
    pub fn pwrkey(&mut self) -> PwrkeyW<WCRrs> {
        PwrkeyW::new(self, 6)
    }
    ///Bit 7 - Write 1 to clear LOWBAT flag
    #[inline(always)]
    pub fn lowbat(&mut self) -> LowbatW<WCRrs> {
        LowbatW::new(self, 7)
    }
    ///Bits 8:30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WCRrs> {
        RsvdW::new(self, 8)
    }
    ///Bit 31 - Write 1 to clear the AON wakeup IRQ status
    #[inline(always)]
    pub fn aon(&mut self) -> AonW<WCRrs> {
        AonW::new(self, 31)
    }
}
///Wakeup Clear register
///
///You can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WCRrs;
impl crate::RegisterSpec for WCRrs {
    type Ux = u32;
}
///`read()` method returns [`wcr::R`](R) reader structure
impl crate::Readable for WCRrs {}
///`write(|w| ..)` method takes [`wcr::W`](W) writer structure
impl crate::Writable for WCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WCR to value 0
impl crate::Resettable for WCRrs {
    const RESET_VALUE: u32 = 0;
}
