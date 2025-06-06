///Register `DAC_CH1_DEBUG` reader
pub type R = crate::R<DAC_CH1_DEBUGrs>;
///Register `DAC_CH1_DEBUG` writer
pub type W = crate::W<DAC_CH1_DEBUGrs>;
///Field `DATA_OUT` reader - debug dac output
pub type DataOutR = crate::FieldReader<u16>;
///Field `DATA_OUT` writer - debug dac output
pub type DataOutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BYPASS` reader - debug bypass mode
pub type BypassR = crate::BitReader;
///Field `BYPASS` writer - debug bypass mode
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - debug dac output
    #[inline(always)]
    pub fn data_out(&self) -> DataOutR {
        DataOutR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - debug bypass mode
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH1_DEBUG")
            .field("bypass", &self.bypass())
            .field("data_out", &self.data_out())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - debug dac output
    #[inline(always)]
    pub fn data_out(&mut self) -> DataOutW<DAC_CH1_DEBUGrs> {
        DataOutW::new(self, 0)
    }
    ///Bit 16 - debug bypass mode
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<DAC_CH1_DEBUGrs> {
        BypassW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH1_DEBUGrs;
impl crate::RegisterSpec for DAC_CH1_DEBUGrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch1_debug::R`](R) reader structure
impl crate::Readable for DAC_CH1_DEBUGrs {}
///`write(|w| ..)` method takes [`dac_ch1_debug::W`](W) writer structure
impl crate::Writable for DAC_CH1_DEBUGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH1_DEBUG to value 0
impl crate::Resettable for DAC_CH1_DEBUGrs {
    const RESET_VALUE: u32 = 0;
}
