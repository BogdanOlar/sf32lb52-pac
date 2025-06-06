///Register `RAND_SEED7` reader
pub type R = crate::R<RAND_SEED7rs>;
///Register `RAND_SEED7` writer
pub type W = crate::W<RAND_SEED7rs>;
///Field `VAL` reader - random seed value7. If using external random seed, write value to this register will update the random seed in use.
pub type ValR = crate::FieldReader<u32>;
///Field `VAL` writer - random seed value7. If using external random seed, write value to this register will update the random seed in use.
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - random seed value7. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAND_SEED7")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - random seed value7. If using external random seed, write value to this register will update the random seed in use.
    #[inline(always)]
    pub fn val(&mut self) -> ValW<RAND_SEED7rs> {
        ValW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rand_seed7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rand_seed7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RAND_SEED7rs;
impl crate::RegisterSpec for RAND_SEED7rs {
    type Ux = u32;
}
///`read()` method returns [`rand_seed7::R`](R) reader structure
impl crate::Readable for RAND_SEED7rs {}
///`write(|w| ..)` method takes [`rand_seed7::W`](W) writer structure
impl crate::Writable for RAND_SEED7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RAND_SEED7 to value 0
impl crate::Resettable for RAND_SEED7rs {
    const RESET_VALUE: u32 = 0;
}
