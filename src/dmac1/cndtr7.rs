///Register `CNDTR7` reader
pub type R = crate::R<CNDTR7rs>;
///Register `CNDTR7` writer
pub type W = crate::W<CNDTR7rs>;
///Field `NDT` reader - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
pub type NdtR = crate::FieldReader<u16>;
///Field `NDT` writer - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR7")
            .field("rsvd", &self.rsvd())
            .field("ndt", &self.ndt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not).
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<CNDTR7rs> {
        NdtW::new(self, 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CNDTR7rs> {
        RsvdW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cndtr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CNDTR7rs;
impl crate::RegisterSpec for CNDTR7rs {
    type Ux = u32;
}
///`read()` method returns [`cndtr7::R`](R) reader structure
impl crate::Readable for CNDTR7rs {}
///`write(|w| ..)` method takes [`cndtr7::W`](W) writer structure
impl crate::Writable for CNDTR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNDTR7 to value 0
impl crate::Resettable for CNDTR7rs {
    const RESET_VALUE: u32 = 0;
}
