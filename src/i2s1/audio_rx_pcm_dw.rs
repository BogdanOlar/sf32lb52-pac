///Register `AUDIO_RX_PCM_DW` reader
pub type R = crate::R<AUDIO_RX_PCM_DWrs>;
///Register `AUDIO_RX_PCM_DW` writer
pub type W = crate::W<AUDIO_RX_PCM_DWrs>;
///Field `PCM_DATA_WIDTH` reader - For I2S and left justified mode, M can be 8,13,14,16 For right justified mode, M can be 8, 13, 14, 16, 18, 20, 22, 24
pub type PcmDataWidthR = crate::FieldReader;
///Field `PCM_DATA_WIDTH` writer - For I2S and left justified mode, M can be 8,13,14,16 For right justified mode, M can be 8, 13, 14, 16, 18, 20, 22, 24
pub type PcmDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 0:4 - For I2S and left justified mode, M can be 8,13,14,16 For right justified mode, M can be 8, 13, 14, 16, 18, 20, 22, 24
    #[inline(always)]
    pub fn pcm_data_width(&self) -> PcmDataWidthR {
        PcmDataWidthR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_RX_PCM_DW")
            .field("rsvd", &self.rsvd())
            .field("pcm_data_width", &self.pcm_data_width())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - For I2S and left justified mode, M can be 8,13,14,16 For right justified mode, M can be 8, 13, 14, 16, 18, 20, 22, 24
    #[inline(always)]
    pub fn pcm_data_width(&mut self) -> PcmDataWidthW<AUDIO_RX_PCM_DWrs> {
        PcmDataWidthW::new(self, 0)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AUDIO_RX_PCM_DWrs> {
        RsvdW::new(self, 5)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_pcm_dw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_pcm_dw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_RX_PCM_DWrs;
impl crate::RegisterSpec for AUDIO_RX_PCM_DWrs {
    type Ux = u32;
}
///`read()` method returns [`audio_rx_pcm_dw::R`](R) reader structure
impl crate::Readable for AUDIO_RX_PCM_DWrs {}
///`write(|w| ..)` method takes [`audio_rx_pcm_dw::W`](W) writer structure
impl crate::Writable for AUDIO_RX_PCM_DWrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_RX_PCM_DW to value 0x10
impl crate::Resettable for AUDIO_RX_PCM_DWrs {
    const RESET_VALUE: u32 = 0x10;
}
