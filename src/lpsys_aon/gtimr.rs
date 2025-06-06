///Register `GTIMR` reader
pub type R = crate::R<GTIMRrs>;
///Register `GTIMR` writer
pub type W = crate::W<GTIMRrs>;
///Field `CNT` reader - Global timer value
pub type CntR = crate::FieldReader<u32>;
///Field `CNT` writer - Global timer value
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Global timer value
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTIMR").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:31 - Global timer value
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<GTIMRrs> {
        CntW::new(self, 0)
    }
}
///Global Timer Register
///
///You can [`read`](crate::Reg::read) this register and get [`gtimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GTIMRrs;
impl crate::RegisterSpec for GTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`gtimr::R`](R) reader structure
impl crate::Readable for GTIMRrs {}
///`write(|w| ..)` method takes [`gtimr::W`](W) writer structure
impl crate::Writable for GTIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GTIMR to value 0
impl crate::Resettable for GTIMRrs {
    const RESET_VALUE: u32 = 0;
}
