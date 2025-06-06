///Register `TOR` reader
pub type R = crate::R<TORrs>;
///Register `TOR` writer
pub type W = crate::W<TORrs>;
///Field `TIMEOUT_CNT` reader - Used to determine how much time waiting response or data bus busy is timeout, and decreased under card clock. Set to 400000 for 1s timeout if interface clock is 400KHz.
pub type TimeoutCntR = crate::FieldReader<u32>;
///Field `TIMEOUT_CNT` writer - Used to determine how much time waiting response or data bus busy is timeout, and decreased under card clock. Set to 400000 for 1s timeout if interface clock is 400KHz.
pub type TimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Used to determine how much time waiting response or data bus busy is timeout, and decreased under card clock. Set to 400000 for 1s timeout if interface clock is 400KHz.
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TimeoutCntR {
        TimeoutCntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOR")
            .field("timeout_cnt", &self.timeout_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Used to determine how much time waiting response or data bus busy is timeout, and decreased under card clock. Set to 400000 for 1s timeout if interface clock is 400KHz.
    #[inline(always)]
    pub fn timeout_cnt(&mut self) -> TimeoutCntW<TORrs> {
        TimeoutCntW::new(self, 0)
    }
}
///timeout count register
///
///You can [`read`](crate::Reg::read) this register and get [`tor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TORrs;
impl crate::RegisterSpec for TORrs {
    type Ux = u32;
}
///`read()` method returns [`tor::R`](R) reader structure
impl crate::Readable for TORrs {}
///`write(|w| ..)` method takes [`tor::W`](W) writer structure
impl crate::Writable for TORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOR to value 0
impl crate::Resettable for TORrs {
    const RESET_VALUE: u32 = 0;
}
