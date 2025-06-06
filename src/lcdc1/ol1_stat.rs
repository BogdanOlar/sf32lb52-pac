///Register `OL1_STAT` reader
pub type R = crate::R<OL1_STATrs>;
///Register `OL1_STAT` writer
pub type W = crate::W<OL1_STATrs>;
///Field `DONE_REQ` reader -
pub type DoneReqR = crate::BitReader;
///Field `DONE_REQ` writer -
pub type DoneReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_OUT` reader -
pub type PrefetchOutR = crate::BitReader;
///Field `PREFETCH_OUT` writer -
pub type PrefetchOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_READ` reader -
pub type PrefetchReadR = crate::FieldReader;
///Field `PREFETCH_READ` writer -
pub type PrefetchReadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATA_CONV` reader -
pub type DataConvR = crate::FieldReader;
///Field `DATA_CONV` writer -
pub type DataConvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PF_DF` reader -
pub type PfDfR = crate::FieldReader;
///Field `PF_DF` writer -
pub type PfDfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PF_PR` reader -
pub type PfPrR = crate::FieldReader;
///Field `PF_PR` writer -
pub type PfPrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn done_req(&self) -> DoneReqR {
        DoneReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_out(&self) -> PrefetchOutR {
        PrefetchOutR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn prefetch_read(&self) -> PrefetchReadR {
        PrefetchReadR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn data_conv(&self) -> DataConvR {
        DataConvR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn pf_df(&self) -> PfDfR {
        PfDfR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn pf_pr(&self) -> PfPrR {
        PfPrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OL1_STAT")
            .field("pf_pr", &self.pf_pr())
            .field("pf_df", &self.pf_df())
            .field("data_conv", &self.data_conv())
            .field("prefetch_read", &self.prefetch_read())
            .field("prefetch_out", &self.prefetch_out())
            .field("done_req", &self.done_req())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn done_req(&mut self) -> DoneReqW<OL1_STATrs> {
        DoneReqW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_out(&mut self) -> PrefetchOutW<OL1_STATrs> {
        PrefetchOutW::new(self, 1)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn prefetch_read(&mut self) -> PrefetchReadW<OL1_STATrs> {
        PrefetchReadW::new(self, 2)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn data_conv(&mut self) -> DataConvW<OL1_STATrs> {
        DataConvW::new(self, 4)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn pf_df(&mut self) -> PfDfW<OL1_STATrs> {
        PfDfW::new(self, 6)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn pf_pr(&mut self) -> PfPrW<OL1_STATrs> {
        PfPrW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ol1_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol1_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OL1_STATrs;
impl crate::RegisterSpec for OL1_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ol1_stat::R`](R) reader structure
impl crate::Readable for OL1_STATrs {}
///`write(|w| ..)` method takes [`ol1_stat::W`](W) writer structure
impl crate::Writable for OL1_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OL1_STAT to value 0
impl crate::Resettable for OL1_STATrs {
    const RESET_VALUE: u32 = 0;
}
