///Register `PMU_TR` reader
pub type R = crate::R<PMU_TRrs>;
///Register `PMU_TR` writer
pub type W = crate::W<PMU_TRrs>;
///Field `PMU_DC_TR` reader - test point select
pub type PmuDcTrR = crate::FieldReader;
///Field `PMU_DC_TR` writer - test point select
pub type PmuDcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PMU_DC_MR` reader - macro select
pub type PmuDcMrR = crate::FieldReader;
///Field `PMU_DC_MR` writer - macro select
pub type PmuDcMrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:2 - test point select
    #[inline(always)]
    pub fn pmu_dc_tr(&self) -> PmuDcTrR {
        PmuDcTrR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - macro select
    #[inline(always)]
    pub fn pmu_dc_mr(&self) -> PmuDcMrR {
        PmuDcMrR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_TR")
            .field("rsvd", &self.rsvd())
            .field("pmu_dc_mr", &self.pmu_dc_mr())
            .field("pmu_dc_tr", &self.pmu_dc_tr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - test point select
    #[inline(always)]
    pub fn pmu_dc_tr(&mut self) -> PmuDcTrW<PMU_TRrs> {
        PmuDcTrW::new(self, 0)
    }
    ///Bits 3:5 - macro select
    #[inline(always)]
    pub fn pmu_dc_mr(&mut self) -> PmuDcMrW<PMU_TRrs> {
        PmuDcMrW::new(self, 3)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PMU_TRrs> {
        RsvdW::new(self, 6)
    }
}
///PMU Test Register
///
///You can [`read`](crate::Reg::read) this register and get [`pmu_tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PMU_TRrs;
impl crate::RegisterSpec for PMU_TRrs {
    type Ux = u32;
}
///`read()` method returns [`pmu_tr::R`](R) reader structure
impl crate::Readable for PMU_TRrs {}
///`write(|w| ..)` method takes [`pmu_tr::W`](W) writer structure
impl crate::Writable for PMU_TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PMU_TR to value 0
impl crate::Resettable for PMU_TRrs {
    const RESET_VALUE: u32 = 0;
}
