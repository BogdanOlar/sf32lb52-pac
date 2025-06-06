///Register `CFG` reader
pub type R = crate::R<CFGrs>;
///Register `CFG` writer
pub type W = crate::W<CFGrs>;
///Field `ENABLE` reader - audprc enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - audprc enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRESET` reader - audprc software reset, high active
pub type SresetR = crate::BitReader;
///Field `SRESET` writer - audprc software reset, high active
pub type SresetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_PATH_FLUSH` reader - dac path fifo flush, high active
pub type DacPathFlushR = crate::BitReader;
///Field `DAC_PATH_FLUSH` writer - dac path fifo flush, high active
pub type DacPathFlushW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_PATH_FLUSH` reader - adc path fifo flush, high active
pub type AdcPathFlushR = crate::BitReader;
///Field `ADC_PATH_FLUSH` writer - adc path fifo flush, high active
pub type AdcPathFlushW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_PATH_SRESET` reader - dac path software reset, high active
pub type DacPathSresetR = crate::BitReader;
///Field `DAC_PATH_SRESET` writer - dac path software reset, high active
pub type DacPathSresetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_PATH_SRESET` reader - adc path software reset, high active
pub type AdcPathSresetR = crate::BitReader;
///Field `ADC_PATH_SRESET` writer - adc path software reset, high active
pub type AdcPathSresetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_PATH_EN` reader - dac path enable
pub type DacPathEnR = crate::BitReader;
///Field `DAC_PATH_EN` writer - dac path enable
pub type DacPathEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_PATH_EN` reader - adc path enable
pub type AdcPathEnR = crate::BitReader;
///Field `ADC_PATH_EN` writer - adc path enable
pub type AdcPathEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTO_GATE_EN` reader - auto clock gating enable, high active
pub type AutoGateEnR = crate::BitReader;
///Field `AUTO_GATE_EN` writer - auto clock gating enable, high active
pub type AutoGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STB_CLK_SEL` reader - audio strobe clock select 0: use xtal clock to generate strobe 1: use pll clock to generate strobe
pub type StbClkSelR = crate::BitReader;
///Field `STB_CLK_SEL` writer - audio strobe clock select 0: use xtal clock to generate strobe 1: use pll clock to generate strobe
pub type StbClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUDCLK_DIV` reader - audprc clock divider, 0 and 1 means divide by 1
pub type AudclkDivR = crate::FieldReader;
///Field `AUDCLK_DIV` writer - audprc clock divider, 0 and 1 means divide by 1
pub type AudclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AUDCLK_DIV_UPDATE` reader - audprc clock divider update, write 1 to update
pub type AudclkDivUpdateR = crate::BitReader;
///Field `AUDCLK_DIV_UPDATE` writer - audprc clock divider update, write 1 to update
pub type AudclkDivUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - audprc enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - audprc software reset, high active
    #[inline(always)]
    pub fn sreset(&self) -> SresetR {
        SresetR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dac path fifo flush, high active
    #[inline(always)]
    pub fn dac_path_flush(&self) -> DacPathFlushR {
        DacPathFlushR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - adc path fifo flush, high active
    #[inline(always)]
    pub fn adc_path_flush(&self) -> AdcPathFlushR {
        AdcPathFlushR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - dac path software reset, high active
    #[inline(always)]
    pub fn dac_path_sreset(&self) -> DacPathSresetR {
        DacPathSresetR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - adc path software reset, high active
    #[inline(always)]
    pub fn adc_path_sreset(&self) -> AdcPathSresetR {
        AdcPathSresetR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - dac path enable
    #[inline(always)]
    pub fn dac_path_en(&self) -> DacPathEnR {
        DacPathEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - adc path enable
    #[inline(always)]
    pub fn adc_path_en(&self) -> AdcPathEnR {
        AdcPathEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - auto clock gating enable, high active
    #[inline(always)]
    pub fn auto_gate_en(&self) -> AutoGateEnR {
        AutoGateEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - audio strobe clock select 0: use xtal clock to generate strobe 1: use pll clock to generate strobe
    #[inline(always)]
    pub fn stb_clk_sel(&self) -> StbClkSelR {
        StbClkSelR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:19 - audprc clock divider, 0 and 1 means divide by 1
    #[inline(always)]
    pub fn audclk_div(&self) -> AudclkDivR {
        AudclkDivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - audprc clock divider update, write 1 to update
    #[inline(always)]
    pub fn audclk_div_update(&self) -> AudclkDivUpdateR {
        AudclkDivUpdateR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("audclk_div_update", &self.audclk_div_update())
            .field("audclk_div", &self.audclk_div())
            .field("stb_clk_sel", &self.stb_clk_sel())
            .field("auto_gate_en", &self.auto_gate_en())
            .field("adc_path_en", &self.adc_path_en())
            .field("dac_path_en", &self.dac_path_en())
            .field("adc_path_sreset", &self.adc_path_sreset())
            .field("dac_path_sreset", &self.dac_path_sreset())
            .field("adc_path_flush", &self.adc_path_flush())
            .field("dac_path_flush", &self.dac_path_flush())
            .field("sreset", &self.sreset())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - audprc enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CFGrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - audprc software reset, high active
    #[inline(always)]
    pub fn sreset(&mut self) -> SresetW<CFGrs> {
        SresetW::new(self, 1)
    }
    ///Bit 2 - dac path fifo flush, high active
    #[inline(always)]
    pub fn dac_path_flush(&mut self) -> DacPathFlushW<CFGrs> {
        DacPathFlushW::new(self, 2)
    }
    ///Bit 3 - adc path fifo flush, high active
    #[inline(always)]
    pub fn adc_path_flush(&mut self) -> AdcPathFlushW<CFGrs> {
        AdcPathFlushW::new(self, 3)
    }
    ///Bit 4 - dac path software reset, high active
    #[inline(always)]
    pub fn dac_path_sreset(&mut self) -> DacPathSresetW<CFGrs> {
        DacPathSresetW::new(self, 4)
    }
    ///Bit 5 - adc path software reset, high active
    #[inline(always)]
    pub fn adc_path_sreset(&mut self) -> AdcPathSresetW<CFGrs> {
        AdcPathSresetW::new(self, 5)
    }
    ///Bit 6 - dac path enable
    #[inline(always)]
    pub fn dac_path_en(&mut self) -> DacPathEnW<CFGrs> {
        DacPathEnW::new(self, 6)
    }
    ///Bit 7 - adc path enable
    #[inline(always)]
    pub fn adc_path_en(&mut self) -> AdcPathEnW<CFGrs> {
        AdcPathEnW::new(self, 7)
    }
    ///Bit 8 - auto clock gating enable, high active
    #[inline(always)]
    pub fn auto_gate_en(&mut self) -> AutoGateEnW<CFGrs> {
        AutoGateEnW::new(self, 8)
    }
    ///Bit 9 - audio strobe clock select 0: use xtal clock to generate strobe 1: use pll clock to generate strobe
    #[inline(always)]
    pub fn stb_clk_sel(&mut self) -> StbClkSelW<CFGrs> {
        StbClkSelW::new(self, 9)
    }
    ///Bits 16:19 - audprc clock divider, 0 and 1 means divide by 1
    #[inline(always)]
    pub fn audclk_div(&mut self) -> AudclkDivW<CFGrs> {
        AudclkDivW::new(self, 16)
    }
    ///Bit 20 - audprc clock divider update, write 1 to update
    #[inline(always)]
    pub fn audclk_div_update(&mut self) -> AudclkDivUpdateW<CFGrs> {
        AudclkDivUpdateW::new(self, 20)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFGrs;
impl crate::RegisterSpec for CFGrs {
    type Ux = u32;
}
///`read()` method returns [`cfg::R`](R) reader structure
impl crate::Readable for CFGrs {}
///`write(|w| ..)` method takes [`cfg::W`](W) writer structure
impl crate::Writable for CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG to value 0
impl crate::Resettable for CFGrs {
    const RESET_VALUE: u32 = 0;
}
