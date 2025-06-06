///Register `APB_STAT` reader
pub type R = crate::R<APB_STATrs>;
///Register `APB_STAT` writer
pub type W = crate::W<APB_STATrs>;
///Field `DAC_CH0_FIFO_CNT` reader -
pub type DacCh0FifoCntR = crate::FieldReader;
///Field `DAC_CH0_FIFO_CNT` writer -
pub type DacCh0FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DAC_CH1_FIFO_CNT` reader -
pub type DacCh1FifoCntR = crate::FieldReader;
///Field `DAC_CH1_FIFO_CNT` writer -
pub type DacCh1FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADC_CH0_FIFO_CNT` reader -
pub type AdcCh0FifoCntR = crate::FieldReader;
///Field `ADC_CH0_FIFO_CNT` writer -
pub type AdcCh0FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADC_CH1_FIFO_CNT` reader -
pub type AdcCh1FifoCntR = crate::FieldReader;
///Field `ADC_CH1_FIFO_CNT` writer -
pub type AdcCh1FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn dac_ch0_fifo_cnt(&self) -> DacCh0FifoCntR {
        DacCh0FifoCntR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn dac_ch1_fifo_cnt(&self) -> DacCh1FifoCntR {
        DacCh1FifoCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19
    #[inline(always)]
    pub fn adc_ch0_fifo_cnt(&self) -> AdcCh0FifoCntR {
        AdcCh0FifoCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23
    #[inline(always)]
    pub fn adc_ch1_fifo_cnt(&self) -> AdcCh1FifoCntR {
        AdcCh1FifoCntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_STAT")
            .field("rsvd", &self.rsvd())
            .field("adc_ch1_fifo_cnt", &self.adc_ch1_fifo_cnt())
            .field("adc_ch0_fifo_cnt", &self.adc_ch0_fifo_cnt())
            .field("rsvd2", &self.rsvd2())
            .field("dac_ch1_fifo_cnt", &self.dac_ch1_fifo_cnt())
            .field("dac_ch0_fifo_cnt", &self.dac_ch0_fifo_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn dac_ch0_fifo_cnt(&mut self) -> DacCh0FifoCntW<APB_STATrs> {
        DacCh0FifoCntW::new(self, 0)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn dac_ch1_fifo_cnt(&mut self) -> DacCh1FifoCntW<APB_STATrs> {
        DacCh1FifoCntW::new(self, 4)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<APB_STATrs> {
        Rsvd2W::new(self, 8)
    }
    ///Bits 16:19
    #[inline(always)]
    pub fn adc_ch0_fifo_cnt(&mut self) -> AdcCh0FifoCntW<APB_STATrs> {
        AdcCh0FifoCntW::new(self, 16)
    }
    ///Bits 20:23
    #[inline(always)]
    pub fn adc_ch1_fifo_cnt(&mut self) -> AdcCh1FifoCntW<APB_STATrs> {
        AdcCh1FifoCntW::new(self, 20)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<APB_STATrs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`apb_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct APB_STATrs;
impl crate::RegisterSpec for APB_STATrs {
    type Ux = u32;
}
///`read()` method returns [`apb_stat::R`](R) reader structure
impl crate::Readable for APB_STATrs {}
///`write(|w| ..)` method takes [`apb_stat::W`](W) writer structure
impl crate::Writable for APB_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_STAT to value 0
impl crate::Resettable for APB_STATrs {
    const RESET_VALUE: u32 = 0;
}
