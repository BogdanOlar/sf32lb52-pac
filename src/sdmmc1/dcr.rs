///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
///Field `DATA_START` reader - Start transfer data set 1 to let the controller begin to transfer data (in fact, go into wait write or wait read state). After begin to transfer, this bit will be back to 0.
pub type DataStartR = crate::BitReader;
///Field `DATA_START` writer - Start transfer data set 1 to let the controller begin to transfer data (in fact, go into wait write or wait read state). After begin to transfer, this bit will be back to 0.
pub type DataStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `TRAN_DATA_EN` reader - Transfer data enable 0: disable transfer data. After disable data transfer, stop command should be sent to card 1: enable data transfer
pub type TranDataEnR = crate::BitReader;
///Field `TRAN_DATA_EN` writer - Transfer data enable 0: disable transfer data. After disable data transfer, stop command should be sent to card 1: enable data transfer
pub type TranDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R_WN` reader - Write or read 0: write data into card 1: read data from card
pub type RWnR = crate::BitReader;
///Field `R_WN` writer - Write or read 0: write data into card 1: read data from card
pub type RWnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STREAM_MODE` reader - Data transfer mode 0: block 1: stream
pub type StreamModeR = crate::BitReader;
///Field `STREAM_MODE` writer - Data transfer mode 0: block 1: stream
pub type StreamModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIRE_MODE` reader - Wide data bus mode 00: 1 wire bus 01: 4 wires wide bus 1X: reserved
pub type WireModeR = crate::FieldReader;
///Field `WIRE_MODE` writer - Wide data bus mode 00: 1 wire bus 01: 4 wires wide bus 1X: reserved
pub type WireModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BLOCK_SIZE` reader - Data block size is block_size+1 (max 2048 bytes) 0: 1 byte 0x1ff: 512 bytes
pub type BlockSizeR = crate::FieldReader<u16>;
///Field `BLOCK_SIZE` writer - Data block size is block_size+1 (max 2048 bytes) 0: 1 byte 0x1ff: 512 bytes
pub type BlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - Start transfer data set 1 to let the controller begin to transfer data (in fact, go into wait write or wait read state). After begin to transfer, this bit will be back to 0.
    #[inline(always)]
    pub fn data_start(&self) -> DataStartR {
        DataStartR::new((self.bits & 1) != 0)
    }
    ///Bits 1:7
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 8 - Transfer data enable 0: disable transfer data. After disable data transfer, stop command should be sent to card 1: enable data transfer
    #[inline(always)]
    pub fn tran_data_en(&self) -> TranDataEnR {
        TranDataEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Write or read 0: write data into card 1: read data from card
    #[inline(always)]
    pub fn r_wn(&self) -> RWnR {
        RWnR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data transfer mode 0: block 1: stream
    #[inline(always)]
    pub fn stream_mode(&self) -> StreamModeR {
        StreamModeR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Wide data bus mode 00: 1 wire bus 01: 4 wires wide bus 1X: reserved
    #[inline(always)]
    pub fn wire_mode(&self) -> WireModeR {
        WireModeR::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:26 - Data block size is block_size+1 (max 2048 bytes) 0: 1 byte 0x1ff: 512 bytes
    #[inline(always)]
    pub fn block_size(&self) -> BlockSizeR {
        BlockSizeR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bits 27:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("rsvd", &self.rsvd())
            .field("block_size", &self.block_size())
            .field("rsvd2", &self.rsvd2())
            .field("wire_mode", &self.wire_mode())
            .field("stream_mode", &self.stream_mode())
            .field("r_wn", &self.r_wn())
            .field("tran_data_en", &self.tran_data_en())
            .field("rsvd3", &self.rsvd3())
            .field("data_start", &self.data_start())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start transfer data set 1 to let the controller begin to transfer data (in fact, go into wait write or wait read state). After begin to transfer, this bit will be back to 0.
    #[inline(always)]
    pub fn data_start(&mut self) -> DataStartW<DCRrs> {
        DataStartW::new(self, 0)
    }
    ///Bits 1:7
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<DCRrs> {
        Rsvd3W::new(self, 1)
    }
    ///Bit 8 - Transfer data enable 0: disable transfer data. After disable data transfer, stop command should be sent to card 1: enable data transfer
    #[inline(always)]
    pub fn tran_data_en(&mut self) -> TranDataEnW<DCRrs> {
        TranDataEnW::new(self, 8)
    }
    ///Bit 9 - Write or read 0: write data into card 1: read data from card
    #[inline(always)]
    pub fn r_wn(&mut self) -> RWnW<DCRrs> {
        RWnW::new(self, 9)
    }
    ///Bit 10 - Data transfer mode 0: block 1: stream
    #[inline(always)]
    pub fn stream_mode(&mut self) -> StreamModeW<DCRrs> {
        StreamModeW::new(self, 10)
    }
    ///Bits 11:12 - Wide data bus mode 00: 1 wire bus 01: 4 wires wide bus 1X: reserved
    #[inline(always)]
    pub fn wire_mode(&mut self) -> WireModeW<DCRrs> {
        WireModeW::new(self, 11)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DCRrs> {
        Rsvd2W::new(self, 13)
    }
    ///Bits 16:26 - Data block size is block_size+1 (max 2048 bytes) 0: 1 byte 0x1ff: 512 bytes
    #[inline(always)]
    pub fn block_size(&mut self) -> BlockSizeW<DCRrs> {
        BlockSizeW::new(self, 16)
    }
    ///Bits 27:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DCRrs> {
        RsvdW::new(self, 27)
    }
}
///data control register
///
///You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DCRrs {
    const RESET_VALUE: u32 = 0;
}
