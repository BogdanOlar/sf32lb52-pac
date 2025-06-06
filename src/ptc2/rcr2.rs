///Register `RCR2` reader
pub type R = crate::R<RCR2rs>;
///Register `RCR2` writer
pub type W = crate::W<RCR2rs>;
///Field `REP` reader - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
pub type RepR = crate::FieldReader<u16>;
///Field `REP` writer - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DLY` reader - Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation.
pub type DlyR = crate::FieldReader<u16>;
///Field `DLY` writer - Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation.
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:9 - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 16:31 - Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation.
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR2")
            .field("dly", &self.dly())
            .field("rsvd", &self.rsvd())
            .field("rep", &self.rep())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered.
    #[inline(always)]
    pub fn rep(&mut self) -> RepW<RCR2rs> {
        RepW::new(self, 0)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RCR2rs> {
        RsvdW::new(self, 10)
    }
    ///Bits 16:31 - Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation.
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<RCR2rs> {
        DlyW::new(self, 16)
    }
}
///task 2 repetition and delay counter register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RCR2rs;
impl crate::RegisterSpec for RCR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcr2::R`](R) reader structure
impl crate::Readable for RCR2rs {}
///`write(|w| ..)` method takes [`rcr2::W`](W) writer structure
impl crate::Writable for RCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCR2 to value 0
impl crate::Resettable for RCR2rs {
    const RESET_VALUE: u32 = 0;
}
