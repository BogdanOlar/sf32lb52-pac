///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `GEN_SEED_START` reader - write 1 to trigger the random seed generation engine
pub type GenSeedStartR = crate::BitReader;
///Field `GEN_SEED_START` writer - write 1 to trigger the random seed generation engine
pub type GenSeedStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GEN_RAND_NUM_START` reader - write 1 to trigger the random number generation engine
pub type GenRandNumStartR = crate::BitReader;
///Field `GEN_RAND_NUM_START` writer - write 1 to trigger the random number generation engine
pub type GenRandNumStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GEN_SEED_STOP` reader - Set 1 to stop random seed generation. This will reset the random seed generation engine. After release the stop bit, user should write 1 to gen_seed_start to trigger the random seed engine.
pub type GenSeedStopR = crate::BitReader;
///Field `GEN_SEED_STOP` writer - Set 1 to stop random seed generation. This will reset the random seed generation engine. After release the stop bit, user should write 1 to gen_seed_start to trigger the random seed engine.
pub type GenSeedStopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GEN_RAND_NUM_STOP` reader - Set 1 to stop random number generation and update. This will reset the random number generation engine. After release the stop bit, user should write 1 to gen_rand_num_start to trigger the random number engine.
pub type GenRandNumStopR = crate::BitReader;
///Field `GEN_RAND_NUM_STOP` writer - Set 1 to stop random number generation and update. This will reset the random number generation engine. After release the stop bit, user should write 1 to gen_rand_num_start to trigger the random number engine.
pub type GenRandNumStopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GEN_RAND_NUM_SUSPEND` reader - Set 1 to suspend random number generation and update. Set 0 to recover the process.
pub type GenRandNumSuspendR = crate::BitReader;
///Field `GEN_RAND_NUM_SUSPEND` writer - Set 1 to suspend random number generation and update. Set 0 to recover the process.
pub type GenRandNumSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - write 1 to trigger the random seed generation engine
    #[inline(always)]
    pub fn gen_seed_start(&self) -> GenSeedStartR {
        GenSeedStartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - write 1 to trigger the random number generation engine
    #[inline(always)]
    pub fn gen_rand_num_start(&self) -> GenRandNumStartR {
        GenRandNumStartR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set 1 to stop random seed generation. This will reset the random seed generation engine. After release the stop bit, user should write 1 to gen_seed_start to trigger the random seed engine.
    #[inline(always)]
    pub fn gen_seed_stop(&self) -> GenSeedStopR {
        GenSeedStopR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to stop random number generation and update. This will reset the random number generation engine. After release the stop bit, user should write 1 to gen_rand_num_start to trigger the random number engine.
    #[inline(always)]
    pub fn gen_rand_num_stop(&self) -> GenRandNumStopR {
        GenRandNumStopR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set 1 to suspend random number generation and update. Set 0 to recover the process.
    #[inline(always)]
    pub fn gen_rand_num_suspend(&self) -> GenRandNumSuspendR {
        GenRandNumSuspendR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("gen_rand_num_suspend", &self.gen_rand_num_suspend())
            .field("gen_rand_num_stop", &self.gen_rand_num_stop())
            .field("gen_seed_stop", &self.gen_seed_stop())
            .field("gen_rand_num_start", &self.gen_rand_num_start())
            .field("gen_seed_start", &self.gen_seed_start())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to trigger the random seed generation engine
    #[inline(always)]
    pub fn gen_seed_start(&mut self) -> GenSeedStartW<CTRLrs> {
        GenSeedStartW::new(self, 0)
    }
    ///Bit 1 - write 1 to trigger the random number generation engine
    #[inline(always)]
    pub fn gen_rand_num_start(&mut self) -> GenRandNumStartW<CTRLrs> {
        GenRandNumStartW::new(self, 1)
    }
    ///Bit 2 - Set 1 to stop random seed generation. This will reset the random seed generation engine. After release the stop bit, user should write 1 to gen_seed_start to trigger the random seed engine.
    #[inline(always)]
    pub fn gen_seed_stop(&mut self) -> GenSeedStopW<CTRLrs> {
        GenSeedStopW::new(self, 2)
    }
    ///Bit 3 - Set 1 to stop random number generation and update. This will reset the random number generation engine. After release the stop bit, user should write 1 to gen_rand_num_start to trigger the random number engine.
    #[inline(always)]
    pub fn gen_rand_num_stop(&mut self) -> GenRandNumStopW<CTRLrs> {
        GenRandNumStopW::new(self, 3)
    }
    ///Bit 4 - Set 1 to suspend random number generation and update. Set 0 to recover the process.
    #[inline(always)]
    pub fn gen_rand_num_suspend(&mut self) -> GenRandNumSuspendW<CTRLrs> {
        GenRandNumSuspendW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
