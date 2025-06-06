///Register `DATA` reader
pub type R = crate::R<DATArs>;
///Register `DATA` writer
pub type W = crate::W<DATArs>;
///Field `DATA` reader - DATA This field is used for data to be written to the TXFIFO read from the RXFIFO.
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - DATA This field is used for data to be written to the TXFIFO read from the RXFIFO.
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATA This field is used for data to be written to the TXFIFO read from the RXFIFO.
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - DATA This field is used for data to be written to the TXFIFO read from the RXFIFO.
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DATArs> {
        DataW::new(self, 0)
    }
}
///SPI DATA Register
///
///You can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DATArs;
impl crate::RegisterSpec for DATArs {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DATArs {}
///`write(|w| ..)` method takes [`data::W`](W) writer structure
impl crate::Writable for DATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA to value 0
impl crate::Resettable for DATArs {
    const RESET_VALUE: u32 = 0;
}
