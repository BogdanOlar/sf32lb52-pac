///Register `HWABR` reader
pub type R = crate::R<HWABRrs>;
///Register `HWABR` writer
pub type W = crate::W<HWABRrs>;
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
        f.debug_struct("HWABR")
            .field("abyte", &self.abyte())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn abyte(&mut self) -> AbyteW<HWABRrs> {
        AbyteW::new(self, 0)
    }
}
///AHB Write Alternate Byte Register
///
///You can [`read`](crate::Reg::read) this register and get [`hwabr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwabr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HWABRrs;
impl crate::RegisterSpec for HWABRrs {
    type Ux = u32;
}
///`read()` method returns [`hwabr::R`](R) reader structure
impl crate::Readable for HWABRrs {}
///`write(|w| ..)` method takes [`hwabr::W`](W) writer structure
impl crate::Writable for HWABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HWABR to value 0
impl crate::Resettable for HWABRrs {
    const RESET_VALUE: u32 = 0;
}
