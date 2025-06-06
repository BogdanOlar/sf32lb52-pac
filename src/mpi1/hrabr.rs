///Register `HRABR` reader
pub type R = crate::R<HRABRrs>;
///Register `HRABR` writer
pub type W = crate::W<HRABRrs>;
///Field `ABYTE` reader -
pub type AbyteR = crate::FieldReader<u32>;
///Field `ABYTE` writer -
pub type AbyteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn abyte(&self) -> AbyteR {
        AbyteR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRABR")
            .field("abyte", &self.abyte())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn abyte(&mut self) -> AbyteW<HRABRrs> {
        AbyteW::new(self, 0)
    }
}
///AHB Read Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`hrabr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrabr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HRABRrs;
impl crate::RegisterSpec for HRABRrs {
    type Ux = u32;
}
///`read()` method returns [`hrabr::R`](R) reader structure
impl crate::Readable for HRABRrs {}
///`write(|w| ..)` method takes [`hrabr::W`](W) writer structure
impl crate::Writable for HRABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRABR to value 0
impl crate::Resettable for HRABRrs {
    const RESET_VALUE: u32 = 0;
}
