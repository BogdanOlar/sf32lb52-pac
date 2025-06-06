///Register `PGM_DATA6` reader
pub type R = crate::R<PGM_DATA6rs>;
///Register `PGM_DATA6` writer
pub type W = crate::W<PGM_DATA6rs>;
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
        f.debug_struct("PGM_DATA6")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    pub fn data(&mut self) -> DataW<PGM_DATA6rs> {
        DataW::new(self, 0)
    }
}
///Program Data6
///
///You can [`read`](crate::Reg::read) this register and get [`pgm_data6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PGM_DATA6rs;
impl crate::RegisterSpec for PGM_DATA6rs {
    type Ux = u32;
}
///`read()` method returns [`pgm_data6::R`](R) reader structure
impl crate::Readable for PGM_DATA6rs {}
///`write(|w| ..)` method takes [`pgm_data6::W`](W) writer structure
impl crate::Writable for PGM_DATA6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PGM_DATA6 to value 0
impl crate::Resettable for PGM_DATA6rs {
    const RESET_VALUE: u32 = 0;
}
