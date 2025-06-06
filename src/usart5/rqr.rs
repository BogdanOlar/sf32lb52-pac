///Register `RQR` reader
pub type R = crate::R<RQRrs>;
///Register `RQR` writer
pub type W = crate::W<RQRrs>;
///Field `RXFRQ` reader - Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR
pub type RxfrqR = crate::BitReader;
///Field `RXFRQ` writer - Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR
pub type RxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFRQ` reader - Tx data flush requestReserved-Do not modify
pub type TxfrqR = crate::BitReader;
///Field `TXFRQ` writer - Tx data flush requestReserved-Do not modify
pub type TxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR
    #[inline(always)]
    pub fn rxfrq(&self) -> RxfrqR {
        RxfrqR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tx data flush requestReserved-Do not modify
    #[inline(always)]
    pub fn txfrq(&self) -> TxfrqR {
        TxfrqR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RQR")
            .field("txfrq", &self.txfrq())
            .field("rxfrq", &self.rxfrq())
            .finish()
    }
}
impl W {
    ///Bit 3 - Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RxfrqW<RQRrs> {
        RxfrqW::new(self, 3)
    }
    ///Bit 4 - Tx data flush requestReserved-Do not modify
    #[inline(always)]
    pub fn txfrq(&mut self) -> TxfrqW<RQRrs> {
        TxfrqW::new(self, 4)
    }
}
///Request Register
///
///You can [`read`](crate::Reg::read) this register and get [`rqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
///`read()` method returns [`rqr::R`](R) reader structure
impl crate::Readable for RQRrs {}
///`write(|w| ..)` method takes [`rqr::W`](W) writer structure
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RQR to value 0
impl crate::Resettable for RQRrs {
    const RESET_VALUE: u32 = 0;
}
