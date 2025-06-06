///Register `BT_PHONE_CTRL` reader
pub type R = crate::R<BT_PHONE_CTRLrs>;
///Register `BT_PHONE_CTRL` writer
pub type W = crate::W<BT_PHONE_CTRLrs>;
///Field `BT_PH_EN` reader - BT phone enable 0: disable, 1: enable
pub type BtPhEnR = crate::BitReader;
///Field `BT_PH_EN` writer - BT phone enable 0: disable, 1: enable
pub type BtPhEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_BACK_MIX_EN` reader - background mixer enable 0: disable, 1: enable
pub type BtBackMixEnR = crate::BitReader;
///Field `BT_BACK_MIX_EN` writer - background mixer enable 0: disable, 1: enable
pub type BtBackMixEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_MIX_SMOOTH_FILTER_EN` reader - 0: disable the smooth filter for background mixer 1: enable the smooth filer for background mixer
pub type BtMixSmoothFilterEnR = crate::BitReader;
///Field `BT_MIX_SMOOTH_FILTER_EN` writer - 0: disable the smooth filter for background mixer 1: enable the smooth filer for background mixer
pub type BtMixSmoothFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_PATH_SEL` reader - BT path select 0: digital path, 1: analog path
pub type BtPathSelR = crate::BitReader;
///Field `BT_PATH_SEL` writer - BT path select 0: digital path, 1: analog path
pub type BtPathSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_PCM_IF_BPS` reader - bypass baseband PCM signals to BT VCI master: 0: no bypass, 1: bypass
pub type BtPcmIfBpsR = crate::BitReader;
///Field `BT_PCM_IF_BPS` writer - bypass baseband PCM signals to BT VCI master: 0: no bypass, 1: bypass
pub type BtPcmIfBpsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BB_I2S_BPS_TO_CDC` reader - bypass baseband I2S interface to audio codec i2s interface 0: no bypass, 1: bypass
pub type BbI2sBpsToCdcR = crate::BitReader;
///Field `BB_I2S_BPS_TO_CDC` writer - bypass baseband I2S interface to audio codec i2s interface 0: no bypass, 1: bypass
pub type BbI2sBpsToCdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BT phone enable 0: disable, 1: enable
    #[inline(always)]
    pub fn bt_ph_en(&self) -> BtPhEnR {
        BtPhEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - background mixer enable 0: disable, 1: enable
    #[inline(always)]
    pub fn bt_back_mix_en(&self) -> BtBackMixEnR {
        BtBackMixEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 0: disable the smooth filter for background mixer 1: enable the smooth filer for background mixer
    #[inline(always)]
    pub fn bt_mix_smooth_filter_en(&self) -> BtMixSmoothFilterEnR {
        BtMixSmoothFilterEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BT path select 0: digital path, 1: analog path
    #[inline(always)]
    pub fn bt_path_sel(&self) -> BtPathSelR {
        BtPathSelR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - bypass baseband PCM signals to BT VCI master: 0: no bypass, 1: bypass
    #[inline(always)]
    pub fn bt_pcm_if_bps(&self) -> BtPcmIfBpsR {
        BtPcmIfBpsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - bypass baseband I2S interface to audio codec i2s interface 0: no bypass, 1: bypass
    #[inline(always)]
    pub fn bb_i2s_bps_to_cdc(&self) -> BbI2sBpsToCdcR {
        BbI2sBpsToCdcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_PHONE_CTRL")
            .field("bb_i2s_bps_to_cdc", &self.bb_i2s_bps_to_cdc())
            .field("bt_pcm_if_bps", &self.bt_pcm_if_bps())
            .field("bt_path_sel", &self.bt_path_sel())
            .field("bt_mix_smooth_filter_en", &self.bt_mix_smooth_filter_en())
            .field("bt_back_mix_en", &self.bt_back_mix_en())
            .field("bt_ph_en", &self.bt_ph_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - BT phone enable 0: disable, 1: enable
    #[inline(always)]
    pub fn bt_ph_en(&mut self) -> BtPhEnW<BT_PHONE_CTRLrs> {
        BtPhEnW::new(self, 0)
    }
    ///Bit 1 - background mixer enable 0: disable, 1: enable
    #[inline(always)]
    pub fn bt_back_mix_en(&mut self) -> BtBackMixEnW<BT_PHONE_CTRLrs> {
        BtBackMixEnW::new(self, 1)
    }
    ///Bit 2 - 0: disable the smooth filter for background mixer 1: enable the smooth filer for background mixer
    #[inline(always)]
    pub fn bt_mix_smooth_filter_en(&mut self) -> BtMixSmoothFilterEnW<BT_PHONE_CTRLrs> {
        BtMixSmoothFilterEnW::new(self, 2)
    }
    ///Bit 3 - BT path select 0: digital path, 1: analog path
    #[inline(always)]
    pub fn bt_path_sel(&mut self) -> BtPathSelW<BT_PHONE_CTRLrs> {
        BtPathSelW::new(self, 3)
    }
    ///Bit 4 - bypass baseband PCM signals to BT VCI master: 0: no bypass, 1: bypass
    #[inline(always)]
    pub fn bt_pcm_if_bps(&mut self) -> BtPcmIfBpsW<BT_PHONE_CTRLrs> {
        BtPcmIfBpsW::new(self, 4)
    }
    ///Bit 5 - bypass baseband I2S interface to audio codec i2s interface 0: no bypass, 1: bypass
    #[inline(always)]
    pub fn bb_i2s_bps_to_cdc(&mut self) -> BbI2sBpsToCdcW<BT_PHONE_CTRLrs> {
        BbI2sBpsToCdcW::new(self, 5)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bt_phone_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_phone_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BT_PHONE_CTRLrs;
impl crate::RegisterSpec for BT_PHONE_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`bt_phone_ctrl::R`](R) reader structure
impl crate::Readable for BT_PHONE_CTRLrs {}
///`write(|w| ..)` method takes [`bt_phone_ctrl::W`](W) writer structure
impl crate::Writable for BT_PHONE_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_PHONE_CTRL to value 0
impl crate::Resettable for BT_PHONE_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
