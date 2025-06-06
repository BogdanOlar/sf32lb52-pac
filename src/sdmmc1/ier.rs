///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::BitReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_DONE_MASK` reader - Command done bit mask for interrupt
pub type CmdDoneMaskR = crate::BitReader;
///Field `CMD_DONE_MASK` writer - Command done bit mask for interrupt
pub type CmdDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_RSP_CRC_MASK` reader - Command CRC error bit mask for interrupt
pub type CmdRspCrcMaskR = crate::BitReader;
///Field `CMD_RSP_CRC_MASK` writer - Command CRC error bit mask for interrupt
pub type CmdRspCrcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_TIMEOUT_MASK` reader - Command timeout bit mask for interrupt
pub type CmdTimeoutMaskR = crate::BitReader;
///Field `CMD_TIMEOUT_MASK` writer - Command timeout bit mask for interrupt
pub type CmdTimeoutMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_DONE_MASK` reader - Data transfer done bit mask for interrupt
pub type DataDoneMaskR = crate::BitReader;
///Field `DATA_DONE_MASK` writer - Data transfer done bit mask for interrupt
pub type DataDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_CRC_MASK` reader - Data CRC error bit mask for interrupt
pub type DataCrcMaskR = crate::BitReader;
///Field `DATA_CRC_MASK` writer - Data CRC error bit mask for interrupt
pub type DataCrcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATA_TIMEOUT_MASK` reader - Data timeout bit mask for interrupt
pub type DataTimeoutMaskR = crate::BitReader;
///Field `DATA_TIMEOUT_MASK` writer - Data timeout bit mask for interrupt
pub type DataTimeoutMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STARTBIT_ERROR_MASK` reader - Wide bus start bits error bit mask for interrupt
pub type StartbitErrorMaskR = crate::BitReader;
///Field `STARTBIT_ERROR_MASK` writer - Wide bus start bits error bit mask for interrupt
pub type StartbitErrorMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_UNDERRUN_MASK` reader - FIFO underrun bit mask for interrupt
pub type FifoUnderrunMaskR = crate::BitReader;
///Field `FIFO_UNDERRUN_MASK` writer - FIFO underrun bit mask for interrupt
pub type FifoUnderrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_OVERRUN_MASK` reader - FIFO overrun bit mask for interrupt
pub type FifoOverrunMaskR = crate::BitReader;
///Field `FIFO_OVERRUN_MASK` writer - FIFO overrun bit mask for interrupt
pub type FifoOverrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_SENT_MASK` reader - Command sent mask for interrupt
pub type CmdSentMaskR = crate::BitReader;
///Field `CMD_SENT_MASK` writer - Command sent mask for interrupt
pub type CmdSentMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARD_INSERT_MASK` reader - Detect card insert mask for interrupt
pub type CardInsertMaskR = crate::BitReader;
///Field `CARD_INSERT_MASK` writer - Detect card insert mask for interrupt
pub type CardInsertMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARD_REMOVE_MASK` reader - Detect card remove mask for interrupt
pub type CardRemoveMaskR = crate::BitReader;
///Field `CARD_REMOVE_MASK` writer - Detect card remove mask for interrupt
pub type CardRemoveMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_MASK` reader - Detect SDIO interrupt(data\[1\]) mask for interrupt
pub type SdioMaskR = crate::BitReader;
///Field `SDIO_MASK` writer - Detect SDIO interrupt(data\[1\]) mask for interrupt
pub type SdioMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_ERR_MASK` reader - cache error mask for interrupt
pub type CacheErrMaskR = crate::BitReader;
///Field `CACHE_ERR_MASK` writer - cache error mask for interrupt
pub type CacheErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command done bit mask for interrupt
    #[inline(always)]
    pub fn cmd_done_mask(&self) -> CmdDoneMaskR {
        CmdDoneMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command CRC error bit mask for interrupt
    #[inline(always)]
    pub fn cmd_rsp_crc_mask(&self) -> CmdRspCrcMaskR {
        CmdRspCrcMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Command timeout bit mask for interrupt
    #[inline(always)]
    pub fn cmd_timeout_mask(&self) -> CmdTimeoutMaskR {
        CmdTimeoutMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Data transfer done bit mask for interrupt
    #[inline(always)]
    pub fn data_done_mask(&self) -> DataDoneMaskR {
        DataDoneMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Data CRC error bit mask for interrupt
    #[inline(always)]
    pub fn data_crc_mask(&self) -> DataCrcMaskR {
        DataCrcMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Data timeout bit mask for interrupt
    #[inline(always)]
    pub fn data_timeout_mask(&self) -> DataTimeoutMaskR {
        DataTimeoutMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Wide bus start bits error bit mask for interrupt
    #[inline(always)]
    pub fn startbit_error_mask(&self) -> StartbitErrorMaskR {
        StartbitErrorMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FIFO underrun bit mask for interrupt
    #[inline(always)]
    pub fn fifo_underrun_mask(&self) -> FifoUnderrunMaskR {
        FifoUnderrunMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FIFO overrun bit mask for interrupt
    #[inline(always)]
    pub fn fifo_overrun_mask(&self) -> FifoOverrunMaskR {
        FifoOverrunMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Command sent mask for interrupt
    #[inline(always)]
    pub fn cmd_sent_mask(&self) -> CmdSentMaskR {
        CmdSentMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Detect card insert mask for interrupt
    #[inline(always)]
    pub fn card_insert_mask(&self) -> CardInsertMaskR {
        CardInsertMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Detect card remove mask for interrupt
    #[inline(always)]
    pub fn card_remove_mask(&self) -> CardRemoveMaskR {
        CardRemoveMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Detect SDIO interrupt(data\[1\]) mask for interrupt
    #[inline(always)]
    pub fn sdio_mask(&self) -> SdioMaskR {
        SdioMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - cache error mask for interrupt
    #[inline(always)]
    pub fn cache_err_mask(&self) -> CacheErrMaskR {
        CacheErrMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rsvd", &self.rsvd())
            .field("cache_err_mask", &self.cache_err_mask())
            .field("sdio_mask", &self.sdio_mask())
            .field("rsvd2", &self.rsvd2())
            .field("card_remove_mask", &self.card_remove_mask())
            .field("card_insert_mask", &self.card_insert_mask())
            .field("cmd_sent_mask", &self.cmd_sent_mask())
            .field("rsvd3", &self.rsvd3())
            .field("fifo_overrun_mask", &self.fifo_overrun_mask())
            .field("fifo_underrun_mask", &self.fifo_underrun_mask())
            .field("startbit_error_mask", &self.startbit_error_mask())
            .field("data_timeout_mask", &self.data_timeout_mask())
            .field("data_crc_mask", &self.data_crc_mask())
            .field("data_done_mask", &self.data_done_mask())
            .field("rsvd4", &self.rsvd4())
            .field("cmd_timeout_mask", &self.cmd_timeout_mask())
            .field("cmd_rsp_crc_mask", &self.cmd_rsp_crc_mask())
            .field("cmd_done_mask", &self.cmd_done_mask())
            .field("rsvd5", &self.rsvd5())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<IERrs> {
        Rsvd5W::new(self, 0)
    }
    ///Bit 1 - Command done bit mask for interrupt
    #[inline(always)]
    pub fn cmd_done_mask(&mut self) -> CmdDoneMaskW<IERrs> {
        CmdDoneMaskW::new(self, 1)
    }
    ///Bit 2 - Command CRC error bit mask for interrupt
    #[inline(always)]
    pub fn cmd_rsp_crc_mask(&mut self) -> CmdRspCrcMaskW<IERrs> {
        CmdRspCrcMaskW::new(self, 2)
    }
    ///Bit 3 - Command timeout bit mask for interrupt
    #[inline(always)]
    pub fn cmd_timeout_mask(&mut self) -> CmdTimeoutMaskW<IERrs> {
        CmdTimeoutMaskW::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<IERrs> {
        Rsvd4W::new(self, 4)
    }
    ///Bit 5 - Data transfer done bit mask for interrupt
    #[inline(always)]
    pub fn data_done_mask(&mut self) -> DataDoneMaskW<IERrs> {
        DataDoneMaskW::new(self, 5)
    }
    ///Bit 6 - Data CRC error bit mask for interrupt
    #[inline(always)]
    pub fn data_crc_mask(&mut self) -> DataCrcMaskW<IERrs> {
        DataCrcMaskW::new(self, 6)
    }
    ///Bit 7 - Data timeout bit mask for interrupt
    #[inline(always)]
    pub fn data_timeout_mask(&mut self) -> DataTimeoutMaskW<IERrs> {
        DataTimeoutMaskW::new(self, 7)
    }
    ///Bit 8 - Wide bus start bits error bit mask for interrupt
    #[inline(always)]
    pub fn startbit_error_mask(&mut self) -> StartbitErrorMaskW<IERrs> {
        StartbitErrorMaskW::new(self, 8)
    }
    ///Bit 9 - FIFO underrun bit mask for interrupt
    #[inline(always)]
    pub fn fifo_underrun_mask(&mut self) -> FifoUnderrunMaskW<IERrs> {
        FifoUnderrunMaskW::new(self, 9)
    }
    ///Bit 10 - FIFO overrun bit mask for interrupt
    #[inline(always)]
    pub fn fifo_overrun_mask(&mut self) -> FifoOverrunMaskW<IERrs> {
        FifoOverrunMaskW::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<IERrs> {
        Rsvd3W::new(self, 11)
    }
    ///Bit 12 - Command sent mask for interrupt
    #[inline(always)]
    pub fn cmd_sent_mask(&mut self) -> CmdSentMaskW<IERrs> {
        CmdSentMaskW::new(self, 12)
    }
    ///Bit 13 - Detect card insert mask for interrupt
    #[inline(always)]
    pub fn card_insert_mask(&mut self) -> CardInsertMaskW<IERrs> {
        CardInsertMaskW::new(self, 13)
    }
    ///Bit 14 - Detect card remove mask for interrupt
    #[inline(always)]
    pub fn card_remove_mask(&mut self) -> CardRemoveMaskW<IERrs> {
        CardRemoveMaskW::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<IERrs> {
        Rsvd2W::new(self, 15)
    }
    ///Bit 16 - Detect SDIO interrupt(data\[1\]) mask for interrupt
    #[inline(always)]
    pub fn sdio_mask(&mut self) -> SdioMaskW<IERrs> {
        SdioMaskW::new(self, 16)
    }
    ///Bit 17 - cache error mask for interrupt
    #[inline(always)]
    pub fn cache_err_mask(&mut self) -> CacheErrMaskW<IERrs> {
        CacheErrMaskW::new(self, 17)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IERrs> {
        RsvdW::new(self, 18)
    }
}
///command and data interrupt mask register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
