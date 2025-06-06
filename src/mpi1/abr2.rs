///Register `ABR2` reader
pub type R = crate::R<ABR2rs>;
///Register `ABR2` writer
pub type W = crate::W<ABR2rs>;
///Field `ABYTE` reader - Alternate byte in CMD2 sequence
pub type AbyteR = crate::FieldReader<u32>;
///Field `ABYTE` writer - Alternate byte in CMD2 sequence
pub type AbyteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Alternate byte in CMD2 sequence
    #[inline(always)]
    pub fn abyte(&self) -> AbyteR {
        AbyteR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ABR2")
            .field("abyte", &self.abyte())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Alternate byte in CMD2 sequence
    #[inline(always)]
    pub fn abyte(&mut self) -> AbyteW<ABR2rs> {
        AbyteW::new(self, 0)
    }
}
///Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`abr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ABR2rs;
impl crate::RegisterSpec for ABR2rs {
    type Ux = u32;
}
///`read()` method returns [`abr2::R`](R) reader structure
impl crate::Readable for ABR2rs {}
///`write(|w| ..)` method takes [`abr2::W`](W) writer structure
impl crate::Writable for ABR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ABR2 to value 0
impl crate::Resettable for ABR2rs {
    const RESET_VALUE: u32 = 0;
}
