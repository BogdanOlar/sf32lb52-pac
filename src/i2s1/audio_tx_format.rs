///Register `AUDIO_TX_FORMAT` reader
pub type R = crate::R<AUDIO_TX_FORMATrs>;
///Register `AUDIO_TX_FORMAT` writer
pub type W = crate::W<AUDIO_TX_FORMATrs>;
///Field `PCM_DATA_WIDTH` reader - I2S out pcm data width M >= 16, common value: 16, 18, 20, 22, 24
pub type PcmDataWidthR = crate::FieldReader;
///Field `PCM_DATA_WIDTH` writer - I2S out pcm data width M >= 16, common value: 16, 18, 20, 22, 24
pub type PcmDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 0:4 - I2S out pcm data width M >= 16, common value: 16, 18, 20, 22, 24
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
        f.debug_struct("AUDIO_TX_FORMAT")
            .field("rsvd", &self.rsvd())
            .field("pcm_data_width", &self.pcm_data_width())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - I2S out pcm data width M >= 16, common value: 16, 18, 20, 22, 24
    #[inline(always)]
    pub fn pcm_data_width(&mut self) -> PcmDataWidthW<AUDIO_TX_FORMATrs> {
        PcmDataWidthW::new(self, 0)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AUDIO_TX_FORMATrs> {
        RsvdW::new(self, 5)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_TX_FORMATrs;
impl crate::RegisterSpec for AUDIO_TX_FORMATrs {
    type Ux = u32;
}
///`read()` method returns [`audio_tx_format::R`](R) reader structure
impl crate::Readable for AUDIO_TX_FORMATrs {}
///`write(|w| ..)` method takes [`audio_tx_format::W`](W) writer structure
impl crate::Writable for AUDIO_TX_FORMATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_TX_FORMAT to value 0
impl crate::Resettable for AUDIO_TX_FORMATrs {
    const RESET_VALUE: u32 = 0;
}
