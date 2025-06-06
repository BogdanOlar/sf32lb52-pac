///Register `TX_EQUALIZER_EN` reader
pub type R = crate::R<TX_EQUALIZER_ENrs>;
///Register `TX_EQUALIZER_EN` writer
pub type W = crate::W<TX_EQUALIZER_ENrs>;
///Field `TX_EQUALIZER_EN` reader - 0: Disable TX equalizer 1: Enable TX equalizer equalizer is not implemented
pub type TxEqualizerEnR = crate::BitReader;
///Field `TX_EQUALIZER_EN` writer - 0: Disable TX equalizer 1: Enable TX equalizer equalizer is not implemented
pub type TxEqualizerEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - 0: Disable TX equalizer 1: Enable TX equalizer equalizer is not implemented
    #[inline(always)]
    pub fn tx_equalizer_en(&self) -> TxEqualizerEnR {
        TxEqualizerEnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_EQUALIZER_EN")
            .field("rsvd", &self.rsvd())
            .field("tx_equalizer_en", &self.tx_equalizer_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: Disable TX equalizer 1: Enable TX equalizer equalizer is not implemented
    #[inline(always)]
    pub fn tx_equalizer_en(&mut self) -> TxEqualizerEnW<TX_EQUALIZER_ENrs> {
        TxEqualizerEnW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TX_EQUALIZER_ENrs> {
        RsvdW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_equalizer_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_equalizer_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_EQUALIZER_ENrs;
impl crate::RegisterSpec for TX_EQUALIZER_ENrs {
    type Ux = u32;
}
///`read()` method returns [`tx_equalizer_en::R`](R) reader structure
impl crate::Readable for TX_EQUALIZER_ENrs {}
///`write(|w| ..)` method takes [`tx_equalizer_en::W`](W) writer structure
impl crate::Writable for TX_EQUALIZER_ENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_EQUALIZER_EN to value 0
impl crate::Resettable for TX_EQUALIZER_ENrs {
    const RESET_VALUE: u32 = 0;
}
