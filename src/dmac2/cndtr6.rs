///Register `CNDTR6` reader
pub type R = crate::R<CNDTR6rs>;
///Register `CNDTR6` writer
pub type W = crate::W<CNDTR6rs>;
///Field `NDT` reader - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
pub type NdtR = crate::FieldReader<u16>;
///Field `NDT` writer - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR6").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<CNDTR6rs> {
        NdtW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CNDTR6rs;
impl crate::RegisterSpec for CNDTR6rs {
    type Ux = u32;
}
///`read()` method returns [`cndtr6::R`](R) reader structure
impl crate::Readable for CNDTR6rs {}
///`write(|w| ..)` method takes [`cndtr6::W`](W) writer structure
impl crate::Writable for CNDTR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNDTR6 to value 0
impl crate::Resettable for CNDTR6rs {
    const RESET_VALUE: u32 = 0;
}
