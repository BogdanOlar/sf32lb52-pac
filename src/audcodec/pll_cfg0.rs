///Register `PLL_CFG0` reader
pub type R = crate::R<PLL_CFG0rs>;
///Register `PLL_CFG0` writer
pub type W = crate::W<PLL_CFG0rs>;
///Field `ICP_OS_SEL` reader - Icp os
pub type IcpOsSelR = crate::FieldReader;
///Field `ICP_OS_SEL` writer - Icp os
pub type IcpOsSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OPEN` reader - 1: pll open
pub type OpenR = crate::BitReader;
///Field `OPEN` writer - 1: pll open
pub type OpenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICP_SEL` reader - select Icp, 1:1.25u
pub type IcpSelR = crate::FieldReader;
///Field `ICP_SEL` writer - select Icp, 1:1.25u
pub type IcpSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SEL_VREF_ANA` reader - ldo vref, 7:1.1V
pub type SelVrefAnaR = crate::FieldReader;
///Field `SEL_VREF_ANA` writer - ldo vref, 7:1.1V
pub type SelVrefAnaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN_ANA` reader - enable ana block
pub type EnAnaR = crate::BitReader;
///Field `EN_ANA` writer - enable ana block
pub type EnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCO_LP_MODE` reader - 1: lp mode
pub type VcoLpModeR = crate::BitReader;
///Field `VCO_LP_MODE` writer - 1: lp mode
pub type VcoLpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FC_VCO` reader - VCO Fcode
pub type FcVcoR = crate::FieldReader;
///Field `FC_VCO` writer - VCO Fcode
pub type FcVcoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EN_VCO_FLT` reader - vco bais filter
pub type EnVcoFltR = crate::BitReader;
///Field `EN_VCO_FLT` writer - vco bais filter
pub type EnVcoFltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_VREF_VCO` reader - ldo vref, 7:1.1V
pub type SelVrefVcoR = crate::FieldReader;
///Field `SEL_VREF_VCO` writer - ldo vref, 7:1.1V
pub type SelVrefVcoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN_VCO` reader - enable vco
pub type EnVcoR = crate::BitReader;
///Field `EN_VCO` writer - enable vco
pub type EnVcoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_IARY` reader - enable I array
pub type EnIaryR = crate::BitReader;
///Field `EN_IARY` writer - enable I array
pub type EnIaryW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CKREF` reader - select ref clock, 2: 24MHz
pub type SelCkrefR = crate::FieldReader;
///Field `SEL_CKREF` writer - select ref clock, 2: 24MHz
pub type SelCkrefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:4 - Icp os
    #[inline(always)]
    pub fn icp_os_sel(&self) -> IcpOsSelR {
        IcpOsSelR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - 1: pll open
    #[inline(always)]
    pub fn open(&self) -> OpenR {
        OpenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - select Icp, 1:1.25u
    #[inline(always)]
    pub fn icp_sel(&self) -> IcpSelR {
        IcpSelR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:14 - ldo vref, 7:1.1V
    #[inline(always)]
    pub fn sel_vref_ana(&self) -> SelVrefAnaR {
        SelVrefAnaR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - enable ana block
    #[inline(always)]
    pub fn en_ana(&self) -> EnAnaR {
        EnAnaR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 1: lp mode
    #[inline(always)]
    pub fn vco_lp_mode(&self) -> VcoLpModeR {
        VcoLpModeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - VCO Fcode
    #[inline(always)]
    pub fn fc_vco(&self) -> FcVcoR {
        FcVcoR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 22 - vco bais filter
    #[inline(always)]
    pub fn en_vco_flt(&self) -> EnVcoFltR {
        EnVcoFltR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:26 - ldo vref, 7:1.1V
    #[inline(always)]
    pub fn sel_vref_vco(&self) -> SelVrefVcoR {
        SelVrefVcoR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    ///Bit 27 - enable vco
    #[inline(always)]
    pub fn en_vco(&self) -> EnVcoR {
        EnVcoR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - enable I array
    #[inline(always)]
    pub fn en_iary(&self) -> EnIaryR {
        EnIaryR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:30 - select ref clock, 2: 24MHz
    #[inline(always)]
    pub fn sel_ckref(&self) -> SelCkrefR {
        SelCkrefR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG0")
            .field("sel_ckref", &self.sel_ckref())
            .field("en_iary", &self.en_iary())
            .field("en_vco", &self.en_vco())
            .field("sel_vref_vco", &self.sel_vref_vco())
            .field("en_vco_flt", &self.en_vco_flt())
            .field("fc_vco", &self.fc_vco())
            .field("vco_lp_mode", &self.vco_lp_mode())
            .field("en_ana", &self.en_ana())
            .field("sel_vref_ana", &self.sel_vref_ana())
            .field("icp_sel", &self.icp_sel())
            .field("open", &self.open())
            .field("icp_os_sel", &self.icp_os_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Icp os
    #[inline(always)]
    pub fn icp_os_sel(&mut self) -> IcpOsSelW<PLL_CFG0rs> {
        IcpOsSelW::new(self, 0)
    }
    ///Bit 5 - 1: pll open
    #[inline(always)]
    pub fn open(&mut self) -> OpenW<PLL_CFG0rs> {
        OpenW::new(self, 5)
    }
    ///Bits 6:10 - select Icp, 1:1.25u
    #[inline(always)]
    pub fn icp_sel(&mut self) -> IcpSelW<PLL_CFG0rs> {
        IcpSelW::new(self, 6)
    }
    ///Bits 11:14 - ldo vref, 7:1.1V
    #[inline(always)]
    pub fn sel_vref_ana(&mut self) -> SelVrefAnaW<PLL_CFG0rs> {
        SelVrefAnaW::new(self, 11)
    }
    ///Bit 15 - enable ana block
    #[inline(always)]
    pub fn en_ana(&mut self) -> EnAnaW<PLL_CFG0rs> {
        EnAnaW::new(self, 15)
    }
    ///Bit 16 - 1: lp mode
    #[inline(always)]
    pub fn vco_lp_mode(&mut self) -> VcoLpModeW<PLL_CFG0rs> {
        VcoLpModeW::new(self, 16)
    }
    ///Bits 17:21 - VCO Fcode
    #[inline(always)]
    pub fn fc_vco(&mut self) -> FcVcoW<PLL_CFG0rs> {
        FcVcoW::new(self, 17)
    }
    ///Bit 22 - vco bais filter
    #[inline(always)]
    pub fn en_vco_flt(&mut self) -> EnVcoFltW<PLL_CFG0rs> {
        EnVcoFltW::new(self, 22)
    }
    ///Bits 23:26 - ldo vref, 7:1.1V
    #[inline(always)]
    pub fn sel_vref_vco(&mut self) -> SelVrefVcoW<PLL_CFG0rs> {
        SelVrefVcoW::new(self, 23)
    }
    ///Bit 27 - enable vco
    #[inline(always)]
    pub fn en_vco(&mut self) -> EnVcoW<PLL_CFG0rs> {
        EnVcoW::new(self, 27)
    }
    ///Bit 28 - enable I array
    #[inline(always)]
    pub fn en_iary(&mut self) -> EnIaryW<PLL_CFG0rs> {
        EnIaryW::new(self, 28)
    }
    ///Bits 29:30 - select ref clock, 2: 24MHz
    #[inline(always)]
    pub fn sel_ckref(&mut self) -> SelCkrefW<PLL_CFG0rs> {
        SelCkrefW::new(self, 29)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG0rs;
impl crate::RegisterSpec for PLL_CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg0::R`](R) reader structure
impl crate::Readable for PLL_CFG0rs {}
///`write(|w| ..)` method takes [`pll_cfg0::W`](W) writer structure
impl crate::Writable for PLL_CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG0 to value 0
impl crate::Resettable for PLL_CFG0rs {
    const RESET_VALUE: u32 = 0;
}
