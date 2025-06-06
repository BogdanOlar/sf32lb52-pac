///Register `DWCFGR` reader
pub type R = crate::R<DWCFGRrs>;
///Register `DWCFGR` writer
pub type W = crate::W<DWCFGRrs>;
///Field `HDIV` reader - hclk_hpsys = clk_hpsys / HDIV during deep wfi
pub type HdivR = crate::FieldReader;
///Field `HDIV` writer - hclk_hpsys = clk_hpsys / HDIV during deep wfi
pub type HdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PDIV1` reader - pclk_hpsys = hclk_hpsys / (2^PDIV1) during deep wfi
pub type Pdiv1R = crate::FieldReader;
///Field `PDIV1` writer - pclk_hpsys = hclk_hpsys / (2^PDIV1) during deep wfi
pub type Pdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDIV2` reader - pclk2_hpsys = hclk_hpsys / (2^PDIV2) during deep wfi
pub type Pdiv2R = crate::FieldReader;
///Field `PDIV2` writer - pclk2_hpsys = hclk_hpsys / (2^PDIV2) during deep wfi
pub type Pdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIV_EN` reader - enable PDIV1, PDIV2 and HDIV reconfiguration during deep wfi
pub type DivEnR = crate::BitReader;
///Field `DIV_EN` writer - enable PDIV1, PDIV2 and HDIV reconfiguration during deep wfi
pub type DivEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_SYS` reader - select clk_hpsys source during deep WFI 0 - clk_hrc48; 1 - clk_hxt48; 2 - RSVD; 3 - clk_dll1
pub type SelSysR = crate::FieldReader;
///Field `SEL_SYS` writer - select clk_hpsys source during deep WFI 0 - clk_hrc48; 1 - clk_hxt48; 2 - RSVD; 3 - clk_dll1
pub type SelSysW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_SYS_LP` reader - select clk_hpsys source during deep WFI 0 - selected by SEL_SYS; 1 - clk_wdt
pub type SelSysLpR = crate::BitReader;
///Field `SEL_SYS_LP` writer - select clk_hpsys source during deep WFI 0 - selected by SEL_SYS; 1 - clk_wdt
pub type SelSysLpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DLL1_OUT_EN` reader - for debug only
pub type Dll1OutEnR = crate::BitReader;
///Field `DLL1_OUT_EN` writer - for debug only
pub type Dll1OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_OUT_RSTB` reader - for debug only
pub type Dll1OutRstbR = crate::BitReader;
///Field `DLL1_OUT_RSTB` writer - for debug only
pub type Dll1OutRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_OUT_EN` reader - for debug only
pub type Dll2OutEnR = crate::BitReader;
///Field `DLL2_OUT_EN` writer - for debug only
pub type Dll2OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_OUT_RSTB` reader - for debug only
pub type Dll2OutRstbR = crate::BitReader;
///Field `DLL2_OUT_RSTB` writer - for debug only
pub type Dll2OutRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - hclk_hpsys = clk_hpsys / HDIV during deep wfi
    #[inline(always)]
    pub fn hdiv(&self) -> HdivR {
        HdivR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - pclk_hpsys = hclk_hpsys / (2^PDIV1) during deep wfi
    #[inline(always)]
    pub fn pdiv1(&self) -> Pdiv1R {
        Pdiv1R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - pclk2_hpsys = hclk_hpsys / (2^PDIV2) during deep wfi
    #[inline(always)]
    pub fn pdiv2(&self) -> Pdiv2R {
        Pdiv2R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - enable PDIV1, PDIV2 and HDIV reconfiguration during deep wfi
    #[inline(always)]
    pub fn div_en(&self) -> DivEnR {
        DivEnR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - select clk_hpsys source during deep WFI 0 - clk_hrc48; 1 - clk_hxt48; 2 - RSVD; 3 - clk_dll1
    #[inline(always)]
    pub fn sel_sys(&self) -> SelSysR {
        SelSysR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - select clk_hpsys source during deep WFI 0 - selected by SEL_SYS; 1 - clk_wdt
    #[inline(always)]
    pub fn sel_sys_lp(&self) -> SelSysLpR {
        SelSysLpR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:23
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bit 24 - for debug only
    #[inline(always)]
    pub fn dll1_out_en(&self) -> Dll1OutEnR {
        Dll1OutEnR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - for debug only
    #[inline(always)]
    pub fn dll1_out_rstb(&self) -> Dll1OutRstbR {
        Dll1OutRstbR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - for debug only
    #[inline(always)]
    pub fn dll2_out_en(&self) -> Dll2OutEnR {
        Dll2OutEnR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - for debug only
    #[inline(always)]
    pub fn dll2_out_rstb(&self) -> Dll2OutRstbR {
        Dll2OutRstbR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DWCFGR")
            .field("rsvd", &self.rsvd())
            .field("dll2_out_rstb", &self.dll2_out_rstb())
            .field("dll2_out_en", &self.dll2_out_en())
            .field("dll1_out_rstb", &self.dll1_out_rstb())
            .field("dll1_out_en", &self.dll1_out_en())
            .field("rsvd2", &self.rsvd2())
            .field("sel_sys_lp", &self.sel_sys_lp())
            .field("sel_sys", &self.sel_sys())
            .field("div_en", &self.div_en())
            .field("pdiv2", &self.pdiv2())
            .field("rsvd3", &self.rsvd3())
            .field("pdiv1", &self.pdiv1())
            .field("hdiv", &self.hdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - hclk_hpsys = clk_hpsys / HDIV during deep wfi
    #[inline(always)]
    pub fn hdiv(&mut self) -> HdivW<DWCFGRrs> {
        HdivW::new(self, 0)
    }
    ///Bits 8:10 - pclk_hpsys = hclk_hpsys / (2^PDIV1) during deep wfi
    #[inline(always)]
    pub fn pdiv1(&mut self) -> Pdiv1W<DWCFGRrs> {
        Pdiv1W::new(self, 8)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<DWCFGRrs> {
        Rsvd3W::new(self, 11)
    }
    ///Bits 12:14 - pclk2_hpsys = hclk_hpsys / (2^PDIV2) during deep wfi
    #[inline(always)]
    pub fn pdiv2(&mut self) -> Pdiv2W<DWCFGRrs> {
        Pdiv2W::new(self, 12)
    }
    ///Bit 15 - enable PDIV1, PDIV2 and HDIV reconfiguration during deep wfi
    #[inline(always)]
    pub fn div_en(&mut self) -> DivEnW<DWCFGRrs> {
        DivEnW::new(self, 15)
    }
    ///Bits 16:17 - select clk_hpsys source during deep WFI 0 - clk_hrc48; 1 - clk_hxt48; 2 - RSVD; 3 - clk_dll1
    #[inline(always)]
    pub fn sel_sys(&mut self) -> SelSysW<DWCFGRrs> {
        SelSysW::new(self, 16)
    }
    ///Bit 18 - select clk_hpsys source during deep WFI 0 - selected by SEL_SYS; 1 - clk_wdt
    #[inline(always)]
    pub fn sel_sys_lp(&mut self) -> SelSysLpW<DWCFGRrs> {
        SelSysLpW::new(self, 18)
    }
    ///Bits 19:23
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DWCFGRrs> {
        Rsvd2W::new(self, 19)
    }
    ///Bit 24 - for debug only
    #[inline(always)]
    pub fn dll1_out_en(&mut self) -> Dll1OutEnW<DWCFGRrs> {
        Dll1OutEnW::new(self, 24)
    }
    ///Bit 25 - for debug only
    #[inline(always)]
    pub fn dll1_out_rstb(&mut self) -> Dll1OutRstbW<DWCFGRrs> {
        Dll1OutRstbW::new(self, 25)
    }
    ///Bit 26 - for debug only
    #[inline(always)]
    pub fn dll2_out_en(&mut self) -> Dll2OutEnW<DWCFGRrs> {
        Dll2OutEnW::new(self, 26)
    }
    ///Bit 27 - for debug only
    #[inline(always)]
    pub fn dll2_out_rstb(&mut self) -> Dll2OutRstbW<DWCFGRrs> {
        Dll2OutRstbW::new(self, 27)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DWCFGRrs> {
        RsvdW::new(self, 28)
    }
}
///Deep WFI mode Clock Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`dwcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DWCFGRrs;
impl crate::RegisterSpec for DWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`dwcfgr::R`](R) reader structure
impl crate::Readable for DWCFGRrs {}
///`write(|w| ..)` method takes [`dwcfgr::W`](W) writer structure
impl crate::Writable for DWCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DWCFGR to value 0
impl crate::Resettable for DWCFGRrs {
    const RESET_VALUE: u32 = 0;
}
