///Register `INT_STATUS` reader
pub type R = crate::R<INT_STATUSrs>;
///Register `INT_STATUS` writer
pub type W = crate::W<INT_STATUSrs>;
///Field `RX_FIFO_OVERFLOW` reader - RX FIFO push overflow
pub type RxFifoOverflowR = crate::BitReader;
///Field `RX_FIFO_OVERFLOW` writer - RX FIFO push overflow
pub type RxFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_FIFO_UNDERFLOW` reader - TX FIFO pop underflow
pub type TxFifoUnderflowR = crate::BitReader;
///Field `TX_FIFO_UNDERFLOW` writer - TX FIFO pop underflow
pub type TxFifoUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RX FIFO push overflow
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RxFifoOverflowR {
        RxFifoOverflowR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX FIFO pop underflow
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("tx_fifo_underflow", &self.tx_fifo_underflow())
            .field("rx_fifo_overflow", &self.rx_fifo_overflow())
            .finish()
    }
}
impl W {
    ///Bit 0 - RX FIFO push overflow
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RxFifoOverflowW<INT_STATUSrs> {
        RxFifoOverflowW::new(self, 0)
    }
    ///Bit 1 - TX FIFO pop underflow
    #[inline(always)]
    pub fn tx_fifo_underflow(&mut self) -> TxFifoUnderflowW<INT_STATUSrs> {
        TxFifoUnderflowW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_STATUSrs;
impl crate::RegisterSpec for INT_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`int_status::R`](R) reader structure
impl crate::Readable for INT_STATUSrs {}
///`write(|w| ..)` method takes [`int_status::W`](W) writer structure
impl crate::Writable for INT_STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_STATUS to value 0
impl crate::Resettable for INT_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
