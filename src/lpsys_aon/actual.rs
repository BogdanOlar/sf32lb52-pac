///Register `ACTUAL` reader
pub type R = crate::R<ACTUALrs>;
///Register `ACTUAL` writer
pub type W = crate::W<ACTUALrs>;
///Field `SLEEP_CNT` reader - bt actual sleep time in cycles of clk_rtc. If not woken up by software or external interrupt, sleep_cnt counts up every clk_rtc cycle, until reaches sleep_target
pub type SleepCntR = crate::FieldReader<u32>;
///Field `SLEEP_CNT` writer - bt actual sleep time in cycles of clk_rtc. If not woken up by software or external interrupt, sleep_cnt counts up every clk_rtc cycle, until reaches sleep_target
pub type SleepCntW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - bt actual sleep time in cycles of clk_rtc. If not woken up by software or external interrupt, sleep_cnt counts up every clk_rtc cycle, until reaches sleep_target
    #[inline(always)]
    pub fn sleep_cnt(&self) -> SleepCntR {
        SleepCntR::new(self.bits & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTUAL")
            .field("sleep_cnt", &self.sleep_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - bt actual sleep time in cycles of clk_rtc. If not woken up by software or external interrupt, sleep_cnt counts up every clk_rtc cycle, until reaches sleep_target
    #[inline(always)]
    pub fn sleep_cnt(&mut self) -> SleepCntW<ACTUALrs> {
        SleepCntW::new(self, 0)
    }
}
///BT actual sleep time
///
///You can [`read`](crate::Reg::read) this register and get [`actual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ACTUALrs;
impl crate::RegisterSpec for ACTUALrs {
    type Ux = u32;
}
///`read()` method returns [`actual::R`](R) reader structure
impl crate::Readable for ACTUALrs {}
///`write(|w| ..)` method takes [`actual::W`](W) writer structure
impl crate::Writable for ACTUALrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ACTUAL to value 0
impl crate::Resettable for ACTUALrs {
    const RESET_VALUE: u32 = 0;
}
