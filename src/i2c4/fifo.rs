///Register `FIFO` reader
pub type R = crate::R<FIFOrs>;
///Register `FIFO` writer
pub type W = crate::W<FIFOrs>;
///Field `DATA` reader - Write to push send data into FIFO. Read to pop received data from FIFO
pub type DataR = crate::FieldReader;
///Field `DATA` writer - Write to push send data into FIFO. Read to pop received data from FIFO
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Write to push send data into FIFO. Read to pop received data from FIFO
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:7 - Write to push send data into FIFO. Read to pop received data from FIFO
    #[inline(always)]
    pub fn data(&mut self) -> DataW<FIFOrs> {
        DataW::new(self, 0)
    }
}
///FIFO Register
///
///You can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FIFOrs;
impl crate::RegisterSpec for FIFOrs {
    type Ux = u32;
}
///`read()` method returns [`fifo::R`](R) reader structure
impl crate::Readable for FIFOrs {}
///`write(|w| ..)` method takes [`fifo::W`](W) writer structure
impl crate::Writable for FIFOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO to value 0
impl crate::Resettable for FIFOrs {
    const RESET_VALUE: u32 = 0;
}
