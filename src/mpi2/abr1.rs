///Register `ABR1` reader
pub type R = crate::R<ABR1rs>;
///Register `ABR1` writer
pub type W = crate::W<ABR1rs>;
///Field `ABYTE` reader - Alternate byte
pub type AbyteR = crate::FieldReader<u32>;
///Field `ABYTE` writer - Alternate byte
pub type AbyteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Alternate byte
    #[inline(always)]
    pub fn abyte(&self) -> AbyteR {
        AbyteR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ABR1")
            .field("abyte", &self.abyte())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Alternate byte
    #[inline(always)]
    pub fn abyte(&mut self) -> AbyteW<ABR1rs> {
        AbyteW::new(self, 0)
    }
}
///Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`abr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ABR1rs;
impl crate::RegisterSpec for ABR1rs {
    type Ux = u32;
}
///`read()` method returns [`abr1::R`](R) reader structure
impl crate::Readable for ABR1rs {}
///`write(|w| ..)` method takes [`abr1::W`](W) writer structure
impl crate::Writable for ABR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ABR1 to value 0
impl crate::Resettable for ABR1rs {
    const RESET_VALUE: u32 = 0;
}
