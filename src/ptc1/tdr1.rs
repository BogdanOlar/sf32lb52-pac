///Register `TDR1` reader
pub type R = crate::R<TDR1rs>;
///Register `TDR1` writer
pub type W = crate::W<TDR1rs>;
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
        f.debug_struct("TDR1").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - data value for task operation
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TDR1rs> {
        DataW::new(self, 0)
    }
}
///task 1 data register
///
///You can [`read`](crate::Reg::read) this register and get [`tdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TDR1rs;
impl crate::RegisterSpec for TDR1rs {
    type Ux = u32;
}
///`read()` method returns [`tdr1::R`](R) reader structure
impl crate::Readable for TDR1rs {}
///`write(|w| ..)` method takes [`tdr1::W`](W) writer structure
impl crate::Writable for TDR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TDR1 to value 0
impl crate::Resettable for TDR1rs {
    const RESET_VALUE: u32 = 0;
}
