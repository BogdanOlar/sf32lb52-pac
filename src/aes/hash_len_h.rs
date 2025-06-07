///Register `HASH_LEN_H` reader
pub type R = crate::R<HASH_LEN_Hrs>;
///Register `HASH_LEN_H` writer
pub type W = crate::W<HASH_LEN_Hrs>;
///Field `DATA` reader - HASH load length h
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - HASH load length h
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - HASH load length h
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_LEN_H")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - HASH load length h
    #[inline(always)]
    pub fn data(&mut self) -> DataW<HASH_LEN_Hrs> {
        DataW::new(self, 0)
    }
}
///HASH load length h
///
///You can [`read`](crate::Reg::read) this register and get [`hash_len_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_len_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_LEN_Hrs;
impl crate::RegisterSpec for HASH_LEN_Hrs {
    type Ux = u32;
}
///`read()` method returns [`hash_len_h::R`](R) reader structure
impl crate::Readable for HASH_LEN_Hrs {}
///`write(|w| ..)` method takes [`hash_len_h::W`](W) writer structure
impl crate::Writable for HASH_LEN_Hrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_LEN_H to value 0
impl crate::Resettable for HASH_LEN_Hrs {
    const RESET_VALUE: u32 = 0;
}
