///Register `BANK3_DATA0` reader
pub type R = crate::R<BANK3_DATA0rs>;
///Register `BANK3_DATA0` writer
pub type W = crate::W<BANK3_DATA0rs>;
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
        f.debug_struct("BANK3_DATA0")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn data(&mut self) -> DataW<BANK3_DATA0rs> {
        DataW::new(self, 0)
    }
}
///Bank3 Data0
///
///You can [`read`](crate::Reg::read) this register and get [`bank3_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank3_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BANK3_DATA0rs;
impl crate::RegisterSpec for BANK3_DATA0rs {
    type Ux = u32;
}
///`read()` method returns [`bank3_data0::R`](R) reader structure
impl crate::Readable for BANK3_DATA0rs {}
///`write(|w| ..)` method takes [`bank3_data0::W`](W) writer structure
impl crate::Writable for BANK3_DATA0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BANK3_DATA0 to value 0
impl crate::Resettable for BANK3_DATA0rs {
    const RESET_VALUE: u32 = 0;
}
