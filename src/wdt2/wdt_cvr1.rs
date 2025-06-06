///Register `WDT_CVR1` reader
pub type R = crate::R<WDT_CVR1rs>;
///Register `WDT_CVR1` writer
pub type W = crate::W<WDT_CVR1rs>;
///Field `COUNT_VALUE_1` reader - Count Value for 2nd TimeOut
pub type CountValue1R = crate::FieldReader<u32>;
///Field `COUNT_VALUE_1` writer - Count Value for 2nd TimeOut
pub type CountValue1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Count Value for 2nd TimeOut
    #[inline(always)]
    pub fn count_value_1(&self) -> CountValue1R {
        CountValue1R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_CVR1")
            .field("count_value_1", &self.count_value_1())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Count Value for 2nd TimeOut
    #[inline(always)]
    pub fn count_value_1(&mut self) -> CountValue1W<WDT_CVR1rs> {
        CountValue1W::new(self, 0)
    }
}
///WatchDog Counter Value 1
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_cvr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cvr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_CVR1rs;
impl crate::RegisterSpec for WDT_CVR1rs {
    type Ux = u32;
}
///`read()` method returns [`wdt_cvr1::R`](R) reader structure
impl crate::Readable for WDT_CVR1rs {}
///`write(|w| ..)` method takes [`wdt_cvr1::W`](W) writer structure
impl crate::Writable for WDT_CVR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_CVR1 to value 0
impl crate::Resettable for WDT_CVR1rs {
    const RESET_VALUE: u32 = 0;
}
