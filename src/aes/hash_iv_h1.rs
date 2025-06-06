///Register `HASH_IV_H1` reader
pub type R = crate::R<HASH_IV_H1rs>;
///Register `HASH_IV_H1` writer
pub type W = crate::W<HASH_IV_H1rs>;
///Field `DATA` reader - HASH IV H1
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - HASH IV H1
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HASH IV H1
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_IV_H1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HASH IV H1
    #[inline(always)]
    pub fn data(&mut self) -> DataW<HASH_IV_H1rs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hash_iv_h1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_iv_h1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_IV_H1rs;
impl crate::RegisterSpec for HASH_IV_H1rs {
    type Ux = u32;
}
///`read()` method returns [`hash_iv_h1::R`](R) reader structure
impl crate::Readable for HASH_IV_H1rs {}
///`write(|w| ..)` method takes [`hash_iv_h1::W`](W) writer structure
impl crate::Writable for HASH_IV_H1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_IV_H1 to value 0
impl crate::Resettable for HASH_IV_H1rs {
    const RESET_VALUE: u32 = 0;
}
