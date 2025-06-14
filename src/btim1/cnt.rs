///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT` reader - counter value
pub type CntR = crate::FieldReader<u32>;
///Field `CNT` writer - counter value
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:31 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<CNTrs> {
        CntW::new(self, 0)
    }
}
///Counter
///
///You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
