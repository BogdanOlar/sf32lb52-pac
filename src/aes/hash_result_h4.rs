///Register `HASH_RESULT_H4` reader
pub type R = crate::R<HASH_RESULT_H4rs>;
///Register `HASH_RESULT_H4` writer
pub type W = crate::W<HASH_RESULT_H4rs>;
///Field `DATA` reader - HASH result H4
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - HASH result H4
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HASH result H4
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_RESULT_H4")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HASH result H4
    #[inline(always)]
    pub fn data(&mut self) -> DataW<HASH_RESULT_H4rs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hash_result_h4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_result_h4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_RESULT_H4rs;
impl crate::RegisterSpec for HASH_RESULT_H4rs {
    type Ux = u32;
}
///`read()` method returns [`hash_result_h4::R`](R) reader structure
impl crate::Readable for HASH_RESULT_H4rs {}
///`write(|w| ..)` method takes [`hash_result_h4::W`](W) writer structure
impl crate::Writable for HASH_RESULT_H4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_RESULT_H4 to value 0
impl crate::Resettable for HASH_RESULT_H4rs {
    const RESET_VALUE: u32 = 0;
}
