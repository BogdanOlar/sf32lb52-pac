///Register `NONCEA` reader
pub type R = crate::R<NONCEArs>;
///Register `NONCEA` writer
pub type W = crate::W<NONCEArs>;
///Field `NONCEA` reader - Used for on-the-fly decryption
pub type NonceaR = crate::FieldReader<u32>;
///Field `NONCEA` writer - Used for on-the-fly decryption
pub type NonceaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Used for on-the-fly decryption
    #[inline(always)]
    pub fn noncea(&self) -> NonceaR {
        NonceaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NONCEA")
            .field("noncea", &self.noncea())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Used for on-the-fly decryption
    #[inline(always)]
    pub fn noncea(&mut self) -> NonceaW<NONCEArs> {
        NonceaW::new(self, 0)
    }
}
///Nonce A Register
///
///You can [`read`](crate::Reg::read) this register and get [`noncea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noncea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct NONCEArs;
impl crate::RegisterSpec for NONCEArs {
    type Ux = u32;
}
///`read()` method returns [`noncea::R`](R) reader structure
impl crate::Readable for NONCEArs {}
///`write(|w| ..)` method takes [`noncea::W`](W) writer structure
impl crate::Writable for NONCEArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets NONCEA to value 0
impl crate::Resettable for NONCEArs {
    const RESET_VALUE: u32 = 0;
}
