///Register `FIFOCR` reader
pub type R = crate::R<FIFOCRrs>;
///Register `FIFOCR` writer
pub type W = crate::W<FIFOCRrs>;
///Field `RXCLR` reader - write 1 to clear Rx FIFO
pub type RxclrR = crate::BitReader;
///Field `RXCLR` writer - write 1 to clear Rx FIFO
pub type RxclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXE` reader - Rx FIFO empty
pub type RxeR = crate::BitReader;
///Field `RXE` writer - Rx FIFO empty
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXCLR` reader - write 1 to clear Tx FIFO
pub type TxclrR = crate::BitReader;
///Field `TXCLR` writer - write 1 to clear Tx FIFO
pub type TxclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXF` reader - Tx FIFO full flag
pub type TxfR = crate::BitReader;
///Field `TXF` writer - Tx FIFO full flag
pub type TxfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSLOTS` reader - When DMA enabled, asserts DMA reqeust if TXFIFO vacant slots is greater than or equal to TXSLOTS. Note: this field should be set in accordance to the burst length in DMA. For example, if DMA employs BURST8 transction, then this filed is set to 8
pub type TxslotsR = crate::FieldReader;
///Field `TXSLOTS` writer - When DMA enabled, asserts DMA reqeust if TXFIFO vacant slots is greater than or equal to TXSLOTS. Note: this field should be set in accordance to the burst length in DMA. For example, if DMA employs BURST8 transction, then this filed is set to 8
pub type TxslotsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - write 1 to clear Rx FIFO
    #[inline(always)]
    pub fn rxclr(&self) -> RxclrR {
        RxclrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO empty
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - write 1 to clear Tx FIFO
    #[inline(always)]
    pub fn txclr(&self) -> TxclrR {
        TxclrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx FIFO full flag
    #[inline(always)]
    pub fn txf(&self) -> TxfR {
        TxfR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:14 - When DMA enabled, asserts DMA reqeust if TXFIFO vacant slots is greater than or equal to TXSLOTS. Note: this field should be set in accordance to the burst length in DMA. For example, if DMA employs BURST8 transction, then this filed is set to 8
    #[inline(always)]
    pub fn txslots(&self) -> TxslotsR {
        TxslotsR::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOCR")
            .field("txslots", &self.txslots())
            .field("txf", &self.txf())
            .field("txclr", &self.txclr())
            .field("rxe", &self.rxe())
            .field("rxclr", &self.rxclr())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to clear Rx FIFO
    #[inline(always)]
    pub fn rxclr(&mut self) -> RxclrW<FIFOCRrs> {
        RxclrW::new(self, 0)
    }
    ///Bit 1 - Rx FIFO empty
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<FIFOCRrs> {
        RxeW::new(self, 1)
    }
    ///Bit 8 - write 1 to clear Tx FIFO
    #[inline(always)]
    pub fn txclr(&mut self) -> TxclrW<FIFOCRrs> {
        TxclrW::new(self, 8)
    }
    ///Bit 9 - Tx FIFO full flag
    #[inline(always)]
    pub fn txf(&mut self) -> TxfW<FIFOCRrs> {
        TxfW::new(self, 9)
    }
    ///Bits 10:14 - When DMA enabled, asserts DMA reqeust if TXFIFO vacant slots is greater than or equal to TXSLOTS. Note: this field should be set in accordance to the burst length in DMA. For example, if DMA employs BURST8 transction, then this filed is set to 8
    #[inline(always)]
    pub fn txslots(&mut self) -> TxslotsW<FIFOCRrs> {
        TxslotsW::new(self, 10)
    }
}
///FIFO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`fifocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FIFOCRrs;
impl crate::RegisterSpec for FIFOCRrs {
    type Ux = u32;
}
///`read()` method returns [`fifocr::R`](R) reader structure
impl crate::Readable for FIFOCRrs {}
///`write(|w| ..)` method takes [`fifocr::W`](W) writer structure
impl crate::Writable for FIFOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFOCR to value 0
impl crate::Resettable for FIFOCRrs {
    const RESET_VALUE: u32 = 0;
}
