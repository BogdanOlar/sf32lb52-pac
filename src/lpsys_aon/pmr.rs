///Register `PMR` reader
pub type R = crate::R<PMRrs>;
///Register `PMR` writer
pub type W = crate::W<PMRrs>;
///Field `MODE` reader - Power Mode: 2'h0 - active/idle; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - Power Mode: 2'h0 - active/idle; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPUWAIT` reader - Stall CPU out of reset. Should be cleared before LCPU run
pub type CpuwaitR = crate::BitReader;
///Field `CPUWAIT` writer - Stall CPU out of reset. Should be cleared before LCPU run
pub type CpuwaitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_SLEEP` reader - Set 1 to force enter low power mode. Will be cleared automatically
pub type ForceSleepR = crate::BitReader;
///Field `FORCE_SLEEP` writer - Set 1 to force enter low power mode. Will be cleared automatically
pub type ForceSleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Power Mode: 2'h0 - active/idle; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Stall CPU out of reset. Should be cleared before LCPU run
    #[inline(always)]
    pub fn cpuwait(&self) -> CpuwaitR {
        CpuwaitR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 31 - Set 1 to force enter low power mode. Will be cleared automatically
    #[inline(always)]
    pub fn force_sleep(&self) -> ForceSleepR {
        ForceSleepR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMR")
            .field("force_sleep", &self.force_sleep())
            .field("cpuwait", &self.cpuwait())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Power Mode: 2'h0 - active/idle; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<PMRrs> {
        ModeW::new(self, 0)
    }
    ///Bit 2 - Stall CPU out of reset. Should be cleared before LCPU run
    #[inline(always)]
    pub fn cpuwait(&mut self) -> CpuwaitW<PMRrs> {
        CpuwaitW::new(self, 2)
    }
    ///Bit 31 - Set 1 to force enter low power mode. Will be cleared automatically
    #[inline(always)]
    pub fn force_sleep(&mut self) -> ForceSleepW<PMRrs> {
        ForceSleepW::new(self, 31)
    }
}
///Power Mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`pmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PMRrs;
impl crate::RegisterSpec for PMRrs {
    type Ux = u32;
}
///`read()` method returns [`pmr::R`](R) reader structure
impl crate::Readable for PMRrs {}
///`write(|w| ..)` method takes [`pmr::W`](W) writer structure
impl crate::Writable for PMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PMR to value 0
impl crate::Resettable for PMRrs {
    const RESET_VALUE: u32 = 0;
}
