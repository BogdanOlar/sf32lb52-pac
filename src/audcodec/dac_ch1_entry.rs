///Register `DAC_CH1_ENTRY` reader
pub type R = crate::R<DAC_CH1_ENTRYrs>;
///Register `DAC_CH1_ENTRY` writer
pub type W = crate::W<DAC_CH1_ENTRYrs>;
///Field `DATA` reader - dac channel0 data input
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - dac channel0 data input
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - dac channel0 data input
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH1_ENTRY")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - dac channel0 data input
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DAC_CH1_ENTRYrs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH1_ENTRYrs;
impl crate::RegisterSpec for DAC_CH1_ENTRYrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch1_entry::R`](R) reader structure
impl crate::Readable for DAC_CH1_ENTRYrs {}
///`write(|w| ..)` method takes [`dac_ch1_entry::W`](W) writer structure
impl crate::Writable for DAC_CH1_ENTRYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH1_ENTRY to value 0
impl crate::Resettable for DAC_CH1_ENTRYrs {
    const RESET_VALUE: u32 = 0;
}
