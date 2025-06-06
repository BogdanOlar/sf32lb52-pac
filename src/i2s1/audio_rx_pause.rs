///Register `AUDIO_RX_PAUSE` reader
pub type R = crate::R<AUDIO_RX_PAUSErs>;
///Register `AUDIO_RX_PAUSE` writer
pub type W = crate::W<AUDIO_RX_PAUSErs>;
///Field `RX_PAUSE` reader - RX pause control when rx_enable = 1. 1: pause 0: RX work
pub type RxPauseR = crate::BitReader;
///Field `RX_PAUSE` writer - RX pause control when rx_enable = 1. 1: pause 0: RX work
pub type RxPauseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RX pause control when rx_enable = 1. 1: pause 0: RX work
    #[inline(always)]
    pub fn rx_pause(&self) -> RxPauseR {
        RxPauseR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_RX_PAUSE")
            .field("rx_pause", &self.rx_pause())
            .finish()
    }
}
impl W {
    ///Bit 0 - RX pause control when rx_enable = 1. 1: pause 0: RX work
    #[inline(always)]
    pub fn rx_pause(&mut self) -> RxPauseW<AUDIO_RX_PAUSErs> {
        RxPauseW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_pause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_pause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_RX_PAUSErs;
impl crate::RegisterSpec for AUDIO_RX_PAUSErs {
    type Ux = u32;
}
///`read()` method returns [`audio_rx_pause::R`](R) reader structure
impl crate::Readable for AUDIO_RX_PAUSErs {}
///`write(|w| ..)` method takes [`audio_rx_pause::W`](W) writer structure
impl crate::Writable for AUDIO_RX_PAUSErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_RX_PAUSE to value 0
impl crate::Resettable for AUDIO_RX_PAUSErs {
    const RESET_VALUE: u32 = 0;
}
