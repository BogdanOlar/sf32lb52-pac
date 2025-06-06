///Register `WDT_SR` reader
pub type R = crate::R<WDT_SRrs>;
///Register `WDT_SR` writer
pub type W = crate::W<WDT_SRrs>;
///Field `INT_ASSERT` reader - Interrupt assert when 1
pub type IntAssertR = crate::BitReader;
///Field `INT_ASSERT` writer - Interrupt assert when 1
pub type IntAssertW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT_ACTIVE` reader - Watchdog runs when 1, else 0
pub type WdtActiveR = crate::BitReader;
///Field `WDT_ACTIVE` writer - Watchdog runs when 1, else 0
pub type WdtActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Interrupt assert when 1
    #[inline(always)]
    pub fn int_assert(&self) -> IntAssertR {
        IntAssertR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog runs when 1, else 0
    #[inline(always)]
    pub fn wdt_active(&self) -> WdtActiveR {
        WdtActiveR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_SR")
            .field("wdt_active", &self.wdt_active())
            .field("int_assert", &self.int_assert())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt assert when 1
    #[inline(always)]
    pub fn int_assert(&mut self) -> IntAssertW<WDT_SRrs> {
        IntAssertW::new(self, 0)
    }
    ///Bit 1 - Watchdog runs when 1, else 0
    #[inline(always)]
    pub fn wdt_active(&mut self) -> WdtActiveW<WDT_SRrs> {
        WdtActiveW::new(self, 1)
    }
}
///WatchDog Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_SRrs;
impl crate::RegisterSpec for WDT_SRrs {
    type Ux = u32;
}
///`read()` method returns [`wdt_sr::R`](R) reader structure
impl crate::Readable for WDT_SRrs {}
///`write(|w| ..)` method takes [`wdt_sr::W`](W) writer structure
impl crate::Writable for WDT_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_SR to value 0
impl crate::Resettable for WDT_SRrs {
    const RESET_VALUE: u32 = 0;
}
