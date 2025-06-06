///Register `NONCEB` reader
pub type R = crate::R<NONCEBrs>;
///Register `NONCEB` writer
pub type W = crate::W<NONCEBrs>;
///Field `NONCEB` reader - Used for on-the-fly decryption
pub type NoncebR = crate::FieldReader<u32>;
///Field `NONCEB` writer - Used for on-the-fly decryption
pub type NoncebW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Used for on-the-fly decryption
    #[inline(always)]
    pub fn nonceb(&self) -> NoncebR {
        NoncebR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NONCEB")
            .field("nonceb", &self.nonceb())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Used for on-the-fly decryption
    #[inline(always)]
    pub fn nonceb(&mut self) -> NoncebW<NONCEBrs> {
        NoncebW::new(self, 0)
    }
}
///Nonce B Register
///
///You can [`read`](crate::Reg::read) this register and get [`nonceb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonceb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct NONCEBrs;
impl crate::RegisterSpec for NONCEBrs {
    type Ux = u32;
}
///`read()` method returns [`nonceb::R`](R) reader structure
impl crate::Readable for NONCEBrs {}
///`write(|w| ..)` method takes [`nonceb::W`](W) writer structure
impl crate::Writable for NONCEBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets NONCEB to value 0
impl crate::Resettable for NONCEBrs {
    const RESET_VALUE: u32 = 0;
}
