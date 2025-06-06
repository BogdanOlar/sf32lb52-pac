///Register `RCR` reader
pub type R = crate::R<RCRrs>;
///Register `RCR` writer
pub type W = crate::W<RCRrs>;
///Field `REP` reader - Repetition register value REP is the repetition value for the LPTIM. Read REP will return left repetition times. It should be noted that for a reliable REP register read access, two consecutive read accesses must be performed and compared. A read access can be considered reliable when the values of the two consecutive read accesses are equal.
pub type RepR = crate::FieldReader;
///Field `REP` writer - Repetition register value REP is the repetition value for the LPTIM. Read REP will return left repetition times. It should be noted that for a reliable REP register read access, two consecutive read accesses must be performed and compared. A read access can be considered reliable when the values of the two consecutive read accesses are equal.
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM. Read REP will return left repetition times. It should be noted that for a reliable REP register read access, two consecutive read accesses must be performed and compared. A read access can be considered reliable when the values of the two consecutive read accesses are equal.
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR").field("rep", &self.rep()).finish()
    }
}
impl W {
    ///Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM. Read REP will return left repetition times. It should be noted that for a reliable REP register read access, two consecutive read accesses must be performed and compared. A read access can be considered reliable when the values of the two consecutive read accesses are equal.
    #[inline(always)]
    pub fn rep(&mut self) -> RepW<RCRrs> {
        RepW::new(self, 0)
    }
}
///LPTIM repetition register
///
///You can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RCRrs;
impl crate::RegisterSpec for RCRrs {
    type Ux = u32;
}
///`read()` method returns [`rcr::R`](R) reader structure
impl crate::Readable for RCRrs {}
///`write(|w| ..)` method takes [`rcr::W`](W) writer structure
impl crate::Writable for RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCR to value 0
impl crate::Resettable for RCRrs {
    const RESET_VALUE: u32 = 0;
}
