///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `PIN0_MODE` reader - mode for wakeup PIN0 (PA24) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
pub type Pin0ModeR = crate::FieldReader;
///Field `PIN0_MODE` writer - mode for wakeup PIN0 (PA24) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
pub type Pin0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN1_MODE` reader - mode for wakeup PIN1 (PA25)
pub type Pin1ModeR = crate::FieldReader;
///Field `PIN1_MODE` writer - mode for wakeup PIN1 (PA25)
pub type Pin1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN2_MODE` reader - mode for wakeup PIN2 (PA26)
pub type Pin2ModeR = crate::FieldReader;
///Field `PIN2_MODE` writer - mode for wakeup PIN2 (PA26)
pub type Pin2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN3_MODE` reader - mode for wakeup PIN3 (PA27)
pub type Pin3ModeR = crate::FieldReader;
///Field `PIN3_MODE` writer - mode for wakeup PIN3 (PA27)
pub type Pin3ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `PINOUT_SEL0` reader - for debug only
pub type PinoutSel0R = crate::FieldReader;
///Field `PINOUT_SEL0` writer - for debug only
pub type PinoutSel0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PINOUT_SEL1` reader - for debug only
pub type PinoutSel1R = crate::FieldReader;
///Field `PINOUT_SEL1` writer - for debug only
pub type PinoutSel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GTIM_EN` reader - Enable global timer
pub type GtimEnR = crate::BitReader;
///Field `GTIM_EN` writer - Enable global timer
pub type GtimEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - mode for wakeup PIN0 (PA24) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
    #[inline(always)]
    pub fn pin0_mode(&self) -> Pin0ModeR {
        Pin0ModeR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - mode for wakeup PIN1 (PA25)
    #[inline(always)]
    pub fn pin1_mode(&self) -> Pin1ModeR {
        Pin1ModeR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - mode for wakeup PIN2 (PA26)
    #[inline(always)]
    pub fn pin2_mode(&self) -> Pin2ModeR {
        Pin2ModeR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - mode for wakeup PIN3 (PA27)
    #[inline(always)]
    pub fn pin3_mode(&self) -> Pin3ModeR {
        Pin3ModeR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:24
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 12) & 0x1fff) as u16)
    }
    ///Bits 25:27 - for debug only
    #[inline(always)]
    pub fn pinout_sel0(&self) -> PinoutSel0R {
        PinoutSel0R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:30 - for debug only
    #[inline(always)]
    pub fn pinout_sel1(&self) -> PinoutSel1R {
        PinoutSel1R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - Enable global timer
    #[inline(always)]
    pub fn gtim_en(&self) -> GtimEnR {
        GtimEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("gtim_en", &self.gtim_en())
            .field("pinout_sel1", &self.pinout_sel1())
            .field("pinout_sel0", &self.pinout_sel0())
            .field("rsvd", &self.rsvd())
            .field("pin3_mode", &self.pin3_mode())
            .field("pin2_mode", &self.pin2_mode())
            .field("pin1_mode", &self.pin1_mode())
            .field("pin0_mode", &self.pin0_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - mode for wakeup PIN0 (PA24) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
    #[inline(always)]
    pub fn pin0_mode(&mut self) -> Pin0ModeW<CR1rs> {
        Pin0ModeW::new(self, 0)
    }
    ///Bits 3:5 - mode for wakeup PIN1 (PA25)
    #[inline(always)]
    pub fn pin1_mode(&mut self) -> Pin1ModeW<CR1rs> {
        Pin1ModeW::new(self, 3)
    }
    ///Bits 6:8 - mode for wakeup PIN2 (PA26)
    #[inline(always)]
    pub fn pin2_mode(&mut self) -> Pin2ModeW<CR1rs> {
        Pin2ModeW::new(self, 6)
    }
    ///Bits 9:11 - mode for wakeup PIN3 (PA27)
    #[inline(always)]
    pub fn pin3_mode(&mut self) -> Pin3ModeW<CR1rs> {
        Pin3ModeW::new(self, 9)
    }
    ///Bits 12:24
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CR1rs> {
        RsvdW::new(self, 12)
    }
    ///Bits 25:27 - for debug only
    #[inline(always)]
    pub fn pinout_sel0(&mut self) -> PinoutSel0W<CR1rs> {
        PinoutSel0W::new(self, 25)
    }
    ///Bits 28:30 - for debug only
    #[inline(always)]
    pub fn pinout_sel1(&mut self) -> PinoutSel1W<CR1rs> {
        PinoutSel1W::new(self, 28)
    }
    ///Bit 31 - Enable global timer
    #[inline(always)]
    pub fn gtim_en(&mut self) -> GtimEnW<CR1rs> {
        GtimEnW::new(self, 31)
    }
}
///Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
