///Register `OL_STAT` reader
pub type R = crate::R<OL_STATrs>;
///Register `OL_STAT` writer
pub type W = crate::W<OL_STATrs>;
///Field `DONE_REQ0` reader -
pub type DoneReq0R = crate::BitReader;
///Field `DONE_REQ0` writer -
pub type DoneReq0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_HOLD0` reader -
pub type PrefetchHold0R = crate::BitReader;
///Field `PREFETCH_HOLD0` writer -
pub type PrefetchHold0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_OUT0` reader -
pub type PrefetchOut0R = crate::BitReader;
///Field `PREFETCH_OUT0` writer -
pub type PrefetchOut0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_READ0` reader -
pub type PrefetchRead0R = crate::FieldReader;
///Field `PREFETCH_READ0` writer -
pub type PrefetchRead0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATA_CONV0` reader -
pub type DataConv0R = crate::FieldReader;
///Field `DATA_CONV0` writer -
pub type DataConv0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PF_DF0` reader -
pub type PfDf0R = crate::FieldReader;
///Field `PF_DF0` writer -
pub type PfDf0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PF_PR0` reader -
pub type PfPr0R = crate::FieldReader;
///Field `PF_PR0` writer -
pub type PfPr0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DONE_REQ1` reader -
pub type DoneReq1R = crate::BitReader;
///Field `DONE_REQ1` writer -
pub type DoneReq1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_HOLD1` reader -
pub type PrefetchHold1R = crate::BitReader;
///Field `PREFETCH_HOLD1` writer -
pub type PrefetchHold1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_OUT1` reader -
pub type PrefetchOut1R = crate::BitReader;
///Field `PREFETCH_OUT1` writer -
pub type PrefetchOut1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_READ1` reader -
pub type PrefetchRead1R = crate::FieldReader;
///Field `PREFETCH_READ1` writer -
pub type PrefetchRead1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATA_CONV1` reader -
pub type DataConv1R = crate::FieldReader;
///Field `DATA_CONV1` writer -
pub type DataConv1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PF_DF1` reader -
pub type PfDf1R = crate::FieldReader;
///Field `PF_DF1` writer -
pub type PfDf1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PF_PR1` reader -
pub type PfPr1R = crate::FieldReader;
///Field `PF_PR1` writer -
pub type PfPr1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn done_req0(&self) -> DoneReq0R {
        DoneReq0R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_hold0(&self) -> PrefetchHold0R {
        PrefetchHold0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn prefetch_out0(&self) -> PrefetchOut0R {
        PrefetchOut0R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn prefetch_read0(&self) -> PrefetchRead0R {
        PrefetchRead0R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6
    #[inline(always)]
    pub fn data_conv0(&self) -> DataConv0R {
        DataConv0R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bits 7:8
    #[inline(always)]
    pub fn pf_df0(&self) -> PfDf0R {
        PfDf0R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bits 9:11
    #[inline(always)]
    pub fn pf_pr0(&self) -> PfPr0R {
        PfPr0R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn done_req1(&self) -> DoneReq1R {
        DoneReq1R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn prefetch_hold1(&self) -> PrefetchHold1R {
        PrefetchHold1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn prefetch_out1(&self) -> PrefetchOut1R {
        PrefetchOut1R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20
    #[inline(always)]
    pub fn prefetch_read1(&self) -> PrefetchRead1R {
        PrefetchRead1R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bits 21:22
    #[inline(always)]
    pub fn data_conv1(&self) -> DataConv1R {
        DataConv1R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn pf_df1(&self) -> PfDf1R {
        PfDf1R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bits 25:27
    #[inline(always)]
    pub fn pf_pr1(&self) -> PfPr1R {
        PfPr1R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OL_STAT")
            .field("pf_pr1", &self.pf_pr1())
            .field("pf_df1", &self.pf_df1())
            .field("data_conv1", &self.data_conv1())
            .field("prefetch_read1", &self.prefetch_read1())
            .field("prefetch_out1", &self.prefetch_out1())
            .field("prefetch_hold1", &self.prefetch_hold1())
            .field("done_req1", &self.done_req1())
            .field("pf_pr0", &self.pf_pr0())
            .field("pf_df0", &self.pf_df0())
            .field("data_conv0", &self.data_conv0())
            .field("prefetch_read0", &self.prefetch_read0())
            .field("prefetch_out0", &self.prefetch_out0())
            .field("prefetch_hold0", &self.prefetch_hold0())
            .field("done_req0", &self.done_req0())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn done_req0(&mut self) -> DoneReq0W<OL_STATrs> {
        DoneReq0W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_hold0(&mut self) -> PrefetchHold0W<OL_STATrs> {
        PrefetchHold0W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn prefetch_out0(&mut self) -> PrefetchOut0W<OL_STATrs> {
        PrefetchOut0W::new(self, 2)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn prefetch_read0(&mut self) -> PrefetchRead0W<OL_STATrs> {
        PrefetchRead0W::new(self, 3)
    }
    ///Bits 5:6
    #[inline(always)]
    pub fn data_conv0(&mut self) -> DataConv0W<OL_STATrs> {
        DataConv0W::new(self, 5)
    }
    ///Bits 7:8
    #[inline(always)]
    pub fn pf_df0(&mut self) -> PfDf0W<OL_STATrs> {
        PfDf0W::new(self, 7)
    }
    ///Bits 9:11
    #[inline(always)]
    pub fn pf_pr0(&mut self) -> PfPr0W<OL_STATrs> {
        PfPr0W::new(self, 9)
    }
    ///Bit 16
    #[inline(always)]
    pub fn done_req1(&mut self) -> DoneReq1W<OL_STATrs> {
        DoneReq1W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn prefetch_hold1(&mut self) -> PrefetchHold1W<OL_STATrs> {
        PrefetchHold1W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn prefetch_out1(&mut self) -> PrefetchOut1W<OL_STATrs> {
        PrefetchOut1W::new(self, 18)
    }
    ///Bits 19:20
    #[inline(always)]
    pub fn prefetch_read1(&mut self) -> PrefetchRead1W<OL_STATrs> {
        PrefetchRead1W::new(self, 19)
    }
    ///Bits 21:22
    #[inline(always)]
    pub fn data_conv1(&mut self) -> DataConv1W<OL_STATrs> {
        DataConv1W::new(self, 21)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn pf_df1(&mut self) -> PfDf1W<OL_STATrs> {
        PfDf1W::new(self, 23)
    }
    ///Bits 25:27
    #[inline(always)]
    pub fn pf_pr1(&mut self) -> PfPr1W<OL_STATrs> {
        PfPr1W::new(self, 25)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ol_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OL_STATrs;
impl crate::RegisterSpec for OL_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ol_stat::R`](R) reader structure
impl crate::Readable for OL_STATrs {}
///`write(|w| ..)` method takes [`ol_stat::W`](W) writer structure
impl crate::Writable for OL_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OL_STAT to value 0
impl crate::Resettable for OL_STATrs {
    const RESET_VALUE: u32 = 0;
}
