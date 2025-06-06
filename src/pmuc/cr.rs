///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `SEL_LPCLK` reader - LP clock for watchdog and FSM. 0 - LRC10, 1 - LRC32
pub type SelLpclkR = crate::BitReader;
///Field `SEL_LPCLK` writer - LP clock for watchdog and FSM. 0 - LRC10, 1 - LRC32
pub type SelLpclkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HIBER_EN` reader - Write 1 to enter hibernate mode; write 0 to clear when exit from hibernate
pub type HiberEnR = crate::BitReader;
///Field `HIBER_EN` writer - Write 1 to enter hibernate mode; write 0 to clear when exit from hibernate
pub type HiberEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REBOOT` reader - Write 1 to reboot; write 0 to clear after boot up
pub type RebootR = crate::BitReader;
///Field `REBOOT` writer - Write 1 to reboot; write 0 to clear after boot up
pub type RebootW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN_RET` reader - If set to 1, IO retained during hibernate mode; otherwise, high-Z
pub type PinRetR = crate::BitReader;
///Field `PIN_RET` writer - If set to 1, IO retained during hibernate mode; otherwise, high-Z
pub type PinRetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIN0_MODE` reader - 4/6 - both edge (high-active detection), 5/7 - both edge (low-active detection)
pub type Pin0ModeR = crate::FieldReader;
///Field `PIN0_MODE` writer - 4/6 - both edge (high-active detection), 5/7 - both edge (low-active detection)
pub type Pin0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN1_MODE` reader - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge
pub type Pin1ModeR = crate::FieldReader;
///Field `PIN1_MODE` writer - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge
pub type Pin1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN0_SEL` reader - select one out of PA\[44:24\]. 0 - PA24, 1 - PA25, 20 - PA44, etc.
pub type Pin0SelR = crate::FieldReader;
///Field `PIN0_SEL` writer - select one out of PA\[44:24\]. 0 - PA24, 1 - PA25, 20 - PA44, etc.
pub type Pin0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PIN1_SEL` reader -
pub type Pin1SelR = crate::FieldReader;
///Field `PIN1_SEL` writer -
pub type Pin1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0 - LP clock for watchdog and FSM. 0 - LRC10, 1 - LRC32
    #[inline(always)]
    pub fn sel_lpclk(&self) -> SelLpclkR {
        SelLpclkR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write 1 to enter hibernate mode; write 0 to clear when exit from hibernate
    #[inline(always)]
    pub fn hiber_en(&self) -> HiberEnR {
        HiberEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write 1 to reboot; write 0 to clear after boot up
    #[inline(always)]
    pub fn reboot(&self) -> RebootR {
        RebootR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - If set to 1, IO retained during hibernate mode; otherwise, high-Z
    #[inline(always)]
    pub fn pin_ret(&self) -> PinRetR {
        PinRetR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - 4/6 - both edge (high-active detection), 5/7 - both edge (low-active detection)
    #[inline(always)]
    pub fn pin0_mode(&self) -> Pin0ModeR {
        Pin0ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge
    #[inline(always)]
    pub fn pin1_mode(&self) -> Pin1ModeR {
        Pin1ModeR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:14 - select one out of PA\[44:24\]. 0 - PA24, 1 - PA25, 20 - PA44, etc.
    #[inline(always)]
    pub fn pin0_sel(&self) -> Pin0SelR {
        Pin0SelR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19
    #[inline(always)]
    pub fn pin1_sel(&self) -> Pin1SelR {
        Pin1SelR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rsvd", &self.rsvd())
            .field("pin1_sel", &self.pin1_sel())
            .field("pin0_sel", &self.pin0_sel())
            .field("pin1_mode", &self.pin1_mode())
            .field("pin0_mode", &self.pin0_mode())
            .field("pin_ret", &self.pin_ret())
            .field("reboot", &self.reboot())
            .field("hiber_en", &self.hiber_en())
            .field("sel_lpclk", &self.sel_lpclk())
            .finish()
    }
}
impl W {
    ///Bit 0 - LP clock for watchdog and FSM. 0 - LRC10, 1 - LRC32
    #[inline(always)]
    pub fn sel_lpclk(&mut self) -> SelLpclkW<CRrs> {
        SelLpclkW::new(self, 0)
    }
    ///Bit 1 - Write 1 to enter hibernate mode; write 0 to clear when exit from hibernate
    #[inline(always)]
    pub fn hiber_en(&mut self) -> HiberEnW<CRrs> {
        HiberEnW::new(self, 1)
    }
    ///Bit 2 - Write 1 to reboot; write 0 to clear after boot up
    #[inline(always)]
    pub fn reboot(&mut self) -> RebootW<CRrs> {
        RebootW::new(self, 2)
    }
    ///Bit 3 - If set to 1, IO retained during hibernate mode; otherwise, high-Z
    #[inline(always)]
    pub fn pin_ret(&mut self) -> PinRetW<CRrs> {
        PinRetW::new(self, 3)
    }
    ///Bits 4:6 - 4/6 - both edge (high-active detection), 5/7 - both edge (low-active detection)
    #[inline(always)]
    pub fn pin0_mode(&mut self) -> Pin0ModeW<CRrs> {
        Pin0ModeW::new(self, 4)
    }
    ///Bits 7:9 - 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge
    #[inline(always)]
    pub fn pin1_mode(&mut self) -> Pin1ModeW<CRrs> {
        Pin1ModeW::new(self, 7)
    }
    ///Bits 10:14 - select one out of PA\[44:24\]. 0 - PA24, 1 - PA25, 20 - PA44, etc.
    #[inline(always)]
    pub fn pin0_sel(&mut self) -> Pin0SelW<CRrs> {
        Pin0SelW::new(self, 10)
    }
    ///Bits 15:19
    #[inline(always)]
    pub fn pin1_sel(&mut self) -> Pin1SelW<CRrs> {
        Pin1SelW::new(self, 15)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CRrs> {
        RsvdW::new(self, 20)
    }
}
///Control Register
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
