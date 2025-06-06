///Register `ADC_PATH_CFG0` reader
pub type R = crate::R<ADC_PATH_CFG0rs>;
///Register `ADC_PATH_CFG0` writer
pub type W = crate::W<ADC_PATH_CFG0rs>;
///Field `ROUGH_VOL_L` reader - adc left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolLR = crate::FieldReader;
///Field `ROUGH_VOL_L` writer - adc left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FINE_VOL_L` reader - adc left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolLR = crate::FieldReader;
///Field `FINE_VOL_L` writer - adc left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ROUGH_VOL_R` reader - adc right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolRR = crate::FieldReader;
///Field `ROUGH_VOL_R` writer - adc right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FINE_VOL_R` reader - adc right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolRR = crate::FieldReader;
///Field `FINE_VOL_R` writer - adc right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SRC_SEL` reader - adc path source select 1'h0: select audio codec 1'h1: select external interface
pub type SrcSelR = crate::BitReader;
///Field `SRC_SEL` writer - adc path source select 1'h0: select audio codec 1'h1: select external interface
pub type SrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_SWAP` reader - swap adc path left and right channel data
pub type DataSwapR = crate::BitReader;
///Field `DATA_SWAP` writer - swap adc path left and right channel data
pub type DataSwapW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX2TX_LOOPBACK` reader - rx to tx loopback enable
pub type Rx2txLoopbackR = crate::BitReader;
///Field `RX2TX_LOOPBACK` writer - rx to tx loopback enable
pub type Rx2txLoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - adc left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_l(&self) -> RoughVolLR {
        RoughVolLR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - adc left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_l(&self) -> FineVolLR {
        FineVolLR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - adc right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_r(&self) -> RoughVolRR {
        RoughVolRR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - adc right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_r(&self) -> FineVolRR {
        FineVolRR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - adc path source select 1'h0: select audio codec 1'h1: select external interface
    #[inline(always)]
    pub fn src_sel(&self) -> SrcSelR {
        SrcSelR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - swap adc path left and right channel data
    #[inline(always)]
    pub fn data_swap(&self) -> DataSwapR {
        DataSwapR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - rx to tx loopback enable
    #[inline(always)]
    pub fn rx2tx_loopback(&self) -> Rx2txLoopbackR {
        Rx2txLoopbackR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PATH_CFG0")
            .field("rx2tx_loopback", &self.rx2tx_loopback())
            .field("data_swap", &self.data_swap())
            .field("src_sel", &self.src_sel())
            .field("fine_vol_r", &self.fine_vol_r())
            .field("rough_vol_r", &self.rough_vol_r())
            .field("fine_vol_l", &self.fine_vol_l())
            .field("rough_vol_l", &self.rough_vol_l())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - adc left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_l(&mut self) -> RoughVolLW<ADC_PATH_CFG0rs> {
        RoughVolLW::new(self, 0)
    }
    ///Bits 4:7 - adc left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_l(&mut self) -> FineVolLW<ADC_PATH_CFG0rs> {
        FineVolLW::new(self, 4)
    }
    ///Bits 8:11 - adc right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_r(&mut self) -> RoughVolRW<ADC_PATH_CFG0rs> {
        RoughVolRW::new(self, 8)
    }
    ///Bits 12:15 - adc right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_r(&mut self) -> FineVolRW<ADC_PATH_CFG0rs> {
        FineVolRW::new(self, 12)
    }
    ///Bit 16 - adc path source select 1'h0: select audio codec 1'h1: select external interface
    #[inline(always)]
    pub fn src_sel(&mut self) -> SrcSelW<ADC_PATH_CFG0rs> {
        SrcSelW::new(self, 16)
    }
    ///Bit 17 - swap adc path left and right channel data
    #[inline(always)]
    pub fn data_swap(&mut self) -> DataSwapW<ADC_PATH_CFG0rs> {
        DataSwapW::new(self, 17)
    }
    ///Bit 18 - rx to tx loopback enable
    #[inline(always)]
    pub fn rx2tx_loopback(&mut self) -> Rx2txLoopbackW<ADC_PATH_CFG0rs> {
        Rx2txLoopbackW::new(self, 18)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`adc_path_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_path_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_PATH_CFG0rs;
impl crate::RegisterSpec for ADC_PATH_CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`adc_path_cfg0::R`](R) reader structure
impl crate::Readable for ADC_PATH_CFG0rs {}
///`write(|w| ..)` method takes [`adc_path_cfg0::W`](W) writer structure
impl crate::Writable for ADC_PATH_CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_PATH_CFG0 to value 0
impl crate::Resettable for ADC_PATH_CFG0rs {
    const RESET_VALUE: u32 = 0;
}
