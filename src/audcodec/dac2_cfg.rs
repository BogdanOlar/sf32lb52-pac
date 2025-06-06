///Register `DAC2_CFG` reader
pub type R = crate::R<DAC2_CFGrs>;
///Register `DAC2_CFG` writer
pub type W = crate::W<DAC2_CFGrs>;
///Field `EN_OS_DAC` reader - enable os dac
pub type EnOsDacR = crate::BitReader;
///Field `EN_OS_DAC` writer - enable os dac
pub type EnOsDacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OS_DAC` reader - os dac
pub type OsDacR = crate::FieldReader;
///Field `OS_DAC` writer - os dac
pub type OsDacW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `GAIN` reader - dac gain
pub type GainR = crate::FieldReader;
///Field `GAIN` writer - dac gain
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SR` reader - dac short switch
pub type SrR = crate::BitReader;
///Field `SR` writer - dac short switch
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL_CLK` reader - dac clk polarity
pub type PolClkR = crate::BitReader;
///Field `POL_CLK` writer - dac clk polarity
pub type PolClkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_MODE` reader - 0: 3.3V sup, 1: 1.8V supply
pub type LpModeR = crate::BitReader;
///Field `LP_MODE` writer - 0: 3.3V sup, 1: 1.8V supply
pub type LpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_VCM` reader - select vcm
pub type SelVcmR = crate::FieldReader;
///Field `SEL_VCM` writer - select vcm
pub type SelVcmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BM` reader - bias mode
pub type BmR = crate::FieldReader;
///Field `BM` writer - bias mode
pub type BmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_CHOP` reader - enable chop
pub type EnChopR = crate::BitReader;
///Field `EN_CHOP` writer - enable chop
pub type EnChopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_AMP` reader - enable amp
pub type EnAmpR = crate::BitReader;
///Field `EN_AMP` writer - enable amp
pub type EnAmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_VCM` reader - enable vcm
pub type EnVcmR = crate::BitReader;
///Field `EN_VCM` writer - enable vcm
pub type EnVcmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_DAC` reader - enable dac
pub type EnDacR = crate::BitReader;
///Field `EN_DAC` writer - enable dac
pub type EnDacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_VSTART` reader - select Vstart
pub type SelVstartR = crate::FieldReader;
///Field `SEL_VSTART` writer - select Vstart
pub type SelVstartW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - enable os dac
    #[inline(always)]
    pub fn en_os_dac(&self) -> EnOsDacR {
        EnOsDacR::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - os dac
    #[inline(always)]
    pub fn os_dac(&self) -> OsDacR {
        OsDacR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bits 8:11 - dac gain
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - dac short switch
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - dac clk polarity
    #[inline(always)]
    pub fn pol_clk(&self) -> PolClkR {
        PolClkR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - 0: 3.3V sup, 1: 1.8V supply
    #[inline(always)]
    pub fn lp_mode(&self) -> LpModeR {
        LpModeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:17 - select vcm
    #[inline(always)]
    pub fn sel_vcm(&self) -> SelVcmR {
        SelVcmR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:19 - bias mode
    #[inline(always)]
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - enable chop
    #[inline(always)]
    pub fn en_chop(&self) -> EnChopR {
        EnChopR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - enable amp
    #[inline(always)]
    pub fn en_amp(&self) -> EnAmpR {
        EnAmpR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - enable vcm
    #[inline(always)]
    pub fn en_vcm(&self) -> EnVcmR {
        EnVcmR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - enable dac
    #[inline(always)]
    pub fn en_dac(&self) -> EnDacR {
        EnDacR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - select Vstart
    #[inline(always)]
    pub fn sel_vstart(&self) -> SelVstartR {
        SelVstartR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC2_CFG")
            .field("rsvd", &self.rsvd())
            .field("sel_vstart", &self.sel_vstart())
            .field("en_dac", &self.en_dac())
            .field("en_vcm", &self.en_vcm())
            .field("en_amp", &self.en_amp())
            .field("en_chop", &self.en_chop())
            .field("bm", &self.bm())
            .field("sel_vcm", &self.sel_vcm())
            .field("lp_mode", &self.lp_mode())
            .field("pol_clk", &self.pol_clk())
            .field("sr", &self.sr())
            .field("gain", &self.gain())
            .field("os_dac", &self.os_dac())
            .field("en_os_dac", &self.en_os_dac())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable os dac
    #[inline(always)]
    pub fn en_os_dac(&mut self) -> EnOsDacW<DAC2_CFGrs> {
        EnOsDacW::new(self, 0)
    }
    ///Bits 1:7 - os dac
    #[inline(always)]
    pub fn os_dac(&mut self) -> OsDacW<DAC2_CFGrs> {
        OsDacW::new(self, 1)
    }
    ///Bits 8:11 - dac gain
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<DAC2_CFGrs> {
        GainW::new(self, 8)
    }
    ///Bit 12 - dac short switch
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<DAC2_CFGrs> {
        SrW::new(self, 12)
    }
    ///Bit 13 - dac clk polarity
    #[inline(always)]
    pub fn pol_clk(&mut self) -> PolClkW<DAC2_CFGrs> {
        PolClkW::new(self, 13)
    }
    ///Bit 14 - 0: 3.3V sup, 1: 1.8V supply
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LpModeW<DAC2_CFGrs> {
        LpModeW::new(self, 14)
    }
    ///Bits 15:17 - select vcm
    #[inline(always)]
    pub fn sel_vcm(&mut self) -> SelVcmW<DAC2_CFGrs> {
        SelVcmW::new(self, 15)
    }
    ///Bits 18:19 - bias mode
    #[inline(always)]
    pub fn bm(&mut self) -> BmW<DAC2_CFGrs> {
        BmW::new(self, 18)
    }
    ///Bit 20 - enable chop
    #[inline(always)]
    pub fn en_chop(&mut self) -> EnChopW<DAC2_CFGrs> {
        EnChopW::new(self, 20)
    }
    ///Bit 21 - enable amp
    #[inline(always)]
    pub fn en_amp(&mut self) -> EnAmpW<DAC2_CFGrs> {
        EnAmpW::new(self, 21)
    }
    ///Bit 22 - enable vcm
    #[inline(always)]
    pub fn en_vcm(&mut self) -> EnVcmW<DAC2_CFGrs> {
        EnVcmW::new(self, 22)
    }
    ///Bit 23 - enable dac
    #[inline(always)]
    pub fn en_dac(&mut self) -> EnDacW<DAC2_CFGrs> {
        EnDacW::new(self, 23)
    }
    ///Bits 24:25 - select Vstart
    #[inline(always)]
    pub fn sel_vstart(&mut self) -> SelVstartW<DAC2_CFGrs> {
        SelVstartW::new(self, 24)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DAC2_CFGrs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac2_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac2_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC2_CFGrs;
impl crate::RegisterSpec for DAC2_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`dac2_cfg::R`](R) reader structure
impl crate::Readable for DAC2_CFGrs {}
///`write(|w| ..)` method takes [`dac2_cfg::W`](W) writer structure
impl crate::Writable for DAC2_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC2_CFG to value 0
impl crate::Resettable for DAC2_CFGrs {
    const RESET_VALUE: u32 = 0;
}
