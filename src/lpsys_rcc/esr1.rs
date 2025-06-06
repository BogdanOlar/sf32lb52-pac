///Register `ESR1` reader
pub type R = crate::R<ESR1rs>;
///Register `ESR1` writer
pub type W = crate::W<ESR1rs>;
///Field `DMAC2` reader - write 1 to set module enable, write 0 has no effect
pub type Dmac2R = crate::BitReader;
///Field `DMAC2` writer - write 1 to set module enable, write 0 has no effect
pub type Dmac2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MAILBOX2` reader - write 1 to set module enable, write 0 has no effect
pub type Mailbox2R = crate::BitReader;
///Field `MAILBOX2` writer - write 1 to set module enable, write 0 has no effect
pub type Mailbox2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINMUX2` reader - write 1 to set module enable, write 0 has no effect
pub type Pinmux2R = crate::BitReader;
///Field `PINMUX2` writer - write 1 to set module enable, write 0 has no effect
pub type Pinmux2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PATCH` reader - write 1 to set module enable, write 0 has no effect
pub type PatchR = crate::BitReader;
///Field `PATCH` writer - write 1 to set module enable, write 0 has no effect
pub type PatchW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4` reader - write 1 to set module enable, write 0 has no effect
pub type Usart4R = crate::BitReader;
///Field `USART4` writer - write 1 to set module enable, write 0 has no effect
pub type Usart4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART5` reader - write 1 to set module enable, write 0 has no effect
pub type Usart5R = crate::BitReader;
///Field `USART5` writer - write 1 to set module enable, write 0 has no effect
pub type Usart5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECU2` reader - write 1 to set module enable, write 0 has no effect
pub type Secu2R = crate::BitReader;
///Field `SECU2` writer - write 1 to set module enable, write 0 has no effect
pub type Secu2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTC2` reader - write 1 to set module enable, write 0 has no effect
pub type Ptc2R = crate::BitReader;
///Field `PTC2` writer - write 1 to set module enable, write 0 has no effect
pub type Ptc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTIM3` reader - write 1 to set module enable, write 0 has no effect
pub type Btim3R = crate::BitReader;
///Field `BTIM3` writer - write 1 to set module enable, write 0 has no effect
pub type Btim3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTIM4` reader - write 1 to set module enable, write 0 has no effect
pub type Btim4R = crate::BitReader;
///Field `BTIM4` writer - write 1 to set module enable, write 0 has no effect
pub type Btim4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFG2` reader - write 1 to set module enable, write 0 has no effect
pub type Syscfg2R = crate::BitReader;
///Field `SYSCFG2` writer - write 1 to set module enable, write 0 has no effect
pub type Syscfg2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO2` reader - write 1 to set module enable, write 0 has no effect
pub type Gpio2R = crate::BitReader;
///Field `GPIO2` writer - write 1 to set module enable, write 0 has no effect
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC` reader - write 1 to set module enable, write 0 has no effect
pub type RfcR = crate::BitReader;
///Field `RFC` writer - write 1 to set module enable, write 0 has no effect
pub type RfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHY` reader - write 1 to set module enable, write 0 has no effect
pub type PhyR = crate::BitReader;
///Field `PHY` writer - write 1 to set module enable, write 0 has no effect
pub type PhyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MAC` reader - write 1 to set module enable, write 0 has no effect
pub type MacR = crate::BitReader;
///Field `MAC` writer - write 1 to set module enable, write 0 has no effect
pub type MacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC2` reader - write 1 to set module enable, write 0 has no effect
pub type Crc2R = crate::BitReader;
///Field `CRC2` writer - write 1 to set module enable, write 0 has no effect
pub type Crc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn dmac2(&self) -> Dmac2R {
        Dmac2R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn mailbox2(&self) -> Mailbox2R {
        Mailbox2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn pinmux2(&self) -> Pinmux2R {
        Pinmux2R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn patch(&self) -> PatchR {
        PatchR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn usart5(&self) -> Usart5R {
        Usart5R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn secu2(&self) -> Secu2R {
        Secu2R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn ptc2(&self) -> Ptc2R {
        Ptc2R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn btim3(&self) -> Btim3R {
        Btim3R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn btim4(&self) -> Btim4R {
        Btim4R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn syscfg2(&self) -> Syscfg2R {
        Syscfg2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn gpio2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn rfc(&self) -> RfcR {
        RfcR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn phy(&self) -> PhyR {
        PhyR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn mac(&self) -> MacR {
        MacR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn crc2(&self) -> Crc2R {
        Crc2R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESR1")
            .field("crc2", &self.crc2())
            .field("mac", &self.mac())
            .field("phy", &self.phy())
            .field("rfc", &self.rfc())
            .field("gpio2", &self.gpio2())
            .field("syscfg2", &self.syscfg2())
            .field("btim4", &self.btim4())
            .field("btim3", &self.btim3())
            .field("ptc2", &self.ptc2())
            .field("secu2", &self.secu2())
            .field("usart5", &self.usart5())
            .field("usart4", &self.usart4())
            .field("patch", &self.patch())
            .field("pinmux2", &self.pinmux2())
            .field("mailbox2", &self.mailbox2())
            .field("dmac2", &self.dmac2())
            .finish()
    }
}
impl W {
    ///Bit 1 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn dmac2(&mut self) -> Dmac2W<ESR1rs> {
        Dmac2W::new(self, 1)
    }
    ///Bit 2 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn mailbox2(&mut self) -> Mailbox2W<ESR1rs> {
        Mailbox2W::new(self, 2)
    }
    ///Bit 3 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn pinmux2(&mut self) -> Pinmux2W<ESR1rs> {
        Pinmux2W::new(self, 3)
    }
    ///Bit 4 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn patch(&mut self) -> PatchW<ESR1rs> {
        PatchW::new(self, 4)
    }
    ///Bit 5 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn usart4(&mut self) -> Usart4W<ESR1rs> {
        Usart4W::new(self, 5)
    }
    ///Bit 6 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn usart5(&mut self) -> Usart5W<ESR1rs> {
        Usart5W::new(self, 6)
    }
    ///Bit 7 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn secu2(&mut self) -> Secu2W<ESR1rs> {
        Secu2W::new(self, 7)
    }
    ///Bit 8 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn ptc2(&mut self) -> Ptc2W<ESR1rs> {
        Ptc2W::new(self, 8)
    }
    ///Bit 9 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn btim3(&mut self) -> Btim3W<ESR1rs> {
        Btim3W::new(self, 9)
    }
    ///Bit 10 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn btim4(&mut self) -> Btim4W<ESR1rs> {
        Btim4W::new(self, 10)
    }
    ///Bit 15 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn syscfg2(&mut self) -> Syscfg2W<ESR1rs> {
        Syscfg2W::new(self, 15)
    }
    ///Bit 16 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn gpio2(&mut self) -> Gpio2W<ESR1rs> {
        Gpio2W::new(self, 16)
    }
    ///Bit 18 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn rfc(&mut self) -> RfcW<ESR1rs> {
        RfcW::new(self, 18)
    }
    ///Bit 19 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn phy(&mut self) -> PhyW<ESR1rs> {
        PhyW::new(self, 19)
    }
    ///Bit 20 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn mac(&mut self) -> MacW<ESR1rs> {
        MacW::new(self, 20)
    }
    ///Bit 21 - write 1 to set module enable, write 0 has no effect
    #[inline(always)]
    pub fn crc2(&mut self) -> Crc2W<ESR1rs> {
        Crc2W::new(self, 21)
    }
}
///Enable Set Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`esr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ESR1rs;
impl crate::RegisterSpec for ESR1rs {
    type Ux = u32;
}
///`read()` method returns [`esr1::R`](R) reader structure
impl crate::Readable for ESR1rs {}
///`write(|w| ..)` method takes [`esr1::W`](W) writer structure
impl crate::Writable for ESR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ESR1 to value 0
impl crate::Resettable for ESR1rs {
    const RESET_VALUE: u32 = 0;
}
