///Register `STAT` reader
pub type R = crate::R<STATrs>;
///Register `STAT` writer
pub type W = crate::W<STATrs>;
///Field `SEED_GEN_BUSY` reader - random seed engine busy flag
pub type SeedGenBusyR = crate::BitReader;
///Field `SEED_GEN_BUSY` writer - random seed engine busy flag
pub type SeedGenBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEED_VALID` reader - random seed valid flag
pub type SeedValidR = crate::BitReader;
///Field `SEED_VALID` writer - random seed valid flag
pub type SeedValidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAND_NUM_GEN_BUSY` reader - random number engine busy flag
pub type RandNumGenBusyR = crate::BitReader;
///Field `RAND_NUM_GEN_BUSY` writer - random number engine busy flag
pub type RandNumGenBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAND_NUM_VALID` reader - random number valid flag
pub type RandNumValidR = crate::BitReader;
///Field `RAND_NUM_VALID` writer - random number valid flag
pub type RandNumValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - random seed engine busy flag
    #[inline(always)]
    pub fn seed_gen_busy(&self) -> SeedGenBusyR {
        SeedGenBusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - random seed valid flag
    #[inline(always)]
    pub fn seed_valid(&self) -> SeedValidR {
        SeedValidR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - random number engine busy flag
    #[inline(always)]
    pub fn rand_num_gen_busy(&self) -> RandNumGenBusyR {
        RandNumGenBusyR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - random number valid flag
    #[inline(always)]
    pub fn rand_num_valid(&self) -> RandNumValidR {
        RandNumValidR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("rand_num_valid", &self.rand_num_valid())
            .field("rand_num_gen_busy", &self.rand_num_gen_busy())
            .field("seed_valid", &self.seed_valid())
            .field("seed_gen_busy", &self.seed_gen_busy())
            .finish()
    }
}
impl W {
    ///Bit 0 - random seed engine busy flag
    #[inline(always)]
    pub fn seed_gen_busy(&mut self) -> SeedGenBusyW<STATrs> {
        SeedGenBusyW::new(self, 0)
    }
    ///Bit 1 - random seed valid flag
    #[inline(always)]
    pub fn seed_valid(&mut self) -> SeedValidW<STATrs> {
        SeedValidW::new(self, 1)
    }
    ///Bit 2 - random number engine busy flag
    #[inline(always)]
    pub fn rand_num_gen_busy(&mut self) -> RandNumGenBusyW<STATrs> {
        RandNumGenBusyW::new(self, 2)
    }
    ///Bit 3 - random number valid flag
    #[inline(always)]
    pub fn rand_num_valid(&mut self) -> RandNumValidW<STATrs> {
        RandNumValidW::new(self, 3)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATrs;
impl crate::RegisterSpec for STATrs {
    type Ux = u32;
}
///`read()` method returns [`stat::R`](R) reader structure
impl crate::Readable for STATrs {}
///`write(|w| ..)` method takes [`stat::W`](W) writer structure
impl crate::Writable for STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STAT to value 0
impl crate::Resettable for STATrs {
    const RESET_VALUE: u32 = 0;
}
