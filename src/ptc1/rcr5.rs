///Register `RCR5` reader
pub type R = crate::R<RCR5rs>;
///Register `RCR5` writer
pub type W = crate::W<RCR5rs>;
///Field `REP` reader - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
pub type RepR = crate::FieldReader<u16>;
///Field `REP` writer - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR5").field("rep", &self.rep()).finish()
    }
}
impl W {
    ///Bits 0:9 - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
    #[inline(always)]
    pub fn rep(&mut self) -> RepW<RCR5rs> {
        RepW::new(self, 0)
    }
}
///task 5 repetition counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RCR5rs;
impl crate::RegisterSpec for RCR5rs {
    type Ux = u32;
}
///`read()` method returns [`rcr5::R`](R) reader structure
impl crate::Readable for RCR5rs {}
///`write(|w| ..)` method takes [`rcr5::W`](W) writer structure
impl crate::Writable for RCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCR5 to value 0
impl crate::Resettable for RCR5rs {
    const RESET_VALUE: u32 = 0;
}
