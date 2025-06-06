///Register `ADC_CH1_CFG` reader
pub type R = crate::R<ADC_CH1_CFGrs>;
///Register `ADC_CH1_CFG` writer
pub type W = crate::W<ADC_CH1_CFGrs>;
///Field `ENABLE` reader - adc channel enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - adc channel enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPF_BYPASS` reader - high-pass filter bypass
pub type HpfBypassR = crate::BitReader;
///Field `HPF_BYPASS` writer - high-pass filter bypass
pub type HpfBypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPF_COEF` reader - high-pass filter coefficient
pub type HpfCoefR = crate::FieldReader;
///Field `HPF_COEF` writer - high-pass filter coefficient
pub type HpfCoefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `STB_INV` reader - adc strobe inverter
pub type StbInvR = crate::BitReader;
///Field `STB_INV` writer - adc strobe inverter
pub type StbInvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_EN` reader - dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface
pub type DmaEnR = crate::BitReader;
///Field `DMA_EN` writer - dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROUGH_VOL` reader - adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB
pub type RoughVolR = crate::FieldReader;
///Field `ROUGH_VOL` writer - adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB
pub type RoughVolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FINE_VOL` reader - adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolR = crate::FieldReader;
///Field `FINE_VOL` writer - adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATA_FORMAT` reader - adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
pub type DataFormatR = crate::BitReader;
///Field `DATA_FORMAT` writer - adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
pub type DataFormatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAT_DET_EN` reader - adc saturation detect
pub type SatDetEnR = crate::BitReader;
///Field `SAT_DET_EN` writer - adc saturation detect
pub type SatDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAT_DET_LEN` reader - adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48
pub type SatDetLenR = crate::FieldReader;
///Field `SAT_DET_LEN` writer - adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48
pub type SatDetLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0 - adc channel enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - high-pass filter bypass
    #[inline(always)]
    pub fn hpf_bypass(&self) -> HpfBypassR {
        HpfBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - high-pass filter coefficient
    #[inline(always)]
    pub fn hpf_coef(&self) -> HpfCoefR {
        HpfCoefR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - adc strobe inverter
    #[inline(always)]
    pub fn stb_inv(&self) -> StbInvR {
        StbInvR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB
    #[inline(always)]
    pub fn rough_vol(&self) -> RoughVolR {
        RoughVolR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol(&self) -> FineVolR {
        FineVolR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
    ///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
    #[inline(always)]
    pub fn data_format(&self) -> DataFormatR {
        DataFormatR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - adc saturation detect
    #[inline(always)]
    pub fn sat_det_en(&self) -> SatDetEnR {
        SatDetEnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48
    #[inline(always)]
    pub fn sat_det_len(&self) -> SatDetLenR {
        SatDetLenR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CH1_CFG")
            .field("rsvd", &self.rsvd())
            .field("sat_det_len", &self.sat_det_len())
            .field("sat_det_en", &self.sat_det_en())
            .field("data_format", &self.data_format())
            .field("fine_vol", &self.fine_vol())
            .field("rough_vol", &self.rough_vol())
            .field("dma_en", &self.dma_en())
            .field("stb_inv", &self.stb_inv())
            .field("hpf_coef", &self.hpf_coef())
            .field("hpf_bypass", &self.hpf_bypass())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - adc channel enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<ADC_CH1_CFGrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - high-pass filter bypass
    #[inline(always)]
    pub fn hpf_bypass(&mut self) -> HpfBypassW<ADC_CH1_CFGrs> {
        HpfBypassW::new(self, 1)
    }
    ///Bits 2:5 - high-pass filter coefficient
    #[inline(always)]
    pub fn hpf_coef(&mut self) -> HpfCoefW<ADC_CH1_CFGrs> {
        HpfCoefW::new(self, 2)
    }
    ///Bit 6 - adc strobe inverter
    #[inline(always)]
    pub fn stb_inv(&mut self) -> StbInvW<ADC_CH1_CFGrs> {
        StbInvW::new(self, 6)
    }
    ///Bit 7 - dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<ADC_CH1_CFGrs> {
        DmaEnW::new(self, 7)
    }
    ///Bits 8:11 - adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB
    #[inline(always)]
    pub fn rough_vol(&mut self) -> RoughVolW<ADC_CH1_CFGrs> {
        RoughVolW::new(self, 8)
    }
    ///Bits 12:15 - adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol(&mut self) -> FineVolW<ADC_CH1_CFGrs> {
        FineVolW::new(self, 12)
    }
    ///Bit 16 - adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\[31:24\]
    ///are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}
    #[inline(always)]
    pub fn data_format(&mut self) -> DataFormatW<ADC_CH1_CFGrs> {
        DataFormatW::new(self, 16)
    }
    ///Bit 17 - adc saturation detect
    #[inline(always)]
    pub fn sat_det_en(&mut self) -> SatDetEnW<ADC_CH1_CFGrs> {
        SatDetEnW::new(self, 17)
    }
    ///Bits 18:19 - adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48
    #[inline(always)]
    pub fn sat_det_len(&mut self) -> SatDetLenW<ADC_CH1_CFGrs> {
        SatDetLenW::new(self, 18)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ADC_CH1_CFGrs> {
        RsvdW::new(self, 20)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ch1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ch1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_CH1_CFGrs;
impl crate::RegisterSpec for ADC_CH1_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ch1_cfg::R`](R) reader structure
impl crate::Readable for ADC_CH1_CFGrs {}
///`write(|w| ..)` method takes [`adc_ch1_cfg::W`](W) writer structure
impl crate::Writable for ADC_CH1_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CH1_CFG to value 0
impl crate::Resettable for ADC_CH1_CFGrs {
    const RESET_VALUE: u32 = 0;
}
