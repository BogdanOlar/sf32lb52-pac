///Register `RX_CH_SEL` reader
pub type R = crate::R<RX_CH_SELrs>;
///Register `RX_CH_SEL` writer
pub type W = crate::W<RX_CH_SELrs>;
///Field `RIGHT_CHANNEL_SEL` reader - RX re-sampling module setting: 00: RD right = RX right 01: RD right = RX left 10,11: RD right = (RX left + RX right)/2
pub type RightChannelSelR = crate::FieldReader;
///Field `RIGHT_CHANNEL_SEL` writer - RX re-sampling module setting: 00: RD right = RX right 01: RD right = RX left 10,11: RD right = (RX left + RX right)/2
pub type RightChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LEFT_CHANNEL_SEL` reader - RX re-sampling module setting: 00: RD left = RX left 01: RD left = RX right 10,11: RD left = (RX left + RX right)/2
pub type LeftChannelSelR = crate::FieldReader;
///Field `LEFT_CHANNEL_SEL` writer - RX re-sampling module setting: 00: RD left = RX left 01: RD left = RX right 10,11: RD left = (RX left + RX right)/2
pub type LeftChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:1 - RX re-sampling module setting: 00: RD right = RX right 01: RD right = RX left 10,11: RD right = (RX left + RX right)/2
    #[inline(always)]
    pub fn right_channel_sel(&self) -> RightChannelSelR {
        RightChannelSelR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - RX re-sampling module setting: 00: RD left = RX left 01: RD left = RX right 10,11: RD left = (RX left + RX right)/2
    #[inline(always)]
    pub fn left_channel_sel(&self) -> LeftChannelSelR {
        LeftChannelSelR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH_SEL")
            .field("rsvd", &self.rsvd())
            .field("left_channel_sel", &self.left_channel_sel())
            .field("right_channel_sel", &self.right_channel_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - RX re-sampling module setting: 00: RD right = RX right 01: RD right = RX left 10,11: RD right = (RX left + RX right)/2
    #[inline(always)]
    pub fn right_channel_sel(&mut self) -> RightChannelSelW<RX_CH_SELrs> {
        RightChannelSelW::new(self, 0)
    }
    ///Bits 2:3 - RX re-sampling module setting: 00: RD left = RX left 01: RD left = RX right 10,11: RD left = (RX left + RX right)/2
    #[inline(always)]
    pub fn left_channel_sel(&mut self) -> LeftChannelSelW<RX_CH_SELrs> {
        LeftChannelSelW::new(self, 2)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RX_CH_SELrs> {
        RsvdW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_ch_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_CH_SELrs;
impl crate::RegisterSpec for RX_CH_SELrs {
    type Ux = u32;
}
///`read()` method returns [`rx_ch_sel::R`](R) reader structure
impl crate::Readable for RX_CH_SELrs {}
///`write(|w| ..)` method takes [`rx_ch_sel::W`](W) writer structure
impl crate::Writable for RX_CH_SELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_CH_SEL to value 0
impl crate::Resettable for RX_CH_SELrs {
    const RESET_VALUE: u32 = 0;
}
