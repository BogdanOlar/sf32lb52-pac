///Register `WDT_ICR` reader
pub type R = crate::R<WDT_ICRrs>;
///Register `WDT_ICR` writer
pub type W = crate::W<WDT_ICRrs>;
///Field `INT_CLR` reader - SinglePulse /A pulse to clear interrupt
pub type IntClrR = crate::BitReader;
///Field `INT_CLR` writer - SinglePulse /A pulse to clear interrupt
pub type IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - SinglePulse /A pulse to clear interrupt
    #[inline(always)]
    pub fn int_clr(&self) -> IntClrR {
        IntClrR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_ICR")
            .field("rsvd", &self.rsvd())
            .field("int_clr", &self.int_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - SinglePulse /A pulse to clear interrupt
    #[inline(always)]
    pub fn int_clr(&mut self) -> IntClrW<WDT_ICRrs> {
        IntClrW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WDT_ICRrs> {
        RsvdW::new(self, 1)
    }
}
///WatchDog Interrupt Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_ICRrs;
impl crate::RegisterSpec for WDT_ICRrs {
    type Ux = u32;
}
///`read()` method returns [`wdt_icr::R`](R) reader structure
impl crate::Readable for WDT_ICRrs {}
///`write(|w| ..)` method takes [`wdt_icr::W`](W) writer structure
impl crate::Writable for WDT_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_ICR to value 0
impl crate::Resettable for WDT_ICRrs {
    const RESET_VALUE: u32 = 0;
}
