///Register `BT_PCM_SYNC_DUTY` reader
pub type R = crate::R<BT_PCM_SYNC_DUTYrs>;
///Register `BT_PCM_SYNC_DUTY` writer
pub type W = crate::W<BT_PCM_SYNC_DUTYrs>;
///Field `SYNC_DUTY` reader - PCM_SYNC duty cycle (bt_pcm_sync frequency = bt_pclk_clk/bt_pcm_sync_duty)
pub type SyncDutyR = crate::FieldReader;
///Field `SYNC_DUTY` writer - PCM_SYNC duty cycle (bt_pcm_sync frequency = bt_pclk_clk/bt_pcm_sync_duty)
pub type SyncDutyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:5 - PCM_SYNC duty cycle (bt_pcm_sync frequency = bt_pclk_clk/bt_pcm_sync_duty)
    #[inline(always)]
    pub fn sync_duty(&self) -> SyncDutyR {
        SyncDutyR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_PCM_SYNC_DUTY")
            .field("rsvd", &self.rsvd())
            .field("sync_duty", &self.sync_duty())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PCM_SYNC duty cycle (bt_pcm_sync frequency = bt_pclk_clk/bt_pcm_sync_duty)
    #[inline(always)]
    pub fn sync_duty(&mut self) -> SyncDutyW<BT_PCM_SYNC_DUTYrs> {
        SyncDutyW::new(self, 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<BT_PCM_SYNC_DUTYrs> {
        RsvdW::new(self, 6)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_sync_duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_sync_duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BT_PCM_SYNC_DUTYrs;
impl crate::RegisterSpec for BT_PCM_SYNC_DUTYrs {
    type Ux = u32;
}
///`read()` method returns [`bt_pcm_sync_duty::R`](R) reader structure
impl crate::Readable for BT_PCM_SYNC_DUTYrs {}
///`write(|w| ..)` method takes [`bt_pcm_sync_duty::W`](W) writer structure
impl crate::Writable for BT_PCM_SYNC_DUTYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_PCM_SYNC_DUTY to value 0
impl crate::Resettable for BT_PCM_SYNC_DUTYrs {
    const RESET_VALUE: u32 = 0;
}
