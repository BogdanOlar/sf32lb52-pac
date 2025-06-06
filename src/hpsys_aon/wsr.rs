///Register `WSR` reader
pub type R = crate::R<WSRrs>;
///Register `WSR` writer
pub type W = crate::W<WSRrs>;
///Field `RTC` reader - Indicates the wakeup status from RTC. Note: the status is masked by WER
pub type RtcR = crate::BitReader;
///Field `RTC` writer - Indicates the wakeup status from RTC. Note: the status is masked by WER
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO1` reader - Indicates the wakeup status from IO(PA). Note: the status is masked by WER
pub type Gpio1R = crate::BitReader;
///Field `GPIO1` writer - Indicates the wakeup status from IO(PA). Note: the status is masked by WER
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1` reader - Indicates the wakeup status from LPTIM1. Note: the status is masked by WER
pub type Lptim1R = crate::BitReader;
///Field `LPTIM1` writer - Indicates the wakeup status from LPTIM1. Note: the status is masked by WER
pub type Lptim1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMUC` reader - Indicates the wakeup status from PMUC. Note: the status is masked by WER
pub type PmucR = crate::BitReader;
///Field `PMUC` writer - Indicates the wakeup status from PMUC. Note: the status is masked by WER
pub type PmucW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_REQ` reader - Indicates the wakeup status from LPSYS request. Note: the status is masked by WER
pub type Lp2hpReqR = crate::BitReader;
///Field `LP2HP_REQ` writer - Indicates the wakeup status from LPSYS request. Note: the status is masked by WER
pub type Lp2hpReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_IRQ` reader - Indicates the wakeup status from MAILBOX2. Note: the status is masked by WER
pub type Lp2hpIrqR = crate::BitReader;
///Field `LP2HP_IRQ` writer - Indicates the wakeup status from MAILBOX2. Note: the status is masked by WER
pub type Lp2hpIrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN0` reader - Indicates the wakeup status from PA24 request. Note: the status is masked by WER
pub type Pin0R = crate::BitReader;
///Field `PIN0` writer - Indicates the wakeup status from PA24 request. Note: the status is masked by WER
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN1` reader - Indicates the wakeup status from PA25 request. Note: the status is masked by WER
pub type Pin1R = crate::BitReader;
///Field `PIN1` writer - Indicates the wakeup status from PA25 request. Note: the status is masked by WER
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN2` reader - Indicates the wakeup status from PA26 request. Note: the status is masked by WER
pub type Pin2R = crate::BitReader;
///Field `PIN2` writer - Indicates the wakeup status from PA26 request. Note: the status is masked by WER
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN3` reader - Indicates the wakeup status from PA27 request. Note: the status is masked by WER
pub type Pin3R = crate::BitReader;
///Field `PIN3` writer - Indicates the wakeup status from PA27 request. Note: the status is masked by WER
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN10` reader - Indicates the wakeup status from PA34 request. Note: the status is masked by WER
pub type Pin10R = crate::BitReader;
///Field `PIN10` writer - Indicates the wakeup status from PA34 request. Note: the status is masked by WER
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN11` reader - Indicates the wakeup status from PA35 request. Note: the status is masked by WER
pub type Pin11R = crate::BitReader;
///Field `PIN11` writer - Indicates the wakeup status from PA35 request. Note: the status is masked by WER
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN12` reader - Indicates the wakeup status from PA36 request. Note: the status is masked by WER
pub type Pin12R = crate::BitReader;
///Field `PIN12` writer - Indicates the wakeup status from PA36 request. Note: the status is masked by WER
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN13` reader - Indicates the wakeup status from PA37 request. Note: the status is masked by WER
pub type Pin13R = crate::BitReader;
///Field `PIN13` writer - Indicates the wakeup status from PA37 request. Note: the status is masked by WER
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN14` reader - Indicates the wakeup status from PA38 request. Note: the status is masked by WER
pub type Pin14R = crate::BitReader;
///Field `PIN14` writer - Indicates the wakeup status from PA38 request. Note: the status is masked by WER
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN15` reader - Indicates the wakeup status from PA39 request. Note: the status is masked by WER
pub type Pin15R = crate::BitReader;
///Field `PIN15` writer - Indicates the wakeup status from PA39 request. Note: the status is masked by WER
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN16` reader - Indicates the wakeup status from PA40 request. Note: the status is masked by WER
pub type Pin16R = crate::BitReader;
///Field `PIN16` writer - Indicates the wakeup status from PA40 request. Note: the status is masked by WER
pub type Pin16W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN17` reader - Indicates the wakeup status from PA41 request. Note: the status is masked by WER
pub type Pin17R = crate::BitReader;
///Field `PIN17` writer - Indicates the wakeup status from PA41 request. Note: the status is masked by WER
pub type Pin17W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN18` reader - Indicates the wakeup status from PA42 request. Note: the status is masked by WER
pub type Pin18R = crate::BitReader;
///Field `PIN18` writer - Indicates the wakeup status from PA42 request. Note: the status is masked by WER
pub type Pin18W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN19` reader - Indicates the wakeup status from PA43 request. Note: the status is masked by WER
pub type Pin19R = crate::BitReader;
///Field `PIN19` writer - Indicates the wakeup status from PA43 request. Note: the status is masked by WER
pub type Pin19W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN20` reader - Indicates the wakeup status from PA44 request. Note: the status is masked by WER
pub type Pin20R = crate::BitReader;
///Field `PIN20` writer - Indicates the wakeup status from PA44 request. Note: the status is masked by WER
pub type Pin20W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Indicates the wakeup status from RTC. Note: the status is masked by WER
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indicates the wakeup status from IO(PA). Note: the status is masked by WER
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Indicates the wakeup status from LPTIM1. Note: the status is masked by WER
    #[inline(always)]
    pub fn lptim1(&self) -> Lptim1R {
        Lptim1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Indicates the wakeup status from PMUC. Note: the status is masked by WER
    #[inline(always)]
    pub fn pmuc(&self) -> PmucR {
        PmucR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Indicates the wakeup status from LPSYS request. Note: the status is masked by WER
    #[inline(always)]
    pub fn lp2hp_req(&self) -> Lp2hpReqR {
        Lp2hpReqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Indicates the wakeup status from MAILBOX2. Note: the status is masked by WER
    #[inline(always)]
    pub fn lp2hp_irq(&self) -> Lp2hpIrqR {
        Lp2hpIrqR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Indicates the wakeup status from PA24 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Indicates the wakeup status from PA25 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Indicates the wakeup status from PA26 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Indicates the wakeup status from PA27 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 18 - Indicates the wakeup status from PA34 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Indicates the wakeup status from PA35 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Indicates the wakeup status from PA36 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Indicates the wakeup status from PA37 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Indicates the wakeup status from PA38 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Indicates the wakeup status from PA39 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Indicates the wakeup status from PA40 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Indicates the wakeup status from PA41 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Indicates the wakeup status from PA42 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Indicates the wakeup status from PA43 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Indicates the wakeup status from PA44 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WSR")
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
    ///Bit 0 - Indicates the wakeup status from RTC. Note: the status is masked by WER
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<WSRrs> {
        RtcW::new(self, 0)
    }
    ///Bit 1 - Indicates the wakeup status from IO(PA). Note: the status is masked by WER
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<WSRrs> {
        Gpio1W::new(self, 1)
    }
    ///Bit 2 - Indicates the wakeup status from LPTIM1. Note: the status is masked by WER
    #[inline(always)]
    pub fn lptim1(&mut self) -> Lptim1W<WSRrs> {
        Lptim1W::new(self, 2)
    }
    ///Bit 3 - Indicates the wakeup status from PMUC. Note: the status is masked by WER
    #[inline(always)]
    pub fn pmuc(&mut self) -> PmucW<WSRrs> {
        PmucW::new(self, 3)
    }
    ///Bit 6 - Indicates the wakeup status from LPSYS request. Note: the status is masked by WER
    #[inline(always)]
    pub fn lp2hp_req(&mut self) -> Lp2hpReqW<WSRrs> {
        Lp2hpReqW::new(self, 6)
    }
    ///Bit 7 - Indicates the wakeup status from MAILBOX2. Note: the status is masked by WER
    #[inline(always)]
    pub fn lp2hp_irq(&mut self) -> Lp2hpIrqW<WSRrs> {
        Lp2hpIrqW::new(self, 7)
    }
    ///Bit 8 - Indicates the wakeup status from PA24 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<WSRrs> {
        Pin0W::new(self, 8)
    }
    ///Bit 9 - Indicates the wakeup status from PA25 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<WSRrs> {
        Pin1W::new(self, 9)
    }
    ///Bit 10 - Indicates the wakeup status from PA26 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<WSRrs> {
        Pin2W::new(self, 10)
    }
    ///Bit 11 - Indicates the wakeup status from PA27 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<WSRrs> {
        Pin3W::new(self, 11)
    }
    ///Bit 18 - Indicates the wakeup status from PA34 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<WSRrs> {
        Pin10W::new(self, 18)
    }
    ///Bit 19 - Indicates the wakeup status from PA35 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<WSRrs> {
        Pin11W::new(self, 19)
    }
    ///Bit 20 - Indicates the wakeup status from PA36 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<WSRrs> {
        Pin12W::new(self, 20)
    }
    ///Bit 21 - Indicates the wakeup status from PA37 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<WSRrs> {
        Pin13W::new(self, 21)
    }
    ///Bit 22 - Indicates the wakeup status from PA38 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<WSRrs> {
        Pin14W::new(self, 22)
    }
    ///Bit 23 - Indicates the wakeup status from PA39 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<WSRrs> {
        Pin15W::new(self, 23)
    }
    ///Bit 24 - Indicates the wakeup status from PA40 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin16(&mut self) -> Pin16W<WSRrs> {
        Pin16W::new(self, 24)
    }
    ///Bit 25 - Indicates the wakeup status from PA41 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin17(&mut self) -> Pin17W<WSRrs> {
        Pin17W::new(self, 25)
    }
    ///Bit 26 - Indicates the wakeup status from PA42 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin18(&mut self) -> Pin18W<WSRrs> {
        Pin18W::new(self, 26)
    }
    ///Bit 27 - Indicates the wakeup status from PA43 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin19(&mut self) -> Pin19W<WSRrs> {
        Pin19W::new(self, 27)
    }
    ///Bit 28 - Indicates the wakeup status from PA44 request. Note: the status is masked by WER
    #[inline(always)]
    pub fn pin20(&mut self) -> Pin20W<WSRrs> {
        Pin20W::new(self, 28)
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
