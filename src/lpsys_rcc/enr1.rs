///Register `ENR1` reader
pub type R = crate::R<ENR1rs>;
///Register `ENR1` writer
pub type W = crate::W<ENR1rs>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAC2` reader - 0 - disabled; 1 - enabled
pub type Dmac2R = crate::BitReader;
///Field `DMAC2` writer - 0 - disabled; 1 - enabled
pub type Dmac2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MAILBOX2` reader - 0 - disabled; 1 - enabled
pub type Mailbox2R = crate::BitReader;
///Field `MAILBOX2` writer - 0 - disabled; 1 - enabled
pub type Mailbox2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINMUX2` reader - 0 - disabled; 1 - enabled
pub type Pinmux2R = crate::BitReader;
///Field `PINMUX2` writer - 0 - disabled; 1 - enabled
pub type Pinmux2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PATCH` reader - 0 - disabled; 1 - enabled
pub type PatchR = crate::BitReader;
///Field `PATCH` writer - 0 - disabled; 1 - enabled
pub type PatchW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4` reader - 0 - disabled; 1 - enabled
pub type Usart4R = crate::BitReader;
///Field `USART4` writer - 0 - disabled; 1 - enabled
pub type Usart4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART5` reader - 0 - disabled; 1 - enabled
pub type Usart5R = crate::BitReader;
///Field `USART5` writer - 0 - disabled; 1 - enabled
pub type Usart5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECU2` reader - 0 - disabled; 1 - enabled
pub type Secu2R = crate::BitReader;
///Field `SECU2` writer - 0 - disabled; 1 - enabled
pub type Secu2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTC2` reader - 0 - disabled; 1 - enabled
pub type Ptc2R = crate::BitReader;
///Field `PTC2` writer - 0 - disabled; 1 - enabled
pub type Ptc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTIM3` reader - 0 - disabled; 1 - enabled
pub type Btim3R = crate::BitReader;
///Field `BTIM3` writer - 0 - disabled; 1 - enabled
pub type Btim3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTIM4` reader - 0 - disabled; 1 - enabled
pub type Btim4R = crate::BitReader;
///Field `BTIM4` writer - 0 - disabled; 1 - enabled
pub type Btim4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SYSCFG2` reader - 0 - disabled; 1 - enabled
pub type Syscfg2R = crate::BitReader;
///Field `SYSCFG2` writer - 0 - disabled; 1 - enabled
pub type Syscfg2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO2` reader - 0 - disabled; 1 - enabled
pub type Gpio2R = crate::BitReader;
///Field `GPIO2` writer - 0 - disabled; 1 - enabled
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC` reader - 0 - disabled; 1 - enabled
pub type RfcR = crate::BitReader;
///Field `RFC` writer - 0 - disabled; 1 - enabled
pub type RfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHY` reader - 0 - disabled; 1 - enabled
pub type PhyR = crate::BitReader;
///Field `PHY` writer - 0 - disabled; 1 - enabled
pub type PhyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MAC` reader - 0 - disabled; 1 - enabled
pub type MacR = crate::BitReader;
///Field `MAC` writer - 0 - disabled; 1 - enabled
pub type MacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC2` reader - 0 - disabled; 1 - enabled
pub type Crc2R = crate::BitReader;
///Field `CRC2` writer - 0 - disabled; 1 - enabled
pub type Crc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn dmac2(&self) -> Dmac2R {
        Dmac2R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn mailbox2(&self) -> Mailbox2R {
        Mailbox2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn pinmux2(&self) -> Pinmux2R {
        Pinmux2R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn patch(&self) -> PatchR {
        PatchR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn usart5(&self) -> Usart5R {
        Usart5R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn secu2(&self) -> Secu2R {
        Secu2R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn ptc2(&self) -> Ptc2R {
        Ptc2R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn btim3(&self) -> Btim3R {
        Btim3R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn btim4(&self) -> Btim4R {
        Btim4R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:14
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn syscfg2(&self) -> Syscfg2R {
        Syscfg2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn gpio2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn rfc(&self) -> RfcR {
        RfcR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn phy(&self) -> PhyR {
        PhyR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn mac(&self) -> MacR {
        MacR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn crc2(&self) -> Crc2R {
        Crc2R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENR1")
            .field("rsvd", &self.rsvd())
            .field("crc2", &self.crc2())
            .field("mac", &self.mac())
            .field("phy", &self.phy())
            .field("rfc", &self.rfc())
            .field("rsvd2", &self.rsvd2())
            .field("gpio2", &self.gpio2())
            .field("syscfg2", &self.syscfg2())
            .field("rsvd3", &self.rsvd3())
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
            .field("rsvd4", &self.rsvd4())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<ENR1rs> {
        Rsvd4W::new(self, 0)
    }
    ///Bit 1 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn dmac2(&mut self) -> Dmac2W<ENR1rs> {
        Dmac2W::new(self, 1)
    }
    ///Bit 2 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn mailbox2(&mut self) -> Mailbox2W<ENR1rs> {
        Mailbox2W::new(self, 2)
    }
    ///Bit 3 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn pinmux2(&mut self) -> Pinmux2W<ENR1rs> {
        Pinmux2W::new(self, 3)
    }
    ///Bit 4 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn patch(&mut self) -> PatchW<ENR1rs> {
        PatchW::new(self, 4)
    }
    ///Bit 5 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn usart4(&mut self) -> Usart4W<ENR1rs> {
        Usart4W::new(self, 5)
    }
    ///Bit 6 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn usart5(&mut self) -> Usart5W<ENR1rs> {
        Usart5W::new(self, 6)
    }
    ///Bit 7 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn secu2(&mut self) -> Secu2W<ENR1rs> {
        Secu2W::new(self, 7)
    }
    ///Bit 8 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn ptc2(&mut self) -> Ptc2W<ENR1rs> {
        Ptc2W::new(self, 8)
    }
    ///Bit 9 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn btim3(&mut self) -> Btim3W<ENR1rs> {
        Btim3W::new(self, 9)
    }
    ///Bit 10 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn btim4(&mut self) -> Btim4W<ENR1rs> {
        Btim4W::new(self, 10)
    }
    ///Bits 11:14
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<ENR1rs> {
        Rsvd3W::new(self, 11)
    }
    ///Bit 15 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn syscfg2(&mut self) -> Syscfg2W<ENR1rs> {
        Syscfg2W::new(self, 15)
    }
    ///Bit 16 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn gpio2(&mut self) -> Gpio2W<ENR1rs> {
        Gpio2W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ENR1rs> {
        Rsvd2W::new(self, 17)
    }
    ///Bit 18 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn rfc(&mut self) -> RfcW<ENR1rs> {
        RfcW::new(self, 18)
    }
    ///Bit 19 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn phy(&mut self) -> PhyW<ENR1rs> {
        PhyW::new(self, 19)
    }
    ///Bit 20 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn mac(&mut self) -> MacW<ENR1rs> {
        MacW::new(self, 20)
    }
    ///Bit 21 - 0 - disabled; 1 - enabled
    #[inline(always)]
    pub fn crc2(&mut self) -> Crc2W<ENR1rs> {
        Crc2W::new(self, 21)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ENR1rs> {
        RsvdW::new(self, 22)
    }
}
///Enable Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ENR1rs;
impl crate::RegisterSpec for ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`enr1::R`](R) reader structure
impl crate::Readable for ENR1rs {}
///`write(|w| ..)` method takes [`enr1::W`](W) writer structure
impl crate::Writable for ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENR1 to value 0
impl crate::Resettable for ENR1rs {
    const RESET_VALUE: u32 = 0;
}
