///Register `DBR` reader
pub type R = crate::R<DBRrs>;
///Register `DBR` writer
pub type W = crate::W<DBRrs>;
///Field `DATA` reader - use the I2C Data Buffer register to transmit and receive data from the I2C bus. The DBR is accessed by software on one Side and by the I2C Shift register on the other. The DBR receives data coming into the I2C unit after a full byte is received and acknowledged. CPU writes data going out of the I2C to the DBR and sends it to the serial bus. When the I2C is in transmit mode (master or slave), CPU writes data to the DBR over the internal bus. CPU write data to the DBR when a master transaction is initiated or when the DBR transmit-empty interrupt is signalled. Data moves from the DBR to the Shift register when the transfer byte bit is set. The DBR transmit-empty interrupt is signalled (if enabled) when a byte is transferred on the I2C bus and the acknowledge cycle is complete. If the DBR is not written, and a Stop condition is not in place before the I2C bus is ready to transfer the next byte packet, the I2C unit inserts wait states until CPU writes the DBR and sets the transfer byte bit. When the I2C is in receive mode (master or slave), CPU reads DBR data over the internal bus. CPU reads data from the DBR when the DBR receive-full interrupt is signalled. The data moves from the Shift register to the DBR when the acknowledge cycle is complete. The I2C inserts wait states until the DBR is read. After the software reads the DBR, CR\[NACK\]
///are written by the software, allowing the next byte transfer to proceed to the I2C bus. In DMA mode, DBR is automatically filled from FIFO in master transmit mode, or fetched and stored in FIFO in master receive mode until DMA done or aborted.
pub type DataR = crate::FieldReader;
///Field `DATA` writer - use the I2C Data Buffer register to transmit and receive data from the I2C bus. The DBR is accessed by software on one Side and by the I2C Shift register on the other. The DBR receives data coming into the I2C unit after a full byte is received and acknowledged. CPU writes data going out of the I2C to the DBR and sends it to the serial bus. When the I2C is in transmit mode (master or slave), CPU writes data to the DBR over the internal bus. CPU write data to the DBR when a master transaction is initiated or when the DBR transmit-empty interrupt is signalled. Data moves from the DBR to the Shift register when the transfer byte bit is set. The DBR transmit-empty interrupt is signalled (if enabled) when a byte is transferred on the I2C bus and the acknowledge cycle is complete. If the DBR is not written, and a Stop condition is not in place before the I2C bus is ready to transfer the next byte packet, the I2C unit inserts wait states until CPU writes the DBR and sets the transfer byte bit. When the I2C is in receive mode (master or slave), CPU reads DBR data over the internal bus. CPU reads data from the DBR when the DBR receive-full interrupt is signalled. The data moves from the Shift register to the DBR when the acknowledge cycle is complete. The I2C inserts wait states until the DBR is read. After the software reads the DBR, CR\[NACK\]
///are written by the software, allowing the next byte transfer to proceed to the I2C bus. In DMA mode, DBR is automatically filled from FIFO in master transmit mode, or fetched and stored in FIFO in master receive mode until DMA done or aborted.
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - use the I2C Data Buffer register to transmit and receive data from the I2C bus. The DBR is accessed by software on one Side and by the I2C Shift register on the other. The DBR receives data coming into the I2C unit after a full byte is received and acknowledged. CPU writes data going out of the I2C to the DBR and sends it to the serial bus. When the I2C is in transmit mode (master or slave), CPU writes data to the DBR over the internal bus. CPU write data to the DBR when a master transaction is initiated or when the DBR transmit-empty interrupt is signalled. Data moves from the DBR to the Shift register when the transfer byte bit is set. The DBR transmit-empty interrupt is signalled (if enabled) when a byte is transferred on the I2C bus and the acknowledge cycle is complete. If the DBR is not written, and a Stop condition is not in place before the I2C bus is ready to transfer the next byte packet, the I2C unit inserts wait states until CPU writes the DBR and sets the transfer byte bit. When the I2C is in receive mode (master or slave), CPU reads DBR data over the internal bus. CPU reads data from the DBR when the DBR receive-full interrupt is signalled. The data moves from the Shift register to the DBR when the acknowledge cycle is complete. The I2C inserts wait states until the DBR is read. After the software reads the DBR, CR\[NACK\]
    ///are written by the software, allowing the next byte transfer to proceed to the I2C bus. In DMA mode, DBR is automatically filled from FIFO in master transmit mode, or fetched and stored in FIFO in master receive mode until DMA done or aborted.
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBR").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:7 - use the I2C Data Buffer register to transmit and receive data from the I2C bus. The DBR is accessed by software on one Side and by the I2C Shift register on the other. The DBR receives data coming into the I2C unit after a full byte is received and acknowledged. CPU writes data going out of the I2C to the DBR and sends it to the serial bus. When the I2C is in transmit mode (master or slave), CPU writes data to the DBR over the internal bus. CPU write data to the DBR when a master transaction is initiated or when the DBR transmit-empty interrupt is signalled. Data moves from the DBR to the Shift register when the transfer byte bit is set. The DBR transmit-empty interrupt is signalled (if enabled) when a byte is transferred on the I2C bus and the acknowledge cycle is complete. If the DBR is not written, and a Stop condition is not in place before the I2C bus is ready to transfer the next byte packet, the I2C unit inserts wait states until CPU writes the DBR and sets the transfer byte bit. When the I2C is in receive mode (master or slave), CPU reads DBR data over the internal bus. CPU reads data from the DBR when the DBR receive-full interrupt is signalled. The data moves from the Shift register to the DBR when the acknowledge cycle is complete. The I2C inserts wait states until the DBR is read. After the software reads the DBR, CR\[NACK\]
    ///are written by the software, allowing the next byte transfer to proceed to the I2C bus. In DMA mode, DBR is automatically filled from FIFO in master transmit mode, or fetched and stored in FIFO in master receive mode until DMA done or aborted.
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DBRrs> {
        DataW::new(self, 0)
    }
}
///Data Buffer register
///
///You can [`read`](crate::Reg::read) this register and get [`dbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBRrs;
impl crate::RegisterSpec for DBRrs {
    type Ux = u32;
}
///`read()` method returns [`dbr::R`](R) reader structure
impl crate::Readable for DBRrs {}
///`write(|w| ..)` method takes [`dbr::W`](W) writer structure
impl crate::Writable for DBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBR to value 0
impl crate::Resettable for DBRrs {
    const RESET_VALUE: u32 = 0;
}
