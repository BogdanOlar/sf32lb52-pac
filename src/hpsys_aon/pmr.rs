///Register `PMR` reader
pub type R = crate::R<PMRrs>;
///Register `PMR` writer
pub type W = crate::W<PMRrs>;
///Field `MODE` reader - Power Mode: 2'h0 - active; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
pub type ModeR = crate::FieldReader;
///Field `MODE` writer - Power Mode: 2'h0 - active; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
///Field `FORCE_LCPU` reader - for debug only
pub type ForceLcpuR = crate::BitReader;
///Field `FORCE_LCPU` writer - for debug only
pub type ForceLcpuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_SLEEP` reader - Set 1 to force enter low power mode. Will be cleared automatically
pub type ForceSleepR = crate::BitReader;
///Field `FORCE_SLEEP` writer - Set 1 to force enter low power mode. Will be cleared automatically
pub type ForceSleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Power Mode: 2'h0 - active; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    ///Bits 2:29
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x0fff_ffff)
    }
    ///Bit 30 - for debug only
    #[inline(always)]
    pub fn force_lcpu(&self) -> ForceLcpuR {
        ForceLcpuR::new(((self.bits >> 30) & 1) != 0)
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
            .field("force_lcpu", &self.force_lcpu())
            .field("rsvd", &self.rsvd())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Power Mode: 2'h0 - active; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<PMRrs> {
        ModeW::new(self, 0)
    }
    ///Bits 2:29
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PMRrs> {
        RsvdW::new(self, 2)
    }
    ///Bit 30 - for debug only
    #[inline(always)]
    pub fn force_lcpu(&mut self) -> ForceLcpuW<PMRrs> {
        ForceLcpuW::new(self, 30)
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
