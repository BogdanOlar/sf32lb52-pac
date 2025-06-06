///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `PIN16_MODE` reader - mode for wakeup PIN16 (PA40) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
pub type Pin16ModeR = crate::FieldReader;
///Field `PIN16_MODE` writer - mode for wakeup PIN16 (PA40) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
pub type Pin16ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN17_MODE` reader - mode for wakeup PIN17 (PA41)
pub type Pin17ModeR = crate::FieldReader;
///Field `PIN17_MODE` writer - mode for wakeup PIN17 (PA41)
pub type Pin17ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN18_MODE` reader - mode for wakeup PIN18 (PA42)
pub type Pin18ModeR = crate::FieldReader;
///Field `PIN18_MODE` writer - mode for wakeup PIN18 (PA42)
pub type Pin18ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN19_MODE` reader - mode for wakeup PIN19 (PA43)
pub type Pin19ModeR = crate::FieldReader;
///Field `PIN19_MODE` writer - mode for wakeup PIN19 (PA43)
pub type Pin19ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PIN20_MODE` reader - mode for wakeup PIN20 (PA44)
pub type Pin20ModeR = crate::FieldReader;
///Field `PIN20_MODE` writer - mode for wakeup PIN20 (PA44)
pub type Pin20ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - mode for wakeup PIN16 (PA40) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
    #[inline(always)]
    pub fn pin16_mode(&self) -> Pin16ModeR {
        Pin16ModeR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - mode for wakeup PIN17 (PA41)
    #[inline(always)]
    pub fn pin17_mode(&self) -> Pin17ModeR {
        Pin17ModeR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - mode for wakeup PIN18 (PA42)
    #[inline(always)]
    pub fn pin18_mode(&self) -> Pin18ModeR {
        Pin18ModeR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - mode for wakeup PIN19 (PA43)
    #[inline(always)]
    pub fn pin19_mode(&self) -> Pin19ModeR {
        Pin19ModeR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - mode for wakeup PIN20 (PA44)
    #[inline(always)]
    pub fn pin20_mode(&self) -> Pin20ModeR {
        Pin20ModeR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("pin20_mode", &self.pin20_mode())
            .field("pin19_mode", &self.pin19_mode())
            .field("pin18_mode", &self.pin18_mode())
            .field("pin17_mode", &self.pin17_mode())
            .field("pin16_mode", &self.pin16_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - mode for wakeup PIN16 (PA40) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge
    #[inline(always)]
    pub fn pin16_mode(&mut self) -> Pin16ModeW<CR3rs> {
        Pin16ModeW::new(self, 0)
    }
    ///Bits 3:5 - mode for wakeup PIN17 (PA41)
    #[inline(always)]
    pub fn pin17_mode(&mut self) -> Pin17ModeW<CR3rs> {
        Pin17ModeW::new(self, 3)
    }
    ///Bits 6:8 - mode for wakeup PIN18 (PA42)
    #[inline(always)]
    pub fn pin18_mode(&mut self) -> Pin18ModeW<CR3rs> {
        Pin18ModeW::new(self, 6)
    }
    ///Bits 9:11 - mode for wakeup PIN19 (PA43)
    #[inline(always)]
    pub fn pin19_mode(&mut self) -> Pin19ModeW<CR3rs> {
        Pin19ModeW::new(self, 9)
    }
    ///Bits 12:14 - mode for wakeup PIN20 (PA44)
    #[inline(always)]
    pub fn pin20_mode(&mut self) -> Pin20ModeW<CR3rs> {
        Pin20ModeW::new(self, 12)
    }
}
///Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
