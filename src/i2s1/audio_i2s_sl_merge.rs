///Register `AUDIO_I2S_SL_MERGE` reader
pub type R = crate::R<AUDIO_I2S_SL_MERGErs>;
///Register `AUDIO_I2S_SL_MERGE` writer
pub type W = crate::W<AUDIO_I2S_SL_MERGErs>;
///Field `SLAVE_TIMING_MERGE` reader - when work as an I2S slave, and external I2S master TX/RX share an only BCLK/LRCK, we need set this bit high. 0: I2S slave use separated timing control port. TX_BCLK_IN/TX_LRCK_IN and RX_BCLK/RX_LRCK_IN are separated. 1: I2S slave use the same BCLK/LRCK, the TX_BCLK_IN/TX_LRCK also is used for RX controller.
pub type SlaveTimingMergeR = crate::BitReader;
///Field `SLAVE_TIMING_MERGE` writer - when work as an I2S slave, and external I2S master TX/RX share an only BCLK/LRCK, we need set this bit high. 0: I2S slave use separated timing control port. TX_BCLK_IN/TX_LRCK_IN and RX_BCLK/RX_LRCK_IN are separated. 1: I2S slave use the same BCLK/LRCK, the TX_BCLK_IN/TX_LRCK also is used for RX controller.
pub type SlaveTimingMergeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - when work as an I2S slave, and external I2S master TX/RX share an only BCLK/LRCK, we need set this bit high. 0: I2S slave use separated timing control port. TX_BCLK_IN/TX_LRCK_IN and RX_BCLK/RX_LRCK_IN are separated. 1: I2S slave use the same BCLK/LRCK, the TX_BCLK_IN/TX_LRCK also is used for RX controller.
    #[inline(always)]
    pub fn slave_timing_merge(&self) -> SlaveTimingMergeR {
        SlaveTimingMergeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_I2S_SL_MERGE")
            .field("slave_timing_merge", &self.slave_timing_merge())
            .finish()
    }
}
impl W {
    ///Bit 0 - when work as an I2S slave, and external I2S master TX/RX share an only BCLK/LRCK, we need set this bit high. 0: I2S slave use separated timing control port. TX_BCLK_IN/TX_LRCK_IN and RX_BCLK/RX_LRCK_IN are separated. 1: I2S slave use the same BCLK/LRCK, the TX_BCLK_IN/TX_LRCK also is used for RX controller.
    #[inline(always)]
    pub fn slave_timing_merge(&mut self) -> SlaveTimingMergeW<AUDIO_I2S_SL_MERGErs> {
        SlaveTimingMergeW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_i2s_sl_merge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_i2s_sl_merge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_I2S_SL_MERGErs;
impl crate::RegisterSpec for AUDIO_I2S_SL_MERGErs {
    type Ux = u32;
}
///`read()` method returns [`audio_i2s_sl_merge::R`](R) reader structure
impl crate::Readable for AUDIO_I2S_SL_MERGErs {}
///`write(|w| ..)` method takes [`audio_i2s_sl_merge::W`](W) writer structure
impl crate::Writable for AUDIO_I2S_SL_MERGErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_I2S_SL_MERGE to value 0
impl crate::Resettable for AUDIO_I2S_SL_MERGErs {
    const RESET_VALUE: u32 = 0;
}
