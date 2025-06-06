///Register `TX_CH3_ENTRY` reader
pub type R = crate::R<TX_CH3_ENTRYrs>;
///Register `TX_CH3_ENTRY` writer
pub type W = crate::W<TX_CH3_ENTRYrs>;
///Field `DATA` reader - tx channel 3 data entry
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - tx channel 3 data entry
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - tx channel 3 data entry
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH3_ENTRY")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - tx channel 3 data entry
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TX_CH3_ENTRYrs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_ch3_entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch3_entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_CH3_ENTRYrs;
impl crate::RegisterSpec for TX_CH3_ENTRYrs {
    type Ux = u32;
}
///`read()` method returns [`tx_ch3_entry::R`](R) reader structure
impl crate::Readable for TX_CH3_ENTRYrs {}
///`write(|w| ..)` method takes [`tx_ch3_entry::W`](W) writer structure
impl crate::Writable for TX_CH3_ENTRYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CH3_ENTRY to value 0
impl crate::Resettable for TX_CH3_ENTRYrs {
    const RESET_VALUE: u32 = 0;
}
