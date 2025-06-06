///Register `OL0_STAT` reader
pub type R = crate::R<OL0_STATrs>;
///Register `OL0_STAT` writer
pub type W = crate::W<OL0_STATrs>;
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
///Field `SC_OUT` reader -
pub type ScOutR = crate::FieldReader;
///Field `SC_OUT` writer -
pub type ScOutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SC_BE` reader -
pub type ScBeR = crate::FieldReader;
///Field `SC_BE` writer -
pub type ScBeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SC_FE` reader -
pub type ScFeR = crate::FieldReader;
///Field `SC_FE` writer -
pub type ScFeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SC_LB1` reader -
pub type ScLb1R = crate::FieldReader;
///Field `SC_LB1` writer -
pub type ScLb1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SC_LB0` reader -
pub type ScLb0R = crate::FieldReader;
///Field `SC_LB0` writer -
pub type ScLb0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    ///Bits 11:12
    #[inline(always)]
    pub fn sc_out(&self) -> ScOutR {
        ScOutR::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn sc_be(&self) -> ScBeR {
        ScBeR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19
    #[inline(always)]
    pub fn sc_fe(&self) -> ScFeR {
        ScFeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21
    #[inline(always)]
    pub fn sc_lb1(&self) -> ScLb1R {
        ScLb1R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23
    #[inline(always)]
    pub fn sc_lb0(&self) -> ScLb0R {
        ScLb0R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OL0_STAT")
            .field("sc_lb0", &self.sc_lb0())
            .field("sc_lb1", &self.sc_lb1())
            .field("sc_fe", &self.sc_fe())
            .field("sc_be", &self.sc_be())
            .field("sc_out", &self.sc_out())
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
    pub fn done_req(&mut self) -> DoneReqW<OL0_STATrs> {
        DoneReqW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn prefetch_out(&mut self) -> PrefetchOutW<OL0_STATrs> {
        PrefetchOutW::new(self, 1)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn prefetch_read(&mut self) -> PrefetchReadW<OL0_STATrs> {
        PrefetchReadW::new(self, 2)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn data_conv(&mut self) -> DataConvW<OL0_STATrs> {
        DataConvW::new(self, 4)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn pf_df(&mut self) -> PfDfW<OL0_STATrs> {
        PfDfW::new(self, 6)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn pf_pr(&mut self) -> PfPrW<OL0_STATrs> {
        PfPrW::new(self, 8)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn sc_out(&mut self) -> ScOutW<OL0_STATrs> {
        ScOutW::new(self, 11)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn sc_be(&mut self) -> ScBeW<OL0_STATrs> {
        ScBeW::new(self, 13)
    }
    ///Bits 16:19
    #[inline(always)]
    pub fn sc_fe(&mut self) -> ScFeW<OL0_STATrs> {
        ScFeW::new(self, 16)
    }
    ///Bits 20:21
    #[inline(always)]
    pub fn sc_lb1(&mut self) -> ScLb1W<OL0_STATrs> {
        ScLb1W::new(self, 20)
    }
    ///Bits 22:23
    #[inline(always)]
    pub fn sc_lb0(&mut self) -> ScLb0W<OL0_STATrs> {
        ScLb0W::new(self, 22)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ol0_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol0_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OL0_STATrs;
impl crate::RegisterSpec for OL0_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ol0_stat::R`](R) reader structure
impl crate::Readable for OL0_STATrs {}
///`write(|w| ..)` method takes [`ol0_stat::W`](W) writer structure
impl crate::Writable for OL0_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OL0_STAT to value 0
impl crate::Resettable for OL0_STATrs {
    const RESET_VALUE: u32 = 0;
}
