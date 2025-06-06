///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `RWM` reader - Read/write Mode: 0 = The I2C is in master-transmit or slave-receive mode. 1 = The I2C is in master-receive or slave-transmit mode. This is the R/nW bit of the slave address. It is cleared automatically by hardware after a Stop state.
pub type RwmR = crate::BitReader;
///Field `RWM` writer - Read/write Mode: 0 = The I2C is in master-transmit or slave-receive mode. 1 = The I2C is in master-receive or slave-transmit mode. This is the R/nW bit of the slave address. It is cleared automatically by hardware after a Stop state.
pub type RwmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - ACK/NACK Status: 0 = The I2C received or sent an ACK on the bus. 1 = The I2C received or sent a NACK.on the bus. This bit is used in slave-transmit mode to determine when the byte transferred is the last one. This bit is updated after each byte and ACK/NACK information is received.
pub type NackR = crate::BitReader;
///Field `NACK` writer - ACK/NACK Status: 0 = The I2C received or sent an ACK on the bus. 1 = The I2C received or sent a NACK.on the bus. This bit is used in slave-transmit mode to determine when the byte transferred is the last one. This bit is updated after each byte and ACK/NACK information is received.
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UB` reader - Unit Busy: 0 = I2C not busy. 1 = Set when local I2C is busy. This is defined as the time between the first Start and Stop.
pub type UbR = crate::BitReader;
///Field `UB` writer - Unit Busy: 0 = I2C not busy. 1 = Set when local I2C is busy. This is defined as the time between the first Start and Stop.
pub type UbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IBB` reader - I2C Bus Busy: 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the I2C bus is busy but local I2C is not involved in the transaction.
pub type IbbR = crate::BitReader;
///Field `IBB` writer - I2C Bus Busy: 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the I2C bus is busy but local I2C is not involved in the transaction.
pub type IbbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSD` reader - Slave Stop Detected: 0 = No Stop detected. 1 = Set when the I2C detects a Stop while in slave-receive or slave-transmit mode. Cleared if write 1
pub type SsdR = crate::BitReader;
///Field `SSD` writer - Slave Stop Detected: 0 = No Stop detected. 1 = Set when the I2C detects a Stop while in slave-receive or slave-transmit mode. Cleared if write 1
pub type SsdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALD` reader - Arbitration Loss Detected: Used during multi-master operation: 0 = Cleared when arbitration is won or never took place. 1 = Set when the I2C loses arbitration. Cleared if write 1
pub type AldR = crate::BitReader;
///Field `ALD` writer - Arbitration Loss Detected: Used during multi-master operation: 0 = Cleared when arbitration is won or never took place. 1 = Set when the I2C loses arbitration. Cleared if write 1
pub type AldW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - DBR Transmit Empty: 0 = The data byte is still being transmitted. 1 = The I2C has finished transmitting a data byte on the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
pub type TeR = crate::BitReader;
///Field `TE` writer - DBR Transmit Empty: 0 = The data byte is still being transmitted. 1 = The I2C has finished transmitting a data byte on the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF` reader - DBR Receive Full: 0 = The DBR has not received a new data byte or the I2C is idle. 1 = The DBR register received a new data byte from the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
pub type RfR = crate::BitReader;
///Field `RF` writer - DBR Receive Full: 0 = The DBR has not received a new data byte or the I2C is idle. 1 = The DBR register received a new data byte from the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
pub type RfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAD` reader - Slave Address Detected: 0 = No slave address was detected. 1 = The I2C detected a seven-bit address that matches the general call address or SAR. An interrupt is signalled when enabled in the CR. Cleared if write 1
pub type SadR = crate::BitReader;
///Field `SAD` writer - Slave Address Detected: 0 = No slave address was detected. 1 = The I2C detected a seven-bit address that matches the general call address or SAR. An interrupt is signalled when enabled in the CR. Cleared if write 1
pub type SadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BED` reader - Bus Error Detected: 0 = No error detected. 1 = The I2C sets this bit when it detects one of the following error conditions: As a master transmitter, no ACK was detected on the interface after a byte was sent. As a slave receiver, the I2C generates a NACK pulse. When an error occurs, I2C bus transactions continue. Software must guarantee that misplaced Start and Stop conditions do not occur. Cleared if write 1
pub type BedR = crate::BitReader;
///Field `BED` writer - Bus Error Detected: 0 = No error detected. 1 = The I2C sets this bit when it detects one of the following error conditions: As a master transmitter, no ACK was detected on the interface after a byte was sent. As a slave receiver, the I2C generates a NACK pulse. When an error occurs, I2C bus transactions continue. Software must guarantee that misplaced Start and Stop conditions do not occur. Cleared if write 1
pub type BedW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EBB` reader - Early Bus Busy 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the unit detects that the SCL or SDA line is low without a START condition. Bit will remain set until the I2C unit detects the bus is idle by detecting a STOP condition. Bit will also be set whenever the IBB bit is set.
pub type EbbR = crate::BitReader;
///Field `EBB` writer - Early Bus Busy 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the unit detects that the SCL or SDA line is low without a START condition. Bit will remain set until the I2C unit detects the bus is idle by detecting a STOP condition. Bit will also be set whenever the IBB bit is set.
pub type EbbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSD` reader - Master Stop Detected: 0 = No Master Stop Detected. 1 = This bit is set by the I2C unit when all of the following are true: This bit is enabled (CR\[MSDE\]
///= 1); I2C unit is configured as a master; I2C transmits a STOP signal
pub type MsdR = crate::BitReader;
///Field `MSD` writer - Master Stop Detected: 0 = No Master Stop Detected. 1 = This bit is set by the I2C unit when all of the following are true: This bit is enabled (CR\[MSDE\]
///= 1); I2C unit is configured as a master; I2C transmits a STOP signal
pub type MsdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMADONE` reader - DMA Transaction Done. Asserted when both APB and I2C bus have finished transfer. Cleared if write 1
pub type DmadoneR = crate::BitReader;
///Field `DMADONE` writer - DMA Transaction Done. Asserted when both APB and I2C bus have finished transfer. Cleared if write 1
pub type DmadoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OF` reader - FIFO Overflow Flag. Asserted when FIFO is full and a PUSH request generated without a POP. Cleared if write 1
pub type OfR = crate::BitReader;
///Field `OF` writer - FIFO Overflow Flag. Asserted when FIFO is full and a PUSH request generated without a POP. Cleared if write 1
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UF` reader - FIFO Underflow Flag. Asserted when FIFO is empty and a POP request generated without a PUSH. Cleared if write 1
pub type UfR = crate::BitReader;
///Field `UF` writer - FIFO Underflow Flag. Asserted when FIFO is empty and a POP request generated without a PUSH. Cleared if write 1
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Read/write Mode: 0 = The I2C is in master-transmit or slave-receive mode. 1 = The I2C is in master-receive or slave-transmit mode. This is the R/nW bit of the slave address. It is cleared automatically by hardware after a Stop state.
    #[inline(always)]
    pub fn rwm(&self) -> RwmR {
        RwmR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACK/NACK Status: 0 = The I2C received or sent an ACK on the bus. 1 = The I2C received or sent a NACK.on the bus. This bit is used in slave-transmit mode to determine when the byte transferred is the last one. This bit is updated after each byte and ACK/NACK information is received.
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Unit Busy: 0 = I2C not busy. 1 = Set when local I2C is busy. This is defined as the time between the first Start and Stop.
    #[inline(always)]
    pub fn ub(&self) -> UbR {
        UbR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C Bus Busy: 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the I2C bus is busy but local I2C is not involved in the transaction.
    #[inline(always)]
    pub fn ibb(&self) -> IbbR {
        IbbR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Slave Stop Detected: 0 = No Stop detected. 1 = Set when the I2C detects a Stop while in slave-receive or slave-transmit mode. Cleared if write 1
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Arbitration Loss Detected: Used during multi-master operation: 0 = Cleared when arbitration is won or never took place. 1 = Set when the I2C loses arbitration. Cleared if write 1
    #[inline(always)]
    pub fn ald(&self) -> AldR {
        AldR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DBR Transmit Empty: 0 = The data byte is still being transmitted. 1 = The I2C has finished transmitting a data byte on the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DBR Receive Full: 0 = The DBR has not received a new data byte or the I2C is idle. 1 = The DBR register received a new data byte from the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
    #[inline(always)]
    pub fn rf(&self) -> RfR {
        RfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Slave Address Detected: 0 = No slave address was detected. 1 = The I2C detected a seven-bit address that matches the general call address or SAR. An interrupt is signalled when enabled in the CR. Cleared if write 1
    #[inline(always)]
    pub fn sad(&self) -> SadR {
        SadR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Bus Error Detected: 0 = No error detected. 1 = The I2C sets this bit when it detects one of the following error conditions: As a master transmitter, no ACK was detected on the interface after a byte was sent. As a slave receiver, the I2C generates a NACK pulse. When an error occurs, I2C bus transactions continue. Software must guarantee that misplaced Start and Stop conditions do not occur. Cleared if write 1
    #[inline(always)]
    pub fn bed(&self) -> BedR {
        BedR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Early Bus Busy 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the unit detects that the SCL or SDA line is low without a START condition. Bit will remain set until the I2C unit detects the bus is idle by detecting a STOP condition. Bit will also be set whenever the IBB bit is set.
    #[inline(always)]
    pub fn ebb(&self) -> EbbR {
        EbbR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Master Stop Detected: 0 = No Master Stop Detected. 1 = This bit is set by the I2C unit when all of the following are true: This bit is enabled (CR\[MSDE\]
    ///= 1); I2C unit is configured as a master; I2C transmits a STOP signal
    #[inline(always)]
    pub fn msd(&self) -> MsdR {
        MsdR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Transaction Done. Asserted when both APB and I2C bus have finished transfer. Cleared if write 1
    #[inline(always)]
    pub fn dmadone(&self) -> DmadoneR {
        DmadoneR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - FIFO Overflow Flag. Asserted when FIFO is full and a PUSH request generated without a POP. Cleared if write 1
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - FIFO Underflow Flag. Asserted when FIFO is empty and a POP request generated without a PUSH. Cleared if write 1
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rsvd", &self.rsvd())
            .field("uf", &self.uf())
            .field("of", &self.of())
            .field("dmadone", &self.dmadone())
            .field("msd", &self.msd())
            .field("ebb", &self.ebb())
            .field("bed", &self.bed())
            .field("sad", &self.sad())
            .field("rsvd2", &self.rsvd2())
            .field("rf", &self.rf())
            .field("te", &self.te())
            .field("ald", &self.ald())
            .field("ssd", &self.ssd())
            .field("ibb", &self.ibb())
            .field("ub", &self.ub())
            .field("nack", &self.nack())
            .field("rwm", &self.rwm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Read/write Mode: 0 = The I2C is in master-transmit or slave-receive mode. 1 = The I2C is in master-receive or slave-transmit mode. This is the R/nW bit of the slave address. It is cleared automatically by hardware after a Stop state.
    #[inline(always)]
    pub fn rwm(&mut self) -> RwmW<SRrs> {
        RwmW::new(self, 0)
    }
    ///Bit 1 - ACK/NACK Status: 0 = The I2C received or sent an ACK on the bus. 1 = The I2C received or sent a NACK.on the bus. This bit is used in slave-transmit mode to determine when the byte transferred is the last one. This bit is updated after each byte and ACK/NACK information is received.
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<SRrs> {
        NackW::new(self, 1)
    }
    ///Bit 2 - Unit Busy: 0 = I2C not busy. 1 = Set when local I2C is busy. This is defined as the time between the first Start and Stop.
    #[inline(always)]
    pub fn ub(&mut self) -> UbW<SRrs> {
        UbW::new(self, 2)
    }
    ///Bit 3 - I2C Bus Busy: 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the I2C bus is busy but local I2C is not involved in the transaction.
    #[inline(always)]
    pub fn ibb(&mut self) -> IbbW<SRrs> {
        IbbW::new(self, 3)
    }
    ///Bit 4 - Slave Stop Detected: 0 = No Stop detected. 1 = Set when the I2C detects a Stop while in slave-receive or slave-transmit mode. Cleared if write 1
    #[inline(always)]
    pub fn ssd(&mut self) -> SsdW<SRrs> {
        SsdW::new(self, 4)
    }
    ///Bit 5 - Arbitration Loss Detected: Used during multi-master operation: 0 = Cleared when arbitration is won or never took place. 1 = Set when the I2C loses arbitration. Cleared if write 1
    #[inline(always)]
    pub fn ald(&mut self) -> AldW<SRrs> {
        AldW::new(self, 5)
    }
    ///Bit 6 - DBR Transmit Empty: 0 = The data byte is still being transmitted. 1 = The I2C has finished transmitting a data byte on the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
    #[inline(always)]
    pub fn te(&mut self) -> TeW<SRrs> {
        TeW::new(self, 6)
    }
    ///Bit 7 - DBR Receive Full: 0 = The DBR has not received a new data byte or the I2C is idle. 1 = The DBR register received a new data byte from the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1
    #[inline(always)]
    pub fn rf(&mut self) -> RfW<SRrs> {
        RfW::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<SRrs> {
        Rsvd2W::new(self, 8)
    }
    ///Bit 9 - Slave Address Detected: 0 = No slave address was detected. 1 = The I2C detected a seven-bit address that matches the general call address or SAR. An interrupt is signalled when enabled in the CR. Cleared if write 1
    #[inline(always)]
    pub fn sad(&mut self) -> SadW<SRrs> {
        SadW::new(self, 9)
    }
    ///Bit 10 - Bus Error Detected: 0 = No error detected. 1 = The I2C sets this bit when it detects one of the following error conditions: As a master transmitter, no ACK was detected on the interface after a byte was sent. As a slave receiver, the I2C generates a NACK pulse. When an error occurs, I2C bus transactions continue. Software must guarantee that misplaced Start and Stop conditions do not occur. Cleared if write 1
    #[inline(always)]
    pub fn bed(&mut self) -> BedW<SRrs> {
        BedW::new(self, 10)
    }
    ///Bit 11 - Early Bus Busy 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the unit detects that the SCL or SDA line is low without a START condition. Bit will remain set until the I2C unit detects the bus is idle by detecting a STOP condition. Bit will also be set whenever the IBB bit is set.
    #[inline(always)]
    pub fn ebb(&mut self) -> EbbW<SRrs> {
        EbbW::new(self, 11)
    }
    ///Bit 12 - Master Stop Detected: 0 = No Master Stop Detected. 1 = This bit is set by the I2C unit when all of the following are true: This bit is enabled (CR\[MSDE\]
    ///= 1); I2C unit is configured as a master; I2C transmits a STOP signal
    #[inline(always)]
    pub fn msd(&mut self) -> MsdW<SRrs> {
        MsdW::new(self, 12)
    }
    ///Bit 13 - DMA Transaction Done. Asserted when both APB and I2C bus have finished transfer. Cleared if write 1
    #[inline(always)]
    pub fn dmadone(&mut self) -> DmadoneW<SRrs> {
        DmadoneW::new(self, 13)
    }
    ///Bit 14 - FIFO Overflow Flag. Asserted when FIFO is full and a PUSH request generated without a POP. Cleared if write 1
    #[inline(always)]
    pub fn of(&mut self) -> OfW<SRrs> {
        OfW::new(self, 14)
    }
    ///Bit 15 - FIFO Underflow Flag. Asserted when FIFO is empty and a POP request generated without a PUSH. Cleared if write 1
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<SRrs> {
        UfW::new(self, 15)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SRrs> {
        RsvdW::new(self, 16)
    }
}
///Status register
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
