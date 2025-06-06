///Register `TOP_CTRL` reader
pub type R = crate::R<TOP_CTRLrs>;
///Register `TOP_CTRL` writer
pub type W = crate::W<TOP_CTRLrs>;
///Field `SSE` reader - SPI controller Enable 0: SPI controller is disabled 1: SPI controller is enabled
pub type SseR = crate::BitReader;
///Field `SSE` writer - SPI controller Enable 0: SPI controller is disabled 1: SPI controller is enabled
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRF` reader - Frame Format 0x0: Motorola* Serial Peripheral Interface (SPI) 0x1: Texas Instruments* Synchronous Serial Protocol (SSP) 0x2: National Semiconductor Microwire* 0x3: RSVD
pub type FrfR = crate::FieldReader;
///Field `FRF` writer - Frame Format 0x0: Motorola* Serial Peripheral Interface (SPI) 0x1: Texas Instruments* Synchronous Serial Protocol (SSP) 0x2: National Semiconductor Microwire* 0x3: RSVD
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SCLKDIR` reader - SPI_CLK Direction 0: Master mode, SPI controller drives SPI_CLK 1: Slave mode, SPI controller receives SPI_CLK
pub type SclkdirR = crate::BitReader;
///Field `SCLKDIR` writer - SPI_CLK Direction 0: Master mode, SPI controller drives SPI_CLK 1: Slave mode, SPI controller receives SPI_CLK
pub type SclkdirW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFRMDIR` reader - SPI_CS Direction 0: Master mode, SPI controller drives SPI_CS 1: Slave mode, SPI controller receives SPI_CS
pub type SfrmdirR = crate::BitReader;
///Field `SFRMDIR` writer - SPI_CS Direction 0: Master mode, SPI controller drives SPI_CS 1: Slave mode, SPI controller receives SPI_CS
pub type SfrmdirW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSS` reader - SPI controller Work data size, register bits value 0~31 indicated data size 1~32 bits, usually use data size 8bits, 16bits, 24bits, 32bits
pub type DssR = crate::FieldReader;
///Field `DSS` writer - SPI controller Work data size, register bits value 0~31 indicated data size 1~32 bits, usually use data size 8bits, 16bits, 24bits, 32bits
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SPO` reader - Motorola SPI SPI_CLK Polarity Setting 0: The inactive or idle state of SPI_CLK is low 1: The inactive or idle state of SPI_CLK is high
pub type SpoR = crate::BitReader;
///Field `SPO` writer - Motorola SPI SPI_CLK Polarity Setting 0: The inactive or idle state of SPI_CLK is low 1: The inactive or idle state of SPI_CLK is high
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPH` reader - Motorola SPI SPI_CLK phase setting 0: SPI_CLK is inactive until one cycle after the start of a frame and active until 1/2 cycle before the end of a frame 1: SPI_CLK is inactive until 1/2 cycle after the start of a frame and active until one cycle before the end of a frame
pub type SphR = crate::BitReader;
///Field `SPH` writer - Motorola SPI SPI_CLK phase setting 0: SPI_CLK is inactive until one cycle after the start of a frame and active until 1/2 cycle before the end of a frame 1: SPI_CLK is inactive until 1/2 cycle after the start of a frame and active until one cycle before the end of a frame
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRAIL` reader - Trailing Byte 0: Trailing bytes are handled by CPU 1: Trailing bytes are handled by DMA bursts
pub type TrailR = crate::BitReader;
///Field `TRAIL` writer - Trailing Byte 0: Trailing bytes are handled by CPU 1: Trailing bytes are handled by DMA bursts
pub type TrailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD_FRAME_LOW` reader - Hold Frame Low Control 0:After this field is set to 1 and the SPI controller is operating in master mode,the output frame signal SPI_CS will be determined by control FSM. 1:After this field is set to 1 and the SPI controller is operating in master mode, the output frame signal SPI_CS will hold low.
pub type HoldFrameLowR = crate::BitReader;
///Field `HOLD_FRAME_LOW` writer - Hold Frame Low Control 0:After this field is set to 1 and the SPI controller is operating in master mode,the output frame signal SPI_CS will be determined by control FSM. 1:After this field is set to 1 and the SPI controller is operating in master mode, the output frame signal SPI_CS will hold low.
pub type HoldFrameLowW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFS` reader - Invert Frame Signal 0: SPI_CS polarity is as defined in protocol 1: SPI_CS will be inverted from normal-SPI_CS
pub type IfsR = crate::BitReader;
///Field `IFS` writer - Invert Frame Signal 0: SPI_CS polarity is as defined in protocol 1: SPI_CS will be inverted from normal-SPI_CS
pub type IfsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTE` reader - SPI_DO Three-State Enable 0: SPI_DO output signal is not three-stated 1: SPI_DO is three-stated when not transmitting data
pub type TteR = crate::BitReader;
///Field `TTE` writer - SPI_DO Three-State Enable 0: SPI_DO output signal is not three-stated 1: SPI_DO is three-stated when not transmitting data
pub type TteW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTELP` reader - SPI_DO Three-state Enable On Last Phase (can be set only when TI-SSP) 0: SPI_DO is three-stated 1/2 clock cycle after the beginning of the LSB 1: SPI_DO output signal is three-stated on the clock edge that ends the LSB
pub type TtelpR = crate::BitReader;
///Field `TTELP` writer - SPI_DO Three-state Enable On Last Phase (can be set only when TI-SSP) 0: SPI_DO is three-stated 1/2 clock cycle after the beginning of the LSB 1: SPI_DO output signal is three-stated on the clock edge that ends the LSB
pub type TtelpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI controller Enable 0: SPI controller is disabled 1: SPI controller is enabled
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Frame Format 0x0: Motorola* Serial Peripheral Interface (SPI) 0x1: Texas Instruments* Synchronous Serial Protocol (SSP) 0x2: National Semiconductor Microwire* 0x3: RSVD
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - SPI_CLK Direction 0: Master mode, SPI controller drives SPI_CLK 1: Slave mode, SPI controller receives SPI_CLK
    #[inline(always)]
    pub fn sclkdir(&self) -> SclkdirR {
        SclkdirR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPI_CS Direction 0: Master mode, SPI controller drives SPI_CS 1: Slave mode, SPI controller receives SPI_CS
    #[inline(always)]
    pub fn sfrmdir(&self) -> SfrmdirR {
        SfrmdirR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:9 - SPI controller Work data size, register bits value 0~31 indicated data size 1~32 bits, usually use data size 8bits, 16bits, 24bits, 32bits
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bit 10 - Motorola SPI SPI_CLK Polarity Setting 0: The inactive or idle state of SPI_CLK is low 1: The inactive or idle state of SPI_CLK is high
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Motorola SPI SPI_CLK phase setting 0: SPI_CLK is inactive until one cycle after the start of a frame and active until 1/2 cycle before the end of a frame 1: SPI_CLK is inactive until 1/2 cycle after the start of a frame and active until one cycle before the end of a frame
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Trailing Byte 0: Trailing bytes are handled by CPU 1: Trailing bytes are handled by DMA bursts
    #[inline(always)]
    pub fn trail(&self) -> TrailR {
        TrailR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Hold Frame Low Control 0:After this field is set to 1 and the SPI controller is operating in master mode,the output frame signal SPI_CS will be determined by control FSM. 1:After this field is set to 1 and the SPI controller is operating in master mode, the output frame signal SPI_CS will hold low.
    #[inline(always)]
    pub fn hold_frame_low(&self) -> HoldFrameLowR {
        HoldFrameLowR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Invert Frame Signal 0: SPI_CS polarity is as defined in protocol 1: SPI_CS will be inverted from normal-SPI_CS
    #[inline(always)]
    pub fn ifs(&self) -> IfsR {
        IfsR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - SPI_DO Three-State Enable 0: SPI_DO output signal is not three-stated 1: SPI_DO is three-stated when not transmitting data
    #[inline(always)]
    pub fn tte(&self) -> TteR {
        TteR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SPI_DO Three-state Enable On Last Phase (can be set only when TI-SSP) 0: SPI_DO is three-stated 1/2 clock cycle after the beginning of the LSB 1: SPI_DO output signal is three-stated on the clock edge that ends the LSB
    #[inline(always)]
    pub fn ttelp(&self) -> TtelpR {
        TtelpR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOP_CTRL")
            .field("ttelp", &self.ttelp())
            .field("tte", &self.tte())
            .field("ifs", &self.ifs())
            .field("hold_frame_low", &self.hold_frame_low())
            .field("trail", &self.trail())
            .field("sph", &self.sph())
            .field("spo", &self.spo())
            .field("dss", &self.dss())
            .field("sfrmdir", &self.sfrmdir())
            .field("sclkdir", &self.sclkdir())
            .field("frf", &self.frf())
            .field("sse", &self.sse())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI controller Enable 0: SPI controller is disabled 1: SPI controller is enabled
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<TOP_CTRLrs> {
        SseW::new(self, 0)
    }
    ///Bits 1:2 - Frame Format 0x0: Motorola* Serial Peripheral Interface (SPI) 0x1: Texas Instruments* Synchronous Serial Protocol (SSP) 0x2: National Semiconductor Microwire* 0x3: RSVD
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<TOP_CTRLrs> {
        FrfW::new(self, 1)
    }
    ///Bit 3 - SPI_CLK Direction 0: Master mode, SPI controller drives SPI_CLK 1: Slave mode, SPI controller receives SPI_CLK
    #[inline(always)]
    pub fn sclkdir(&mut self) -> SclkdirW<TOP_CTRLrs> {
        SclkdirW::new(self, 3)
    }
    ///Bit 4 - SPI_CS Direction 0: Master mode, SPI controller drives SPI_CS 1: Slave mode, SPI controller receives SPI_CS
    #[inline(always)]
    pub fn sfrmdir(&mut self) -> SfrmdirW<TOP_CTRLrs> {
        SfrmdirW::new(self, 4)
    }
    ///Bits 5:9 - SPI controller Work data size, register bits value 0~31 indicated data size 1~32 bits, usually use data size 8bits, 16bits, 24bits, 32bits
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<TOP_CTRLrs> {
        DssW::new(self, 5)
    }
    ///Bit 10 - Motorola SPI SPI_CLK Polarity Setting 0: The inactive or idle state of SPI_CLK is low 1: The inactive or idle state of SPI_CLK is high
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<TOP_CTRLrs> {
        SpoW::new(self, 10)
    }
    ///Bit 11 - Motorola SPI SPI_CLK phase setting 0: SPI_CLK is inactive until one cycle after the start of a frame and active until 1/2 cycle before the end of a frame 1: SPI_CLK is inactive until 1/2 cycle after the start of a frame and active until one cycle before the end of a frame
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<TOP_CTRLrs> {
        SphW::new(self, 11)
    }
    ///Bit 13 - Trailing Byte 0: Trailing bytes are handled by CPU 1: Trailing bytes are handled by DMA bursts
    #[inline(always)]
    pub fn trail(&mut self) -> TrailW<TOP_CTRLrs> {
        TrailW::new(self, 13)
    }
    ///Bit 14 - Hold Frame Low Control 0:After this field is set to 1 and the SPI controller is operating in master mode,the output frame signal SPI_CS will be determined by control FSM. 1:After this field is set to 1 and the SPI controller is operating in master mode, the output frame signal SPI_CS will hold low.
    #[inline(always)]
    pub fn hold_frame_low(&mut self) -> HoldFrameLowW<TOP_CTRLrs> {
        HoldFrameLowW::new(self, 14)
    }
    ///Bit 15 - Invert Frame Signal 0: SPI_CS polarity is as defined in protocol 1: SPI_CS will be inverted from normal-SPI_CS
    #[inline(always)]
    pub fn ifs(&mut self) -> IfsW<TOP_CTRLrs> {
        IfsW::new(self, 15)
    }
    ///Bit 17 - SPI_DO Three-State Enable 0: SPI_DO output signal is not three-stated 1: SPI_DO is three-stated when not transmitting data
    #[inline(always)]
    pub fn tte(&mut self) -> TteW<TOP_CTRLrs> {
        TteW::new(self, 17)
    }
    ///Bit 18 - SPI_DO Three-state Enable On Last Phase (can be set only when TI-SSP) 0: SPI_DO is three-stated 1/2 clock cycle after the beginning of the LSB 1: SPI_DO output signal is three-stated on the clock edge that ends the LSB
    #[inline(always)]
    pub fn ttelp(&mut self) -> TtelpW<TOP_CTRLrs> {
        TtelpW::new(self, 18)
    }
}
///Top Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`top_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TOP_CTRLrs;
impl crate::RegisterSpec for TOP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`top_ctrl::R`](R) reader structure
impl crate::Readable for TOP_CTRLrs {}
///`write(|w| ..)` method takes [`top_ctrl::W`](W) writer structure
impl crate::Writable for TOP_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOP_CTRL to value 0
impl crate::Resettable for TOP_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
