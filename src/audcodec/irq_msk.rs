///Register `IRQ_MSK` reader
pub type R = crate::R<IRQ_MSKrs>;
///Register `IRQ_MSK` writer
pub type W = crate::W<IRQ_MSKrs>;
///Field `DAC_CH0_APB_OF` reader - interrupt mask. 0: mask the interrupt.
pub type DacCh0ApbOfR = crate::BitReader;
///Field `DAC_CH0_APB_OF` writer - interrupt mask. 0: mask the interrupt.
pub type DacCh0ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH0_OUT_UF` reader - interrupt mask. 0: mask the interrupt.
pub type DacCh0OutUfR = crate::BitReader;
///Field `DAC_CH0_OUT_UF` writer - interrupt mask. 0: mask the interrupt.
pub type DacCh0OutUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH0_STB_OF` reader - interrupt mask. 0: mask the interrupt.
pub type DacCh0StbOfR = crate::BitReader;
///Field `DAC_CH0_STB_OF` writer - interrupt mask. 0: mask the interrupt.
pub type DacCh0StbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH1_APB_OF` reader - interrupt mask. 0: mask the interrupt.
pub type DacCh1ApbOfR = crate::BitReader;
///Field `DAC_CH1_APB_OF` writer - interrupt mask. 0: mask the interrupt.
pub type DacCh1ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH1_OUT_UF` reader - interrupt mask. 0: mask the interrupt.
pub type DacCh1OutUfR = crate::BitReader;
///Field `DAC_CH1_OUT_UF` writer - interrupt mask. 0: mask the interrupt.
pub type DacCh1OutUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CH1_STB_OF` reader - interrupt mask. 0: mask the interrupt.
pub type DacCh1StbOfR = crate::BitReader;
///Field `DAC_CH1_STB_OF` writer - interrupt mask. 0: mask the interrupt.
pub type DacCh1StbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader<u16>;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `ADC_CH0_APB_OF` reader - interrupt mask. 0: mask the interrupt.
pub type AdcCh0ApbOfR = crate::BitReader;
///Field `ADC_CH0_APB_OF` writer - interrupt mask. 0: mask the interrupt.
pub type AdcCh0ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH0_APB_UF` reader - interrupt mask. 0: mask the interrupt.
pub type AdcCh0ApbUfR = crate::BitReader;
///Field `ADC_CH0_APB_UF` writer - interrupt mask. 0: mask the interrupt.
pub type AdcCh0ApbUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH0_SAT` reader - interrupt mask. 0: mask the interrupt.
pub type AdcCh0SatR = crate::BitReader;
///Field `ADC_CH0_SAT` writer - interrupt mask. 0: mask the interrupt.
pub type AdcCh0SatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH1_APB_OF` reader - interrupt mask. 0: mask the interrupt.
pub type AdcCh1ApbOfR = crate::BitReader;
///Field `ADC_CH1_APB_OF` writer - interrupt mask. 0: mask the interrupt.
pub type AdcCh1ApbOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH1_APB_UF` reader - interrupt mask. 0: mask the interrupt.
pub type AdcCh1ApbUfR = crate::BitReader;
///Field `ADC_CH1_APB_UF` writer - interrupt mask. 0: mask the interrupt.
pub type AdcCh1ApbUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CH1_SAT` reader - interrupt mask. 0: mask the interrupt.
pub type AdcCh1SatR = crate::BitReader;
///Field `ADC_CH1_SAT` writer - interrupt mask. 0: mask the interrupt.
pub type AdcCh1SatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bit 0 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch0_apb_of(&self) -> DacCh0ApbOfR {
        DacCh0ApbOfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch0_out_uf(&self) -> DacCh0OutUfR {
        DacCh0OutUfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch0_stb_of(&self) -> DacCh0StbOfR {
        DacCh0StbOfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch1_apb_of(&self) -> DacCh1ApbOfR {
        DacCh1ApbOfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch1_out_uf(&self) -> DacCh1OutUfR {
        DacCh1OutUfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch1_stb_of(&self) -> DacCh1StbOfR {
        DacCh1StbOfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    ///Bit 16 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch0_apb_of(&self) -> AdcCh0ApbOfR {
        AdcCh0ApbOfR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch0_apb_uf(&self) -> AdcCh0ApbUfR {
        AdcCh0ApbUfR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch0_sat(&self) -> AdcCh0SatR {
        AdcCh0SatR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch1_apb_of(&self) -> AdcCh1ApbOfR {
        AdcCh1ApbOfR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch1_apb_uf(&self) -> AdcCh1ApbUfR {
        AdcCh1ApbUfR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - interrupt mask. 0: mask the interrupt.
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
        f.debug_struct("IRQ_MSK")
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
    ///Bit 0 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch0_apb_of(&mut self) -> DacCh0ApbOfW<IRQ_MSKrs> {
        DacCh0ApbOfW::new(self, 0)
    }
    ///Bit 1 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch0_out_uf(&mut self) -> DacCh0OutUfW<IRQ_MSKrs> {
        DacCh0OutUfW::new(self, 1)
    }
    ///Bit 2 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch0_stb_of(&mut self) -> DacCh0StbOfW<IRQ_MSKrs> {
        DacCh0StbOfW::new(self, 2)
    }
    ///Bit 3 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch1_apb_of(&mut self) -> DacCh1ApbOfW<IRQ_MSKrs> {
        DacCh1ApbOfW::new(self, 3)
    }
    ///Bit 4 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch1_out_uf(&mut self) -> DacCh1OutUfW<IRQ_MSKrs> {
        DacCh1OutUfW::new(self, 4)
    }
    ///Bit 5 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn dac_ch1_stb_of(&mut self) -> DacCh1StbOfW<IRQ_MSKrs> {
        DacCh1StbOfW::new(self, 5)
    }
    ///Bits 6:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<IRQ_MSKrs> {
        Rsvd2W::new(self, 6)
    }
    ///Bit 16 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch0_apb_of(&mut self) -> AdcCh0ApbOfW<IRQ_MSKrs> {
        AdcCh0ApbOfW::new(self, 16)
    }
    ///Bit 17 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch0_apb_uf(&mut self) -> AdcCh0ApbUfW<IRQ_MSKrs> {
        AdcCh0ApbUfW::new(self, 17)
    }
    ///Bit 18 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch0_sat(&mut self) -> AdcCh0SatW<IRQ_MSKrs> {
        AdcCh0SatW::new(self, 18)
    }
    ///Bit 19 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch1_apb_of(&mut self) -> AdcCh1ApbOfW<IRQ_MSKrs> {
        AdcCh1ApbOfW::new(self, 19)
    }
    ///Bit 20 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch1_apb_uf(&mut self) -> AdcCh1ApbUfW<IRQ_MSKrs> {
        AdcCh1ApbUfW::new(self, 20)
    }
    ///Bit 21 - interrupt mask. 0: mask the interrupt.
    #[inline(always)]
    pub fn adc_ch1_sat(&mut self) -> AdcCh1SatW<IRQ_MSKrs> {
        AdcCh1SatW::new(self, 21)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IRQ_MSKrs> {
        RsvdW::new(self, 22)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`irq_msk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_msk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRQ_MSKrs;
impl crate::RegisterSpec for IRQ_MSKrs {
    type Ux = u32;
}
///`read()` method returns [`irq_msk::R`](R) reader structure
impl crate::Readable for IRQ_MSKrs {}
///`write(|w| ..)` method takes [`irq_msk::W`](W) writer structure
impl crate::Writable for IRQ_MSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ_MSK to value 0
impl crate::Resettable for IRQ_MSKrs {
    const RESET_VALUE: u32 = 0;
}
