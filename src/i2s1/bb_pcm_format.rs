///Register `BB_PCM_FORMAT` reader
pub type R = crate::R<BB_PCM_FORMATrs>;
///Register `BB_PCM_FORMAT` writer
pub type W = crate::W<BB_PCM_FORMATrs>;
///Field `PCM_DW` reader - Baseband Master PCM data width (>=8) Common value: 8, 13,14, 16, 18, 20, 22, 24. for I2S/Left Justified/Right Kistified timing, bb_pcm_dw >=16 For PCM timing, only 8, 13, 14, 16 configure value is available.
pub type PcmDwR = crate::FieldReader;
///Field `PCM_DW` writer - Baseband Master PCM data width (>=8) Common value: 8, 13,14, 16, 18, 20, 22, 24. for I2S/Left Justified/Right Kistified timing, bb_pcm_dw >=16 For PCM timing, only 8, 13, 14, 16 configure value is available.
pub type PcmDwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PCM_TIM_SEL` reader - 00: I2S timing, 01: Left Justified 10: Right Justified, 11: PCM timing
pub type PcmTimSelR = crate::FieldReader;
///Field `PCM_TIM_SEL` writer - 00: I2S timing, 01: Left Justified 10: Right Justified, 11: PCM timing
pub type PcmTimSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PCM_SYNC_FLAG` reader - 0: short sync, 1: long sync
pub type PcmSyncFlagR = crate::BitReader;
///Field `PCM_SYNC_FLAG` writer - 0: short sync, 1: long sync
pub type PcmSyncFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCM_LSB_FLAG` reader - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
pub type PcmLsbFlagR = crate::BitReader;
///Field `PCM_LSB_FLAG` writer - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
pub type PcmLsbFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S_LRCK_POL` reader - 0: no bb_i2s_lrck input inventor 1: enable bb_i2s_lrck input inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to high
pub type I2sLrckPolR = crate::BitReader;
///Field `I2S_LRCK_POL` writer - 0: no bb_i2s_lrck input inventor 1: enable bb_i2s_lrck input inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to high
pub type I2sLrckPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCM_CLK_POL` reader - input BB pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
pub type PcmClkPolR = crate::BitReader;
///Field `PCM_CLK_POL` writer - input BB pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
pub type PcmClkPolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Baseband Master PCM data width (>=8) Common value: 8, 13,14, 16, 18, 20, 22, 24. for I2S/Left Justified/Right Kistified timing, bb_pcm_dw >=16 For PCM timing, only 8, 13, 14, 16 configure value is available.
    #[inline(always)]
    pub fn pcm_dw(&self) -> PcmDwR {
        PcmDwR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:6 - 00: I2S timing, 01: Left Justified 10: Right Justified, 11: PCM timing
    #[inline(always)]
    pub fn pcm_tim_sel(&self) -> PcmTimSelR {
        PcmTimSelR::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - 0: short sync, 1: long sync
    #[inline(always)]
    pub fn pcm_sync_flag(&self) -> PcmSyncFlagR {
        PcmSyncFlagR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
    #[inline(always)]
    pub fn pcm_lsb_flag(&self) -> PcmLsbFlagR {
        PcmLsbFlagR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 0: no bb_i2s_lrck input inventor 1: enable bb_i2s_lrck input inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to high
    #[inline(always)]
    pub fn i2s_lrck_pol(&self) -> I2sLrckPolR {
        I2sLrckPolR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - input BB pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
    #[inline(always)]
    pub fn pcm_clk_pol(&self) -> PcmClkPolR {
        PcmClkPolR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BB_PCM_FORMAT")
            .field("pcm_clk_pol", &self.pcm_clk_pol())
            .field("i2s_lrck_pol", &self.i2s_lrck_pol())
            .field("pcm_lsb_flag", &self.pcm_lsb_flag())
            .field("pcm_sync_flag", &self.pcm_sync_flag())
            .field("pcm_tim_sel", &self.pcm_tim_sel())
            .field("pcm_dw", &self.pcm_dw())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Baseband Master PCM data width (>=8) Common value: 8, 13,14, 16, 18, 20, 22, 24. for I2S/Left Justified/Right Kistified timing, bb_pcm_dw >=16 For PCM timing, only 8, 13, 14, 16 configure value is available.
    #[inline(always)]
    pub fn pcm_dw(&mut self) -> PcmDwW<BB_PCM_FORMATrs> {
        PcmDwW::new(self, 0)
    }
    ///Bits 5:6 - 00: I2S timing, 01: Left Justified 10: Right Justified, 11: PCM timing
    #[inline(always)]
    pub fn pcm_tim_sel(&mut self) -> PcmTimSelW<BB_PCM_FORMATrs> {
        PcmTimSelW::new(self, 5)
    }
    ///Bit 7 - 0: short sync, 1: long sync
    #[inline(always)]
    pub fn pcm_sync_flag(&mut self) -> PcmSyncFlagW<BB_PCM_FORMATrs> {
        PcmSyncFlagW::new(self, 7)
    }
    ///Bit 8 - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
    #[inline(always)]
    pub fn pcm_lsb_flag(&mut self) -> PcmLsbFlagW<BB_PCM_FORMATrs> {
        PcmLsbFlagW::new(self, 8)
    }
    ///Bit 9 - 0: no bb_i2s_lrck input inventor 1: enable bb_i2s_lrck input inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to high
    #[inline(always)]
    pub fn i2s_lrck_pol(&mut self) -> I2sLrckPolW<BB_PCM_FORMATrs> {
        I2sLrckPolW::new(self, 9)
    }
    ///Bit 10 - input BB pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
    #[inline(always)]
    pub fn pcm_clk_pol(&mut self) -> PcmClkPolW<BB_PCM_FORMATrs> {
        PcmClkPolW::new(self, 10)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bb_pcm_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bb_pcm_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BB_PCM_FORMATrs;
impl crate::RegisterSpec for BB_PCM_FORMATrs {
    type Ux = u32;
}
///`read()` method returns [`bb_pcm_format::R`](R) reader structure
impl crate::Readable for BB_PCM_FORMATrs {}
///`write(|w| ..)` method takes [`bb_pcm_format::W`](W) writer structure
impl crate::Writable for BB_PCM_FORMATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BB_PCM_FORMAT to value 0
impl crate::Resettable for BB_PCM_FORMATrs {
    const RESET_VALUE: u32 = 0;
}
