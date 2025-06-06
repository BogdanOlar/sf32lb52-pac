///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `CMD_BUSY` reader - Command busy 1: busy, and when busy, start TX command bit is no usage and should not modify the relative register 0: command idle
pub type CmdBusyR = crate::BitReader;
///Field `CMD_BUSY` writer - Command busy 1: busy, and when busy, start TX command bit is no usage and should not modify the relative register 0: command idle
pub type CmdBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_DONE` reader - Command done Read 1: transfer command done, and start a new transfer will take the bit into 0 Read 0: command transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdDoneR = crate::BitReader;
///Field `CMD_DONE` writer - Command done Read 1: transfer command done, and start a new transfer will take the bit into 0 Read 0: command transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_RSP_CRC` reader - Command response CRC error status Read 1: response CRC error Read 0: response CRC right Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdRspCrcR = crate::BitReader;
///Field `CMD_RSP_CRC` writer - Command response CRC error status Read 1: response CRC error Read 0: response CRC right Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdRspCrcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_TIMEOUT` reader - Command timeout (response timeout) Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdTimeoutR = crate::BitReader;
///Field `CMD_TIMEOUT` writer - Command timeout (response timeout) Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_BUSY` reader - Transfer Data busy 1: busy, and when busy, start transfer data bit is no usage and you should not modify the relative register. If want to do this, first disable transfer data enable bit, then the busy bit will be back to 0, and this transfer will also be cancelled. 0: data idle
pub type DataBusyR = crate::BitReader;
///Field `DATA_BUSY` writer - Transfer Data busy 1: busy, and when busy, start transfer data bit is no usage and you should not modify the relative register. If want to do this, first disable transfer data enable bit, then the busy bit will be back to 0, and this transfer will also be cancelled. 0: data idle
pub type DataBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_DONE` reader - Data transfer done Read 1: transfer data done, and start a new transfer will take the bit into 0 Read 0: data transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
pub type DataDoneR = crate::BitReader;
///Field `DATA_DONE` writer - Data transfer done Read 1: transfer data done, and start a new transfer will take the bit into 0 Read 0: data transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
pub type DataDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_CRC` reader - Data CRC error Read 1: data CRC error Read 0: data CRC right Write 1: clear the bit Write 0: no any influence to the bit
pub type DataCrcR = crate::BitReader;
///Field `DATA_CRC` writer - Data CRC error Read 1: data CRC error Read 0: data CRC right Write 1: clear the bit Write 0: no any influence to the bit
pub type DataCrcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_TIMEOUT` reader - Data timeout Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
pub type DataTimeoutR = crate::BitReader;
///Field `DATA_TIMEOUT` writer - Data timeout Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
pub type DataTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STARTBIT_ERROR` reader - Wide bus start bits error Didn't detect all start bits in data bus Read 1: start bits error Read 0: no start bits error Write 1: clear the bit Write 0: no any influence to the bit
pub type StartbitErrorR = crate::BitReader;
///Field `STARTBIT_ERROR` writer - Wide bus start bits error Didn't detect all start bits in data bus Read 1: start bits error Read 0: no start bits error Write 1: clear the bit Write 0: no any influence to the bit
pub type StartbitErrorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_UNDERRUN` reader - FIFO underrun Read 1: FIFO underrun error Read 0: no FIFO underrun error Write 1: clear the bit Write 0: no any influence to the bit
pub type FifoUnderrunR = crate::BitReader;
///Field `FIFO_UNDERRUN` writer - FIFO underrun Read 1: FIFO underrun error Read 0: no FIFO underrun error Write 1: clear the bit Write 0: no any influence to the bit
pub type FifoUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_OVERRUN` reader - FIFO overrun Read 1: FIFO overrun error Read 0: no FIFO overrun error Write 1: clear the bit Write 0: no any influence to the bit
pub type FifoOverrunR = crate::BitReader;
///Field `FIFO_OVERRUN` writer - FIFO overrun Read 1: FIFO overrun error Read 0: no FIFO overrun error Write 1: clear the bit Write 0: no any influence to the bit
pub type FifoOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_SENT` reader - Command sent (perhaps no response back yet) Read 1: command sent. When command start bit is set, the bit will also be back to 0 Read 0: command transferring or others Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdSentR = crate::BitReader;
///Field `CMD_SENT` writer - Command sent (perhaps no response back yet) Read 1: command sent. When command start bit is set, the bit will also be back to 0 Read 0: command transferring or others Write 1: clear the bit Write 0: no any influence to the bit
pub type CmdSentW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARD_INSERT` reader - Detect card inserted Read 1: detect card inserted. When detect card removed bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
pub type CardInsertR = crate::BitReader;
///Field `CARD_INSERT` writer - Detect card inserted Read 1: detect card inserted. When detect card removed bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
pub type CardInsertW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARD_REMOVE` reader - Detect card removed Read 1: detect card removed. When detect card inserted bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
pub type CardRemoveR = crate::BitReader;
///Field `CARD_REMOVE` writer - Detect card removed Read 1: detect card removed. When detect card inserted bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
pub type CardRemoveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARD_EXIST` reader - Card exist status Read 1: card exist Read 0: no card exist This bit will be valid after enable detect card.
pub type CardExistR = crate::BitReader;
///Field `CARD_EXIST` writer - Card exist status Read 1: card exist Read 0: no card exist This bit will be valid after enable detect card.
pub type CardExistW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO` reader - Detect SDIO Card Interrupt Read 1: detect sdio card generating interrupt Read 0: no interrupt Write 1: clear the bit Write 0: no any influence to the bit
pub type SdioR = crate::BitReader;
///Field `SDIO` writer - Detect SDIO Card Interrupt Read 1: detect sdio card generating interrupt Read 0: no interrupt Write 1: clear the bit Write 0: no any influence to the bit
pub type SdioW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_ERR` reader - Detect cache error Read 1: cache error occur Read 0: no cache error Write 1: clear the bit Write 0: no any influence to the bit
pub type CacheErrR = crate::BitReader;
///Field `CACHE_ERR` writer - Detect cache error Read 1: cache error occur Read 0: no cache error Write 1: clear the bit Write 0: no any influence to the bit
pub type CacheErrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0 - Command busy 1: busy, and when busy, start TX command bit is no usage and should not modify the relative register 0: command idle
    #[inline(always)]
    pub fn cmd_busy(&self) -> CmdBusyR {
        CmdBusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command done Read 1: transfer command done, and start a new transfer will take the bit into 0 Read 0: command transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_done(&self) -> CmdDoneR {
        CmdDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command response CRC error status Read 1: response CRC error Read 0: response CRC right Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_rsp_crc(&self) -> CmdRspCrcR {
        CmdRspCrcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Command timeout (response timeout) Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_timeout(&self) -> CmdTimeoutR {
        CmdTimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transfer Data busy 1: busy, and when busy, start transfer data bit is no usage and you should not modify the relative register. If want to do this, first disable transfer data enable bit, then the busy bit will be back to 0, and this transfer will also be cancelled. 0: data idle
    #[inline(always)]
    pub fn data_busy(&self) -> DataBusyR {
        DataBusyR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Data transfer done Read 1: transfer data done, and start a new transfer will take the bit into 0 Read 0: data transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn data_done(&self) -> DataDoneR {
        DataDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Data CRC error Read 1: data CRC error Read 0: data CRC right Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Data timeout Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn data_timeout(&self) -> DataTimeoutR {
        DataTimeoutR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Wide bus start bits error Didn't detect all start bits in data bus Read 1: start bits error Read 0: no start bits error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn startbit_error(&self) -> StartbitErrorR {
        StartbitErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FIFO underrun Read 1: FIFO underrun error Read 0: no FIFO underrun error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn fifo_underrun(&self) -> FifoUnderrunR {
        FifoUnderrunR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FIFO overrun Read 1: FIFO overrun error Read 0: no FIFO overrun error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn fifo_overrun(&self) -> FifoOverrunR {
        FifoOverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Command sent (perhaps no response back yet) Read 1: command sent. When command start bit is set, the bit will also be back to 0 Read 0: command transferring or others Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_sent(&self) -> CmdSentR {
        CmdSentR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Detect card inserted Read 1: detect card inserted. When detect card removed bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn card_insert(&self) -> CardInsertR {
        CardInsertR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Detect card removed Read 1: detect card removed. When detect card inserted bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn card_remove(&self) -> CardRemoveR {
        CardRemoveR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Card exist status Read 1: card exist Read 0: no card exist This bit will be valid after enable detect card.
    #[inline(always)]
    pub fn card_exist(&self) -> CardExistR {
        CardExistR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Detect SDIO Card Interrupt Read 1: detect sdio card generating interrupt Read 0: no interrupt Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn sdio(&self) -> SdioR {
        SdioR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Detect cache error Read 1: cache error occur Read 0: no cache error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cache_err(&self) -> CacheErrR {
        CacheErrR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rsvd", &self.rsvd())
            .field("cache_err", &self.cache_err())
            .field("sdio", &self.sdio())
            .field("card_exist", &self.card_exist())
            .field("card_remove", &self.card_remove())
            .field("card_insert", &self.card_insert())
            .field("cmd_sent", &self.cmd_sent())
            .field("rsvd2", &self.rsvd2())
            .field("fifo_overrun", &self.fifo_overrun())
            .field("fifo_underrun", &self.fifo_underrun())
            .field("startbit_error", &self.startbit_error())
            .field("data_timeout", &self.data_timeout())
            .field("data_crc", &self.data_crc())
            .field("data_done", &self.data_done())
            .field("data_busy", &self.data_busy())
            .field("cmd_timeout", &self.cmd_timeout())
            .field("cmd_rsp_crc", &self.cmd_rsp_crc())
            .field("cmd_done", &self.cmd_done())
            .field("cmd_busy", &self.cmd_busy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command busy 1: busy, and when busy, start TX command bit is no usage and should not modify the relative register 0: command idle
    #[inline(always)]
    pub fn cmd_busy(&mut self) -> CmdBusyW<SRrs> {
        CmdBusyW::new(self, 0)
    }
    ///Bit 1 - Command done Read 1: transfer command done, and start a new transfer will take the bit into 0 Read 0: command transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_done(&mut self) -> CmdDoneW<SRrs> {
        CmdDoneW::new(self, 1)
    }
    ///Bit 2 - Command response CRC error status Read 1: response CRC error Read 0: response CRC right Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_rsp_crc(&mut self) -> CmdRspCrcW<SRrs> {
        CmdRspCrcW::new(self, 2)
    }
    ///Bit 3 - Command timeout (response timeout) Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_timeout(&mut self) -> CmdTimeoutW<SRrs> {
        CmdTimeoutW::new(self, 3)
    }
    ///Bit 4 - Transfer Data busy 1: busy, and when busy, start transfer data bit is no usage and you should not modify the relative register. If want to do this, first disable transfer data enable bit, then the busy bit will be back to 0, and this transfer will also be cancelled. 0: data idle
    #[inline(always)]
    pub fn data_busy(&mut self) -> DataBusyW<SRrs> {
        DataBusyW::new(self, 4)
    }
    ///Bit 5 - Data transfer done Read 1: transfer data done, and start a new transfer will take the bit into 0 Read 0: data transferring or idle Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn data_done(&mut self) -> DataDoneW<SRrs> {
        DataDoneW::new(self, 5)
    }
    ///Bit 6 - Data CRC error Read 1: data CRC error Read 0: data CRC right Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn data_crc(&mut self) -> DataCrcW<SRrs> {
        DataCrcW::new(self, 6)
    }
    ///Bit 7 - Data timeout Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn data_timeout(&mut self) -> DataTimeoutW<SRrs> {
        DataTimeoutW::new(self, 7)
    }
    ///Bit 8 - Wide bus start bits error Didn't detect all start bits in data bus Read 1: start bits error Read 0: no start bits error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn startbit_error(&mut self) -> StartbitErrorW<SRrs> {
        StartbitErrorW::new(self, 8)
    }
    ///Bit 9 - FIFO underrun Read 1: FIFO underrun error Read 0: no FIFO underrun error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn fifo_underrun(&mut self) -> FifoUnderrunW<SRrs> {
        FifoUnderrunW::new(self, 9)
    }
    ///Bit 10 - FIFO overrun Read 1: FIFO overrun error Read 0: no FIFO overrun error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn fifo_overrun(&mut self) -> FifoOverrunW<SRrs> {
        FifoOverrunW::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<SRrs> {
        Rsvd2W::new(self, 11)
    }
    ///Bit 12 - Command sent (perhaps no response back yet) Read 1: command sent. When command start bit is set, the bit will also be back to 0 Read 0: command transferring or others Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cmd_sent(&mut self) -> CmdSentW<SRrs> {
        CmdSentW::new(self, 12)
    }
    ///Bit 13 - Detect card inserted Read 1: detect card inserted. When detect card removed bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn card_insert(&mut self) -> CardInsertW<SRrs> {
        CardInsertW::new(self, 13)
    }
    ///Bit 14 - Detect card removed Read 1: detect card removed. When detect card inserted bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn card_remove(&mut self) -> CardRemoveW<SRrs> {
        CardRemoveW::new(self, 14)
    }
    ///Bit 15 - Card exist status Read 1: card exist Read 0: no card exist This bit will be valid after enable detect card.
    #[inline(always)]
    pub fn card_exist(&mut self) -> CardExistW<SRrs> {
        CardExistW::new(self, 15)
    }
    ///Bit 16 - Detect SDIO Card Interrupt Read 1: detect sdio card generating interrupt Read 0: no interrupt Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn sdio(&mut self) -> SdioW<SRrs> {
        SdioW::new(self, 16)
    }
    ///Bit 17 - Detect cache error Read 1: cache error occur Read 0: no cache error Write 1: clear the bit Write 0: no any influence to the bit
    #[inline(always)]
    pub fn cache_err(&mut self) -> CacheErrW<SRrs> {
        CacheErrW::new(self, 17)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SRrs> {
        RsvdW::new(self, 18)
    }
}
///command and data status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
