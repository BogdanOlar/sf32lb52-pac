///Register `FIFO_CTRL` reader
pub type R = crate::R<FIFO_CTRLrs>;
///Register `FIFO_CTRL` writer
pub type W = crate::W<FIFO_CTRLrs>;
///Field `TFT` reader - TXFIFO Trigger Threshold This field sets the threshold level at which TXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
pub type TftR = crate::FieldReader;
///Field `TFT` writer - TXFIFO Trigger Threshold This field sets the threshold level at which TXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
pub type TftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RFT` reader - RXFIFO Trigger Threshold This field sets the threshold level at which RXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
pub type RftR = crate::FieldReader;
///Field `RFT` writer - RXFIFO Trigger Threshold This field sets the threshold level at which RXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
pub type RftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TSRE` reader - Transmit Service Request Enable 0: TxFIFO DMA service request is disabled 1: TxFIFO DMA service request is enabled
pub type TsreR = crate::BitReader;
///Field `TSRE` writer - Transmit Service Request Enable 0: TxFIFO DMA service request is disabled 1: TxFIFO DMA service request is enabled
pub type TsreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSRE` reader - Receive Service Request Enable 0: RxFIFO DMA service request is disabled 1: RxFIFO DMA service request is enabled
pub type RsreR = crate::BitReader;
///Field `RSRE` writer - Receive Service Request Enable 0: RxFIFO DMA service request is disabled 1: RxFIFO DMA service request is enabled
pub type RsreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFO_RD_ENDIAN` reader - apb_prdata Read from Rx FIFO Endian 0x0 = apb_prdata\[31:0\]
///= rxfifo_wdata\[31:0\]
///0x1 = apb_prdata\[31:0\]
///= {rxfifo_wdata\[15:0\], rxfifo_wdata\[31:16\]} 0x2 = apb_prdata\[31:0\]= {rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\], rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\]} 0x3 = apb_prdata\[31:0\]= {rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\], rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\]}
pub type RxfifoRdEndianR = crate::FieldReader;
///Field `RXFIFO_RD_ENDIAN` writer - apb_prdata Read from Rx FIFO Endian 0x0 = apb_prdata\[31:0\]
///= rxfifo_wdata\[31:0\]
///0x1 = apb_prdata\[31:0\]
///= {rxfifo_wdata\[15:0\], rxfifo_wdata\[31:16\]} 0x2 = apb_prdata\[31:0\]= {rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\], rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\]} 0x3 = apb_prdata\[31:0\]= {rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\], rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\]}
pub type RxfifoRdEndianW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXFIFO_WR_ENDIAN` reader - apb_pwdata Write to TxFIFO Endian 0x0: txfifo_wdata\[31:0\]
///= apb_pwdata\[31:0\]
///0x1: fifo_wdata\[31:0\]
///= {apb_pwdata\[15:0\], apb_pwdata\[31:16\]} 0x2: txfifo_wdata\[31:0\]
///= {apb_pwdata\[7:0\], apb_pwdata\[15:8\], apb_pwdata\[23:16\], apb_pwdata\[31:24\]} 0x3: txfifo_wdata\[31:0\]
///= {apb_pwdata\[23:16\], apb_pwdata\[31:24\], apb_pwdata\[7:0\], apb_pwdata\[15:8\]}
pub type TxfifoWrEndianR = crate::FieldReader;
///Field `TXFIFO_WR_ENDIAN` writer - apb_pwdata Write to TxFIFO Endian 0x0: txfifo_wdata\[31:0\]
///= apb_pwdata\[31:0\]
///0x1: fifo_wdata\[31:0\]
///= {apb_pwdata\[15:0\], apb_pwdata\[31:16\]} 0x2: txfifo_wdata\[31:0\]
///= {apb_pwdata\[7:0\], apb_pwdata\[15:8\], apb_pwdata\[23:16\], apb_pwdata\[31:24\]} 0x3: txfifo_wdata\[31:0\]
///= {apb_pwdata\[23:16\], apb_pwdata\[31:24\], apb_pwdata\[7:0\], apb_pwdata\[15:8\]}
pub type TxfifoWrEndianW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FPCKE` reader - FIFO Packing Enable 0: FIFO packing mode disabled 1: FIFO packing mode enabled
pub type FpckeR = crate::BitReader;
///Field `FPCKE` writer - FIFO Packing Enable 0: FIFO packing mode disabled 1: FIFO packing mode enabled
pub type FpckeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFO_AUTO_FULL_CTRL` reader - Rx FIFO Auto Full Control: After this field is set to 1 and the SPI controller is operating in master mode, the controller FSM returns to IDLE state and stops the SPI_CLK. When Rx FIFO is full, the controller FSM continues transferring data after the RxFIFO is not full. This field is used to avoid an RxFIFO overrun issue. 1: Enable Rx FIFO auto full control
pub type RxfifoAutoFullCtrlR = crate::BitReader;
///Field `RXFIFO_AUTO_FULL_CTRL` writer - Rx FIFO Auto Full Control: After this field is set to 1 and the SPI controller is operating in master mode, the controller FSM returns to IDLE state and stops the SPI_CLK. When Rx FIFO is full, the controller FSM continues transferring data after the RxFIFO is not full. This field is used to avoid an RxFIFO overrun issue. 1: Enable Rx FIFO auto full control
pub type RxfifoAutoFullCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - TXFIFO Trigger Threshold This field sets the threshold level at which TXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
    #[inline(always)]
    pub fn tft(&self) -> TftR {
        TftR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - RXFIFO Trigger Threshold This field sets the threshold level at which RXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
    #[inline(always)]
    pub fn rft(&self) -> RftR {
        RftR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bit 10 - Transmit Service Request Enable 0: TxFIFO DMA service request is disabled 1: TxFIFO DMA service request is enabled
    #[inline(always)]
    pub fn tsre(&self) -> TsreR {
        TsreR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receive Service Request Enable 0: RxFIFO DMA service request is disabled 1: RxFIFO DMA service request is enabled
    #[inline(always)]
    pub fn rsre(&self) -> RsreR {
        RsreR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - apb_prdata Read from Rx FIFO Endian 0x0 = apb_prdata\[31:0\]
    ///= rxfifo_wdata\[31:0\]
    ///0x1 = apb_prdata\[31:0\]
    ///= {rxfifo_wdata\[15:0\], rxfifo_wdata\[31:16\]} 0x2 = apb_prdata\[31:0\]= {rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\], rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\]} 0x3 = apb_prdata\[31:0\]= {rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\], rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\]}
    #[inline(always)]
    pub fn rxfifo_rd_endian(&self) -> RxfifoRdEndianR {
        RxfifoRdEndianR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - apb_pwdata Write to TxFIFO Endian 0x0: txfifo_wdata\[31:0\]
    ///= apb_pwdata\[31:0\]
    ///0x1: fifo_wdata\[31:0\]
    ///= {apb_pwdata\[15:0\], apb_pwdata\[31:16\]} 0x2: txfifo_wdata\[31:0\]
    ///= {apb_pwdata\[7:0\], apb_pwdata\[15:8\], apb_pwdata\[23:16\], apb_pwdata\[31:24\]} 0x3: txfifo_wdata\[31:0\]
    ///= {apb_pwdata\[23:16\], apb_pwdata\[31:24\], apb_pwdata\[7:0\], apb_pwdata\[15:8\]}
    #[inline(always)]
    pub fn txfifo_wr_endian(&self) -> TxfifoWrEndianR {
        TxfifoWrEndianR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - FIFO Packing Enable 0: FIFO packing mode disabled 1: FIFO packing mode enabled
    #[inline(always)]
    pub fn fpcke(&self) -> FpckeR {
        FpckeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rx FIFO Auto Full Control: After this field is set to 1 and the SPI controller is operating in master mode, the controller FSM returns to IDLE state and stops the SPI_CLK. When Rx FIFO is full, the controller FSM continues transferring data after the RxFIFO is not full. This field is used to avoid an RxFIFO overrun issue. 1: Enable Rx FIFO auto full control
    #[inline(always)]
    pub fn rxfifo_auto_full_ctrl(&self) -> RxfifoAutoFullCtrlR {
        RxfifoAutoFullCtrlR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CTRL")
            .field("rxfifo_auto_full_ctrl", &self.rxfifo_auto_full_ctrl())
            .field("fpcke", &self.fpcke())
            .field("txfifo_wr_endian", &self.txfifo_wr_endian())
            .field("rxfifo_rd_endian", &self.rxfifo_rd_endian())
            .field("rsre", &self.rsre())
            .field("tsre", &self.tsre())
            .field("rft", &self.rft())
            .field("tft", &self.tft())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - TXFIFO Trigger Threshold This field sets the threshold level at which TXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
    #[inline(always)]
    pub fn tft(&mut self) -> TftW<FIFO_CTRLrs> {
        TftW::new(self, 0)
    }
    ///Bits 5:9 - RXFIFO Trigger Threshold This field sets the threshold level at which RXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1.
    #[inline(always)]
    pub fn rft(&mut self) -> RftW<FIFO_CTRLrs> {
        RftW::new(self, 5)
    }
    ///Bit 10 - Transmit Service Request Enable 0: TxFIFO DMA service request is disabled 1: TxFIFO DMA service request is enabled
    #[inline(always)]
    pub fn tsre(&mut self) -> TsreW<FIFO_CTRLrs> {
        TsreW::new(self, 10)
    }
    ///Bit 11 - Receive Service Request Enable 0: RxFIFO DMA service request is disabled 1: RxFIFO DMA service request is enabled
    #[inline(always)]
    pub fn rsre(&mut self) -> RsreW<FIFO_CTRLrs> {
        RsreW::new(self, 11)
    }
    ///Bits 12:13 - apb_prdata Read from Rx FIFO Endian 0x0 = apb_prdata\[31:0\]
    ///= rxfifo_wdata\[31:0\]
    ///0x1 = apb_prdata\[31:0\]
    ///= {rxfifo_wdata\[15:0\], rxfifo_wdata\[31:16\]} 0x2 = apb_prdata\[31:0\]= {rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\], rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\]} 0x3 = apb_prdata\[31:0\]= {rxfifo_wdata\[23:16\], rxfifo_wdata\[31:24\], rxfifo_wdata\[7:0\], rxfifo_wdata\[15:8\]}
    #[inline(always)]
    pub fn rxfifo_rd_endian(&mut self) -> RxfifoRdEndianW<FIFO_CTRLrs> {
        RxfifoRdEndianW::new(self, 12)
    }
    ///Bits 14:15 - apb_pwdata Write to TxFIFO Endian 0x0: txfifo_wdata\[31:0\]
    ///= apb_pwdata\[31:0\]
    ///0x1: fifo_wdata\[31:0\]
    ///= {apb_pwdata\[15:0\], apb_pwdata\[31:16\]} 0x2: txfifo_wdata\[31:0\]
    ///= {apb_pwdata\[7:0\], apb_pwdata\[15:8\], apb_pwdata\[23:16\], apb_pwdata\[31:24\]} 0x3: txfifo_wdata\[31:0\]
    ///= {apb_pwdata\[23:16\], apb_pwdata\[31:24\], apb_pwdata\[7:0\], apb_pwdata\[15:8\]}
    #[inline(always)]
    pub fn txfifo_wr_endian(&mut self) -> TxfifoWrEndianW<FIFO_CTRLrs> {
        TxfifoWrEndianW::new(self, 14)
    }
    ///Bit 16 - FIFO Packing Enable 0: FIFO packing mode disabled 1: FIFO packing mode enabled
    #[inline(always)]
    pub fn fpcke(&mut self) -> FpckeW<FIFO_CTRLrs> {
        FpckeW::new(self, 16)
    }
    ///Bit 17 - Rx FIFO Auto Full Control: After this field is set to 1 and the SPI controller is operating in master mode, the controller FSM returns to IDLE state and stops the SPI_CLK. When Rx FIFO is full, the controller FSM continues transferring data after the RxFIFO is not full. This field is used to avoid an RxFIFO overrun issue. 1: Enable Rx FIFO auto full control
    #[inline(always)]
    pub fn rxfifo_auto_full_ctrl(&mut self) -> RxfifoAutoFullCtrlW<FIFO_CTRLrs> {
        RxfifoAutoFullCtrlW::new(self, 17)
    }
}
///FIFO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FIFO_CTRLrs;
impl crate::RegisterSpec for FIFO_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`fifo_ctrl::R`](R) reader structure
impl crate::Readable for FIFO_CTRLrs {}
///`write(|w| ..)` method takes [`fifo_ctrl::W`](W) writer structure
impl crate::Writable for FIFO_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_CTRL to value 0
impl crate::Resettable for FIFO_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
