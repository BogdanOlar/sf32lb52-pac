///Register `IRQ` reader
pub type R = crate::R<IRQrs>;
///Register `IRQ` writer
pub type W = crate::W<IRQrs>;
///Field `SEED_GEN_DONE` reader - random seed generation done raw interrupt
pub type SeedGenDoneR = crate::BitReader;
///Field `SEED_GEN_DONE` writer - random seed generation done raw interrupt
pub type SeedGenDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAND_NUM_AVAIL` reader - random number available raw interrupt
pub type RandNumAvailR = crate::BitReader;
///Field `RAND_NUM_AVAIL` writer - random number available raw interrupt
pub type RandNumAvailW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRNG_LOCKUP` reader - prng lockup raw interrupt
pub type PrngLockupR = crate::BitReader;
///Field `PRNG_LOCKUP` writer - prng lockup raw interrupt
pub type PrngLockupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEED_GEN_DONE_MSK` reader - random seed generation done interrupt mask
pub type SeedGenDoneMskR = crate::BitReader;
///Field `SEED_GEN_DONE_MSK` writer - random seed generation done interrupt mask
pub type SeedGenDoneMskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAND_NUM_AVAIL_MSK` reader - random number available interrupt mask
pub type RandNumAvailMskR = crate::BitReader;
///Field `RAND_NUM_AVAIL_MSK` writer - random number available interrupt mask
pub type RandNumAvailMskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRNG_LOCKUP_MSK` reader - prng lockup interrupt mask
pub type PrngLockupMskR = crate::BitReader;
///Field `PRNG_LOCKUP_MSK` writer - prng lockup interrupt mask
pub type PrngLockupMskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - random seed generation done raw interrupt
    #[inline(always)]
    pub fn seed_gen_done(&self) -> SeedGenDoneR {
        SeedGenDoneR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - random number available raw interrupt
    #[inline(always)]
    pub fn rand_num_avail(&self) -> RandNumAvailR {
        RandNumAvailR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - prng lockup raw interrupt
    #[inline(always)]
    pub fn prng_lockup(&self) -> PrngLockupR {
        PrngLockupR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - random seed generation done interrupt mask
    #[inline(always)]
    pub fn seed_gen_done_msk(&self) -> SeedGenDoneMskR {
        SeedGenDoneMskR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - random number available interrupt mask
    #[inline(always)]
    pub fn rand_num_avail_msk(&self) -> RandNumAvailMskR {
        RandNumAvailMskR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - prng lockup interrupt mask
    #[inline(always)]
    pub fn prng_lockup_msk(&self) -> PrngLockupMskR {
        PrngLockupMskR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("prng_lockup_msk", &self.prng_lockup_msk())
            .field("rand_num_avail_msk", &self.rand_num_avail_msk())
            .field("seed_gen_done_msk", &self.seed_gen_done_msk())
            .field("prng_lockup", &self.prng_lockup())
            .field("rand_num_avail", &self.rand_num_avail())
            .field("seed_gen_done", &self.seed_gen_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - random seed generation done raw interrupt
    #[inline(always)]
    pub fn seed_gen_done(&mut self) -> SeedGenDoneW<IRQrs> {
        SeedGenDoneW::new(self, 0)
    }
    ///Bit 1 - random number available raw interrupt
    #[inline(always)]
    pub fn rand_num_avail(&mut self) -> RandNumAvailW<IRQrs> {
        RandNumAvailW::new(self, 1)
    }
    ///Bit 2 - prng lockup raw interrupt
    #[inline(always)]
    pub fn prng_lockup(&mut self) -> PrngLockupW<IRQrs> {
        PrngLockupW::new(self, 2)
    }
    ///Bit 16 - random seed generation done interrupt mask
    #[inline(always)]
    pub fn seed_gen_done_msk(&mut self) -> SeedGenDoneMskW<IRQrs> {
        SeedGenDoneMskW::new(self, 16)
    }
    ///Bit 17 - random number available interrupt mask
    #[inline(always)]
    pub fn rand_num_avail_msk(&mut self) -> RandNumAvailMskW<IRQrs> {
        RandNumAvailMskW::new(self, 17)
    }
    ///Bit 18 - prng lockup interrupt mask
    #[inline(always)]
    pub fn prng_lockup_msk(&mut self) -> PrngLockupMskW<IRQrs> {
        PrngLockupMskW::new(self, 18)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRQrs;
impl crate::RegisterSpec for IRQrs {
    type Ux = u32;
}
///`read()` method returns [`irq::R`](R) reader structure
impl crate::Readable for IRQrs {}
///`write(|w| ..)` method takes [`irq::W`](W) writer structure
impl crate::Writable for IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ to value 0x0007_0000
impl crate::Resettable for IRQrs {
    const RESET_VALUE: u32 = 0x0007_0000;
}
