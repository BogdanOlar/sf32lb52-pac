///Register `WDT_CCR` reader
pub type R = crate::R<WDT_CCRrs>;
///Register `WDT_CCR` writer
pub type W = crate::W<WDT_CCRrs>;
///Field `COUNTER_CONTROL` reader - SinglePulse /Write 8'h76 to restart, write8'h34 to stop, else do nothing
pub type CounterControlR = crate::FieldReader;
///Field `COUNTER_CONTROL` writer - SinglePulse /Write 8'h76 to restart, write8'h34 to stop, else do nothing
pub type CounterControlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:7 - SinglePulse /Write 8'h76 to restart, write8'h34 to stop, else do nothing
    #[inline(always)]
    pub fn counter_control(&self) -> CounterControlR {
        CounterControlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_CCR")
            .field("rsvd", &self.rsvd())
            .field("counter_control", &self.counter_control())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - SinglePulse /Write 8'h76 to restart, write8'h34 to stop, else do nothing
    #[inline(always)]
    pub fn counter_control(&mut self) -> CounterControlW<WDT_CCRrs> {
        CounterControlW::new(self, 0)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WDT_CCRrs> {
        RsvdW::new(self, 8)
    }
}
///WatchDog Counter Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_CCRrs;
impl crate::RegisterSpec for WDT_CCRrs {
    type Ux = u32;
}
///`read()` method returns [`wdt_ccr::R`](R) reader structure
impl crate::Readable for WDT_CCRrs {}
///`write(|w| ..)` method takes [`wdt_ccr::W`](W) writer structure
impl crate::Writable for WDT_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_CCR to value 0
impl crate::Resettable for WDT_CCRrs {
    const RESET_VALUE: u32 = 0;
}
