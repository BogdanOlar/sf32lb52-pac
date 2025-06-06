///Register `HASH_LEN_L` reader
pub type R = crate::R<HASH_LEN_Lrs>;
///Register `HASH_LEN_L` writer
pub type W = crate::W<HASH_LEN_Lrs>;
///Field `DATA` reader - HASH load length l
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - HASH load length l
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HASH load length l
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_LEN_L")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HASH load length l
    #[inline(always)]
    pub fn data(&mut self) -> DataW<HASH_LEN_Lrs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hash_len_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_len_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_LEN_Lrs;
impl crate::RegisterSpec for HASH_LEN_Lrs {
    type Ux = u32;
}
///`read()` method returns [`hash_len_l::R`](R) reader structure
impl crate::Readable for HASH_LEN_Lrs {}
///`write(|w| ..)` method takes [`hash_len_l::W`](W) writer structure
impl crate::Writable for HASH_LEN_Lrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_LEN_L to value 0
impl crate::Resettable for HASH_LEN_Lrs {
    const RESET_VALUE: u32 = 0;
}
