///Register `INT_MASK` reader
pub type R = crate::R<INT_MASKrs>;
///Register `INT_MASK` writer
pub type W = crate::W<INT_MASKrs>;
///Field `RX_FIFO_INT_MASK` reader - Interrupt mask for RX FIFO push overflow, high active
pub type RxFifoIntMaskR = crate::BitReader;
///Field `RX_FIFO_INT_MASK` writer - Interrupt mask for RX FIFO push overflow, high active
pub type RxFifoIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_FIFO_INT_MASK` reader - Interrupt mask for TX FIFO pop underflow, high active
pub type TxFifoIntMaskR = crate::BitReader;
///Field `TX_FIFO_INT_MASK` writer - Interrupt mask for TX FIFO pop underflow, high active
pub type TxFifoIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Interrupt mask for RX FIFO push overflow, high active
    #[inline(always)]
    pub fn rx_fifo_int_mask(&self) -> RxFifoIntMaskR {
        RxFifoIntMaskR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt mask for TX FIFO pop underflow, high active
    #[inline(always)]
    pub fn tx_fifo_int_mask(&self) -> TxFifoIntMaskR {
        TxFifoIntMaskR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MASK")
            .field("tx_fifo_int_mask", &self.tx_fifo_int_mask())
            .field("rx_fifo_int_mask", &self.rx_fifo_int_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt mask for RX FIFO push overflow, high active
    #[inline(always)]
    pub fn rx_fifo_int_mask(&mut self) -> RxFifoIntMaskW<INT_MASKrs> {
        RxFifoIntMaskW::new(self, 0)
    }
    ///Bit 1 - Interrupt mask for TX FIFO pop underflow, high active
    #[inline(always)]
    pub fn tx_fifo_int_mask(&mut self) -> TxFifoIntMaskW<INT_MASKrs> {
        TxFifoIntMaskW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_MASKrs;
impl crate::RegisterSpec for INT_MASKrs {
    type Ux = u32;
}
///`read()` method returns [`int_mask::R`](R) reader structure
impl crate::Readable for INT_MASKrs {}
///`write(|w| ..)` method takes [`int_mask::W`](W) writer structure
impl crate::Writable for INT_MASKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_MASK to value 0x03
impl crate::Resettable for INT_MASKrs {
    const RESET_VALUE: u32 = 0x03;
}
