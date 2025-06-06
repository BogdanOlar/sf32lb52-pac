///Register `MEM4` reader
pub type R = crate::R<MEM4rs>;
///Register `MEM4` writer
pub type W = crate::W<MEM4rs>;
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
        f.debug_struct("MEM4").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - memory to store temporary variables
    #[inline(always)]
    pub fn data(&mut self) -> DataW<MEM4rs> {
        DataW::new(self, 0)
    }
}
///temporary memory 4
///
///You can [`read`](crate::Reg::read) this register and get [`mem4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MEM4rs;
impl crate::RegisterSpec for MEM4rs {
    type Ux = u32;
}
///`read()` method returns [`mem4::R`](R) reader structure
impl crate::Readable for MEM4rs {}
///`write(|w| ..)` method takes [`mem4::W`](W) writer structure
impl crate::Writable for MEM4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM4 to value 0
impl crate::Resettable for MEM4rs {
    const RESET_VALUE: u32 = 0;
}
