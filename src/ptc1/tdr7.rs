///Register `TDR7` reader
pub type R = crate::R<TDR7rs>;
///Register `TDR7` writer
pub type W = crate::W<TDR7rs>;
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
        f.debug_struct("TDR7").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - data value for task operation
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TDR7rs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tdr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TDR7rs;
impl crate::RegisterSpec for TDR7rs {
    type Ux = u32;
}
///`read()` method returns [`tdr7::R`](R) reader structure
impl crate::Readable for TDR7rs {}
///`write(|w| ..)` method takes [`tdr7::W`](W) writer structure
impl crate::Writable for TDR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TDR7 to value 0
impl crate::Resettable for TDR7rs {
    const RESET_VALUE: u32 = 0;
}
