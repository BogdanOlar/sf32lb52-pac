///Register `IRQ` reader
pub type R = crate::R<IRQrs>;
///Register `IRQ` writer
pub type W = crate::W<IRQrs>;
///Field `DAC_CH0_APB_OF` reader - dac ch0 apb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh0ApbOfR = crate::BitReader;
///Field `DAC_CH0_APB_OF` writer - dac ch0 apb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh0ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH0_OUT_UF` reader - dac ch0 output fifo underflow interrupt status. Write 1 to clear.
pub type DacCh0OutUfR = crate::BitReader;
///Field `DAC_CH0_OUT_UF` writer - dac ch0 output fifo underflow interrupt status. Write 1 to clear.
pub type DacCh0OutUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH0_STB_OF` reader - dac ch0 input stb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh0StbOfR = crate::BitReader;
///Field `DAC_CH0_STB_OF` writer - dac ch0 input stb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh0StbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH1_APB_OF` reader - dac ch1 apb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh1ApbOfR = crate::BitReader;
///Field `DAC_CH1_APB_OF` writer - dac ch1 apb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh1ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH1_OUT_UF` reader - dac ch1 output fifo underflow interrupt status. Write 1 to clear.
pub type DacCh1OutUfR = crate::BitReader;
///Field `DAC_CH1_OUT_UF` writer - dac ch1 output fifo underflow interrupt status. Write 1 to clear.
pub type DacCh1OutUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH1_STB_OF` reader - dac ch1 input stb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh1StbOfR = crate::BitReader;
///Field `DAC_CH1_STB_OF` writer - dac ch1 input stb fifo overflow interrupt status. Write 1 to clear.
pub type DacCh1StbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader<u16>;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `ADC_CH0_APB_OF` reader - adc ch0 apb fifo overflow interrupt status. Write 1 to clear.
pub type AdcCh0ApbOfR = crate::BitReader;
///Field `ADC_CH0_APB_OF` writer - adc ch0 apb fifo overflow interrupt status. Write 1 to clear.
pub type AdcCh0ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH0_APB_UF` reader - adc ch0 apb fifo underflow interrupt status. Write 1 to clear.
pub type AdcCh0ApbUfR = crate::BitReader;
///Field `ADC_CH0_APB_UF` writer - adc ch0 apb fifo underflow interrupt status. Write 1 to clear.
pub type AdcCh0ApbUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH0_SAT` reader - adc ch0 saturation interrupt
pub type AdcCh0SatR = crate::BitReader;
///Field `ADC_CH0_SAT` writer - adc ch0 saturation interrupt
pub type AdcCh0SatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH1_APB_OF` reader - adc ch1 apb fifo overflow interrupt status. Write 1 to clear.
pub type AdcCh1ApbOfR = crate::BitReader;
///Field `ADC_CH1_APB_OF` writer - adc ch1 apb fifo overflow interrupt status. Write 1 to clear.
pub type AdcCh1ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH1_APB_UF` reader - adc ch1 apb fifo underflow interrupt status. Write 1 to clear.
pub type AdcCh1ApbUfR = crate::BitReader;
///Field `ADC_CH1_APB_UF` writer - adc ch1 apb fifo underflow interrupt status. Write 1 to clear.
pub type AdcCh1ApbUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH1_SAT` reader - adc ch1 saturation interrupt
pub type AdcCh1SatR = crate::BitReader;
///Field `ADC_CH1_SAT` writer - adc ch1 saturation interrupt
pub type AdcCh1SatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bit 0 - dac ch0 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch0_apb_of(&self) -> DacCh0ApbOfR {
        DacCh0ApbOfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - dac ch0 output fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch0_out_uf(&self) -> DacCh0OutUfR {
        DacCh0OutUfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dac ch0 input stb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch0_stb_of(&self) -> DacCh0StbOfR {
        DacCh0StbOfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - dac ch1 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch1_apb_of(&self) -> DacCh1ApbOfR {
        DacCh1ApbOfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - dac ch1 output fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch1_out_uf(&self) -> DacCh1OutUfR {
        DacCh1OutUfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - dac ch1 input stb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch1_stb_of(&self) -> DacCh1StbOfR {
        DacCh1StbOfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    ///Bit 16 - adc ch0 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch0_apb_of(&self) -> AdcCh0ApbOfR {
        AdcCh0ApbOfR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - adc ch0 apb fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch0_apb_uf(&self) -> AdcCh0ApbUfR {
        AdcCh0ApbUfR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - adc ch0 saturation interrupt
    #[inline(always)]
    pub fn adc_ch0_sat(&self) -> AdcCh0SatR {
        AdcCh0SatR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - adc ch1 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch1_apb_of(&self) -> AdcCh1ApbOfR {
        AdcCh1ApbOfR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - adc ch1 apb fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch1_apb_uf(&self) -> AdcCh1ApbUfR {
        AdcCh1ApbUfR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - adc ch1 saturation interrupt
    #[inline(always)]
    pub fn adc_ch1_sat(&self) -> AdcCh1SatR {
        AdcCh1SatR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("rsvd", &self.rsvd())
            .field("adc_ch1_sat", &self.adc_ch1_sat())
            .field("adc_ch1_apb_uf", &self.adc_ch1_apb_uf())
            .field("adc_ch1_apb_of", &self.adc_ch1_apb_of())
            .field("adc_ch0_sat", &self.adc_ch0_sat())
            .field("adc_ch0_apb_uf", &self.adc_ch0_apb_uf())
            .field("adc_ch0_apb_of", &self.adc_ch0_apb_of())
            .field("rsvd2", &self.rsvd2())
            .field("dac_ch1_stb_of", &self.dac_ch1_stb_of())
            .field("dac_ch1_out_uf", &self.dac_ch1_out_uf())
            .field("dac_ch1_apb_of", &self.dac_ch1_apb_of())
            .field("dac_ch0_stb_of", &self.dac_ch0_stb_of())
            .field("dac_ch0_out_uf", &self.dac_ch0_out_uf())
            .field("dac_ch0_apb_of", &self.dac_ch0_apb_of())
            .finish()
    }
}
impl W {
    ///Bit 0 - dac ch0 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch0_apb_of(&mut self) -> DacCh0ApbOfW<IRQrs> {
        DacCh0ApbOfW::new(self, 0)
    }
    ///Bit 1 - dac ch0 output fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch0_out_uf(&mut self) -> DacCh0OutUfW<IRQrs> {
        DacCh0OutUfW::new(self, 1)
    }
    ///Bit 2 - dac ch0 input stb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch0_stb_of(&mut self) -> DacCh0StbOfW<IRQrs> {
        DacCh0StbOfW::new(self, 2)
    }
    ///Bit 3 - dac ch1 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch1_apb_of(&mut self) -> DacCh1ApbOfW<IRQrs> {
        DacCh1ApbOfW::new(self, 3)
    }
    ///Bit 4 - dac ch1 output fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch1_out_uf(&mut self) -> DacCh1OutUfW<IRQrs> {
        DacCh1OutUfW::new(self, 4)
    }
    ///Bit 5 - dac ch1 input stb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn dac_ch1_stb_of(&mut self) -> DacCh1StbOfW<IRQrs> {
        DacCh1StbOfW::new(self, 5)
    }
    ///Bits 6:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<IRQrs> {
        Rsvd2W::new(self, 6)
    }
    ///Bit 16 - adc ch0 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch0_apb_of(&mut self) -> AdcCh0ApbOfW<IRQrs> {
        AdcCh0ApbOfW::new(self, 16)
    }
    ///Bit 17 - adc ch0 apb fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch0_apb_uf(&mut self) -> AdcCh0ApbUfW<IRQrs> {
        AdcCh0ApbUfW::new(self, 17)
    }
    ///Bit 18 - adc ch0 saturation interrupt
    #[inline(always)]
    pub fn adc_ch0_sat(&mut self) -> AdcCh0SatW<IRQrs> {
        AdcCh0SatW::new(self, 18)
    }
    ///Bit 19 - adc ch1 apb fifo overflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch1_apb_of(&mut self) -> AdcCh1ApbOfW<IRQrs> {
        AdcCh1ApbOfW::new(self, 19)
    }
    ///Bit 20 - adc ch1 apb fifo underflow interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn adc_ch1_apb_uf(&mut self) -> AdcCh1ApbUfW<IRQrs> {
        AdcCh1ApbUfW::new(self, 20)
    }
    ///Bit 21 - adc ch1 saturation interrupt
    #[inline(always)]
    pub fn adc_ch1_sat(&mut self) -> AdcCh1SatW<IRQrs> {
        AdcCh1SatW::new(self, 21)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IRQrs> {
        RsvdW::new(self, 22)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRQrs;
impl crate::RegisterSpec for IRQrs {
    type Ux = u32;
}
///`read()` method returns [`irq::R`](R) reader structure
impl crate::Readable for IRQrs {}
///`write(|w| ..)` method takes [`irq::W`](W) writer structure
impl crate::Writable for IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ to value 0
impl crate::Resettable for IRQrs {
    const RESET_VALUE: u32 = 0;
}
