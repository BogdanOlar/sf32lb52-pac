///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Register `STATUS` writer
pub type W = crate::W<STATUSrs>;
///Field `BSY` reader - SPI controller Busy 0: SPI controller is idle or disabled 1: SPI controller is currently transmitting or receiving framed data
pub type BsyR = crate::BitReader;
///Field `BSY` writer - SPI controller Busy 0: SPI controller is idle or disabled 1: SPI controller is currently transmitting or receiving framed data
pub type BsyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSS` reader - Clock Synchronization Status 0: SPI controller is ready for slave clock operations 1: SPI controller is currently busy synchronizing slave mode signals
pub type CssR = crate::BitReader;
///Field `CSS` writer - Clock Synchronization Status 0: SPI controller is ready for slave clock operations 1: SPI controller is currently busy synchronizing slave mode signals
pub type CssW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD6` reader -
pub type Rsvd6R = crate::BitReader;
///Field `RSVD6` writer -
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TINT` reader - Receiver Time-out Interrupt 0: No receiver time-out is pending 1: Receiver time-out pending, causes an interrupt request
pub type TintR = crate::BitReader;
///Field `TINT` writer - Receiver Time-out Interrupt 0: No receiver time-out is pending 1: Receiver time-out pending, causes an interrupt request
pub type TintW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::BitReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFS` reader - Transmit FIFO Service Request 0: TX FIFO level exceeds the TFT threshold (TFT + 1) or SPI controller is disabled 1: TXFIFO level is at or below TFT threshold (TFT + 1), causes an interrupt request
pub type TfsR = crate::BitReader;
///Field `TFS` writer - Transmit FIFO Service Request 0: TX FIFO level exceeds the TFT threshold (TFT + 1) or SPI controller is disabled 1: TXFIFO level is at or below TFT threshold (TFT + 1), causes an interrupt request
pub type TfsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TNF` reader - Transmit FIFO Not Full 0: TXFIFO is full 1: TXFIFO is not full
pub type TnfR = crate::BitReader;
///Field `TNF` writer - Transmit FIFO Not Full 0: TXFIFO is full 1: TXFIFO is not full
pub type TnfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFL` reader - Transmit FIFO Level This field is the number of entries in TXFIFO.When the value 0x0 is read, the TXFIFO is either empty or full, and software should read the \[Transmit FIFO Not Full\]
///field.
pub type TflR = crate::FieldReader;
///Field `TFL` writer - Transmit FIFO Level This field is the number of entries in TXFIFO.When the value 0x0 is read, the TXFIFO is either empty or full, and software should read the \[Transmit FIFO Not Full\]
///field.
pub type TflW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TUR` reader - Transmit FIFO Underrun 0: The TXFIFO has not experienced an underrun 1: A read from the TXFIFO was attempted when the TXFIFO was empty, causes an interrupt if it is enabled (\[Transmit FIFO Underrun Interrupt Mask\]
///in the INT EN Register is 0)
pub type TurR = crate::BitReader;
///Field `TUR` writer - Transmit FIFO Underrun 0: The TXFIFO has not experienced an underrun 1: A read from the TXFIFO was attempted when the TXFIFO was empty, causes an interrupt if it is enabled (\[Transmit FIFO Underrun Interrupt Mask\]
///in the INT EN Register is 0)
pub type TurW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFS` reader - Receive FIFO Service Request 0: RXFIFO level is at or below RFT threshold (RFT) or SPI controller is disabled 1: RXFIFO level exceeds RFT threshold (RFT), causes an interrupt request
pub type RfsR = crate::BitReader;
///Field `RFS` writer - Receive FIFO Service Request 0: RXFIFO level is at or below RFT threshold (RFT) or SPI controller is disabled 1: RXFIFO level exceeds RFT threshold (RFT), causes an interrupt request
pub type RfsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNE` reader - Receive FIFO Not Empty 0: RXFIFO is empty 1: RXFIFO is not empty
pub type RneR = crate::BitReader;
///Field `RNE` writer - Receive FIFO Not Empty 0: RXFIFO is empty 1: RXFIFO is not empty
pub type RneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFL` reader - Receive FIFO Level This field is the number of entries minus one in RXFIFO. When the value 0xF is read, the RXFIFO is either empty or full, and software should read the \[Receive FIFO Not Empty\]
///field.
pub type RflR = crate::FieldReader;
///Field `RFL` writer - Receive FIFO Level This field is the number of entries minus one in RXFIFO. When the value 0xF is read, the RXFIFO is either empty or full, and software should read the \[Receive FIFO Not Empty\]
///field.
pub type RflW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROR` reader - Receive FIFO Overrun 0: RXFIFO has not experienced an overrun 1: Attempted data write to full RXFIFO, causes an interrupt request
pub type RorR = crate::BitReader;
///Field `ROR` writer - Receive FIFO Overrun 0: RXFIFO has not experienced an overrun 1: Attempted data write to full RXFIFO, causes an interrupt request
pub type RorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OSS` reader - TX FIFO Odd Sample Status When SPI controller is in packed mode, the number of samples in the TX FIFO is: (\[Transmit FIFO Level\]*2 + this field), when \[Transmit FIFO Not Full\]
///= 1 32, when \[Transmit FIFO Not Full\]
///= 0. The TX FIFO cannot accept new data when \[Transmit FIFO Not Full\]
///= 1 and \[Transmit FIFO Level\]
///= 15 and this field = 1. (The TX FIFO has 31 samples). 0: TxFIFO entry has an even number of samples 1: TxFIFO entry has an odd number of samples Note that this bit needs to be read only when FIFO Packing is enabled (\[FIFO Packing Enable\]
///in the FIFO Control Register is set). Otherwise, this bit is zero.
pub type TxOssR = crate::BitReader;
///Field `TX_OSS` writer - TX FIFO Odd Sample Status When SPI controller is in packed mode, the number of samples in the TX FIFO is: (\[Transmit FIFO Level\]*2 + this field), when \[Transmit FIFO Not Full\]
///= 1 32, when \[Transmit FIFO Not Full\]
///= 0. The TX FIFO cannot accept new data when \[Transmit FIFO Not Full\]
///= 1 and \[Transmit FIFO Level\]
///= 15 and this field = 1. (The TX FIFO has 31 samples). 0: TxFIFO entry has an even number of samples 1: TxFIFO entry has an odd number of samples Note that this bit needs to be read only when FIFO Packing is enabled (\[FIFO Packing Enable\]
///in the FIFO Control Register is set). Otherwise, this bit is zero.
pub type TxOssW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSS` reader - Odd Sample Status 0: RxFIFO entry has two samples 1: RxFIFO entry has one sample Note that this bit needs to be looked at only when FIFO Packing is enabled (FPCKE field in FIFO Control Register is set). Otherwise, this bit is zero. When SPI controller is in Packed mode and the CPU is used instead of DMA to read the RxFIFO, the CPU should make sure that \[Receive FIFO Not Empty\]
///= 1 AND this field = 0 before it attempts to read the RxFIFO.
pub type OssR = crate::BitReader;
///Field `OSS` writer - Odd Sample Status 0: RxFIFO entry has two samples 1: RxFIFO entry has one sample Note that this bit needs to be looked at only when FIFO Packing is enabled (FPCKE field in FIFO Control Register is set). Otherwise, this bit is zero. When SPI controller is in Packed mode and the CPU is used instead of DMA to read the RxFIFO, the CPU should make sure that \[Receive FIFO Not Empty\]
///= 1 AND this field = 0 before it attempts to read the RxFIFO.
pub type OssW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - SPI controller Busy 0: SPI controller is idle or disabled 1: SPI controller is currently transmitting or receiving framed data
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock Synchronization Status 0: SPI controller is ready for slave clock operations 1: SPI controller is currently busy synchronizing slave mode signals
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receiver Time-out Interrupt 0: No receiver time-out is pending 1: Receiver time-out pending, causes an interrupt request
    #[inline(always)]
    pub fn tint(&self) -> TintR {
        TintR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit FIFO Service Request 0: TX FIFO level exceeds the TFT threshold (TFT + 1) or SPI controller is disabled 1: TXFIFO level is at or below TFT threshold (TFT + 1), causes an interrupt request
    #[inline(always)]
    pub fn tfs(&self) -> TfsR {
        TfsR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit FIFO Not Full 0: TXFIFO is full 1: TXFIFO is not full
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - Transmit FIFO Level This field is the number of entries in TXFIFO.When the value 0x0 is read, the TXFIFO is either empty or full, and software should read the \[Transmit FIFO Not Full\]
    ///field.
    #[inline(always)]
    pub fn tfl(&self) -> TflR {
        TflR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmit FIFO Underrun 0: The TXFIFO has not experienced an underrun 1: A read from the TXFIFO was attempted when the TXFIFO was empty, causes an interrupt if it is enabled (\[Transmit FIFO Underrun Interrupt Mask\]
    ///in the INT EN Register is 0)
    #[inline(always)]
    pub fn tur(&self) -> TurR {
        TurR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Receive FIFO Service Request 0: RXFIFO level is at or below RFT threshold (RFT) or SPI controller is disabled 1: RXFIFO level exceeds RFT threshold (RFT), causes an interrupt request
    #[inline(always)]
    pub fn rfs(&self) -> RfsR {
        RfsR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Receive FIFO Not Empty 0: RXFIFO is empty 1: RXFIFO is not empty
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:18 - Receive FIFO Level This field is the number of entries minus one in RXFIFO. When the value 0xF is read, the RXFIFO is either empty or full, and software should read the \[Receive FIFO Not Empty\]
    ///field.
    #[inline(always)]
    pub fn rfl(&self) -> RflR {
        RflR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Receive FIFO Overrun 0: RXFIFO has not experienced an overrun 1: Attempted data write to full RXFIFO, causes an interrupt request
    #[inline(always)]
    pub fn ror(&self) -> RorR {
        RorR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - TX FIFO Odd Sample Status When SPI controller is in packed mode, the number of samples in the TX FIFO is: (\[Transmit FIFO Level\]*2 + this field), when \[Transmit FIFO Not Full\]
    ///= 1 32, when \[Transmit FIFO Not Full\]
    ///= 0. The TX FIFO cannot accept new data when \[Transmit FIFO Not Full\]
    ///= 1 and \[Transmit FIFO Level\]
    ///= 15 and this field = 1. (The TX FIFO has 31 samples). 0: TxFIFO entry has an even number of samples 1: TxFIFO entry has an odd number of samples Note that this bit needs to be read only when FIFO Packing is enabled (\[FIFO Packing Enable\]
    ///in the FIFO Control Register is set). Otherwise, this bit is zero.
    #[inline(always)]
    pub fn tx_oss(&self) -> TxOssR {
        TxOssR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Odd Sample Status 0: RxFIFO entry has two samples 1: RxFIFO entry has one sample Note that this bit needs to be looked at only when FIFO Packing is enabled (FPCKE field in FIFO Control Register is set). Otherwise, this bit is zero. When SPI controller is in Packed mode and the CPU is used instead of DMA to read the RxFIFO, the CPU should make sure that \[Receive FIFO Not Empty\]
    ///= 1 AND this field = 0 before it attempts to read the RxFIFO.
    #[inline(always)]
    pub fn oss(&self) -> OssR {
        OssR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rsvd", &self.rsvd())
            .field("oss", &self.oss())
            .field("tx_oss", &self.tx_oss())
            .field("rsvd2", &self.rsvd2())
            .field("ror", &self.ror())
            .field("rsvd3", &self.rsvd3())
            .field("rfl", &self.rfl())
            .field("rne", &self.rne())
            .field("rfs", &self.rfs())
            .field("tur", &self.tur())
            .field("rsvd4", &self.rsvd4())
            .field("tfl", &self.tfl())
            .field("tnf", &self.tnf())
            .field("tfs", &self.tfs())
            .field("rsvd5", &self.rsvd5())
            .field("tint", &self.tint())
            .field("rsvd6", &self.rsvd6())
            .field("css", &self.css())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI controller Busy 0: SPI controller is idle or disabled 1: SPI controller is currently transmitting or receiving framed data
    #[inline(always)]
    pub fn bsy(&mut self) -> BsyW<STATUSrs> {
        BsyW::new(self, 0)
    }
    ///Bit 1 - Clock Synchronization Status 0: SPI controller is ready for slave clock operations 1: SPI controller is currently busy synchronizing slave mode signals
    #[inline(always)]
    pub fn css(&mut self) -> CssW<STATUSrs> {
        CssW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd6(&mut self) -> Rsvd6W<STATUSrs> {
        Rsvd6W::new(self, 2)
    }
    ///Bit 3 - Receiver Time-out Interrupt 0: No receiver time-out is pending 1: Receiver time-out pending, causes an interrupt request
    #[inline(always)]
    pub fn tint(&mut self) -> TintW<STATUSrs> {
        TintW::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<STATUSrs> {
        Rsvd5W::new(self, 4)
    }
    ///Bit 5 - Transmit FIFO Service Request 0: TX FIFO level exceeds the TFT threshold (TFT + 1) or SPI controller is disabled 1: TXFIFO level is at or below TFT threshold (TFT + 1), causes an interrupt request
    #[inline(always)]
    pub fn tfs(&mut self) -> TfsW<STATUSrs> {
        TfsW::new(self, 5)
    }
    ///Bit 6 - Transmit FIFO Not Full 0: TXFIFO is full 1: TXFIFO is not full
    #[inline(always)]
    pub fn tnf(&mut self) -> TnfW<STATUSrs> {
        TnfW::new(self, 6)
    }
    ///Bits 7:10 - Transmit FIFO Level This field is the number of entries in TXFIFO.When the value 0x0 is read, the TXFIFO is either empty or full, and software should read the \[Transmit FIFO Not Full\]
    ///field.
    #[inline(always)]
    pub fn tfl(&mut self) -> TflW<STATUSrs> {
        TflW::new(self, 7)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<STATUSrs> {
        Rsvd4W::new(self, 11)
    }
    ///Bit 12 - Transmit FIFO Underrun 0: The TXFIFO has not experienced an underrun 1: A read from the TXFIFO was attempted when the TXFIFO was empty, causes an interrupt if it is enabled (\[Transmit FIFO Underrun Interrupt Mask\]
    ///in the INT EN Register is 0)
    #[inline(always)]
    pub fn tur(&mut self) -> TurW<STATUSrs> {
        TurW::new(self, 12)
    }
    ///Bit 13 - Receive FIFO Service Request 0: RXFIFO level is at or below RFT threshold (RFT) or SPI controller is disabled 1: RXFIFO level exceeds RFT threshold (RFT), causes an interrupt request
    #[inline(always)]
    pub fn rfs(&mut self) -> RfsW<STATUSrs> {
        RfsW::new(self, 13)
    }
    ///Bit 14 - Receive FIFO Not Empty 0: RXFIFO is empty 1: RXFIFO is not empty
    #[inline(always)]
    pub fn rne(&mut self) -> RneW<STATUSrs> {
        RneW::new(self, 14)
    }
    ///Bits 15:18 - Receive FIFO Level This field is the number of entries minus one in RXFIFO. When the value 0xF is read, the RXFIFO is either empty or full, and software should read the \[Receive FIFO Not Empty\]
    ///field.
    #[inline(always)]
    pub fn rfl(&mut self) -> RflW<STATUSrs> {
        RflW::new(self, 15)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<STATUSrs> {
        Rsvd3W::new(self, 19)
    }
    ///Bit 20 - Receive FIFO Overrun 0: RXFIFO has not experienced an overrun 1: Attempted data write to full RXFIFO, causes an interrupt request
    #[inline(always)]
    pub fn ror(&mut self) -> RorW<STATUSrs> {
        RorW::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<STATUSrs> {
        Rsvd2W::new(self, 21)
    }
    ///Bit 22 - TX FIFO Odd Sample Status When SPI controller is in packed mode, the number of samples in the TX FIFO is: (\[Transmit FIFO Level\]*2 + this field), when \[Transmit FIFO Not Full\]
    ///= 1 32, when \[Transmit FIFO Not Full\]
    ///= 0. The TX FIFO cannot accept new data when \[Transmit FIFO Not Full\]
    ///= 1 and \[Transmit FIFO Level\]
    ///= 15 and this field = 1. (The TX FIFO has 31 samples). 0: TxFIFO entry has an even number of samples 1: TxFIFO entry has an odd number of samples Note that this bit needs to be read only when FIFO Packing is enabled (\[FIFO Packing Enable\]
    ///in the FIFO Control Register is set). Otherwise, this bit is zero.
    #[inline(always)]
    pub fn tx_oss(&mut self) -> TxOssW<STATUSrs> {
        TxOssW::new(self, 22)
    }
    ///Bit 23 - Odd Sample Status 0: RxFIFO entry has two samples 1: RxFIFO entry has one sample Note that this bit needs to be looked at only when FIFO Packing is enabled (FPCKE field in FIFO Control Register is set). Otherwise, this bit is zero. When SPI controller is in Packed mode and the CPU is used instead of DMA to read the RxFIFO, the CPU should make sure that \[Receive FIFO Not Empty\]
    ///= 1 AND this field = 0 before it attempts to read the RxFIFO.
    #[inline(always)]
    pub fn oss(&mut self) -> OssW<STATUSrs> {
        OssW::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<STATUSrs> {
        RsvdW::new(self, 24)
    }
}
///Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
