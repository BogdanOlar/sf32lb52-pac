///Register `AUDIO_RX_SERIAL_TIMING` reader
pub type R = crate::R<AUDIO_RX_SERIAL_TIMINGrs>;
///Register `AUDIO_RX_SERIAL_TIMING` writer
pub type W = crate::W<AUDIO_RX_SERIAL_TIMINGrs>;
///Field `TIMING` reader - 00: I2S 01: Left justified 10: right justified 11: reserved
pub type TimingR = crate::FieldReader;
///Field `TIMING` writer - 00: I2S 01: Left justified 10: right justified 11: reserved
pub type TimingW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SLAVE_EN` reader - audio code receiver mode select. 0: master mode, 1: slave mode
pub type SlaveEnR = crate::BitReader;
///Field `SLAVE_EN` writer - audio code receiver mode select. 0: master mode, 1: slave mode
pub type SlaveEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LRCK_POL` reader - RX LRCK polarity control. 0: disable RX_LRCK inventor 1: enable RX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih
pub type LrckPolR = crate::BitReader;
///Field `LRCK_POL` writer - RX LRCK polarity control. 0: disable RX_LRCK inventor 1: enable RX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih
pub type LrckPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:1 - 00: I2S 01: Left justified 10: right justified 11: reserved
    #[inline(always)]
    pub fn timing(&self) -> TimingR {
        TimingR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - audio code receiver mode select. 0: master mode, 1: slave mode
    #[inline(always)]
    pub fn slave_en(&self) -> SlaveEnR {
        SlaveEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX LRCK polarity control. 0: disable RX_LRCK inventor 1: enable RX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih
    #[inline(always)]
    pub fn lrck_pol(&self) -> LrckPolR {
        LrckPolR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_RX_SERIAL_TIMING")
            .field("rsvd", &self.rsvd())
            .field("lrck_pol", &self.lrck_pol())
            .field("slave_en", &self.slave_en())
            .field("timing", &self.timing())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - 00: I2S 01: Left justified 10: right justified 11: reserved
    #[inline(always)]
    pub fn timing(&mut self) -> TimingW<AUDIO_RX_SERIAL_TIMINGrs> {
        TimingW::new(self, 0)
    }
    ///Bit 2 - audio code receiver mode select. 0: master mode, 1: slave mode
    #[inline(always)]
    pub fn slave_en(&mut self) -> SlaveEnW<AUDIO_RX_SERIAL_TIMINGrs> {
        SlaveEnW::new(self, 2)
    }
    ///Bit 3 - RX LRCK polarity control. 0: disable RX_LRCK inventor 1: enable RX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih
    #[inline(always)]
    pub fn lrck_pol(&mut self) -> LrckPolW<AUDIO_RX_SERIAL_TIMINGrs> {
        LrckPolW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AUDIO_RX_SERIAL_TIMINGrs> {
        RsvdW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_serial_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_serial_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_RX_SERIAL_TIMINGrs;
impl crate::RegisterSpec for AUDIO_RX_SERIAL_TIMINGrs {
    type Ux = u32;
}
///`read()` method returns [`audio_rx_serial_timing::R`](R) reader structure
impl crate::Readable for AUDIO_RX_SERIAL_TIMINGrs {}
///`write(|w| ..)` method takes [`audio_rx_serial_timing::W`](W) writer structure
impl crate::Writable for AUDIO_RX_SERIAL_TIMINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_RX_SERIAL_TIMING to value 0x0004_0000
impl crate::Resettable for AUDIO_RX_SERIAL_TIMINGrs {
    const RESET_VALUE: u32 = 0x0004_0000;
}
