///Register `MEM1` reader
pub type R = crate::R<MEM1rs>;
///Register `MEM1` writer
pub type W = crate::W<MEM1rs>;
///Field `DATA` reader - memory to store temporary variables
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - memory to store temporary variables
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - memory to store temporary variables
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM1").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - memory to store temporary variables
    #[inline(always)]
    pub fn data(&mut self) -> DataW<MEM1rs> {
        DataW::new(self, 0)
    }
}
///temporary memory 1
///
///You can [`read`](crate::Reg::read) this register and get [`mem1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MEM1rs;
impl crate::RegisterSpec for MEM1rs {
    type Ux = u32;
}
///`read()` method returns [`mem1::R`](R) reader structure
impl crate::Readable for MEM1rs {}
///`write(|w| ..)` method takes [`mem1::W`](W) writer structure
impl crate::Writable for MEM1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM1 to value 0
impl crate::Resettable for MEM1rs {
    const RESET_VALUE: u32 = 0;
}
