///Register `RX_CH0_ENTRY` reader
pub type R = crate::R<RX_CH0_ENTRYrs>;
///Register `RX_CH0_ENTRY` writer
pub type W = crate::W<RX_CH0_ENTRYrs>;
///Field `DATA` reader - rx channel 0 data entry
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - rx channel 0 data entry
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - rx channel 0 data entry
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH0_ENTRY")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - rx channel 0 data entry
    #[inline(always)]
    pub fn data(&mut self) -> DataW<RX_CH0_ENTRYrs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch0_entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch0_entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_CH0_ENTRYrs;
impl crate::RegisterSpec for RX_CH0_ENTRYrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ch0_entry::R`](R) reader structure
impl crate::Readable for RX_CH0_ENTRYrs {}
///`write(|w| ..)` method takes [`rx_ch0_entry::W`](W) writer structure
impl crate::Writable for RX_CH0_ENTRYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_CH0_ENTRY to value 0
impl crate::Resettable for RX_CH0_ENTRYrs {
    const RESET_VALUE: u32 = 0;
}
