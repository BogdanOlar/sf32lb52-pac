///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::FieldReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SSDIE` reader - Slave Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when it detects a Stop condition while in slave mode.
pub type SsdieR = crate::BitReader;
///Field `SSDIE` writer - Slave Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when it detects a Stop condition while in slave mode.
pub type SsdieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALDIE` reader - Arbitration Loss Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon losing arbitration while in master mode.
pub type AldieR = crate::BitReader;
///Field `ALDIE` writer - Arbitration Loss Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon losing arbitration while in master mode.
pub type AldieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - DBR Transmit Empty Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt after transmitting a byte onto the I2C bus.
pub type TeieR = crate::BitReader;
///Field `TEIE` writer - DBR Transmit Empty Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt after transmitting a byte onto the I2C bus.
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIE` reader - DBR Receive Full Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when the DBR has received a data byte from the I2C bus.
pub type RfieR = crate::BitReader;
///Field `RFIE` writer - DBR Receive Full Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when the DBR has received a data byte from the I2C bus.
pub type RfieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SADIE` reader - Slave Address Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon detecting a slave address match or a general call address.
pub type SadieR = crate::BitReader;
///Field `SADIE` writer - Slave Address Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon detecting a slave address match or a general call address.
pub type SadieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BEDIE` reader - Bus Error Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt for the following I2C bus errors: As a master transmitter, no ACK was detected after a byte was sent. As a slave receiver, the I2C generated a NACK pulse. Software is responsible for guaranteeing that misplaced Start and Stop conditions do not occur.
pub type BedieR = crate::BitReader;
///Field `BEDIE` writer - Bus Error Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt for the following I2C bus errors: As a master transmitter, no ACK was detected after a byte was sent. As a slave receiver, the I2C generated a NACK pulse. Software is responsible for guaranteeing that misplaced Start and Stop conditions do not occur.
pub type BedieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSDIE` reader - Master Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C unit to interrupt upon detecting a Master Stop sent by the I2C unit.
pub type MsdieR = crate::BitReader;
///Field `MSDIE` writer - Master Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C unit to interrupt upon detecting a Master Stop sent by the I2C unit.
pub type MsdieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMADONEIE` reader - DMA Transaction Done Interrupt Enable 0 = DMA Transaction done interrupt is not enabled. 1 = DMA Transaction done interrupt is enabled.
pub type DmadoneieR = crate::BitReader;
///Field `DMADONEIE` writer - DMA Transaction Done Interrupt Enable 0 = DMA Transaction done interrupt is not enabled. 1 = DMA Transaction done interrupt is enabled.
pub type DmadoneieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFIE` reader - FIFO Overflow Interrupt Enable 0 = FIFO Overflow interrupt is not enabled 1 = FIFO Overflow interrupt is enabled
pub type OfieR = crate::BitReader;
///Field `OFIE` writer - FIFO Overflow Interrupt Enable 0 = FIFO Overflow interrupt is not enabled 1 = FIFO Overflow interrupt is enabled
pub type OfieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UFIE` reader - FIFO Underflow Interrupt Enable 0 = FIFO Underflow interrupt is not enabled 1 = FIFO Underflow interrupt is enabled
pub type UfieR = crate::BitReader;
///Field `UFIE` writer - FIFO Underflow Interrupt Enable 0 = FIFO Underflow interrupt is not enabled 1 = FIFO Underflow interrupt is enabled
pub type UfieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Slave Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when it detects a Stop condition while in slave mode.
    #[inline(always)]
    pub fn ssdie(&self) -> SsdieR {
        SsdieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Arbitration Loss Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon losing arbitration while in master mode.
    #[inline(always)]
    pub fn aldie(&self) -> AldieR {
        AldieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DBR Transmit Empty Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt after transmitting a byte onto the I2C bus.
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DBR Receive Full Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when the DBR has received a data byte from the I2C bus.
    #[inline(always)]
    pub fn rfie(&self) -> RfieR {
        RfieR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Slave Address Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon detecting a slave address match or a general call address.
    #[inline(always)]
    pub fn sadie(&self) -> SadieR {
        SadieR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Bus Error Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt for the following I2C bus errors: As a master transmitter, no ACK was detected after a byte was sent. As a slave receiver, the I2C generated a NACK pulse. Software is responsible for guaranteeing that misplaced Start and Stop conditions do not occur.
    #[inline(always)]
    pub fn bedie(&self) -> BedieR {
        BedieR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Master Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C unit to interrupt upon detecting a Master Stop sent by the I2C unit.
    #[inline(always)]
    pub fn msdie(&self) -> MsdieR {
        MsdieR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Transaction Done Interrupt Enable 0 = DMA Transaction done interrupt is not enabled. 1 = DMA Transaction done interrupt is enabled.
    #[inline(always)]
    pub fn dmadoneie(&self) -> DmadoneieR {
        DmadoneieR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - FIFO Overflow Interrupt Enable 0 = FIFO Overflow interrupt is not enabled 1 = FIFO Overflow interrupt is enabled
    #[inline(always)]
    pub fn ofie(&self) -> OfieR {
        OfieR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - FIFO Underflow Interrupt Enable 0 = FIFO Underflow interrupt is not enabled 1 = FIFO Underflow interrupt is enabled
    #[inline(always)]
    pub fn ufie(&self) -> UfieR {
        UfieR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rsvd", &self.rsvd())
            .field("ufie", &self.ufie())
            .field("ofie", &self.ofie())
            .field("dmadoneie", &self.dmadoneie())
            .field("msdie", &self.msdie())
            .field("rsvd2", &self.rsvd2())
            .field("bedie", &self.bedie())
            .field("sadie", &self.sadie())
            .field("rsvd3", &self.rsvd3())
            .field("rfie", &self.rfie())
            .field("teie", &self.teie())
            .field("aldie", &self.aldie())
            .field("ssdie", &self.ssdie())
            .field("rsvd4", &self.rsvd4())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<IERrs> {
        Rsvd4W::new(self, 0)
    }
    ///Bit 4 - Slave Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when it detects a Stop condition while in slave mode.
    #[inline(always)]
    pub fn ssdie(&mut self) -> SsdieW<IERrs> {
        SsdieW::new(self, 4)
    }
    ///Bit 5 - Arbitration Loss Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon losing arbitration while in master mode.
    #[inline(always)]
    pub fn aldie(&mut self) -> AldieW<IERrs> {
        AldieW::new(self, 5)
    }
    ///Bit 6 - DBR Transmit Empty Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt after transmitting a byte onto the I2C bus.
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<IERrs> {
        TeieW::new(self, 6)
    }
    ///Bit 7 - DBR Receive Full Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when the DBR has received a data byte from the I2C bus.
    #[inline(always)]
    pub fn rfie(&mut self) -> RfieW<IERrs> {
        RfieW::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<IERrs> {
        Rsvd3W::new(self, 8)
    }
    ///Bit 9 - Slave Address Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon detecting a slave address match or a general call address.
    #[inline(always)]
    pub fn sadie(&mut self) -> SadieW<IERrs> {
        SadieW::new(self, 9)
    }
    ///Bit 10 - Bus Error Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt for the following I2C bus errors: As a master transmitter, no ACK was detected after a byte was sent. As a slave receiver, the I2C generated a NACK pulse. Software is responsible for guaranteeing that misplaced Start and Stop conditions do not occur.
    #[inline(always)]
    pub fn bedie(&mut self) -> BedieW<IERrs> {
        BedieW::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<IERrs> {
        Rsvd2W::new(self, 11)
    }
    ///Bit 12 - Master Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C unit to interrupt upon detecting a Master Stop sent by the I2C unit.
    #[inline(always)]
    pub fn msdie(&mut self) -> MsdieW<IERrs> {
        MsdieW::new(self, 12)
    }
    ///Bit 13 - DMA Transaction Done Interrupt Enable 0 = DMA Transaction done interrupt is not enabled. 1 = DMA Transaction done interrupt is enabled.
    #[inline(always)]
    pub fn dmadoneie(&mut self) -> DmadoneieW<IERrs> {
        DmadoneieW::new(self, 13)
    }
    ///Bit 14 - FIFO Overflow Interrupt Enable 0 = FIFO Overflow interrupt is not enabled 1 = FIFO Overflow interrupt is enabled
    #[inline(always)]
    pub fn ofie(&mut self) -> OfieW<IERrs> {
        OfieW::new(self, 14)
    }
    ///Bit 15 - FIFO Underflow Interrupt Enable 0 = FIFO Underflow interrupt is not enabled 1 = FIFO Underflow interrupt is enabled
    #[inline(always)]
    pub fn ufie(&mut self) -> UfieW<IERrs> {
        UfieW::new(self, 15)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IERrs> {
        RsvdW::new(self, 16)
    }
}
///Interrupt Enable register
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
