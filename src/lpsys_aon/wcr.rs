///Register `WCR` reader
pub type R = crate::R<WCRrs>;
///Register `WCR` writer
pub type W = crate::W<WCRrs>;
///Field `RSVD3` reader - Note: for RTC/GPIO/LPTIM/BT, clear the wakeup status directly in the corresponding module
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer - Note: for RTC/GPIO/LPTIM/BT, clear the wakeup status directly in the corresponding module
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PIN0` reader - Write 1 to clear PA24 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin0R = crate::BitReader;
///Field `PIN0` writer - Write 1 to clear PA24 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN1` reader - Write 1 to clear PA25 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin1R = crate::BitReader;
///Field `PIN1` writer - Write 1 to clear PA25 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN2` reader - Write 1 to clear PA26 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin2R = crate::BitReader;
///Field `PIN2` writer - Write 1 to clear PA26 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN3` reader - Write 1 to clear PA27 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin3R = crate::BitReader;
///Field `PIN3` writer - Write 1 to clear PA27 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PIN10` reader - Write 1 to clear PA34 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin10R = crate::BitReader;
///Field `PIN10` writer - Write 1 to clear PA34 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN11` reader - Write 1 to clear PA35 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin11R = crate::BitReader;
///Field `PIN11` writer - Write 1 to clear PA35 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN12` reader - Write 1 to clear PA36 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin12R = crate::BitReader;
///Field `PIN12` writer - Write 1 to clear PA36 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN13` reader - Write 1 to clear PA37 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin13R = crate::BitReader;
///Field `PIN13` writer - Write 1 to clear PA37 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN14` reader - Write 1 to clear PA38 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin14R = crate::BitReader;
///Field `PIN14` writer - Write 1 to clear PA38 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN15` reader - Write 1 to clear PA39 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin15R = crate::BitReader;
///Field `PIN15` writer - Write 1 to clear PA39 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN16` reader - Write 1 to clear PA40 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin16R = crate::BitReader;
///Field `PIN16` writer - Write 1 to clear PA40 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin16W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN17` reader - Write 1 to clear PA41 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin17R = crate::BitReader;
///Field `PIN17` writer - Write 1 to clear PA41 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin17W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN18` reader - Write 1 to clear PA42 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin18R = crate::BitReader;
///Field `PIN18` writer - Write 1 to clear PA42 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin18W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN19` reader - Write 1 to clear PA43 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin19R = crate::BitReader;
///Field `PIN19` writer - Write 1 to clear PA43 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin19W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN20` reader - Write 1 to clear PA44 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin20R = crate::BitReader;
///Field `PIN20` writer - Write 1 to clear PA44 wakeup source. Only valid if PIN wakeup is configured as edge trigger
pub type Pin20W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AON` reader - Write 1 to clear the AON wakeup IRQ status
pub type AonR = crate::BitReader;
///Field `AON` writer - Write 1 to clear the AON wakeup IRQ status
pub type AonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Note: for RTC/GPIO/LPTIM/BT, clear the wakeup status directly in the corresponding module
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Write 1 to clear PA24 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Write 1 to clear PA25 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Write 1 to clear PA26 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Write 1 to clear PA27 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:17
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bit 18 - Write 1 to clear PA34 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Write 1 to clear PA35 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Write 1 to clear PA36 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Write 1 to clear PA37 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Write 1 to clear PA38 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Write 1 to clear PA39 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Write 1 to clear PA40 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Write 1 to clear PA41 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Write 1 to clear PA42 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Write 1 to clear PA43 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Write 1 to clear PA44 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 29) & 3) as u8)
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
            .field("rsvd2", &self.rsvd2())
            .field("pin3", &self.pin3())
            .field("pin2", &self.pin2())
            .field("pin1", &self.pin1())
            .field("pin0", &self.pin0())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Note: for RTC/GPIO/LPTIM/BT, clear the wakeup status directly in the corresponding module
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<WCRrs> {
        Rsvd3W::new(self, 0)
    }
    ///Bit 8 - Write 1 to clear PA24 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<WCRrs> {
        Pin0W::new(self, 8)
    }
    ///Bit 9 - Write 1 to clear PA25 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<WCRrs> {
        Pin1W::new(self, 9)
    }
    ///Bit 10 - Write 1 to clear PA26 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<WCRrs> {
        Pin2W::new(self, 10)
    }
    ///Bit 11 - Write 1 to clear PA27 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<WCRrs> {
        Pin3W::new(self, 11)
    }
    ///Bits 12:17
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<WCRrs> {
        Rsvd2W::new(self, 12)
    }
    ///Bit 18 - Write 1 to clear PA34 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<WCRrs> {
        Pin10W::new(self, 18)
    }
    ///Bit 19 - Write 1 to clear PA35 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<WCRrs> {
        Pin11W::new(self, 19)
    }
    ///Bit 20 - Write 1 to clear PA36 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<WCRrs> {
        Pin12W::new(self, 20)
    }
    ///Bit 21 - Write 1 to clear PA37 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<WCRrs> {
        Pin13W::new(self, 21)
    }
    ///Bit 22 - Write 1 to clear PA38 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<WCRrs> {
        Pin14W::new(self, 22)
    }
    ///Bit 23 - Write 1 to clear PA39 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<WCRrs> {
        Pin15W::new(self, 23)
    }
    ///Bit 24 - Write 1 to clear PA40 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin16(&mut self) -> Pin16W<WCRrs> {
        Pin16W::new(self, 24)
    }
    ///Bit 25 - Write 1 to clear PA41 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin17(&mut self) -> Pin17W<WCRrs> {
        Pin17W::new(self, 25)
    }
    ///Bit 26 - Write 1 to clear PA42 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin18(&mut self) -> Pin18W<WCRrs> {
        Pin18W::new(self, 26)
    }
    ///Bit 27 - Write 1 to clear PA43 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin19(&mut self) -> Pin19W<WCRrs> {
        Pin19W::new(self, 27)
    }
    ///Bit 28 - Write 1 to clear PA44 wakeup source. Only valid if PIN wakeup is configured as edge trigger
    #[inline(always)]
    pub fn pin20(&mut self) -> Pin20W<WCRrs> {
        Pin20W::new(self, 28)
    }
    ///Bits 29:30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WCRrs> {
        RsvdW::new(self, 29)
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
