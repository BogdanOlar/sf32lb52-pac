///Register `WER` reader
pub type R = crate::R<WERrs>;
///Register `WER` writer
pub type W = crate::W<WERrs>;
///Field `RTC` reader - Set 1 to enable RTC as wakeup source
pub type RtcR = crate::BitReader;
///Field `RTC` writer - Set 1 to enable RTC as wakeup source
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT1` reader - Set 1 to enable WDT1 as reboot cause
pub type Wdt1R = crate::BitReader;
///Field `WDT1` writer - Set 1 to enable WDT1 as reboot cause
pub type Wdt1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT2` reader - Set 1 to enable WDT2 as reboot cause
pub type Wdt2R = crate::BitReader;
///Field `WDT2` writer - Set 1 to enable WDT2 as reboot cause
pub type Wdt2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN0` reader - Set 1 to enable PIN0 as wakeup source
pub type Pin0R = crate::BitReader;
///Field `PIN0` writer - Set 1 to enable PIN0 as wakeup source
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN1` reader - Set 1 to enable PIN1 as wakeup source
pub type Pin1R = crate::BitReader;
///Field `PIN1` writer - Set 1 to enable PIN1 as wakeup source
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LOWBAT` reader - If enabled, auto shut down upon battery low; and will power up if battery ready
pub type LowbatR = crate::BitReader;
///Field `LOWBAT` writer - If enabled, auto shut down upon battery low; and will power up if battery ready
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
    ///Bit 0 - Set 1 to enable RTC as wakeup source
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to enable WDT1 as reboot cause
    #[inline(always)]
    pub fn wdt1(&self) -> Wdt1R {
        Wdt1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set 1 to enable WDT2 as reboot cause
    #[inline(always)]
    pub fn wdt2(&self) -> Wdt2R {
        Wdt2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to enable PIN0 as wakeup source
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set 1 to enable PIN1 as wakeup source
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - If enabled, auto shut down upon battery low; and will power up if battery ready
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
        f.debug_struct("WER")
            .field("rsvd", &self.rsvd())
            .field("chg", &self.chg())
            .field("lowbat", &self.lowbat())
            .field("rsvd2", &self.rsvd2())
            .field("pin1", &self.pin1())
            .field("pin0", &self.pin0())
            .field("wdt2", &self.wdt2())
            .field("wdt1", &self.wdt1())
            .field("rtc", &self.rtc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable RTC as wakeup source
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<WERrs> {
        RtcW::new(self, 0)
    }
    ///Bit 1 - Set 1 to enable WDT1 as reboot cause
    #[inline(always)]
    pub fn wdt1(&mut self) -> Wdt1W<WERrs> {
        Wdt1W::new(self, 1)
    }
    ///Bit 2 - Set 1 to enable WDT2 as reboot cause
    #[inline(always)]
    pub fn wdt2(&mut self) -> Wdt2W<WERrs> {
        Wdt2W::new(self, 2)
    }
    ///Bit 3 - Set 1 to enable PIN0 as wakeup source
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<WERrs> {
        Pin0W::new(self, 3)
    }
    ///Bit 4 - Set 1 to enable PIN1 as wakeup source
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<WERrs> {
        Pin1W::new(self, 4)
    }
    ///Bits 5:6
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<WERrs> {
        Rsvd2W::new(self, 5)
    }
    ///Bit 7 - If enabled, auto shut down upon battery low; and will power up if battery ready
    #[inline(always)]
    pub fn lowbat(&mut self) -> LowbatW<WERrs> {
        LowbatW::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn chg(&mut self) -> ChgW<WERrs> {
        ChgW::new(self, 8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WERrs> {
        RsvdW::new(self, 9)
    }
}
///Wakeup Enable register
///
///You can [`read`](crate::Reg::read) this register and get [`wer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WERrs;
impl crate::RegisterSpec for WERrs {
    type Ux = u32;
}
///`read()` method returns [`wer::R`](R) reader structure
impl crate::Readable for WERrs {}
///`write(|w| ..)` method takes [`wer::W`](W) writer structure
impl crate::Writable for WERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WER to value 0
impl crate::Resettable for WERrs {
    const RESET_VALUE: u32 = 0;
}
