///Register `BANK3_DATA1` reader
pub type R = crate::R<BANK3_DATA1rs>;
///Register `BANK3_DATA1` writer
pub type W = crate::W<BANK3_DATA1rs>;
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
        f.debug_struct("BANK3_DATA1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn data(&mut self) -> DataW<BANK3_DATA1rs> {
        DataW::new(self, 0)
    }
}
///Bank3 Data1
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BANK3_DATA1rs;
impl crate::RegisterSpec for BANK3_DATA1rs {
    type Ux = u32;
}
///`read()` method returns [`bank3_data1::R`](R) reader structure
impl crate::Readable for BANK3_DATA1rs {}
///`write(|w| ..)` method takes [`bank3_data1::W`](W) writer structure
impl crate::Writable for BANK3_DATA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BANK3_DATA1 to value 0
impl crate::Resettable for BANK3_DATA1rs {
    const RESET_VALUE: u32 = 0;
}
