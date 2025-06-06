///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PIN10_MODE` reader - mode for wakeup PIN10 (PA34) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
pub type Pin10ModeR = crate::FieldReader;
///Field `PIN10_MODE` writer - mode for wakeup PIN10 (PA34) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
pub type Pin10ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN11_MODE` reader - mode for wakeup PIN11 (PA35)
pub type Pin11ModeR = crate::FieldReader;
///Field `PIN11_MODE` writer - mode for wakeup PIN11 (PA35)
pub type Pin11ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN12_MODE` reader - mode for wakeup PIN12 (PA36)
pub type Pin12ModeR = crate::FieldReader;
///Field `PIN12_MODE` writer - mode for wakeup PIN12 (PA36)
pub type Pin12ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN13_MODE` reader - mode for wakeup PIN13 (PA37)
pub type Pin13ModeR = crate::FieldReader;
///Field `PIN13_MODE` writer - mode for wakeup PIN13 (PA37)
pub type Pin13ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN14_MODE` reader - mode for wakeup PIN14 (PA38)
pub type Pin14ModeR = crate::FieldReader;
///Field `PIN14_MODE` writer - mode for wakeup PIN14 (PA38)
pub type Pin14ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN15_MODE` reader - mode for wakeup PIN15 (PA39)
pub type Pin15ModeR = crate::FieldReader;
///Field `PIN15_MODE` writer - mode for wakeup PIN15 (PA39)
pub type Pin15ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:5
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:8 - mode for wakeup PIN10 (PA34) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
    #[inline(always)]
    pub fn pin10_mode(&self) -> Pin10ModeR {
        Pin10ModeR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - mode for wakeup PIN11 (PA35)
    #[inline(always)]
    pub fn pin11_mode(&self) -> Pin11ModeR {
        Pin11ModeR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - mode for wakeup PIN12 (PA36)
    #[inline(always)]
    pub fn pin12_mode(&self) -> Pin12ModeR {
        Pin12ModeR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - mode for wakeup PIN13 (PA37)
    #[inline(always)]
    pub fn pin13_mode(&self) -> Pin13ModeR {
        Pin13ModeR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - mode for wakeup PIN14 (PA38)
    #[inline(always)]
    pub fn pin14_mode(&self) -> Pin14ModeR {
        Pin14ModeR::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - mode for wakeup PIN15 (PA39)
    #[inline(always)]
    pub fn pin15_mode(&self) -> Pin15ModeR {
        Pin15ModeR::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("rsvd", &self.rsvd())
            .field("pin15_mode", &self.pin15_mode())
            .field("pin14_mode", &self.pin14_mode())
            .field("pin13_mode", &self.pin13_mode())
            .field("pin12_mode", &self.pin12_mode())
            .field("pin11_mode", &self.pin11_mode())
            .field("pin10_mode", &self.pin10_mode())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
impl W {
    ///Bits 0:5
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CR2rs> {
        Rsvd2W::new(self, 0)
    }
    ///Bits 6:8 - mode for wakeup PIN10 (PA34) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
    #[inline(always)]
    pub fn pin10_mode(&mut self) -> Pin10ModeW<CR2rs> {
        Pin10ModeW::new(self, 6)
    }
    ///Bits 9:11 - mode for wakeup PIN11 (PA35)
    #[inline(always)]
    pub fn pin11_mode(&mut self) -> Pin11ModeW<CR2rs> {
        Pin11ModeW::new(self, 9)
    }
    ///Bits 12:14 - mode for wakeup PIN12 (PA36)
    #[inline(always)]
    pub fn pin12_mode(&mut self) -> Pin12ModeW<CR2rs> {
        Pin12ModeW::new(self, 12)
    }
    ///Bits 15:17 - mode for wakeup PIN13 (PA37)
    #[inline(always)]
    pub fn pin13_mode(&mut self) -> Pin13ModeW<CR2rs> {
        Pin13ModeW::new(self, 15)
    }
    ///Bits 18:20 - mode for wakeup PIN14 (PA38)
    #[inline(always)]
    pub fn pin14_mode(&mut self) -> Pin14ModeW<CR2rs> {
        Pin14ModeW::new(self, 18)
    }
    ///Bits 21:23 - mode for wakeup PIN15 (PA39)
    #[inline(always)]
    pub fn pin15_mode(&mut self) -> Pin15ModeW<CR2rs> {
        Pin15ModeW::new(self, 21)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CR2rs> {
        RsvdW::new(self, 24)
    }
}
///Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
