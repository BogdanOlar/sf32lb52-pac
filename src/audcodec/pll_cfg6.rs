///Register `PLL_CFG6` reader
pub type R = crate::R<PLL_CFG6rs>;
///Register `PLL_CFG6` writer
pub type W = crate::W<PLL_CFG6rs>;
///Field `SEL_TST_CLK` reader - select clk to test
pub type SelTstClkR = crate::FieldReader;
///Field `SEL_TST_CLK` writer - select clk to test
pub type SelTstClkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN_TST_CLK` reader - enable test clk
pub type EnTstClkR = crate::BitReader;
///Field `EN_TST_CLK` writer - enable test clk
pub type EnTstClkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_CLK_RCCAL` reader - enable RC CAL clk
pub type EnClkRccalR = crate::BitReader;
///Field `EN_CLK_RCCAL` writer - enable RC CAL clk
pub type EnClkRccalW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_CHOP_MICBIAS` reader - select micbias chop clk
pub type SelClkChopMicbiasR = crate::FieldReader;
///Field `SEL_CLK_CHOP_MICBIAS` writer - select micbias chop clk
pub type SelClkChopMicbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_CLK_CHOP_MICBIAS` reader - enable micbias chop clk
pub type EnClkChopMicbiasR = crate::BitReader;
///Field `EN_CLK_CHOP_MICBIAS` writer - enable micbias chop clk
pub type EnClkChopMicbiasW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_ADC2` reader - select adc2 clk
pub type SelClkAdc2R = crate::BitReader;
///Field `SEL_CLK_ADC2` writer - select adc2 clk
pub type SelClkAdc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVA_CLK_ADC2` reader - DIVA adc2 clk
pub type DivaClkAdc2R = crate::FieldReader;
///Field `DIVA_CLK_ADC2` writer - DIVA adc2 clk
pub type DivaClkAdc2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_CLK_ADC2` reader - enable adc2 clk
pub type EnClkAdc2R = crate::BitReader;
///Field `EN_CLK_ADC2` writer - enable adc2 clk
pub type EnClkAdc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_ADC1` reader - select adc1 clk
pub type SelClkAdc1R = crate::BitReader;
///Field `SEL_CLK_ADC1` writer - select adc1 clk
pub type SelClkAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVA_CLK_ADC1` reader - DIVA adc1 clk
pub type DivaClkAdc1R = crate::FieldReader;
///Field `DIVA_CLK_ADC1` writer - DIVA adc1 clk
pub type DivaClkAdc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_CLK_ADC1` reader - enable adc1 clk
pub type EnClkAdc1R = crate::BitReader;
///Field `EN_CLK_ADC1` writer - enable adc1 clk
pub type EnClkAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_ADC0` reader - select adc0 clk
pub type SelClkAdc0R = crate::BitReader;
///Field `SEL_CLK_ADC0` writer - select adc0 clk
pub type SelClkAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVA_CLK_ADC0` reader - DIVA adc0 clk
pub type DivaClkAdc0R = crate::FieldReader;
///Field `DIVA_CLK_ADC0` writer - DIVA adc0 clk
pub type DivaClkAdc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_CLK_ADC0` reader - enable adc0 clk
pub type EnClkAdc0R = crate::BitReader;
///Field `EN_CLK_ADC0` writer - enable adc0 clk
pub type EnClkAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_ADC_SOURCE` reader - 0: xtal, 1: pll
pub type SelClkAdcSourceR = crate::FieldReader;
///Field `SEL_CLK_ADC_SOURCE` writer - 0: xtal, 1: pll
pub type SelClkAdcSourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - select clk to test
    #[inline(always)]
    pub fn sel_tst_clk(&self) -> SelTstClkR {
        SelTstClkR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - enable test clk
    #[inline(always)]
    pub fn en_tst_clk(&self) -> EnTstClkR {
        EnTstClkR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable RC CAL clk
    #[inline(always)]
    pub fn en_clk_rccal(&self) -> EnClkRccalR {
        EnClkRccalR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - select micbias chop clk
    #[inline(always)]
    pub fn sel_clk_chop_micbias(&self) -> SelClkChopMicbiasR {
        SelClkChopMicbiasR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - enable micbias chop clk
    #[inline(always)]
    pub fn en_clk_chop_micbias(&self) -> EnClkChopMicbiasR {
        EnClkChopMicbiasR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - select adc2 clk
    #[inline(always)]
    pub fn sel_clk_adc2(&self) -> SelClkAdc2R {
        SelClkAdc2R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - DIVA adc2 clk
    #[inline(always)]
    pub fn diva_clk_adc2(&self) -> DivaClkAdc2R {
        DivaClkAdc2R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 13 - enable adc2 clk
    #[inline(always)]
    pub fn en_clk_adc2(&self) -> EnClkAdc2R {
        EnClkAdc2R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - select adc1 clk
    #[inline(always)]
    pub fn sel_clk_adc1(&self) -> SelClkAdc1R {
        SelClkAdc1R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:17 - DIVA adc1 clk
    #[inline(always)]
    pub fn diva_clk_adc1(&self) -> DivaClkAdc1R {
        DivaClkAdc1R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bit 18 - enable adc1 clk
    #[inline(always)]
    pub fn en_clk_adc1(&self) -> EnClkAdc1R {
        EnClkAdc1R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - select adc0 clk
    #[inline(always)]
    pub fn sel_clk_adc0(&self) -> SelClkAdc0R {
        SelClkAdc0R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:22 - DIVA adc0 clk
    #[inline(always)]
    pub fn diva_clk_adc0(&self) -> DivaClkAdc0R {
        DivaClkAdc0R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - enable adc0 clk
    #[inline(always)]
    pub fn en_clk_adc0(&self) -> EnClkAdc0R {
        EnClkAdc0R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - 0: xtal, 1: pll
    #[inline(always)]
    pub fn sel_clk_adc_source(&self) -> SelClkAdcSourceR {
        SelClkAdcSourceR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG6")
            .field("sel_clk_adc_source", &self.sel_clk_adc_source())
            .field("en_clk_adc0", &self.en_clk_adc0())
            .field("diva_clk_adc0", &self.diva_clk_adc0())
            .field("sel_clk_adc0", &self.sel_clk_adc0())
            .field("en_clk_adc1", &self.en_clk_adc1())
            .field("diva_clk_adc1", &self.diva_clk_adc1())
            .field("sel_clk_adc1", &self.sel_clk_adc1())
            .field("en_clk_adc2", &self.en_clk_adc2())
            .field("diva_clk_adc2", &self.diva_clk_adc2())
            .field("sel_clk_adc2", &self.sel_clk_adc2())
            .field("en_clk_chop_micbias", &self.en_clk_chop_micbias())
            .field("sel_clk_chop_micbias", &self.sel_clk_chop_micbias())
            .field("en_clk_rccal", &self.en_clk_rccal())
            .field("en_tst_clk", &self.en_tst_clk())
            .field("sel_tst_clk", &self.sel_tst_clk())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - select clk to test
    #[inline(always)]
    pub fn sel_tst_clk(&mut self) -> SelTstClkW<PLL_CFG6rs> {
        SelTstClkW::new(self, 0)
    }
    ///Bit 4 - enable test clk
    #[inline(always)]
    pub fn en_tst_clk(&mut self) -> EnTstClkW<PLL_CFG6rs> {
        EnTstClkW::new(self, 4)
    }
    ///Bit 5 - enable RC CAL clk
    #[inline(always)]
    pub fn en_clk_rccal(&mut self) -> EnClkRccalW<PLL_CFG6rs> {
        EnClkRccalW::new(self, 5)
    }
    ///Bits 6:7 - select micbias chop clk
    #[inline(always)]
    pub fn sel_clk_chop_micbias(&mut self) -> SelClkChopMicbiasW<PLL_CFG6rs> {
        SelClkChopMicbiasW::new(self, 6)
    }
    ///Bit 8 - enable micbias chop clk
    #[inline(always)]
    pub fn en_clk_chop_micbias(&mut self) -> EnClkChopMicbiasW<PLL_CFG6rs> {
        EnClkChopMicbiasW::new(self, 8)
    }
    ///Bit 9 - select adc2 clk
    #[inline(always)]
    pub fn sel_clk_adc2(&mut self) -> SelClkAdc2W<PLL_CFG6rs> {
        SelClkAdc2W::new(self, 9)
    }
    ///Bits 10:12 - DIVA adc2 clk
    #[inline(always)]
    pub fn diva_clk_adc2(&mut self) -> DivaClkAdc2W<PLL_CFG6rs> {
        DivaClkAdc2W::new(self, 10)
    }
    ///Bit 13 - enable adc2 clk
    #[inline(always)]
    pub fn en_clk_adc2(&mut self) -> EnClkAdc2W<PLL_CFG6rs> {
        EnClkAdc2W::new(self, 13)
    }
    ///Bit 14 - select adc1 clk
    #[inline(always)]
    pub fn sel_clk_adc1(&mut self) -> SelClkAdc1W<PLL_CFG6rs> {
        SelClkAdc1W::new(self, 14)
    }
    ///Bits 15:17 - DIVA adc1 clk
    #[inline(always)]
    pub fn diva_clk_adc1(&mut self) -> DivaClkAdc1W<PLL_CFG6rs> {
        DivaClkAdc1W::new(self, 15)
    }
    ///Bit 18 - enable adc1 clk
    #[inline(always)]
    pub fn en_clk_adc1(&mut self) -> EnClkAdc1W<PLL_CFG6rs> {
        EnClkAdc1W::new(self, 18)
    }
    ///Bit 19 - select adc0 clk
    #[inline(always)]
    pub fn sel_clk_adc0(&mut self) -> SelClkAdc0W<PLL_CFG6rs> {
        SelClkAdc0W::new(self, 19)
    }
    ///Bits 20:22 - DIVA adc0 clk
    #[inline(always)]
    pub fn diva_clk_adc0(&mut self) -> DivaClkAdc0W<PLL_CFG6rs> {
        DivaClkAdc0W::new(self, 20)
    }
    ///Bit 23 - enable adc0 clk
    #[inline(always)]
    pub fn en_clk_adc0(&mut self) -> EnClkAdc0W<PLL_CFG6rs> {
        EnClkAdc0W::new(self, 23)
    }
    ///Bits 24:25 - 0: xtal, 1: pll
    #[inline(always)]
    pub fn sel_clk_adc_source(&mut self) -> SelClkAdcSourceW<PLL_CFG6rs> {
        SelClkAdcSourceW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG6rs;
impl crate::RegisterSpec for PLL_CFG6rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg6::R`](R) reader structure
impl crate::Readable for PLL_CFG6rs {}
///`write(|w| ..)` method takes [`pll_cfg6::W`](W) writer structure
impl crate::Writable for PLL_CFG6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG6 to value 0
impl crate::Resettable for PLL_CFG6rs {
    const RESET_VALUE: u32 = 0;
}
