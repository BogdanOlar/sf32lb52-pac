///Register `TARGET` reader
pub type R = crate::R<TARGETrs>;
///Register `TARGET` writer
pub type W = crate::W<TARGETrs>;
///Field `SLEEP_TARGET` reader - bt sleep time target in cycles of clk_rtc
pub type SleepTargetR = crate::FieldReader<u32>;
///Field `SLEEP_TARGET` writer - bt sleep time target in cycles of clk_rtc
pub type SleepTargetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:27 - bt sleep time target in cycles of clk_rtc
    #[inline(always)]
    pub fn sleep_target(&self) -> SleepTargetR {
        SleepTargetR::new(self.bits & 0x0fff_ffff)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET")
            .field("rsvd", &self.rsvd())
            .field("sleep_target", &self.sleep_target())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - bt sleep time target in cycles of clk_rtc
    #[inline(always)]
    pub fn sleep_target(&mut self) -> SleepTargetW<TARGETrs> {
        SleepTargetW::new(self, 0)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TARGETrs> {
        RsvdW::new(self, 28)
    }
}
///BT sleep time target
///
///You can [`read`](crate::Reg::read) this register and get [`target::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TARGETrs;
impl crate::RegisterSpec for TARGETrs {
    type Ux = u32;
}
///`read()` method returns [`target::R`](R) reader structure
impl crate::Readable for TARGETrs {}
///`write(|w| ..)` method takes [`target::W`](W) writer structure
impl crate::Writable for TARGETrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TARGET to value 0
impl crate::Resettable for TARGETrs {
    const RESET_VALUE: u32 = 0;
}
