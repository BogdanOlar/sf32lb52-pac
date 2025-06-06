///Register `ML_STAT` reader
pub type R = crate::R<ML_STATrs>;
///Register `ML_STAT` writer
pub type W = crate::W<ML_STATrs>;
///Field `DONE_REQ` reader -
pub type DoneReqR = crate::BitReader;
///Field `DONE_REQ` writer -
pub type DoneReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_HOLD` reader -
pub type PrefetchHoldR = crate::BitReader;
///Field `PREFETCH_HOLD` writer -
pub type PrefetchHoldW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_OUT` reader -
pub type PrefetchOutR = crate::BitReader;
///Field `PREFETCH_OUT` writer -
pub type PrefetchOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_READ` reader -
pub type PrefetchReadR = crate::FieldReader;
///Field `PREFETCH_READ` writer -
pub type PrefetchReadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MF_DF` reader -
pub type MfDfR = crate::FieldReader;
///Field `MF_DF` writer -
pub type MfDfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MF_PR` reader -
pub type MfPrR = crate::FieldReader;
///Field `MF_PR` writer -
pub type MfPrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn done_req(&self) -> DoneReqR {
        DoneReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_hold(&self) -> PrefetchHoldR {
        PrefetchHoldR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn prefetch_out(&self) -> PrefetchOutR {
        PrefetchOutR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn prefetch_read(&self) -> PrefetchReadR {
        PrefetchReadR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6
    #[inline(always)]
    pub fn mf_df(&self) -> MfDfR {
        MfDfR::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn mf_pr(&self) -> MfPrR {
        MfPrR::new(((self.bits >> 7) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ML_STAT")
            .field("mf_pr", &self.mf_pr())
            .field("mf_df", &self.mf_df())
            .field("prefetch_read", &self.prefetch_read())
            .field("prefetch_out", &self.prefetch_out())
            .field("prefetch_hold", &self.prefetch_hold())
            .field("done_req", &self.done_req())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn done_req(&mut self) -> DoneReqW<ML_STATrs> {
        DoneReqW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_hold(&mut self) -> PrefetchHoldW<ML_STATrs> {
        PrefetchHoldW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn prefetch_out(&mut self) -> PrefetchOutW<ML_STATrs> {
        PrefetchOutW::new(self, 2)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn prefetch_read(&mut self) -> PrefetchReadW<ML_STATrs> {
        PrefetchReadW::new(self, 3)
    }
    ///Bits 5:6
    #[inline(always)]
    pub fn mf_df(&mut self) -> MfDfW<ML_STATrs> {
        MfDfW::new(self, 5)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn mf_pr(&mut self) -> MfPrW<ML_STATrs> {
        MfPrW::new(self, 7)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ml_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ml_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ML_STATrs;
impl crate::RegisterSpec for ML_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ml_stat::R`](R) reader structure
impl crate::Readable for ML_STATrs {}
///`write(|w| ..)` method takes [`ml_stat::W`](W) writer structure
impl crate::Writable for ML_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ML_STAT to value 0
impl crate::Resettable for ML_STATrs {
    const RESET_VALUE: u32 = 0;
}
