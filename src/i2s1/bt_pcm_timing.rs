///Register `BT_PCM_TIMING` reader
pub type R = crate::R<BT_PCM_TIMINGrs>;
///Register `BT_PCM_TIMING` writer
pub type W = crate::W<BT_PCM_TIMINGrs>;
///Field `LSB_FLAG` reader - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
pub type LsbFlagR = crate::BitReader;
///Field `LSB_FLAG` writer - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
pub type LsbFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_FLAG` reader - 0: short sync, 1: long sync
pub type SyncFlagR = crate::BitReader;
///Field `SYNC_FLAG` writer - 0: short sync, 1: long sync
pub type SyncFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_POL` reader - BT PCM master output pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
pub type ClkPolR = crate::BitReader;
///Field `CLK_POL` writer - BT PCM master output pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
pub type ClkPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bit 0 - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
    #[inline(always)]
    pub fn lsb_flag(&self) -> LsbFlagR {
        LsbFlagR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 0: short sync, 1: long sync
    #[inline(always)]
    pub fn sync_flag(&self) -> SyncFlagR {
        SyncFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BT PCM master output pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
    #[inline(always)]
    pub fn clk_pol(&self) -> ClkPolR {
        ClkPolR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_PCM_TIMING")
            .field("rsvd", &self.rsvd())
            .field("clk_pol", &self.clk_pol())
            .field("sync_flag", &self.sync_flag())
            .field("lsb_flag", &self.lsb_flag())
            .finish()
    }
}
impl W {
    ///Bit 0 - Serial PCM data bit sequence. 0: MSB first, 1: LSB first
    #[inline(always)]
    pub fn lsb_flag(&mut self) -> LsbFlagW<BT_PCM_TIMINGrs> {
        LsbFlagW::new(self, 0)
    }
    ///Bit 1 - 0: short sync, 1: long sync
    #[inline(always)]
    pub fn sync_flag(&mut self) -> SyncFlagW<BT_PCM_TIMINGrs> {
        SyncFlagW::new(self, 1)
    }
    ///Bit 2 - BT PCM master output pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting
    #[inline(always)]
    pub fn clk_pol(&mut self) -> ClkPolW<BT_PCM_TIMINGrs> {
        ClkPolW::new(self, 2)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<BT_PCM_TIMINGrs> {
        RsvdW::new(self, 3)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BT_PCM_TIMINGrs;
impl crate::RegisterSpec for BT_PCM_TIMINGrs {
    type Ux = u32;
}
///`read()` method returns [`bt_pcm_timing::R`](R) reader structure
impl crate::Readable for BT_PCM_TIMINGrs {}
///`write(|w| ..)` method takes [`bt_pcm_timing::W`](W) writer structure
impl crate::Writable for BT_PCM_TIMINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_PCM_TIMING to value 0
impl crate::Resettable for BT_PCM_TIMINGrs {
    const RESET_VALUE: u32 = 0;
}
