///Register `WDT_CVR0` reader
pub type R = crate::R<WDT_CVR0rs>;
///Register `WDT_CVR0` writer
pub type W = crate::W<WDT_CVR0rs>;
///Field `COUNT_VALUE_0` reader - Count Value for 1st TimeOut
pub type CountValue0R = crate::FieldReader<u32>;
///Field `COUNT_VALUE_0` writer - Count Value for 1st TimeOut
pub type CountValue0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - Count Value for 1st TimeOut
    #[inline(always)]
    pub fn count_value_0(&self) -> CountValue0R {
        CountValue0R::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_CVR0")
            .field("rsvd", &self.rsvd())
            .field("count_value_0", &self.count_value_0())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Count Value for 1st TimeOut
    #[inline(always)]
    pub fn count_value_0(&mut self) -> CountValue0W<WDT_CVR0rs> {
        CountValue0W::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WDT_CVR0rs> {
        RsvdW::new(self, 24)
    }
}
///WatchDog Counter Value 0
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_cvr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cvr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_CVR0rs;
impl crate::RegisterSpec for WDT_CVR0rs {
    type Ux = u32;
}
///`read()` method returns [`wdt_cvr0::R`](R) reader structure
impl crate::Readable for WDT_CVR0rs {}
///`write(|w| ..)` method takes [`wdt_cvr0::W`](W) writer structure
impl crate::Writable for WDT_CVR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_CVR0 to value 0
impl crate::Resettable for WDT_CVR0rs {
    const RESET_VALUE: u32 = 0;
}
