///Register `IRQ` reader
pub type R = crate::R<IRQrs>;
///Register `IRQ` writer
pub type W = crate::W<IRQrs>;
///Field `TX0_FIFO_OF` reader - tx channel 0 fifo overflow, write 1 to clear
pub type Tx0FifoOfR = crate::BitReader;
///Field `TX0_FIFO_OF` writer - tx channel 0 fifo overflow, write 1 to clear
pub type Tx0FifoOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX1_FIFO_OF` reader - tx channel 1 fifo overflow, write 1 to clear
pub type Tx1FifoOfR = crate::BitReader;
///Field `TX1_FIFO_OF` writer - tx channel 1 fifo overflow, write 1 to clear
pub type Tx1FifoOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX2_FIFO_OF` reader - tx channel 2 fifo overflow, write 1 to clear
pub type Tx2FifoOfR = crate::BitReader;
///Field `TX2_FIFO_OF` writer - tx channel 2 fifo overflow, write 1 to clear
pub type Tx2FifoOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX3_FIFO_OF` reader - tx channel 3 fifo overflow, write 1 to clear
pub type Tx3FifoOfR = crate::BitReader;
///Field `TX3_FIFO_OF` writer - tx channel 3 fifo overflow, write 1 to clear
pub type Tx3FifoOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX0_FIFO_UF` reader - rx channel 0 fifo underflow, write 1 to clear
pub type Rx0FifoUfR = crate::BitReader;
///Field `RX0_FIFO_UF` writer - rx channel 0 fifo underflow, write 1 to clear
pub type Rx0FifoUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX1_FIFO_UF` reader - rx channel 1 fifo underflow, write 1 to clear
pub type Rx1FifoUfR = crate::BitReader;
///Field `RX1_FIFO_UF` writer - rx channel 1 fifo underflow, write 1 to clear
pub type Rx1FifoUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OUT_FIFO_UF` reader - tx output fifo underflow, write 1 to clear
pub type TxOutFifoUfR = crate::BitReader;
///Field `TX_OUT_FIFO_UF` writer - tx output fifo underflow, write 1 to clear
pub type TxOutFifoUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IN_FIFO_OF` reader - rx input fifo overflow, write 1 to clear
pub type RxInFifoOfR = crate::BitReader;
///Field `RX_IN_FIFO_OF` writer - rx input fifo overflow, write 1 to clear
pub type RxInFifoOfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OUT0_FIFO_UF` reader - tx_out channel 0 fifo underflow, write 1 to clear
pub type TxOut0FifoUfR = crate::BitReader;
///Field `TX_OUT0_FIFO_UF` writer - tx_out channel 0 fifo underflow, write 1 to clear
pub type TxOut0FifoUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OUT1_FIFO_UF` reader - tx_out channel 1 fifo underflow, write 1 to clear
pub type TxOut1FifoUfR = crate::BitReader;
///Field `TX_OUT1_FIFO_UF` writer - tx_out channel 1 fifo underflow, write 1 to clear
pub type TxOut1FifoUfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX0_FIFO_OF_MASK` reader - tx channel 0 fifo overflow mask, 0: mask the interrupt
pub type Tx0FifoOfMaskR = crate::BitReader;
///Field `TX0_FIFO_OF_MASK` writer - tx channel 0 fifo overflow mask, 0: mask the interrupt
pub type Tx0FifoOfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX1_FIFO_OF_MASK` reader - tx channel 1 fifo overflow mask, 0: mask the interrupt
pub type Tx1FifoOfMaskR = crate::BitReader;
///Field `TX1_FIFO_OF_MASK` writer - tx channel 1 fifo overflow mask, 0: mask the interrupt
pub type Tx1FifoOfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX2_FIFO_OF_MASK` reader - tx channel 2 fifo overflow mask, 0: mask the interrupt
pub type Tx2FifoOfMaskR = crate::BitReader;
///Field `TX2_FIFO_OF_MASK` writer - tx channel 2 fifo overflow mask, 0: mask the interrupt
pub type Tx2FifoOfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX3_FIFO_OF_MASK` reader - tx channel 3 fifo overflow mask, 0: mask the interrupt
pub type Tx3FifoOfMaskR = crate::BitReader;
///Field `TX3_FIFO_OF_MASK` writer - tx channel 3 fifo overflow mask, 0: mask the interrupt
pub type Tx3FifoOfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX0_FIFO_UF_MASK` reader - rx channel 0 fifo underflow mask, 0: mask the interrupt
pub type Rx0FifoUfMaskR = crate::BitReader;
///Field `RX0_FIFO_UF_MASK` writer - rx channel 0 fifo underflow mask, 0: mask the interrupt
pub type Rx0FifoUfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX1_FIFO_UF_MASK` reader - rx channel 1 fifo underflow mask, 0: mask the interrupt
pub type Rx1FifoUfMaskR = crate::BitReader;
///Field `RX1_FIFO_UF_MASK` writer - rx channel 1 fifo underflow mask, 0: mask the interrupt
pub type Rx1FifoUfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OUT_FIFO_UF_MASK` reader - tx output fifo underflow mask, 0: mask the interrupt
pub type TxOutFifoUfMaskR = crate::BitReader;
///Field `TX_OUT_FIFO_UF_MASK` writer - tx output fifo underflow mask, 0: mask the interrupt
pub type TxOutFifoUfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IN_FIFO_OF_MASK` reader - rx input fifo overflow mask, 0: mask the interrupt
pub type RxInFifoOfMaskR = crate::BitReader;
///Field `RX_IN_FIFO_OF_MASK` writer - rx input fifo overflow mask, 0: mask the interrupt
pub type RxInFifoOfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OUT0_FIFO_UF_MASK` reader - tx_out channel 0 fifo underflow mask, 0: mask the interrupt
pub type TxOut0FifoUfMaskR = crate::BitReader;
///Field `TX_OUT0_FIFO_UF_MASK` writer - tx_out channel 0 fifo underflow mask, 0: mask the interrupt
pub type TxOut0FifoUfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OUT1_FIFO_UF_MASK` reader - tx_out channel 1 fifo underflow mask, 0: mask the interrupt
pub type TxOut1FifoUfMaskR = crate::BitReader;
///Field `TX_OUT1_FIFO_UF_MASK` writer - tx_out channel 1 fifo underflow mask, 0: mask the interrupt
pub type TxOut1FifoUfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - tx channel 0 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx0_fifo_of(&self) -> Tx0FifoOfR {
        Tx0FifoOfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - tx channel 1 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx1_fifo_of(&self) -> Tx1FifoOfR {
        Tx1FifoOfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tx channel 2 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx2_fifo_of(&self) -> Tx2FifoOfR {
        Tx2FifoOfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - tx channel 3 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx3_fifo_of(&self) -> Tx3FifoOfR {
        Tx3FifoOfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - rx channel 0 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn rx0_fifo_uf(&self) -> Rx0FifoUfR {
        Rx0FifoUfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - rx channel 1 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn rx1_fifo_uf(&self) -> Rx1FifoUfR {
        Rx1FifoUfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - tx output fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn tx_out_fifo_uf(&self) -> TxOutFifoUfR {
        TxOutFifoUfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - rx input fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn rx_in_fifo_of(&self) -> RxInFifoOfR {
        RxInFifoOfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - tx_out channel 0 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn tx_out0_fifo_uf(&self) -> TxOut0FifoUfR {
        TxOut0FifoUfR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - tx_out channel 1 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn tx_out1_fifo_uf(&self) -> TxOut1FifoUfR {
        TxOut1FifoUfR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - tx channel 0 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx0_fifo_of_mask(&self) -> Tx0FifoOfMaskR {
        Tx0FifoOfMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - tx channel 1 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx1_fifo_of_mask(&self) -> Tx1FifoOfMaskR {
        Tx1FifoOfMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - tx channel 2 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx2_fifo_of_mask(&self) -> Tx2FifoOfMaskR {
        Tx2FifoOfMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - tx channel 3 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx3_fifo_of_mask(&self) -> Tx3FifoOfMaskR {
        Tx3FifoOfMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - rx channel 0 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn rx0_fifo_uf_mask(&self) -> Rx0FifoUfMaskR {
        Rx0FifoUfMaskR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - rx channel 1 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn rx1_fifo_uf_mask(&self) -> Rx1FifoUfMaskR {
        Rx1FifoUfMaskR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - tx output fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx_out_fifo_uf_mask(&self) -> TxOutFifoUfMaskR {
        TxOutFifoUfMaskR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - rx input fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn rx_in_fifo_of_mask(&self) -> RxInFifoOfMaskR {
        RxInFifoOfMaskR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - tx_out channel 0 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx_out0_fifo_uf_mask(&self) -> TxOut0FifoUfMaskR {
        TxOut0FifoUfMaskR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - tx_out channel 1 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx_out1_fifo_uf_mask(&self) -> TxOut1FifoUfMaskR {
        TxOut1FifoUfMaskR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("tx_out1_fifo_uf_mask", &self.tx_out1_fifo_uf_mask())
            .field("tx_out0_fifo_uf_mask", &self.tx_out0_fifo_uf_mask())
            .field("rx_in_fifo_of_mask", &self.rx_in_fifo_of_mask())
            .field("tx_out_fifo_uf_mask", &self.tx_out_fifo_uf_mask())
            .field("rx1_fifo_uf_mask", &self.rx1_fifo_uf_mask())
            .field("rx0_fifo_uf_mask", &self.rx0_fifo_uf_mask())
            .field("tx3_fifo_of_mask", &self.tx3_fifo_of_mask())
            .field("tx2_fifo_of_mask", &self.tx2_fifo_of_mask())
            .field("tx1_fifo_of_mask", &self.tx1_fifo_of_mask())
            .field("tx0_fifo_of_mask", &self.tx0_fifo_of_mask())
            .field("tx_out1_fifo_uf", &self.tx_out1_fifo_uf())
            .field("tx_out0_fifo_uf", &self.tx_out0_fifo_uf())
            .field("rx_in_fifo_of", &self.rx_in_fifo_of())
            .field("tx_out_fifo_uf", &self.tx_out_fifo_uf())
            .field("rx1_fifo_uf", &self.rx1_fifo_uf())
            .field("rx0_fifo_uf", &self.rx0_fifo_uf())
            .field("tx3_fifo_of", &self.tx3_fifo_of())
            .field("tx2_fifo_of", &self.tx2_fifo_of())
            .field("tx1_fifo_of", &self.tx1_fifo_of())
            .field("tx0_fifo_of", &self.tx0_fifo_of())
            .finish()
    }
}
impl W {
    ///Bit 0 - tx channel 0 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx0_fifo_of(&mut self) -> Tx0FifoOfW<IRQrs> {
        Tx0FifoOfW::new(self, 0)
    }
    ///Bit 1 - tx channel 1 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx1_fifo_of(&mut self) -> Tx1FifoOfW<IRQrs> {
        Tx1FifoOfW::new(self, 1)
    }
    ///Bit 2 - tx channel 2 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx2_fifo_of(&mut self) -> Tx2FifoOfW<IRQrs> {
        Tx2FifoOfW::new(self, 2)
    }
    ///Bit 3 - tx channel 3 fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn tx3_fifo_of(&mut self) -> Tx3FifoOfW<IRQrs> {
        Tx3FifoOfW::new(self, 3)
    }
    ///Bit 4 - rx channel 0 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn rx0_fifo_uf(&mut self) -> Rx0FifoUfW<IRQrs> {
        Rx0FifoUfW::new(self, 4)
    }
    ///Bit 5 - rx channel 1 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn rx1_fifo_uf(&mut self) -> Rx1FifoUfW<IRQrs> {
        Rx1FifoUfW::new(self, 5)
    }
    ///Bit 6 - tx output fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn tx_out_fifo_uf(&mut self) -> TxOutFifoUfW<IRQrs> {
        TxOutFifoUfW::new(self, 6)
    }
    ///Bit 7 - rx input fifo overflow, write 1 to clear
    #[inline(always)]
    pub fn rx_in_fifo_of(&mut self) -> RxInFifoOfW<IRQrs> {
        RxInFifoOfW::new(self, 7)
    }
    ///Bit 8 - tx_out channel 0 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn tx_out0_fifo_uf(&mut self) -> TxOut0FifoUfW<IRQrs> {
        TxOut0FifoUfW::new(self, 8)
    }
    ///Bit 9 - tx_out channel 1 fifo underflow, write 1 to clear
    #[inline(always)]
    pub fn tx_out1_fifo_uf(&mut self) -> TxOut1FifoUfW<IRQrs> {
        TxOut1FifoUfW::new(self, 9)
    }
    ///Bit 16 - tx channel 0 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx0_fifo_of_mask(&mut self) -> Tx0FifoOfMaskW<IRQrs> {
        Tx0FifoOfMaskW::new(self, 16)
    }
    ///Bit 17 - tx channel 1 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx1_fifo_of_mask(&mut self) -> Tx1FifoOfMaskW<IRQrs> {
        Tx1FifoOfMaskW::new(self, 17)
    }
    ///Bit 18 - tx channel 2 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx2_fifo_of_mask(&mut self) -> Tx2FifoOfMaskW<IRQrs> {
        Tx2FifoOfMaskW::new(self, 18)
    }
    ///Bit 19 - tx channel 3 fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx3_fifo_of_mask(&mut self) -> Tx3FifoOfMaskW<IRQrs> {
        Tx3FifoOfMaskW::new(self, 19)
    }
    ///Bit 20 - rx channel 0 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn rx0_fifo_uf_mask(&mut self) -> Rx0FifoUfMaskW<IRQrs> {
        Rx0FifoUfMaskW::new(self, 20)
    }
    ///Bit 21 - rx channel 1 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn rx1_fifo_uf_mask(&mut self) -> Rx1FifoUfMaskW<IRQrs> {
        Rx1FifoUfMaskW::new(self, 21)
    }
    ///Bit 22 - tx output fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx_out_fifo_uf_mask(&mut self) -> TxOutFifoUfMaskW<IRQrs> {
        TxOutFifoUfMaskW::new(self, 22)
    }
    ///Bit 23 - rx input fifo overflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn rx_in_fifo_of_mask(&mut self) -> RxInFifoOfMaskW<IRQrs> {
        RxInFifoOfMaskW::new(self, 23)
    }
    ///Bit 24 - tx_out channel 0 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx_out0_fifo_uf_mask(&mut self) -> TxOut0FifoUfMaskW<IRQrs> {
        TxOut0FifoUfMaskW::new(self, 24)
    }
    ///Bit 25 - tx_out channel 1 fifo underflow mask, 0: mask the interrupt
    #[inline(always)]
    pub fn tx_out1_fifo_uf_mask(&mut self) -> TxOut1FifoUfMaskW<IRQrs> {
        TxOut1FifoUfMaskW::new(self, 25)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRQrs;
impl crate::RegisterSpec for IRQrs {
    type Ux = u32;
}
///`read()` method returns [`irq::R`](R) reader structure
impl crate::Readable for IRQrs {}
///`write(|w| ..)` method takes [`irq::W`](W) writer structure
impl crate::Writable for IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ to value 0
impl crate::Resettable for IRQrs {
    const RESET_VALUE: u32 = 0;
}
