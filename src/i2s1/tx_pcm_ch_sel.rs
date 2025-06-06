///Register `TX_PCM_CH_SEL` reader
pub type R = crate::R<TX_PCM_CH_SELrs>;
///Register `TX_PCM_CH_SEL` writer
pub type W = crate::W<TX_PCM_CH_SELrs>;
///Field `RIGHT_CHANNEL_SEL` reader - TX re-sampling module setting: 00: TX right = source right 01: TX right = source left 10,11: TX right = (source left + source right)/2
pub type RightChannelSelR = crate::FieldReader;
///Field `RIGHT_CHANNEL_SEL` writer - TX re-sampling module setting: 00: TX right = source right 01: TX right = source left 10,11: TX right = (source left + source right)/2
pub type RightChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LEFT_CHANNEL_SEL` reader - TX re-sampling module setting: 00: TX left = source left 01: TX left = source right 10,11: TX left = (source left + source right)/2
pub type LeftChannelSelR = crate::FieldReader;
///Field `LEFT_CHANNEL_SEL` writer - TX re-sampling module setting: 00: TX left = source left 01: TX left = source right 10,11: TX left = (source left + source right)/2
pub type LeftChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - TX re-sampling module setting: 00: TX right = source right 01: TX right = source left 10,11: TX right = (source left + source right)/2
    #[inline(always)]
    pub fn right_channel_sel(&self) -> RightChannelSelR {
        RightChannelSelR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - TX re-sampling module setting: 00: TX left = source left 01: TX left = source right 10,11: TX left = (source left + source right)/2
    #[inline(always)]
    pub fn left_channel_sel(&self) -> LeftChannelSelR {
        LeftChannelSelR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PCM_CH_SEL")
            .field("left_channel_sel", &self.left_channel_sel())
            .field("right_channel_sel", &self.right_channel_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TX re-sampling module setting: 00: TX right = source right 01: TX right = source left 10,11: TX right = (source left + source right)/2
    #[inline(always)]
    pub fn right_channel_sel(&mut self) -> RightChannelSelW<TX_PCM_CH_SELrs> {
        RightChannelSelW::new(self, 0)
    }
    ///Bits 2:3 - TX re-sampling module setting: 00: TX left = source left 01: TX left = source right 10,11: TX left = (source left + source right)/2
    #[inline(always)]
    pub fn left_channel_sel(&mut self) -> LeftChannelSelW<TX_PCM_CH_SELrs> {
        LeftChannelSelW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_pcm_ch_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm_ch_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_PCM_CH_SELrs;
impl crate::RegisterSpec for TX_PCM_CH_SELrs {
    type Ux = u32;
}
///`read()` method returns [`tx_pcm_ch_sel::R`](R) reader structure
impl crate::Readable for TX_PCM_CH_SELrs {}
///`write(|w| ..)` method takes [`tx_pcm_ch_sel::W`](W) writer structure
impl crate::Writable for TX_PCM_CH_SELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_PCM_CH_SEL to value 0
impl crate::Resettable for TX_PCM_CH_SELrs {
    const RESET_VALUE: u32 = 0;
}
