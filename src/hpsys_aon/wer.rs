///Register `WER` reader
pub type R = crate::R<WERrs>;
///Register `WER` writer
pub type W = crate::W<WERrs>;
///Field `RTC` reader - Set 1 to enable RTC as wakeup source
pub type RtcR = crate::BitReader;
///Field `RTC` writer - Set 1 to enable RTC as wakeup source
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO1` reader - Set 1 to enable IO(PA) as wakeup source
pub type Gpio1R = crate::BitReader;
///Field `GPIO1` writer - Set 1 to enable IO(PA) as wakeup source
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1` reader - Set 1 to enable LPTIM1 as wakeup source
pub type Lptim1R = crate::BitReader;
///Field `LPTIM1` writer - Set 1 to enable LPTIM1 as wakeup source
pub type Lptim1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMUC` reader - Set 1 to enable PMUC as wakeup source
pub type PmucR = crate::BitReader;
///Field `PMUC` writer - Set 1 to enable PMUC as wakeup source
pub type PmucW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_REQ` reader - Set 1 to enable LPSYS request as wakeup source
pub type Lp2hpReqR = crate::BitReader;
///Field `LP2HP_REQ` writer - Set 1 to enable LPSYS request as wakeup source
pub type Lp2hpReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_IRQ` reader - Set 1 to enable MAILBOX2 as wakeup source
pub type Lp2hpIrqR = crate::BitReader;
///Field `LP2HP_IRQ` writer - Set 1 to enable MAILBOX2 as wakeup source
pub type Lp2hpIrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN0` reader - Set 1 to enable PA24 as wakeup source
pub type Pin0R = crate::BitReader;
///Field `PIN0` writer - Set 1 to enable PA24 as wakeup source
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN1` reader - Set 1 to enable PA25 as wakeup source
pub type Pin1R = crate::BitReader;
///Field `PIN1` writer - Set 1 to enable PA25 as wakeup source
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN2` reader - Set 1 to enable PA26 as wakeup source
pub type Pin2R = crate::BitReader;
///Field `PIN2` writer - Set 1 to enable PA26 as wakeup source
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN3` reader - Set 1 to enable PA27 as wakeup source
pub type Pin3R = crate::BitReader;
///Field `PIN3` writer - Set 1 to enable PA27 as wakeup source
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN10` reader - Set 1 to enable PA34 as wakeup source
pub type Pin10R = crate::BitReader;
///Field `PIN10` writer - Set 1 to enable PA34 as wakeup source
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN11` reader - Set 1 to enable PA35 as wakeup source
pub type Pin11R = crate::BitReader;
///Field `PIN11` writer - Set 1 to enable PA35 as wakeup source
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN12` reader - Set 1 to enable PA36 as wakeup source
pub type Pin12R = crate::BitReader;
///Field `PIN12` writer - Set 1 to enable PA36 as wakeup source
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN13` reader - Set 1 to enable PA37 as wakeup source
pub type Pin13R = crate::BitReader;
///Field `PIN13` writer - Set 1 to enable PA37 as wakeup source
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN14` reader - Set 1 to enable PA38 as wakeup source
pub type Pin14R = crate::BitReader;
///Field `PIN14` writer - Set 1 to enable PA38 as wakeup source
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN15` reader - Set 1 to enable PA39 as wakeup source
pub type Pin15R = crate::BitReader;
///Field `PIN15` writer - Set 1 to enable PA39 as wakeup source
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN16` reader - Set 1 to enable PA40 as wakeup source
pub type Pin16R = crate::BitReader;
///Field `PIN16` writer - Set 1 to enable PA40 as wakeup source
pub type Pin16W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN17` reader - Set 1 to enable PA41 as wakeup source
pub type Pin17R = crate::BitReader;
///Field `PIN17` writer - Set 1 to enable PA41 as wakeup source
pub type Pin17W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN18` reader - Set 1 to enable PA42 as wakeup source
pub type Pin18R = crate::BitReader;
///Field `PIN18` writer - Set 1 to enable PA42 as wakeup source
pub type Pin18W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN19` reader - Set 1 to enable PA43 as wakeup source
pub type Pin19R = crate::BitReader;
///Field `PIN19` writer - Set 1 to enable PA43 as wakeup source
pub type Pin19W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN20` reader - Set 1 to enable PA44 as wakeup source
pub type Pin20R = crate::BitReader;
///Field `PIN20` writer - Set 1 to enable PA44 as wakeup source
pub type Pin20W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable RTC as wakeup source
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to enable IO(PA) as wakeup source
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set 1 to enable LPTIM1 as wakeup source
    #[inline(always)]
    pub fn lptim1(&self) -> Lptim1R {
        Lptim1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to enable PMUC as wakeup source
    #[inline(always)]
    pub fn pmuc(&self) -> PmucR {
        PmucR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Set 1 to enable LPSYS request as wakeup source
    #[inline(always)]
    pub fn lp2hp_req(&self) -> Lp2hpReqR {
        Lp2hpReqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set 1 to enable MAILBOX2 as wakeup source
    #[inline(always)]
    pub fn lp2hp_irq(&self) -> Lp2hpIrqR {
        Lp2hpIrqR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Set 1 to enable PA24 as wakeup source
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Set 1 to enable PA25 as wakeup source
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Set 1 to enable PA26 as wakeup source
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Set 1 to enable PA27 as wakeup source
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 18 - Set 1 to enable PA34 as wakeup source
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Set 1 to enable PA35 as wakeup source
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Set 1 to enable PA36 as wakeup source
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Set 1 to enable PA37 as wakeup source
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Set 1 to enable PA38 as wakeup source
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Set 1 to enable PA39 as wakeup source
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Set 1 to enable PA40 as wakeup source
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Set 1 to enable PA41 as wakeup source
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Set 1 to enable PA42 as wakeup source
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Set 1 to enable PA43 as wakeup source
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Set 1 to enable PA44 as wakeup source
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WER")
            .field("pin20", &self.pin20())
            .field("pin19", &self.pin19())
            .field("pin18", &self.pin18())
            .field("pin17", &self.pin17())
            .field("pin16", &self.pin16())
            .field("pin15", &self.pin15())
            .field("pin14", &self.pin14())
            .field("pin13", &self.pin13())
            .field("pin12", &self.pin12())
            .field("pin11", &self.pin11())
            .field("pin10", &self.pin10())
            .field("pin3", &self.pin3())
            .field("pin2", &self.pin2())
            .field("pin1", &self.pin1())
            .field("pin0", &self.pin0())
            .field("lp2hp_irq", &self.lp2hp_irq())
            .field("lp2hp_req", &self.lp2hp_req())
            .field("pmuc", &self.pmuc())
            .field("lptim1", &self.lptim1())
            .field("gpio1", &self.gpio1())
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
    ///Bit 1 - Set 1 to enable IO(PA) as wakeup source
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<WERrs> {
        Gpio1W::new(self, 1)
    }
    ///Bit 2 - Set 1 to enable LPTIM1 as wakeup source
    #[inline(always)]
    pub fn lptim1(&mut self) -> Lptim1W<WERrs> {
        Lptim1W::new(self, 2)
    }
    ///Bit 3 - Set 1 to enable PMUC as wakeup source
    #[inline(always)]
    pub fn pmuc(&mut self) -> PmucW<WERrs> {
        PmucW::new(self, 3)
    }
    ///Bit 6 - Set 1 to enable LPSYS request as wakeup source
    #[inline(always)]
    pub fn lp2hp_req(&mut self) -> Lp2hpReqW<WERrs> {
        Lp2hpReqW::new(self, 6)
    }
    ///Bit 7 - Set 1 to enable MAILBOX2 as wakeup source
    #[inline(always)]
    pub fn lp2hp_irq(&mut self) -> Lp2hpIrqW<WERrs> {
        Lp2hpIrqW::new(self, 7)
    }
    ///Bit 8 - Set 1 to enable PA24 as wakeup source
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<WERrs> {
        Pin0W::new(self, 8)
    }
    ///Bit 9 - Set 1 to enable PA25 as wakeup source
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<WERrs> {
        Pin1W::new(self, 9)
    }
    ///Bit 10 - Set 1 to enable PA26 as wakeup source
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<WERrs> {
        Pin2W::new(self, 10)
    }
    ///Bit 11 - Set 1 to enable PA27 as wakeup source
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<WERrs> {
        Pin3W::new(self, 11)
    }
    ///Bit 18 - Set 1 to enable PA34 as wakeup source
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<WERrs> {
        Pin10W::new(self, 18)
    }
    ///Bit 19 - Set 1 to enable PA35 as wakeup source
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<WERrs> {
        Pin11W::new(self, 19)
    }
    ///Bit 20 - Set 1 to enable PA36 as wakeup source
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<WERrs> {
        Pin12W::new(self, 20)
    }
    ///Bit 21 - Set 1 to enable PA37 as wakeup source
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<WERrs> {
        Pin13W::new(self, 21)
    }
    ///Bit 22 - Set 1 to enable PA38 as wakeup source
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<WERrs> {
        Pin14W::new(self, 22)
    }
    ///Bit 23 - Set 1 to enable PA39 as wakeup source
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<WERrs> {
        Pin15W::new(self, 23)
    }
    ///Bit 24 - Set 1 to enable PA40 as wakeup source
    #[inline(always)]
    pub fn pin16(&mut self) -> Pin16W<WERrs> {
        Pin16W::new(self, 24)
    }
    ///Bit 25 - Set 1 to enable PA41 as wakeup source
    #[inline(always)]
    pub fn pin17(&mut self) -> Pin17W<WERrs> {
        Pin17W::new(self, 25)
    }
    ///Bit 26 - Set 1 to enable PA42 as wakeup source
    #[inline(always)]
    pub fn pin18(&mut self) -> Pin18W<WERrs> {
        Pin18W::new(self, 26)
    }
    ///Bit 27 - Set 1 to enable PA43 as wakeup source
    #[inline(always)]
    pub fn pin19(&mut self) -> Pin19W<WERrs> {
        Pin19W::new(self, 27)
    }
    ///Bit 28 - Set 1 to enable PA44 as wakeup source
    #[inline(always)]
    pub fn pin20(&mut self) -> Pin20W<WERrs> {
        Pin20W::new(self, 28)
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
