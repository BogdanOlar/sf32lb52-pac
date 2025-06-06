///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `TB` reader - Transfer Byte: Used to send or receive a byte on the I2C bus: 0 = Cleared by I2C when the byte is sent/received. 1 = Send/receive a byte. CPU can monitor this bit to determine when the byte transfer has completed. In master or slave mode, after each byte transfer including acknowledge pulse, the I2C holds the SCL line low (inserting wait states) until TB is set.
pub type TbR = crate::BitReader;
///Field `TB` writer - Transfer Byte: Used to send or receive a byte on the I2C bus: 0 = Cleared by I2C when the byte is sent/received. 1 = Send/receive a byte. CPU can monitor this bit to determine when the byte transfer has completed. In master or slave mode, after each byte transfer including acknowledge pulse, the I2C holds the SCL line low (inserting wait states) until TB is set.
pub type TbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start: Used to initiate a Start condition to the I2C unit when in master mode. 0 = Do not send a Start pulse. 1 = Send a Start pulse.
pub type StartR = crate::BitReader;
///Field `START` writer - Start: Used to initiate a Start condition to the I2C unit when in master mode. 0 = Do not send a Start pulse. 1 = Send a Start pulse.
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - Stop: Used to initiate a Stop condition after transferring the next data byte on the I2C bus when in master mode. In master-receive mode, the NACK control bit must be set in conjunction with the STOP bit. 0 = Do not send a Stop. 1 = Send a Stop.
pub type StopR = crate::BitReader;
///Field `STOP` writer - Stop: Used to initiate a Stop condition after transferring the next data byte on the I2C bus when in master mode. In master-receive mode, the NACK control bit must be set in conjunction with the STOP bit. 0 = Do not send a Stop. 1 = Send a Stop.
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - The positive/negative acknowledge control bit, defines the type of acknowledge pulse sent by the I2C when in master receive mode: 0 = Send a positive acknowledge (ACK) pulse after receiving a data byte. 1 = Send a negative acknowledge (NACK) pulse after receiving a data byte. The I2C automatically sends an ACK pulse when responding to its slave address or when responding in slave-receive mode, regardless of the NACK control-bit setting.
pub type NackR = crate::BitReader;
///Field `NACK` writer - The positive/negative acknowledge control bit, defines the type of acknowledge pulse sent by the I2C when in master receive mode: 0 = Send a positive acknowledge (ACK) pulse after receiving a data byte. 1 = Send a negative acknowledge (NACK) pulse after receiving a data byte. The I2C automatically sends an ACK pulse when responding to its slave address or when responding in slave-receive mode, regardless of the NACK control-bit setting.
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MA` reader - Master Abort: Used by the I2C in master mode to generate a Stop without transmitting another data byte: 0 = The I2C transmits Stop on if TCR\[STOP\]
///is set. 1 = The I2C sends Stop without data transmission. When in master-transmit mode, after transmitting a data byte, the TCR\[TB\]
///bit is cleared. When no more data bytes need to be sent, setting master abort bit sends the Stop. The TCR\[TB\]
///bit must remain clear. In master-receive mode, when a NAK is sent without a Stop (TCR\[STOP\]
///bit was not set) and CPU does not send a repeated Start, setting this bit sends the Stop. Once again, the TCR\[TB\]
///bit must remain clear. Master Abort can be done immediately after the address phase (Master Transmit mode only).
pub type MaR = crate::BitReader;
///Field `MA` writer - Master Abort: Used by the I2C in master mode to generate a Stop without transmitting another data byte: 0 = The I2C transmits Stop on if TCR\[STOP\]
///is set. 1 = The I2C sends Stop without data transmission. When in master-transmit mode, after transmitting a data byte, the TCR\[TB\]
///bit is cleared. When no more data bytes need to be sent, setting master abort bit sends the Stop. The TCR\[TB\]
///bit must remain clear. In master-receive mode, when a NAK is sent without a Stop (TCR\[STOP\]
///bit was not set) and CPU does not send a repeated Start, setting this bit sends the Stop. Once again, the TCR\[TB\]
///bit must remain clear. Master Abort can be done immediately after the address phase (Master Transmit mode only).
pub type MaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXREQ` reader - Request DMA TX. Will be cleared by HW automatically
pub type TxreqR = crate::BitReader;
///Field `TXREQ` writer - Request DMA TX. Will be cleared by HW automatically
pub type TxreqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXREQ` reader - Request DMA RX. Will be cleared by HW automatically
pub type RxreqR = crate::BitReader;
///Field `RXREQ` writer - Request DMA RX. Will be cleared by HW automatically
pub type RxreqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORTDMA` reader - Abort DMA operation. Will be cleared by HW automatically
pub type AbortdmaR = crate::BitReader;
///Field `ABORTDMA` writer - Abort DMA operation. Will be cleared by HW automatically
pub type AbortdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer Byte: Used to send or receive a byte on the I2C bus: 0 = Cleared by I2C when the byte is sent/received. 1 = Send/receive a byte. CPU can monitor this bit to determine when the byte transfer has completed. In master or slave mode, after each byte transfer including acknowledge pulse, the I2C holds the SCL line low (inserting wait states) until TB is set.
    #[inline(always)]
    pub fn tb(&self) -> TbR {
        TbR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start: Used to initiate a Start condition to the I2C unit when in master mode. 0 = Do not send a Start pulse. 1 = Send a Start pulse.
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Stop: Used to initiate a Stop condition after transferring the next data byte on the I2C bus when in master mode. In master-receive mode, the NACK control bit must be set in conjunction with the STOP bit. 0 = Do not send a Stop. 1 = Send a Stop.
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The positive/negative acknowledge control bit, defines the type of acknowledge pulse sent by the I2C when in master receive mode: 0 = Send a positive acknowledge (ACK) pulse after receiving a data byte. 1 = Send a negative acknowledge (NACK) pulse after receiving a data byte. The I2C automatically sends an ACK pulse when responding to its slave address or when responding in slave-receive mode, regardless of the NACK control-bit setting.
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Abort: Used by the I2C in master mode to generate a Stop without transmitting another data byte: 0 = The I2C transmits Stop on if TCR\[STOP\]
    ///is set. 1 = The I2C sends Stop without data transmission. When in master-transmit mode, after transmitting a data byte, the TCR\[TB\]
    ///bit is cleared. When no more data bytes need to be sent, setting master abort bit sends the Stop. The TCR\[TB\]
    ///bit must remain clear. In master-receive mode, when a NAK is sent without a Stop (TCR\[STOP\]
    ///bit was not set) and CPU does not send a repeated Start, setting this bit sends the Stop. Once again, the TCR\[TB\]
    ///bit must remain clear. Master Abort can be done immediately after the address phase (Master Transmit mode only).
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Request DMA TX. Will be cleared by HW automatically
    #[inline(always)]
    pub fn txreq(&self) -> TxreqR {
        TxreqR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Request DMA RX. Will be cleared by HW automatically
    #[inline(always)]
    pub fn rxreq(&self) -> RxreqR {
        RxreqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Abort DMA operation. Will be cleared by HW automatically
    #[inline(always)]
    pub fn abortdma(&self) -> AbortdmaR {
        AbortdmaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("abortdma", &self.abortdma())
            .field("rxreq", &self.rxreq())
            .field("txreq", &self.txreq())
            .field("ma", &self.ma())
            .field("nack", &self.nack())
            .field("stop", &self.stop())
            .field("start", &self.start())
            .field("tb", &self.tb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer Byte: Used to send or receive a byte on the I2C bus: 0 = Cleared by I2C when the byte is sent/received. 1 = Send/receive a byte. CPU can monitor this bit to determine when the byte transfer has completed. In master or slave mode, after each byte transfer including acknowledge pulse, the I2C holds the SCL line low (inserting wait states) until TB is set.
    #[inline(always)]
    pub fn tb(&mut self) -> TbW<TCRrs> {
        TbW::new(self, 0)
    }
    ///Bit 1 - Start: Used to initiate a Start condition to the I2C unit when in master mode. 0 = Do not send a Start pulse. 1 = Send a Start pulse.
    #[inline(always)]
    pub fn start(&mut self) -> StartW<TCRrs> {
        StartW::new(self, 1)
    }
    ///Bit 2 - Stop: Used to initiate a Stop condition after transferring the next data byte on the I2C bus when in master mode. In master-receive mode, the NACK control bit must be set in conjunction with the STOP bit. 0 = Do not send a Stop. 1 = Send a Stop.
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<TCRrs> {
        StopW::new(self, 2)
    }
    ///Bit 3 - The positive/negative acknowledge control bit, defines the type of acknowledge pulse sent by the I2C when in master receive mode: 0 = Send a positive acknowledge (ACK) pulse after receiving a data byte. 1 = Send a negative acknowledge (NACK) pulse after receiving a data byte. The I2C automatically sends an ACK pulse when responding to its slave address or when responding in slave-receive mode, regardless of the NACK control-bit setting.
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<TCRrs> {
        NackW::new(self, 3)
    }
    ///Bit 4 - Master Abort: Used by the I2C in master mode to generate a Stop without transmitting another data byte: 0 = The I2C transmits Stop on if TCR\[STOP\]
    ///is set. 1 = The I2C sends Stop without data transmission. When in master-transmit mode, after transmitting a data byte, the TCR\[TB\]
    ///bit is cleared. When no more data bytes need to be sent, setting master abort bit sends the Stop. The TCR\[TB\]
    ///bit must remain clear. In master-receive mode, when a NAK is sent without a Stop (TCR\[STOP\]
    ///bit was not set) and CPU does not send a repeated Start, setting this bit sends the Stop. Once again, the TCR\[TB\]
    ///bit must remain clear. Master Abort can be done immediately after the address phase (Master Transmit mode only).
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<TCRrs> {
        MaW::new(self, 4)
    }
    ///Bit 5 - Request DMA TX. Will be cleared by HW automatically
    #[inline(always)]
    pub fn txreq(&mut self) -> TxreqW<TCRrs> {
        TxreqW::new(self, 5)
    }
    ///Bit 6 - Request DMA RX. Will be cleared by HW automatically
    #[inline(always)]
    pub fn rxreq(&mut self) -> RxreqW<TCRrs> {
        RxreqW::new(self, 6)
    }
    ///Bit 7 - Abort DMA operation. Will be cleared by HW automatically
    #[inline(always)]
    pub fn abortdma(&mut self) -> AbortdmaW<TCRrs> {
        AbortdmaW::new(self, 7)
    }
}
///Transfer Control register
///
///You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {
    const RESET_VALUE: u32 = 0;
}
