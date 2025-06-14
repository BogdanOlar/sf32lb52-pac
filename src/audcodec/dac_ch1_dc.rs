///Register `DAC_CH1_DC` reader
pub type R = crate::R<DAC_CH1_DCrs>;
///Register `DAC_CH1_DC` writer
pub type W = crate::W<DAC_CH1_DCrs>;
///Field `OFFSET` reader - dac ch1 dc offset
pub type OffsetR = crate::FieldReader<u32>;
///Field `OFFSET` writer - dac ch1 dc offset
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - dac ch1 dc offset
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH1_DC")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - dac ch1 dc offset
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<DAC_CH1_DCrs> {
        OffsetW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH1_DCrs;
impl crate::RegisterSpec for DAC_CH1_DCrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch1_dc::R`](R) reader structure
impl crate::Readable for DAC_CH1_DCrs {}
///`write(|w| ..)` method takes [`dac_ch1_dc::W`](W) writer structure
impl crate::Writable for DAC_CH1_DCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH1_DC to value 0
impl crate::Resettable for DAC_CH1_DCrs {
    const RESET_VALUE: u32 = 0;
}
