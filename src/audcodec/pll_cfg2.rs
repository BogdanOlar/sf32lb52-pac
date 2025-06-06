///Register `PLL_CFG2` reader
pub type R = crate::R<PLL_CFG2rs>;
///Register `PLL_CFG2` writer
pub type W = crate::W<PLL_CFG2rs>;
///Field `MMD_STG` reader - mmd stg
pub type MmdStgR = crate::FieldReader;
///Field `MMD_STG` writer - mmd stg
pub type MmdStgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TR_DTEST` reader - select dtest
pub type TrDtestR = crate::FieldReader;
///Field `TR_DTEST` writer - select dtest
pub type TrDtestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TE_DTEST` reader - enable dtest
pub type TeDtestR = crate::BitReader;
///Field `TE_DTEST` writer - enable dtest
pub type TeDtestW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTB_SYNC_EN` reader - resetb sync
pub type RstbSyncEnR = crate::BitReader;
///Field `RSTB_SYNC_EN` writer - resetb sync
pub type RstbSyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTB` reader - resetb
pub type RstbR = crate::BitReader;
///Field `RSTB` writer - resetb
pub type RstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_VREF_DIG` reader - ldo vref, 7:1.1V
pub type SelVrefDigR = crate::FieldReader;
///Field `SEL_VREF_DIG` writer - ldo vref, 7:1.1V
pub type SelVrefDigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN_DIG` reader - enable dig block
pub type EnDigR = crate::BitReader;
///Field `EN_DIG` writer - enable dig block
pub type EnDigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_LF_TSTBUF` reader - enable vctrl buf
pub type EnLfTstbufR = crate::BitReader;
///Field `EN_LF_TSTBUF` writer - enable vctrl buf
pub type EnLfTstbufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_LF_VCIN` reader - select vcin, 4: 550mV
pub type SelLfVcinR = crate::FieldReader;
///Field `SEL_LF_VCIN` writer - select vcin, 4: 550mV
pub type SelLfVcinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_LF_VCIN` reader - enable vcin for vco
pub type EnLfVcinR = crate::BitReader;
///Field `EN_LF_VCIN` writer - enable vcin for vco
pub type EnLfVcinW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:1 - mmd stg
    #[inline(always)]
    pub fn mmd_stg(&self) -> MmdStgR {
        MmdStgR::new((self.bits & 3) as u8)
    }
    ///Bits 2:5 - select dtest
    #[inline(always)]
    pub fn tr_dtest(&self) -> TrDtestR {
        TrDtestR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - enable dtest
    #[inline(always)]
    pub fn te_dtest(&self) -> TeDtestR {
        TeDtestR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - resetb sync
    #[inline(always)]
    pub fn rstb_sync_en(&self) -> RstbSyncEnR {
        RstbSyncEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - resetb
    #[inline(always)]
    pub fn rstb(&self) -> RstbR {
        RstbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12 - ldo vref, 7:1.1V
    #[inline(always)]
    pub fn sel_vref_dig(&self) -> SelVrefDigR {
        SelVrefDigR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bit 13 - enable dig block
    #[inline(always)]
    pub fn en_dig(&self) -> EnDigR {
        EnDigR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - enable vctrl buf
    #[inline(always)]
    pub fn en_lf_tstbuf(&self) -> EnLfTstbufR {
        EnLfTstbufR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:17 - select vcin, 4: 550mV
    #[inline(always)]
    pub fn sel_lf_vcin(&self) -> SelLfVcinR {
        SelLfVcinR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bit 18 - enable vcin for vco
    #[inline(always)]
    pub fn en_lf_vcin(&self) -> EnLfVcinR {
        EnLfVcinR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG2")
            .field("rsvd", &self.rsvd())
            .field("en_lf_vcin", &self.en_lf_vcin())
            .field("sel_lf_vcin", &self.sel_lf_vcin())
            .field("en_lf_tstbuf", &self.en_lf_tstbuf())
            .field("en_dig", &self.en_dig())
            .field("sel_vref_dig", &self.sel_vref_dig())
            .field("rstb", &self.rstb())
            .field("rstb_sync_en", &self.rstb_sync_en())
            .field("te_dtest", &self.te_dtest())
            .field("tr_dtest", &self.tr_dtest())
            .field("mmd_stg", &self.mmd_stg())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - mmd stg
    #[inline(always)]
    pub fn mmd_stg(&mut self) -> MmdStgW<PLL_CFG2rs> {
        MmdStgW::new(self, 0)
    }
    ///Bits 2:5 - select dtest
    #[inline(always)]
    pub fn tr_dtest(&mut self) -> TrDtestW<PLL_CFG2rs> {
        TrDtestW::new(self, 2)
    }
    ///Bit 6 - enable dtest
    #[inline(always)]
    pub fn te_dtest(&mut self) -> TeDtestW<PLL_CFG2rs> {
        TeDtestW::new(self, 6)
    }
    ///Bit 7 - resetb sync
    #[inline(always)]
    pub fn rstb_sync_en(&mut self) -> RstbSyncEnW<PLL_CFG2rs> {
        RstbSyncEnW::new(self, 7)
    }
    ///Bit 8 - resetb
    #[inline(always)]
    pub fn rstb(&mut self) -> RstbW<PLL_CFG2rs> {
        RstbW::new(self, 8)
    }
    ///Bits 9:12 - ldo vref, 7:1.1V
    #[inline(always)]
    pub fn sel_vref_dig(&mut self) -> SelVrefDigW<PLL_CFG2rs> {
        SelVrefDigW::new(self, 9)
    }
    ///Bit 13 - enable dig block
    #[inline(always)]
    pub fn en_dig(&mut self) -> EnDigW<PLL_CFG2rs> {
        EnDigW::new(self, 13)
    }
    ///Bit 14 - enable vctrl buf
    #[inline(always)]
    pub fn en_lf_tstbuf(&mut self) -> EnLfTstbufW<PLL_CFG2rs> {
        EnLfTstbufW::new(self, 14)
    }
    ///Bits 15:17 - select vcin, 4: 550mV
    #[inline(always)]
    pub fn sel_lf_vcin(&mut self) -> SelLfVcinW<PLL_CFG2rs> {
        SelLfVcinW::new(self, 15)
    }
    ///Bit 18 - enable vcin for vco
    #[inline(always)]
    pub fn en_lf_vcin(&mut self) -> EnLfVcinW<PLL_CFG2rs> {
        EnLfVcinW::new(self, 18)
    }
    ///Bits 19:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PLL_CFG2rs> {
        RsvdW::new(self, 19)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG2rs;
impl crate::RegisterSpec for PLL_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg2::R`](R) reader structure
impl crate::Readable for PLL_CFG2rs {}
///`write(|w| ..)` method takes [`pll_cfg2::W`](W) writer structure
impl crate::Writable for PLL_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG2 to value 0
impl crate::Resettable for PLL_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
