///Register `ADC_CFG` reader
pub type R = crate::R<ADC_CFGrs>;
///Register `ADC_CFG` writer
pub type W = crate::W<ADC_CFGrs>;
///Field `OSR_SEL` reader - ADC oversample rate 3'b000: 200 3'b001: 300 3'b010: 400 3'b011: 600 other: reserved
pub type OsrSelR = crate::FieldReader;
///Field `OSR_SEL` writer - ADC oversample rate 3'b000: 200 3'b001: 300 3'b010: 400 3'b011: 600 other: reserved
pub type OsrSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OP_MODE` reader - adc operation mode 2'h0: normal mode: send adc data out through rx interface 2'h1: apb mode: send adc data out through apb interface 2'h2: raw data apb mode: send adc raw data out through apb interface 2'h3: reserved
pub type OpModeR = crate::FieldReader;
///Field `OP_MODE` writer - adc operation mode 2'h0: normal mode: send adc data out through rx interface 2'h1: apb mode: send adc data out through apb interface 2'h2: raw data apb mode: send adc raw data out through apb interface 2'h3: reserved
pub type OpModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PATH_RESET` reader - adc path reset, set 1 to reset adc path
pub type PathResetR = crate::BitReader;
///Field `PATH_RESET` writer - adc path reset, set 1 to reset adc path
pub type PathResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_SRC_SEL` reader - adc clock source select 1: pll 0: xtal
pub type ClkSrcSelR = crate::BitReader;
///Field `CLK_SRC_SEL` writer - adc clock source select 1: pll 0: xtal
pub type ClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DIV` reader - adc clock divider
pub type ClkDivR = crate::FieldReader;
///Field `CLK_DIV` writer - adc clock divider
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:2 - ADC oversample rate 3'b000: 200 3'b001: 300 3'b010: 400 3'b011: 600 other: reserved
    #[inline(always)]
    pub fn osr_sel(&self) -> OsrSelR {
        OsrSelR::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - adc operation mode 2'h0: normal mode: send adc data out through rx interface 2'h1: apb mode: send adc data out through apb interface 2'h2: raw data apb mode: send adc raw data out through apb interface 2'h3: reserved
    #[inline(always)]
    pub fn op_mode(&self) -> OpModeR {
        OpModeR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - adc path reset, set 1 to reset adc path
    #[inline(always)]
    pub fn path_reset(&self) -> PathResetR {
        PathResetR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - adc clock source select 1: pll 0: xtal
    #[inline(always)]
    pub fn clk_src_sel(&self) -> ClkSrcSelR {
        ClkSrcSelR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - adc clock divider
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFG")
            .field("clk_div", &self.clk_div())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("path_reset", &self.path_reset())
            .field("op_mode", &self.op_mode())
            .field("osr_sel", &self.osr_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - ADC oversample rate 3'b000: 200 3'b001: 300 3'b010: 400 3'b011: 600 other: reserved
    #[inline(always)]
    pub fn osr_sel(&mut self) -> OsrSelW<ADC_CFGrs> {
        OsrSelW::new(self, 0)
    }
    ///Bits 3:4 - adc operation mode 2'h0: normal mode: send adc data out through rx interface 2'h1: apb mode: send adc data out through apb interface 2'h2: raw data apb mode: send adc raw data out through apb interface 2'h3: reserved
    #[inline(always)]
    pub fn op_mode(&mut self) -> OpModeW<ADC_CFGrs> {
        OpModeW::new(self, 3)
    }
    ///Bit 5 - adc path reset, set 1 to reset adc path
    #[inline(always)]
    pub fn path_reset(&mut self) -> PathResetW<ADC_CFGrs> {
        PathResetW::new(self, 5)
    }
    ///Bit 6 - adc clock source select 1: pll 0: xtal
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<ADC_CFGrs> {
        ClkSrcSelW::new(self, 6)
    }
    ///Bits 8:15 - adc clock divider
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<ADC_CFGrs> {
        ClkDivW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`adc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_CFGrs;
impl crate::RegisterSpec for ADC_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfg::R`](R) reader structure
impl crate::Readable for ADC_CFGrs {}
///`write(|w| ..)` method takes [`adc_cfg::W`](W) writer structure
impl crate::Writable for ADC_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CFG to value 0
impl crate::Resettable for ADC_CFGrs {
    const RESET_VALUE: u32 = 0;
}
