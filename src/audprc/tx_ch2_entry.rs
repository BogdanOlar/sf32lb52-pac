///Register `TX_CH2_ENTRY` reader
pub type R = crate::R<TX_CH2_ENTRYrs>;
///Register `TX_CH2_ENTRY` writer
pub type W = crate::W<TX_CH2_ENTRYrs>;
///Field `DATA` reader - tx channel 2 data entry
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - tx channel 2 data entry
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - tx channel 2 data entry
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH2_ENTRY")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - tx channel 2 data entry
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TX_CH2_ENTRYrs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch2_entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch2_entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_CH2_ENTRYrs;
impl crate::RegisterSpec for TX_CH2_ENTRYrs {
    type Ux = u32;
}
///`read()` method returns [`tx_ch2_entry::R`](R) reader structure
impl crate::Readable for TX_CH2_ENTRYrs {}
///`write(|w| ..)` method takes [`tx_ch2_entry::W`](W) writer structure
impl crate::Writable for TX_CH2_ENTRYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CH2_ENTRY to value 0
impl crate::Resettable for TX_CH2_ENTRYrs {
    const RESET_VALUE: u32 = 0;
}
