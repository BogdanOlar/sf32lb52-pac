///Register `DAC_CFG` reader
pub type R = crate::R<DAC_CFGrs>;
///Register `DAC_CFG` writer
pub type W = crate::W<DAC_CFGrs>;
///Field `OSR_SEL` reader - DAC oversample rate 4'b0000: 100 4'b0001: 150 4'b0010: 200 4'b0011: 300(sdm osr = 150) 4'b0100: 300(sdm osr = 300) 4'b0101: 400 4'b0110: 600 4'b0111: 800 4'b1000: 1200 4'b1001: 256 4'b1010: 512 4'b1011: 1024 other: reserved
pub type OsrSelR = crate::FieldReader;
///Field `OSR_SEL` writer - DAC oversample rate 4'b0000: 100 4'b0001: 150 4'b0010: 200 4'b0011: 300(sdm osr = 150) 4'b0100: 300(sdm osr = 300) 4'b0101: 400 4'b0110: 600 4'b0111: 800 4'b1000: 1200 4'b1001: 256 4'b1010: 512 4'b1011: 1024 other: reserved
pub type OsrSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OP_MODE` reader - dac operation mode 2'h0: normal mode: send dac data through tx interface 2'h1: apb mode: send dac data out through apb interface 2'h2, 2'h3: reserved
pub type OpModeR = crate::FieldReader;
///Field `OP_MODE` writer - dac operation mode 2'h0: normal mode: send dac data through tx interface 2'h1: apb mode: send dac data out through apb interface 2'h2, 2'h3: reserved
pub type OpModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PATH_RESET` reader - dac path reset, set 1 to reset dac path
pub type PathResetR = crate::BitReader;
///Field `PATH_RESET` writer - dac path reset, set 1 to reset dac path
pub type PathResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_SRC_SEL` reader - dac clock source select 1: pll 0: xtal
pub type ClkSrcSelR = crate::BitReader;
///Field `CLK_SRC_SEL` writer - dac clock source select 1: pll 0: xtal
pub type ClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DIV` reader - dac clock divider
pub type ClkDivR = crate::FieldReader;
///Field `CLK_DIV` writer - dac clock divider
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MANUAL_OSR_MODE` reader - set 1 to manually set hbf, interp3, sinc and sdm module
pub type ManualOsrModeR = crate::BitReader;
///Field `MANUAL_OSR_MODE` writer - set 1 to manually set hbf, interp3, sinc and sdm module
pub type ManualOsrModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBF1_BYPASS_M` reader -
pub type Hbf1BypassMR = crate::BitReader;
///Field `HBF1_BYPASS_M` writer -
pub type Hbf1BypassMW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBF2_BYPASS_M` reader -
pub type Hbf2BypassMR = crate::BitReader;
///Field `HBF2_BYPASS_M` writer -
pub type Hbf2BypassMW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBF3_BYPASS_M` reader -
pub type Hbf3BypassMR = crate::BitReader;
///Field `HBF3_BYPASS_M` writer -
pub type Hbf3BypassMW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBF4_BYPASS_M` reader -
pub type Hbf4BypassMR = crate::BitReader;
///Field `HBF4_BYPASS_M` writer -
pub type Hbf4BypassMW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTERP3_BYPASS_M` reader -
pub type Interp3BypassMR = crate::BitReader;
///Field `INTERP3_BYPASS_M` writer -
pub type Interp3BypassMW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINC_RATE_SEL_M` reader - 0:25 1:50 2:16 3:32 4:64
pub type SincRateSelMR = crate::FieldReader;
///Field `SINC_RATE_SEL_M` writer - 0:25 1:50 2:16 3:32 4:64
pub type SincRateSelMW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SDM_OSR_SEL_M` reader - 0:100 1:150 2:300 3:256
pub type SdmOsrSelMR = crate::FieldReader;
///Field `SDM_OSR_SEL_M` writer - 0:100 1:150 2:300 3:256
pub type SdmOsrSelMW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - DAC oversample rate 4'b0000: 100 4'b0001: 150 4'b0010: 200 4'b0011: 300(sdm osr = 150) 4'b0100: 300(sdm osr = 300) 4'b0101: 400 4'b0110: 600 4'b0111: 800 4'b1000: 1200 4'b1001: 256 4'b1010: 512 4'b1011: 1024 other: reserved
    #[inline(always)]
    pub fn osr_sel(&self) -> OsrSelR {
        OsrSelR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - dac operation mode 2'h0: normal mode: send dac data through tx interface 2'h1: apb mode: send dac data out through apb interface 2'h2, 2'h3: reserved
    #[inline(always)]
    pub fn op_mode(&self) -> OpModeR {
        OpModeR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - dac path reset, set 1 to reset dac path
    #[inline(always)]
    pub fn path_reset(&self) -> PathResetR {
        PathResetR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - dac clock source select 1: pll 0: xtal
    #[inline(always)]
    pub fn clk_src_sel(&self) -> ClkSrcSelR {
        ClkSrcSelR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - dac clock divider
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - set 1 to manually set hbf, interp3, sinc and sdm module
    #[inline(always)]
    pub fn manual_osr_mode(&self) -> ManualOsrModeR {
        ManualOsrModeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn hbf1_bypass_m(&self) -> Hbf1BypassMR {
        Hbf1BypassMR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn hbf2_bypass_m(&self) -> Hbf2BypassMR {
        Hbf2BypassMR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn hbf3_bypass_m(&self) -> Hbf3BypassMR {
        Hbf3BypassMR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn hbf4_bypass_m(&self) -> Hbf4BypassMR {
        Hbf4BypassMR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn interp3_bypass_m(&self) -> Interp3BypassMR {
        Interp3BypassMR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:24 - 0:25 1:50 2:16 3:32 4:64
    #[inline(always)]
    pub fn sinc_rate_sel_m(&self) -> SincRateSelMR {
        SincRateSelMR::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:26 - 0:100 1:150 2:300 3:256
    #[inline(always)]
    pub fn sdm_osr_sel_m(&self) -> SdmOsrSelMR {
        SdmOsrSelMR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CFG")
            .field("sdm_osr_sel_m", &self.sdm_osr_sel_m())
            .field("sinc_rate_sel_m", &self.sinc_rate_sel_m())
            .field("interp3_bypass_m", &self.interp3_bypass_m())
            .field("hbf4_bypass_m", &self.hbf4_bypass_m())
            .field("hbf3_bypass_m", &self.hbf3_bypass_m())
            .field("hbf2_bypass_m", &self.hbf2_bypass_m())
            .field("hbf1_bypass_m", &self.hbf1_bypass_m())
            .field("manual_osr_mode", &self.manual_osr_mode())
            .field("clk_div", &self.clk_div())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("path_reset", &self.path_reset())
            .field("op_mode", &self.op_mode())
            .field("osr_sel", &self.osr_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DAC oversample rate 4'b0000: 100 4'b0001: 150 4'b0010: 200 4'b0011: 300(sdm osr = 150) 4'b0100: 300(sdm osr = 300) 4'b0101: 400 4'b0110: 600 4'b0111: 800 4'b1000: 1200 4'b1001: 256 4'b1010: 512 4'b1011: 1024 other: reserved
    #[inline(always)]
    pub fn osr_sel(&mut self) -> OsrSelW<DAC_CFGrs> {
        OsrSelW::new(self, 0)
    }
    ///Bits 4:5 - dac operation mode 2'h0: normal mode: send dac data through tx interface 2'h1: apb mode: send dac data out through apb interface 2'h2, 2'h3: reserved
    #[inline(always)]
    pub fn op_mode(&mut self) -> OpModeW<DAC_CFGrs> {
        OpModeW::new(self, 4)
    }
    ///Bit 6 - dac path reset, set 1 to reset dac path
    #[inline(always)]
    pub fn path_reset(&mut self) -> PathResetW<DAC_CFGrs> {
        PathResetW::new(self, 6)
    }
    ///Bit 7 - dac clock source select 1: pll 0: xtal
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<DAC_CFGrs> {
        ClkSrcSelW::new(self, 7)
    }
    ///Bits 8:15 - dac clock divider
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<DAC_CFGrs> {
        ClkDivW::new(self, 8)
    }
    ///Bit 16 - set 1 to manually set hbf, interp3, sinc and sdm module
    #[inline(always)]
    pub fn manual_osr_mode(&mut self) -> ManualOsrModeW<DAC_CFGrs> {
        ManualOsrModeW::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn hbf1_bypass_m(&mut self) -> Hbf1BypassMW<DAC_CFGrs> {
        Hbf1BypassMW::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn hbf2_bypass_m(&mut self) -> Hbf2BypassMW<DAC_CFGrs> {
        Hbf2BypassMW::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn hbf3_bypass_m(&mut self) -> Hbf3BypassMW<DAC_CFGrs> {
        Hbf3BypassMW::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn hbf4_bypass_m(&mut self) -> Hbf4BypassMW<DAC_CFGrs> {
        Hbf4BypassMW::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn interp3_bypass_m(&mut self) -> Interp3BypassMW<DAC_CFGrs> {
        Interp3BypassMW::new(self, 21)
    }
    ///Bits 22:24 - 0:25 1:50 2:16 3:32 4:64
    #[inline(always)]
    pub fn sinc_rate_sel_m(&mut self) -> SincRateSelMW<DAC_CFGrs> {
        SincRateSelMW::new(self, 22)
    }
    ///Bits 25:26 - 0:100 1:150 2:300 3:256
    #[inline(always)]
    pub fn sdm_osr_sel_m(&mut self) -> SdmOsrSelMW<DAC_CFGrs> {
        SdmOsrSelMW::new(self, 25)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CFGrs;
impl crate::RegisterSpec for DAC_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`dac_cfg::R`](R) reader structure
impl crate::Readable for DAC_CFGrs {}
///`write(|w| ..)` method takes [`dac_cfg::W`](W) writer structure
impl crate::Writable for DAC_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CFG to value 0
impl crate::Resettable for DAC_CFGrs {
    const RESET_VALUE: u32 = 0;
}
