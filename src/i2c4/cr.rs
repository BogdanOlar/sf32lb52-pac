///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `MODE` reader - Bus Mode (Master operation): 2'b00: standard-mode 2'b01: fast-mode and fast-mode plus 2'b10: HS-mode (standard mode when not doing a high speed transfer) 2'b11: HS-mode (fast mode when not doing a high speed transfer) Bus Mode (Slave operation): 2'b0x: HS-mode is disabled. I2C unit uses Standard/Fast mode timing on the SDA pin. 2'b1x: HS-mode is enabled. I2C unit uses HS-mode timing on the SDA pin when a master code is received.
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - Bus Mode (Master operation): 2'b00: standard-mode 2'b01: fast-mode and fast-mode plus 2'b10: HS-mode (standard mode when not doing a high speed transfer) 2'b11: HS-mode (fast mode when not doing a high speed transfer) Bus Mode (Slave operation): 2'b0x: HS-mode is disabled. I2C unit uses Standard/Fast mode timing on the SDA pin. 2'b1x: HS-mode is enabled. I2C unit uses HS-mode timing on the SDA pin when a master code is received.
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IUE` reader - I2C Unit Enable: 0 = Disables the unit and does not master any transactions or respond to any slave transactions. 1 = Enables the I2C (defaults to slave-receive mode). Software must guarantee the I2C bus is idle before setting this bit.
pub type IueR = crate::BitReader;
///Field `IUE` writer - I2C Unit Enable: 0 = Disables the unit and does not master any transactions or respond to any slave transactions. 1 = Enables the I2C (defaults to slave-receive mode). Software must guarantee the I2C bus is idle before setting this bit.
pub type IueW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCLE` reader - SCL Enable: 0 = Disables the I2C from driving the SCL line. 1 = Enables the I2C clock output for master-mode operation.
pub type ScleR = crate::BitReader;
///Field `SCLE` writer - SCL Enable: 0 = Disables the I2C from driving the SCL line. 1 = Enables the I2C clock output for master-mode operation.
pub type ScleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA Enable for both TX and RX 0 = DMA mode is NOT enabled 1 = DMA mode enabled
pub type DmaenR = crate::BitReader;
///Field `DMAEN` writer - DMA Enable for both TX and RX 0 = DMA mode is NOT enabled 1 = DMA mode enabled
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LASTNACK` reader - Generate NACK for last DMA Read transfer
pub type LastnackR = crate::BitReader;
///Field `LASTNACK` writer - Generate NACK for last DMA Read transfer
pub type LastnackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LASTSTOP` reader - Generate STOP for last DMA transfer
pub type LaststopR = crate::BitReader;
///Field `LASTSTOP` writer - Generate STOP for last DMA transfer
pub type LaststopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSDE` reader - Master Stop Detected Enable: 0 = Master Stop Detect (MSD) status is not enabled. 1 = Master Stop Detect (MSD) status is enabled.
pub type MsdeR = crate::BitReader;
///Field `MSDE` writer - Master Stop Detected Enable: 0 = Master Stop Detect (MSD) status is not enabled. 1 = Master Stop Detect (MSD) status is enabled.
pub type MsdeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCLPP` reader - Push-pull mode Enable for SCL. 0 = open drain output for SCL. 1 = Push-pull output for SCL
pub type SclppR = crate::BitReader;
///Field `SCLPP` writer - Push-pull mode Enable for SCL. 0 = open drain output for SCL. 1 = Push-pull output for SCL
pub type SclppW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLVEN` reader - Slave mode Enable for SCL. 0 = Disable slave mode. Will not monitor slave address on I2C bus. 1 = Enable slave mode. Will monitor slave address on I2C bus.
pub type SlvenR = crate::BitReader;
///Field `SLVEN` writer - Slave mode Enable for SCL. 0 = Disable slave mode. Will not monitor slave address on I2C bus. 1 = Enable slave mode. Will monitor slave address on I2C bus.
pub type SlvenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF*Tfclk. 0: Digital filter disabled 1: Digital filter enabled and filtering capability up to 1 Tfclk ... 7: digital filter enabled and filtering capability up to 7 Tfclk Digital filter is added to analog filter. Digital filter will introduce delay on SCL and SDA processing, which is essential in hs-mode.
pub type DnfR = crate::FieldReader;
///Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF*Tfclk. 0: Digital filter disabled 1: Digital filter enabled and filtering capability up to 1 Tfclk ... 7: digital filter enabled and filtering capability up to 7 Tfclk Digital filter is added to analog filter. Digital filter will introduce delay on SCL and SDA processing, which is essential in hs-mode.
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BRGRST` reader - Reset bus related state machine and signals. Will be cleared by HW automatically 1 = request for reset 0 = reset finished
pub type BrgrstR = crate::BitReader;
///Field `BRGRST` writer - Reset bus related state machine and signals. Will be cleared by HW automatically 1 = request for reset 0 = reset finished
pub type BrgrstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTREQ` reader - I2C will do bus reset upon this bit set. Will be cleared by HW automatically after RSTCYC cycles of SCL generated. 1 = request for i2c bus reset 0 = bus reset finished
pub type RstreqR = crate::BitReader;
///Field `RSTREQ` writer - I2C will do bus reset upon this bit set. Will be cleared by HW automatically after RSTCYC cycles of SCL generated. 1 = request for i2c bus reset 0 = bus reset finished
pub type RstreqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UR` reader - Unit Reset. Software need first assert to reset then deassert to release. 0 = No reset. 1 = Reset I2C module.
pub type UrR = crate::BitReader;
///Field `UR` writer - Unit Reset. Software need first assert to reset then deassert to release. 0 = No reset. 1 = Reset I2C module.
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Bus Mode (Master operation): 2'b00: standard-mode 2'b01: fast-mode and fast-mode plus 2'b10: HS-mode (standard mode when not doing a high speed transfer) 2'b11: HS-mode (fast mode when not doing a high speed transfer) Bus Mode (Slave operation): 2'b0x: HS-mode is disabled. I2C unit uses Standard/Fast mode timing on the SDA pin. 2'b1x: HS-mode is enabled. I2C unit uses HS-mode timing on the SDA pin when a master code is received.
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - I2C Unit Enable: 0 = Disables the unit and does not master any transactions or respond to any slave transactions. 1 = Enables the I2C (defaults to slave-receive mode). Software must guarantee the I2C bus is idle before setting this bit.
    #[inline(always)]
    pub fn iue(&self) -> IueR {
        IueR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SCL Enable: 0 = Disables the I2C from driving the SCL line. 1 = Enables the I2C clock output for master-mode operation.
    #[inline(always)]
    pub fn scle(&self) -> ScleR {
        ScleR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DMA Enable for both TX and RX 0 = DMA mode is NOT enabled 1 = DMA mode enabled
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Generate NACK for last DMA Read transfer
    #[inline(always)]
    pub fn lastnack(&self) -> LastnackR {
        LastnackR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Generate STOP for last DMA transfer
    #[inline(always)]
    pub fn laststop(&self) -> LaststopR {
        LaststopR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Master Stop Detected Enable: 0 = Master Stop Detect (MSD) status is not enabled. 1 = Master Stop Detect (MSD) status is enabled.
    #[inline(always)]
    pub fn msde(&self) -> MsdeR {
        MsdeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Push-pull mode Enable for SCL. 0 = open drain output for SCL. 1 = Push-pull output for SCL
    #[inline(always)]
    pub fn sclpp(&self) -> SclppR {
        SclppR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Slave mode Enable for SCL. 0 = Disable slave mode. Will not monitor slave address on I2C bus. 1 = Enable slave mode. Will monitor slave address on I2C bus.
    #[inline(always)]
    pub fn slven(&self) -> SlvenR {
        SlvenR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF*Tfclk. 0: Digital filter disabled 1: Digital filter enabled and filtering capability up to 1 Tfclk ... 7: digital filter enabled and filtering capability up to 7 Tfclk Digital filter is added to analog filter. Digital filter will introduce delay on SCL and SDA processing, which is essential in hs-mode.
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 29 - Reset bus related state machine and signals. Will be cleared by HW automatically 1 = request for reset 0 = reset finished
    #[inline(always)]
    pub fn brgrst(&self) -> BrgrstR {
        BrgrstR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - I2C will do bus reset upon this bit set. Will be cleared by HW automatically after RSTCYC cycles of SCL generated. 1 = request for i2c bus reset 0 = bus reset finished
    #[inline(always)]
    pub fn rstreq(&self) -> RstreqR {
        RstreqR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Unit Reset. Software need first assert to reset then deassert to release. 0 = No reset. 1 = Reset I2C module.
    #[inline(always)]
    pub fn ur(&self) -> UrR {
        UrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("ur", &self.ur())
            .field("rstreq", &self.rstreq())
            .field("brgrst", &self.brgrst())
            .field("dnf", &self.dnf())
            .field("slven", &self.slven())
            .field("sclpp", &self.sclpp())
            .field("msde", &self.msde())
            .field("laststop", &self.laststop())
            .field("lastnack", &self.lastnack())
            .field("dmaen", &self.dmaen())
            .field("scle", &self.scle())
            .field("iue", &self.iue())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Bus Mode (Master operation): 2'b00: standard-mode 2'b01: fast-mode and fast-mode plus 2'b10: HS-mode (standard mode when not doing a high speed transfer) 2'b11: HS-mode (fast mode when not doing a high speed transfer) Bus Mode (Slave operation): 2'b0x: HS-mode is disabled. I2C unit uses Standard/Fast mode timing on the SDA pin. 2'b1x: HS-mode is enabled. I2C unit uses HS-mode timing on the SDA pin when a master code is received.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CRrs> {
        ModeW::new(self, 0)
    }
    ///Bit 2 - I2C Unit Enable: 0 = Disables the unit and does not master any transactions or respond to any slave transactions. 1 = Enables the I2C (defaults to slave-receive mode). Software must guarantee the I2C bus is idle before setting this bit.
    #[inline(always)]
    pub fn iue(&mut self) -> IueW<CRrs> {
        IueW::new(self, 2)
    }
    ///Bit 3 - SCL Enable: 0 = Disables the I2C from driving the SCL line. 1 = Enables the I2C clock output for master-mode operation.
    #[inline(always)]
    pub fn scle(&mut self) -> ScleW<CRrs> {
        ScleW::new(self, 3)
    }
    ///Bit 4 - DMA Enable for both TX and RX 0 = DMA mode is NOT enabled 1 = DMA mode enabled
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<CRrs> {
        DmaenW::new(self, 4)
    }
    ///Bit 5 - Generate NACK for last DMA Read transfer
    #[inline(always)]
    pub fn lastnack(&mut self) -> LastnackW<CRrs> {
        LastnackW::new(self, 5)
    }
    ///Bit 6 - Generate STOP for last DMA transfer
    #[inline(always)]
    pub fn laststop(&mut self) -> LaststopW<CRrs> {
        LaststopW::new(self, 6)
    }
    ///Bit 8 - Master Stop Detected Enable: 0 = Master Stop Detect (MSD) status is not enabled. 1 = Master Stop Detect (MSD) status is enabled.
    #[inline(always)]
    pub fn msde(&mut self) -> MsdeW<CRrs> {
        MsdeW::new(self, 8)
    }
    ///Bit 9 - Push-pull mode Enable for SCL. 0 = open drain output for SCL. 1 = Push-pull output for SCL
    #[inline(always)]
    pub fn sclpp(&mut self) -> SclppW<CRrs> {
        SclppW::new(self, 9)
    }
    ///Bit 11 - Slave mode Enable for SCL. 0 = Disable slave mode. Will not monitor slave address on I2C bus. 1 = Enable slave mode. Will monitor slave address on I2C bus.
    #[inline(always)]
    pub fn slven(&mut self) -> SlvenW<CRrs> {
        SlvenW::new(self, 11)
    }
    ///Bits 12:14 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF*Tfclk. 0: Digital filter disabled 1: Digital filter enabled and filtering capability up to 1 Tfclk ... 7: digital filter enabled and filtering capability up to 7 Tfclk Digital filter is added to analog filter. Digital filter will introduce delay on SCL and SDA processing, which is essential in hs-mode.
    #[inline(always)]
    pub fn dnf(&mut self) -> DnfW<CRrs> {
        DnfW::new(self, 12)
    }
    ///Bit 29 - Reset bus related state machine and signals. Will be cleared by HW automatically 1 = request for reset 0 = reset finished
    #[inline(always)]
    pub fn brgrst(&mut self) -> BrgrstW<CRrs> {
        BrgrstW::new(self, 29)
    }
    ///Bit 30 - I2C will do bus reset upon this bit set. Will be cleared by HW automatically after RSTCYC cycles of SCL generated. 1 = request for i2c bus reset 0 = bus reset finished
    #[inline(always)]
    pub fn rstreq(&mut self) -> RstreqW<CRrs> {
        RstreqW::new(self, 30)
    }
    ///Bit 31 - Unit Reset. Software need first assert to reset then deassert to release. 0 = No reset. 1 = Reset I2C module.
    #[inline(always)]
    pub fn ur(&mut self) -> UrW<CRrs> {
        UrW::new(self, 31)
    }
}
///Control register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
