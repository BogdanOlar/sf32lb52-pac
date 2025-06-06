///Register `ADC_CTRL_REG` reader
pub type R = crate::R<ADC_CTRL_REGrs>;
///Register `ADC_CTRL_REG` writer
pub type W = crate::W<ADC_CTRL_REGrs>;
///Field `ADC_OP_MODE` reader - 0: single conversion mode 1: continuous conversion mode
pub type AdcOpModeR = crate::BitReader;
///Field `ADC_OP_MODE` writer - 0: single conversion mode 1: continuous conversion mode
pub type AdcOpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_START` reader - Write 1 to start GPADC,(don't need clear )
pub type AdcStartR = crate::BitReader;
///Field `ADC_START` writer - Write 1 to start GPADC,(don't need clear )
pub type AdcStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_STOP` reader - Write 1 to stop GPADC in continuous mode(need write 0 to clear)
pub type AdcStopR = crate::BitReader;
///Field `ADC_STOP` writer - Write 1 to stop GPADC in continuous mode(need write 0 to clear)
pub type AdcStopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INIT_TIME` reader - GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged
pub type InitTimeR = crate::FieldReader;
///Field `INIT_TIME` writer - GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged
pub type InitTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMA_EN` reader - Enable DMA interface
pub type DmaEnR = crate::BitReader;
///Field `DMA_EN` writer - Enable DMA interface
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_TRIG_EN` reader - Enable timer trigger function
pub type TimerTrigEnR = crate::BitReader;
///Field `TIMER_TRIG_EN` writer - Enable timer trigger function
pub type TimerTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHNL_SEL_FRC_EN` reader - Enable input channel setting in ADC_CFG_REG1
pub type ChnlSelFrcEnR = crate::BitReader;
///Field `CHNL_SEL_FRC_EN` writer - Enable input channel setting in ADC_CFG_REG1
pub type ChnlSelFrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRC_EN_ADC` reader - Enable GPADC core
pub type FrcEnAdcR = crate::BitReader;
///Field `FRC_EN_ADC` writer - Enable GPADC core
pub type FrcEnAdcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_TRIG_SRC_SEL` reader - Timer trigger source select
pub type TimerTrigSrcSelR = crate::FieldReader;
///Field `TIMER_TRIG_SRC_SEL` writer - Timer trigger source select
pub type TimerTrigSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TIMER_TRIG_TYP` reader - 0: pulse no edge detect needed 1: level,need edge detect
pub type TimerTrigTypR = crate::BitReader;
///Field `TIMER_TRIG_TYP` writer - 0: pulse no edge detect needed 1: level,need edge detect
pub type TimerTrigTypW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_DATA_SEL` reader - 0: combined data 1: raw data
pub type DmaDataSelR = crate::BitReader;
///Field `DMA_DATA_SEL` writer - 0: combined data 1: raw data
pub type DmaDataSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_SAMP_DLY` reader -
pub type DataSampDlyR = crate::FieldReader;
///Field `DATA_SAMP_DLY` writer -
pub type DataSampDlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - 0: single conversion mode 1: continuous conversion mode
    #[inline(always)]
    pub fn adc_op_mode(&self) -> AdcOpModeR {
        AdcOpModeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write 1 to start GPADC,(don't need clear )
    #[inline(always)]
    pub fn adc_start(&self) -> AdcStartR {
        AdcStartR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write 1 to stop GPADC in continuous mode(need write 0 to clear)
    #[inline(always)]
    pub fn adc_stop(&self) -> AdcStopR {
        AdcStopR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged
    #[inline(always)]
    pub fn init_time(&self) -> InitTimeR {
        InitTimeR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bit 7 - Enable DMA interface
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Enable timer trigger function
    #[inline(always)]
    pub fn timer_trig_en(&self) -> TimerTrigEnR {
        TimerTrigEnR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable input channel setting in ADC_CFG_REG1
    #[inline(always)]
    pub fn chnl_sel_frc_en(&self) -> ChnlSelFrcEnR {
        ChnlSelFrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable GPADC core
    #[inline(always)]
    pub fn frc_en_adc(&self) -> FrcEnAdcR {
        FrcEnAdcR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Timer trigger source select
    #[inline(always)]
    pub fn timer_trig_src_sel(&self) -> TimerTrigSrcSelR {
        TimerTrigSrcSelR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - 0: pulse no edge detect needed 1: level,need edge detect
    #[inline(always)]
    pub fn timer_trig_typ(&self) -> TimerTrigTypR {
        TimerTrigTypR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 0: combined data 1: raw data
    #[inline(always)]
    pub fn dma_data_sel(&self) -> DmaDataSelR {
        DmaDataSelR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20
    #[inline(always)]
    pub fn data_samp_dly(&self) -> DataSampDlyR {
        DataSampDlyR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CTRL_REG")
            .field("data_samp_dly", &self.data_samp_dly())
            .field("dma_data_sel", &self.dma_data_sel())
            .field("timer_trig_typ", &self.timer_trig_typ())
            .field("timer_trig_src_sel", &self.timer_trig_src_sel())
            .field("frc_en_adc", &self.frc_en_adc())
            .field("chnl_sel_frc_en", &self.chnl_sel_frc_en())
            .field("timer_trig_en", &self.timer_trig_en())
            .field("dma_en", &self.dma_en())
            .field("init_time", &self.init_time())
            .field("adc_stop", &self.adc_stop())
            .field("adc_start", &self.adc_start())
            .field("adc_op_mode", &self.adc_op_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: single conversion mode 1: continuous conversion mode
    #[inline(always)]
    pub fn adc_op_mode(&mut self) -> AdcOpModeW<ADC_CTRL_REGrs> {
        AdcOpModeW::new(self, 0)
    }
    ///Bit 1 - Write 1 to start GPADC,(don't need clear )
    #[inline(always)]
    pub fn adc_start(&mut self) -> AdcStartW<ADC_CTRL_REGrs> {
        AdcStartW::new(self, 1)
    }
    ///Bit 2 - Write 1 to stop GPADC in continuous mode(need write 0 to clear)
    #[inline(always)]
    pub fn adc_stop(&mut self) -> AdcStopW<ADC_CTRL_REGrs> {
        AdcStopW::new(self, 2)
    }
    ///Bits 3:6 - GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged
    #[inline(always)]
    pub fn init_time(&mut self) -> InitTimeW<ADC_CTRL_REGrs> {
        InitTimeW::new(self, 3)
    }
    ///Bit 7 - Enable DMA interface
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<ADC_CTRL_REGrs> {
        DmaEnW::new(self, 7)
    }
    ///Bit 9 - Enable timer trigger function
    #[inline(always)]
    pub fn timer_trig_en(&mut self) -> TimerTrigEnW<ADC_CTRL_REGrs> {
        TimerTrigEnW::new(self, 9)
    }
    ///Bit 10 - Enable input channel setting in ADC_CFG_REG1
    #[inline(always)]
    pub fn chnl_sel_frc_en(&mut self) -> ChnlSelFrcEnW<ADC_CTRL_REGrs> {
        ChnlSelFrcEnW::new(self, 10)
    }
    ///Bit 11 - Enable GPADC core
    #[inline(always)]
    pub fn frc_en_adc(&mut self) -> FrcEnAdcW<ADC_CTRL_REGrs> {
        FrcEnAdcW::new(self, 11)
    }
    ///Bits 12:14 - Timer trigger source select
    #[inline(always)]
    pub fn timer_trig_src_sel(&mut self) -> TimerTrigSrcSelW<ADC_CTRL_REGrs> {
        TimerTrigSrcSelW::new(self, 12)
    }
    ///Bit 15 - 0: pulse no edge detect needed 1: level,need edge detect
    #[inline(always)]
    pub fn timer_trig_typ(&mut self) -> TimerTrigTypW<ADC_CTRL_REGrs> {
        TimerTrigTypW::new(self, 15)
    }
    ///Bit 16 - 0: combined data 1: raw data
    #[inline(always)]
    pub fn dma_data_sel(&mut self) -> DmaDataSelW<ADC_CTRL_REGrs> {
        DmaDataSelW::new(self, 16)
    }
    ///Bits 17:20
    #[inline(always)]
    pub fn data_samp_dly(&mut self) -> DataSampDlyW<ADC_CTRL_REGrs> {
        DataSampDlyW::new(self, 17)
    }
}
///ADC Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ctrl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_CTRL_REGrs;
impl crate::RegisterSpec for ADC_CTRL_REGrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ctrl_reg::R`](R) reader structure
impl crate::Readable for ADC_CTRL_REGrs {}
///`write(|w| ..)` method takes [`adc_ctrl_reg::W`](W) writer structure
impl crate::Writable for ADC_CTRL_REGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CTRL_REG to value 0x0130
impl crate::Resettable for ADC_CTRL_REGrs {
    const RESET_VALUE: u32 = 0x0130;
}
