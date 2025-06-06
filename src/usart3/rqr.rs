///Register `RQR` reader
pub type R = crate::R<RQRrs>;
///Register `RQR` writer
pub type W = crate::W<RQRrs>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFRQ` reader - Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR
pub type RxfrqR = crate::BitReader;
///Field `RXFRQ` writer - Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR
pub type RxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFRQ` reader - Tx data flush requestReserved-Do not modify
pub type TxfrqR = crate::BitReader;
///Field `TXFRQ` writer - Tx data flush requestReserved-Do not modify
pub type TxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 1) != 0)
    }
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
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RQR")
            .field("rsvd", &self.rsvd())
            .field("txfrq", &self.txfrq())
            .field("rxfrq", &self.rxfrq())
            .field("rsvd2", &self.rsvd2())
            .field("rsvd3", &self.rsvd3())
            .field("rsvd4", &self.rsvd4())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<RQRrs> {
        Rsvd4W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<RQRrs> {
        Rsvd3W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<RQRrs> {
        Rsvd2W::new(self, 2)
    }
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
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RQRrs> {
        RsvdW::new(self, 5)
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
