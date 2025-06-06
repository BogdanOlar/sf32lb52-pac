///Register `TDR2` reader
pub type R = crate::R<TDR2rs>;
///Register `TDR2` writer
pub type W = crate::W<TDR2rs>;
///Field `DATA` reader - data value for task operation
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - data value for task operation
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - data value for task operation
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDR2").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - data value for task operation
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TDR2rs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tdr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TDR2rs;
impl crate::RegisterSpec for TDR2rs {
    type Ux = u32;
}
///`read()` method returns [`tdr2::R`](R) reader structure
impl crate::Readable for TDR2rs {}
///`write(|w| ..)` method takes [`tdr2::W`](W) writer structure
impl crate::Writable for TDR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TDR2 to value 0
impl crate::Resettable for TDR2rs {
    const RESET_VALUE: u32 = 0;
}
