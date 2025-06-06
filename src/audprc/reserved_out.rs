///Register `RESERVED_OUT` reader
pub type R = crate::R<RESERVED_OUTrs>;
///Register `RESERVED_OUT` writer
pub type W = crate::W<RESERVED_OUTrs>;
///Field `STAT` reader - reserved status
pub type StatR = crate::FieldReader;
///Field `STAT` writer - reserved status
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - reserved status
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED_OUT")
            .field("stat", &self.stat())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reserved status
    #[inline(always)]
    pub fn stat(&mut self) -> StatW<RESERVED_OUTrs> {
        StatW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVED_OUTrs;
impl crate::RegisterSpec for RESERVED_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`reserved_out::R`](R) reader structure
impl crate::Readable for RESERVED_OUTrs {}
///`write(|w| ..)` method takes [`reserved_out::W`](W) writer structure
impl crate::Writable for RESERVED_OUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVED_OUT to value 0
impl crate::Resettable for RESERVED_OUTrs {
    const RESET_VALUE: u32 = 0;
}
