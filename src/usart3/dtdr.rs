///Register `DTDR` reader
pub type R = crate::R<DTDRrs>;
///Register `DTDR` writer
pub type W = crate::W<DTDRrs>;
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
        f.debug_struct("DTDR").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DTDRrs> {
        DataW::new(self, 0)
    }
}
///Debug Receive Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`dtdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTDRrs;
impl crate::RegisterSpec for DTDRrs {
    type Ux = u32;
}
///`read()` method returns [`dtdr::R`](R) reader structure
impl crate::Readable for DTDRrs {}
///`write(|w| ..)` method takes [`dtdr::W`](W) writer structure
impl crate::Writable for DTDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTDR to value 0
impl crate::Resettable for DTDRrs {
    const RESET_VALUE: u32 = 0;
}
