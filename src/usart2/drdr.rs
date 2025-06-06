///Register `DRDR` reader
pub type R = crate::R<DRDRrs>;
///Register `DRDR` writer
pub type W = crate::W<DRDRrs>;
///Field `DATA` reader -
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer -
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRDR").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DRDRrs> {
        DataW::new(self, 0)
    }
}
///Debug Receive Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`drdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DRDRrs;
impl crate::RegisterSpec for DRDRrs {
    type Ux = u32;
}
///`read()` method returns [`drdr::R`](R) reader structure
impl crate::Readable for DRDRrs {}
///`write(|w| ..)` method takes [`drdr::W`](W) writer structure
impl crate::Writable for DRDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DRDR to value 0
impl crate::Resettable for DRDRrs {
    const RESET_VALUE: u32 = 0;
}
