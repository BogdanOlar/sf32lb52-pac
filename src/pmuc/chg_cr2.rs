///Register `CHG_CR2` reader
pub type R = crate::R<CHG_CR2rs>;
///Register `CHG_CR2` writer
pub type W = crate::W<CHG_CR2rs>;
///Field `BG_PROG_V1P2` reader -
pub type BgProgV1p2R = crate::FieldReader;
///Field `BG_PROG_V1P2` writer -
pub type BgProgV1p2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRECC_RANGE` reader -
pub type PreccRangeR = crate::FieldReader;
///Field `PRECC_RANGE` writer -
pub type PreccRangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRECC_ICTRL` reader -
pub type PreccIctrlR = crate::FieldReader;
///Field `PRECC_ICTRL` writer -
pub type PreccIctrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `REP_VCTRL` reader -
pub type RepVctrlR = crate::FieldReader;
///Field `REP_VCTRL` writer -
pub type RepVctrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `HIGH_VCTRL` reader -
pub type HighVctrlR = crate::FieldReader;
///Field `HIGH_VCTRL` writer -
pub type HighVctrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `BM_EOC` reader -
pub type BmEocR = crate::FieldReader;
///Field `BM_EOC` writer -
pub type BmEocW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RANGE_EOC` reader -
pub type RangeEocR = crate::BitReader;
///Field `RANGE_EOC` writer -
pub type RangeEocW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBAT_RANGE` reader -
pub type VbatRangeR = crate::FieldReader;
///Field `VBAT_RANGE` writer -
pub type VbatRangeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn bg_prog_v1p2(&self) -> BgProgV1p2R {
        BgProgV1p2R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn precc_range(&self) -> PreccRangeR {
        PreccRangeR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:11
    #[inline(always)]
    pub fn precc_ictrl(&self) -> PreccIctrlR {
        PreccIctrlR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bits 12:17
    #[inline(always)]
    pub fn rep_vctrl(&self) -> RepVctrlR {
        RepVctrlR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bits 18:23
    #[inline(always)]
    pub fn high_vctrl(&self) -> HighVctrlR {
        HighVctrlR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    ///Bits 24:26
    #[inline(always)]
    pub fn bm_eoc(&self) -> BmEocR {
        BmEocR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27
    #[inline(always)]
    pub fn range_eoc(&self) -> RangeEocR {
        RangeEocR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn vbat_range(&self) -> VbatRangeR {
        VbatRangeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHG_CR2")
            .field("vbat_range", &self.vbat_range())
            .field("range_eoc", &self.range_eoc())
            .field("bm_eoc", &self.bm_eoc())
            .field("high_vctrl", &self.high_vctrl())
            .field("rep_vctrl", &self.rep_vctrl())
            .field("precc_ictrl", &self.precc_ictrl())
            .field("precc_range", &self.precc_range())
            .field("bg_prog_v1p2", &self.bg_prog_v1p2())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn bg_prog_v1p2(&mut self) -> BgProgV1p2W<CHG_CR2rs> {
        BgProgV1p2W::new(self, 0)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn precc_range(&mut self) -> PreccRangeW<CHG_CR2rs> {
        PreccRangeW::new(self, 4)
    }
    ///Bits 6:11
    #[inline(always)]
    pub fn precc_ictrl(&mut self) -> PreccIctrlW<CHG_CR2rs> {
        PreccIctrlW::new(self, 6)
    }
    ///Bits 12:17
    #[inline(always)]
    pub fn rep_vctrl(&mut self) -> RepVctrlW<CHG_CR2rs> {
        RepVctrlW::new(self, 12)
    }
    ///Bits 18:23
    #[inline(always)]
    pub fn high_vctrl(&mut self) -> HighVctrlW<CHG_CR2rs> {
        HighVctrlW::new(self, 18)
    }
    ///Bits 24:26
    #[inline(always)]
    pub fn bm_eoc(&mut self) -> BmEocW<CHG_CR2rs> {
        BmEocW::new(self, 24)
    }
    ///Bit 27
    #[inline(always)]
    pub fn range_eoc(&mut self) -> RangeEocW<CHG_CR2rs> {
        RangeEocW::new(self, 27)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn vbat_range(&mut self) -> VbatRangeW<CHG_CR2rs> {
        VbatRangeW::new(self, 28)
    }
}
///Charger Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHG_CR2rs;
impl crate::RegisterSpec for CHG_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`chg_cr2::R`](R) reader structure
impl crate::Readable for CHG_CR2rs {}
///`write(|w| ..)` method takes [`chg_cr2::W`](W) writer structure
impl crate::Writable for CHG_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHG_CR2 to value 0
impl crate::Resettable for CHG_CR2rs {
    const RESET_VALUE: u32 = 0;
}
