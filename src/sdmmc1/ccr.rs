///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CMD_START` reader - Command start write 1 to start command TX, and when begin to TX command, the bit will return into 0.
pub type CmdStartR = crate::BitReader;
///Field `CMD_START` writer - Command start write 1 to start command TX, and when begin to TX command, the bit will return into 0.
pub type CmdStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CMD_TX_EN` reader - TX command enable 1: enable TX command 0: disable TX command
pub type CmdTxEnR = crate::BitReader;
///Field `CMD_TX_EN` writer - TX command enable 1: enable TX command 0: disable TX command
pub type CmdTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_PEND` reader - Command pending enable When prepare to send stop command, this bit should be set. Controller will calculate a proper time point to send out the command to guarantee all the data have been transferred. And this is mainly used in stream mode. Recommend using set_block_count (SD/MMC basis command) to control transferring data for block mode. If send stop command for canceling this transfer (such as CRC error in multi-block), no need to set the bit.
pub type CmdPendR = crate::BitReader;
///Field `CMD_PEND` writer - Command pending enable When prepare to send stop command, this bit should be set. Controller will calculate a proper time point to send out the command to guarantee all the data have been transferred. And this is mainly used in stream mode. Recommend using set_block_count (SD/MMC basis command) to control transferring data for block mode. If send stop command for canceling this transfer (such as CRC error in multi-block), no need to set the bit.
pub type CmdPendW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CMD_HAS_RSP` reader - 1: Response expected after command 0: No response expected after command
pub type CmdHasRspR = crate::BitReader;
///Field `CMD_HAS_RSP` writer - 1: Response expected after command 0: No response expected after command
pub type CmdHasRspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_LONG_RSP` reader - 1: Response will be 136-bit, long response 0: Response will be 48-bit, normal response
pub type CmdLongRspR = crate::BitReader;
///Field `CMD_LONG_RSP` writer - 1: Response will be 136-bit, long response 0: Response will be 48-bit, normal response
pub type CmdLongRspW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_INDEX` reader - Command index
pub type CmdIndexR = crate::FieldReader;
///Field `CMD_INDEX` writer - Command index
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Command start write 1 to start command TX, and when begin to TX command, the bit will return into 0.
    #[inline(always)]
    pub fn cmd_start(&self) -> CmdStartR {
        CmdStartR::new((self.bits & 1) != 0)
    }
    ///Bits 1:7
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 8 - TX command enable 1: enable TX command 0: disable TX command
    #[inline(always)]
    pub fn cmd_tx_en(&self) -> CmdTxEnR {
        CmdTxEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Command pending enable When prepare to send stop command, this bit should be set. Controller will calculate a proper time point to send out the command to guarantee all the data have been transferred. And this is mainly used in stream mode. Recommend using set_block_count (SD/MMC basis command) to control transferring data for block mode. If send stop command for canceling this transfer (such as CRC error in multi-block), no need to set the bit.
    #[inline(always)]
    pub fn cmd_pend(&self) -> CmdPendR {
        CmdPendR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bit 16 - 1: Response expected after command 0: No response expected after command
    #[inline(always)]
    pub fn cmd_has_rsp(&self) -> CmdHasRspR {
        CmdHasRspR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - 1: Response will be 136-bit, long response 0: Response will be 48-bit, normal response
    #[inline(always)]
    pub fn cmd_long_rsp(&self) -> CmdLongRspR {
        CmdLongRspR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:23 - Command index
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("rsvd", &self.rsvd())
            .field("cmd_index", &self.cmd_index())
            .field("cmd_long_rsp", &self.cmd_long_rsp())
            .field("cmd_has_rsp", &self.cmd_has_rsp())
            .field("rsvd2", &self.rsvd2())
            .field("cmd_pend", &self.cmd_pend())
            .field("cmd_tx_en", &self.cmd_tx_en())
            .field("rsvd3", &self.rsvd3())
            .field("cmd_start", &self.cmd_start())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command start write 1 to start command TX, and when begin to TX command, the bit will return into 0.
    #[inline(always)]
    pub fn cmd_start(&mut self) -> CmdStartW<CCRrs> {
        CmdStartW::new(self, 0)
    }
    ///Bits 1:7
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CCRrs> {
        Rsvd3W::new(self, 1)
    }
    ///Bit 8 - TX command enable 1: enable TX command 0: disable TX command
    #[inline(always)]
    pub fn cmd_tx_en(&mut self) -> CmdTxEnW<CCRrs> {
        CmdTxEnW::new(self, 8)
    }
    ///Bit 9 - Command pending enable When prepare to send stop command, this bit should be set. Controller will calculate a proper time point to send out the command to guarantee all the data have been transferred. And this is mainly used in stream mode. Recommend using set_block_count (SD/MMC basis command) to control transferring data for block mode. If send stop command for canceling this transfer (such as CRC error in multi-block), no need to set the bit.
    #[inline(always)]
    pub fn cmd_pend(&mut self) -> CmdPendW<CCRrs> {
        CmdPendW::new(self, 9)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CCRrs> {
        Rsvd2W::new(self, 10)
    }
    ///Bit 16 - 1: Response expected after command 0: No response expected after command
    #[inline(always)]
    pub fn cmd_has_rsp(&mut self) -> CmdHasRspW<CCRrs> {
        CmdHasRspW::new(self, 16)
    }
    ///Bit 17 - 1: Response will be 136-bit, long response 0: Response will be 48-bit, normal response
    #[inline(always)]
    pub fn cmd_long_rsp(&mut self) -> CmdLongRspW<CCRrs> {
        CmdLongRspW::new(self, 17)
    }
    ///Bits 18:23 - Command index
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CmdIndexW<CCRrs> {
        CmdIndexW::new(self, 18)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CCRrs> {
        RsvdW::new(self, 24)
    }
}
///command control register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
