///Register `AUDIO_TX_PAUSE` reader
pub type R = crate::R<AUDIO_TX_PAUSErs>;
///Register `AUDIO_TX_PAUSE` writer
pub type W = crate::W<AUDIO_TX_PAUSErs>;
///Field `TX_PAUSE` reader - TX pause control when tx_enable = 1. 1: pause 0: TX work
pub type TxPauseR = crate::BitReader;
///Field `TX_PAUSE` writer - TX pause control when tx_enable = 1. 1: pause 0: TX work
pub type TxPauseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - TX pause control when tx_enable = 1. 1: pause 0: TX work
    #[inline(always)]
    pub fn tx_pause(&self) -> TxPauseR {
        TxPauseR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_TX_PAUSE")
            .field("rsvd", &self.rsvd())
            .field("tx_pause", &self.tx_pause())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX pause control when tx_enable = 1. 1: pause 0: TX work
    #[inline(always)]
    pub fn tx_pause(&mut self) -> TxPauseW<AUDIO_TX_PAUSErs> {
        TxPauseW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AUDIO_TX_PAUSErs> {
        RsvdW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_pause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_pause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_TX_PAUSErs;
impl crate::RegisterSpec for AUDIO_TX_PAUSErs {
    type Ux = u32;
}
///`read()` method returns [`audio_tx_pause::R`](R) reader structure
impl crate::Readable for AUDIO_TX_PAUSErs {}
///`write(|w| ..)` method takes [`audio_tx_pause::W`](W) writer structure
impl crate::Writable for AUDIO_TX_PAUSErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_TX_PAUSE to value 0
impl crate::Resettable for AUDIO_TX_PAUSErs {
    const RESET_VALUE: u32 = 0;
}
