///Register `OL2_STAT` reader
pub type R = crate::R<OL2_STATrs>;
///Register `OL2_STAT` writer
pub type W = crate::W<OL2_STATrs>;
///Field `PREFETCH_OUT` reader -
pub type PrefetchOutR = crate::BitReader;
///Field `PREFETCH_OUT` writer -
pub type PrefetchOutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFETCH_READ` reader -
pub type PrefetchReadR = crate::FieldReader;
///Field `PREFETCH_READ` writer -
pub type PrefetchReadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RF_ROT` reader -
pub type RfRotR = crate::FieldReader;
///Field `RF_ROT` writer -
pub type RfRotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NF_PR` reader -
pub type NfPrR = crate::FieldReader;
///Field `NF_PR` writer -
pub type NfPrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NF_DF` reader -
pub type NfDfR = crate::FieldReader;
///Field `NF_DF` writer -
pub type NfDfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NF_DATA_CONV` reader -
pub type NfDataConvR = crate::FieldReader;
///Field `NF_DATA_CONV` writer -
pub type NfDataConvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    pub fn prefetch_out(&self) -> PrefetchOutR {
        PrefetchOutR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2
    #[inline(always)]
    pub fn prefetch_read(&self) -> PrefetchReadR {
        PrefetchReadR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:6
    #[inline(always)]
    pub fn rf_rot(&self) -> RfRotR {
        RfRotR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn nf_pr(&self) -> NfPrR {
        NfPrR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:11
    #[inline(always)]
    pub fn nf_df(&self) -> NfDfR {
        NfDfR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13
    #[inline(always)]
    pub fn nf_data_conv(&self) -> NfDataConvR {
        NfDataConvR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn sc_out(&self) -> ScOutR {
        ScOutR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:18
    #[inline(always)]
    pub fn sc_be(&self) -> ScBeR {
        ScBeR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22
    #[inline(always)]
    pub fn sc_fe(&self) -> ScFeR {
        ScFeR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn sc_lb1(&self) -> ScLb1R {
        ScLb1R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bits 25:26
    #[inline(always)]
    pub fn sc_lb0(&self) -> ScLb0R {
        ScLb0R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OL2_STAT")
            .field("sc_lb0", &self.sc_lb0())
            .field("sc_lb1", &self.sc_lb1())
            .field("sc_fe", &self.sc_fe())
            .field("sc_be", &self.sc_be())
            .field("sc_out", &self.sc_out())
            .field("nf_data_conv", &self.nf_data_conv())
            .field("nf_df", &self.nf_df())
            .field("nf_pr", &self.nf_pr())
            .field("rf_rot", &self.rf_rot())
            .field("prefetch_read", &self.prefetch_read())
            .field("prefetch_out", &self.prefetch_out())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn prefetch_out(&mut self) -> PrefetchOutW<OL2_STATrs> {
        PrefetchOutW::new(self, 0)
    }
    ///Bits 1:2
    #[inline(always)]
    pub fn prefetch_read(&mut self) -> PrefetchReadW<OL2_STATrs> {
        PrefetchReadW::new(self, 1)
    }
    ///Bits 3:6
    #[inline(always)]
    pub fn rf_rot(&mut self) -> RfRotW<OL2_STATrs> {
        RfRotW::new(self, 3)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn nf_pr(&mut self) -> NfPrW<OL2_STATrs> {
        NfPrW::new(self, 7)
    }
    ///Bits 10:11
    #[inline(always)]
    pub fn nf_df(&mut self) -> NfDfW<OL2_STATrs> {
        NfDfW::new(self, 10)
    }
    ///Bits 12:13
    #[inline(always)]
    pub fn nf_data_conv(&mut self) -> NfDataConvW<OL2_STATrs> {
        NfDataConvW::new(self, 12)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn sc_out(&mut self) -> ScOutW<OL2_STATrs> {
        ScOutW::new(self, 14)
    }
    ///Bits 16:18
    #[inline(always)]
    pub fn sc_be(&mut self) -> ScBeW<OL2_STATrs> {
        ScBeW::new(self, 16)
    }
    ///Bits 19:22
    #[inline(always)]
    pub fn sc_fe(&mut self) -> ScFeW<OL2_STATrs> {
        ScFeW::new(self, 19)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn sc_lb1(&mut self) -> ScLb1W<OL2_STATrs> {
        ScLb1W::new(self, 23)
    }
    ///Bits 25:26
    #[inline(always)]
    pub fn sc_lb0(&mut self) -> ScLb0W<OL2_STATrs> {
        ScLb0W::new(self, 25)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ol2_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ol2_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OL2_STATrs;
impl crate::RegisterSpec for OL2_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ol2_stat::R`](R) reader structure
impl crate::Readable for OL2_STATrs {}
///`write(|w| ..)` method takes [`ol2_stat::W`](W) writer structure
impl crate::Writable for OL2_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OL2_STAT to value 0
impl crate::Resettable for OL2_STATrs {
    const RESET_VALUE: u32 = 0;
}
