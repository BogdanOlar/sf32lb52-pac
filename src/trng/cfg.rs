///Register `CFG` reader
pub type R = crate::R<CFGrs>;
///Register `CFG` writer
pub type W = crate::W<CFGrs>;
///Field `AUTO_CLOCK_ENABLE` reader - auto clock gating enable
pub type AutoClockEnableR = crate::BitReader;
///Field `AUTO_CLOCK_ENABLE` writer - auto clock gating enable
pub type AutoClockEnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USE_EXT_SEED` reader - set 1 to use external seed to generate random number
pub type UseExtSeedR = crate::BitReader;
///Field `USE_EXT_SEED` writer - set 1 to use external seed to generate random number
pub type UseExtSeedW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `REJECT_THRESHOLD` reader - random seed internal VN corrector check threshold
pub type RejectThresholdR = crate::FieldReader;
///Field `REJECT_THRESHOLD` writer - random seed internal VN corrector check threshold
pub type RejectThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - auto clock gating enable
    #[inline(always)]
    pub fn auto_clock_enable(&self) -> AutoClockEnableR {
        AutoClockEnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - set 1 to use external seed to generate random number
    #[inline(always)]
    pub fn use_ext_seed(&self) -> UseExtSeedR {
        UseExtSeedR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    ///Bits 8:15 - random seed internal VN corrector check threshold
    #[inline(always)]
    pub fn reject_threshold(&self) -> RejectThresholdR {
        RejectThresholdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("rsvd", &self.rsvd())
            .field("reject_threshold", &self.reject_threshold())
            .field("rsvd2", &self.rsvd2())
            .field("use_ext_seed", &self.use_ext_seed())
            .field("auto_clock_enable", &self.auto_clock_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - auto clock gating enable
    #[inline(always)]
    pub fn auto_clock_enable(&mut self) -> AutoClockEnableW<CFGrs> {
        AutoClockEnableW::new(self, 0)
    }
    ///Bit 1 - set 1 to use external seed to generate random number
    #[inline(always)]
    pub fn use_ext_seed(&mut self) -> UseExtSeedW<CFGrs> {
        UseExtSeedW::new(self, 1)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CFGrs> {
        Rsvd2W::new(self, 2)
    }
    ///Bits 8:15 - random seed internal VN corrector check threshold
    #[inline(always)]
    pub fn reject_threshold(&mut self) -> RejectThresholdW<CFGrs> {
        RejectThresholdW::new(self, 8)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CFGrs> {
        RsvdW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFGrs;
impl crate::RegisterSpec for CFGrs {
    type Ux = u32;
}
///`read()` method returns [`cfg::R`](R) reader structure
impl crate::Readable for CFGrs {}
///`write(|w| ..)` method takes [`cfg::W`](W) writer structure
impl crate::Writable for CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG to value 0
impl crate::Resettable for CFGrs {
    const RESET_VALUE: u32 = 0;
}
