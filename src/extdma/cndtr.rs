///Register `CNDTR` reader
pub type R = crate::R<CNDTRrs>;
///Register `CNDTR` writer
pub type W = crate::W<CNDTRrs>;
///Field `NDT` reader - number of data to transfer (0 to 2^20 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached. If this field is zero, no transfer can be served whatever the channel enabled or not
pub type NdtR = crate::FieldReader<u32>;
///Field `NDT` writer - number of data to transfer (0 to 2^20 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached. If this field is zero, no transfer can be served whatever the channel enabled or not
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - number of data to transfer (0 to 2^20 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached. If this field is zero, no transfer can be served whatever the channel enabled or not
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:19 - number of data to transfer (0 to 2^20 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached. If this field is zero, no transfer can be served whatever the channel enabled or not
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<CNDTRrs> {
        NdtW::new(self, 0)
    }
}
///number of data register
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CNDTRrs;
impl crate::RegisterSpec for CNDTRrs {
    type Ux = u32;
}
///`read()` method returns [`cndtr::R`](R) reader structure
impl crate::Readable for CNDTRrs {}
///`write(|w| ..)` method takes [`cndtr::W`](W) writer structure
impl crate::Writable for CNDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNDTR to value 0
impl crate::Resettable for CNDTRrs {
    const RESET_VALUE: u32 = 0;
}
