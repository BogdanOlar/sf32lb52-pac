///Register `PLL_CFG5` reader
pub type R = crate::R<PLL_CFG5rs>;
///Register `PLL_CFG5` writer
pub type W = crate::W<PLL_CFG5rs>;
///Field `DIVB_CLK_CHOP_BG` reader - DIVB bg chop clk
pub type DivbClkChopBgR = crate::FieldReader;
///Field `DIVB_CLK_CHOP_BG` writer - DIVB bg chop clk
pub type DivbClkChopBgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIVA_CLK_CHOP_BG` reader - DIVA bg chop clk
pub type DivaClkChopBgR = crate::FieldReader;
///Field `DIVA_CLK_CHOP_BG` writer - DIVA bg chop clk
pub type DivaClkChopBgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EN_CLK_CHOP_BG` reader - enable bg chop clk
pub type EnClkChopBgR = crate::BitReader;
///Field `EN_CLK_CHOP_BG` writer - enable bg chop clk
pub type EnClkChopBgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVB_CLK_CHOP_REFGEN` reader - DIVB ref chop clk
pub type DivbClkChopRefgenR = crate::FieldReader;
///Field `DIVB_CLK_CHOP_REFGEN` writer - DIVB ref chop clk
pub type DivbClkChopRefgenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIVA_CLK_CHOP_REFGEN` reader - DIVA ref chop clk
pub type DivaClkChopRefgenR = crate::FieldReader;
///Field `DIVA_CLK_CHOP_REFGEN` writer - DIVA ref chop clk
pub type DivaClkChopRefgenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EN_CLK_CHOP_REFGEN` reader - enable ref chop clk
pub type EnClkChopRefgenR = crate::BitReader;
///Field `EN_CLK_CHOP_REFGEN` writer - enable ref chop clk
pub type EnClkChopRefgenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVB_CLK_CHOP_DAC2` reader - DIVB dac2 chop clk
pub type DivbClkChopDac2R = crate::FieldReader;
///Field `DIVB_CLK_CHOP_DAC2` writer - DIVB dac2 chop clk
pub type DivbClkChopDac2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIVA_CLK_CHOP_DAC2` reader - DIVA dac2 chop clk
pub type DivaClkChopDac2R = crate::FieldReader;
///Field `DIVA_CLK_CHOP_DAC2` writer - DIVA dac2 chop clk
pub type DivaClkChopDac2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_CLK_CHOP_DAC2` reader - enable dac2 chop clk
pub type EnClkChopDac2R = crate::BitReader;
///Field `EN_CLK_CHOP_DAC2` writer - enable dac2 chop clk
pub type EnClkChopDac2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVA_CLK_DAC2` reader - DIVA dac2 clk
pub type DivaClkDac2R = crate::FieldReader;
///Field `DIVA_CLK_DAC2` writer - DIVA dac2 clk
pub type DivaClkDac2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EN_CLK_DAC2` reader - enable dac2 clk
pub type EnClkDac2R = crate::BitReader;
///Field `EN_CLK_DAC2` writer - enable dac2 clk
pub type EnClkDac2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_DAC2` reader - 1: select 9.6MHz as DAC clock
pub type SelClkDac2R = crate::BitReader;
///Field `SEL_CLK_DAC2` writer - 1: select 9.6MHz as DAC clock
pub type SelClkDac2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - DIVB bg chop clk
    #[inline(always)]
    pub fn divb_clk_chop_bg(&self) -> DivbClkChopBgR {
        DivbClkChopBgR::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - DIVA bg chop clk
    #[inline(always)]
    pub fn diva_clk_chop_bg(&self) -> DivaClkChopBgR {
        DivaClkChopBgR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - enable bg chop clk
    #[inline(always)]
    pub fn en_clk_chop_bg(&self) -> EnClkChopBgR {
        EnClkChopBgR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - DIVB ref chop clk
    #[inline(always)]
    pub fn divb_clk_chop_refgen(&self) -> DivbClkChopRefgenR {
        DivbClkChopRefgenR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:14 - DIVA ref chop clk
    #[inline(always)]
    pub fn diva_clk_chop_refgen(&self) -> DivaClkChopRefgenR {
        DivaClkChopRefgenR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - enable ref chop clk
    #[inline(always)]
    pub fn en_clk_chop_refgen(&self) -> EnClkChopRefgenR {
        EnClkChopRefgenR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - DIVB dac2 chop clk
    #[inline(always)]
    pub fn divb_clk_chop_dac2(&self) -> DivbClkChopDac2R {
        DivbClkChopDac2R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - DIVA dac2 chop clk
    #[inline(always)]
    pub fn diva_clk_chop_dac2(&self) -> DivaClkChopDac2R {
        DivaClkChopDac2R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - enable dac2 chop clk
    #[inline(always)]
    pub fn en_clk_chop_dac2(&self) -> EnClkChopDac2R {
        EnClkChopDac2R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:26 - DIVA dac2 clk
    #[inline(always)]
    pub fn diva_clk_dac2(&self) -> DivaClkDac2R {
        DivaClkDac2R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    ///Bit 27 - enable dac2 clk
    #[inline(always)]
    pub fn en_clk_dac2(&self) -> EnClkDac2R {
        EnClkDac2R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - 1: select 9.6MHz as DAC clock
    #[inline(always)]
    pub fn sel_clk_dac2(&self) -> SelClkDac2R {
        SelClkDac2R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG5")
            .field("sel_clk_dac2", &self.sel_clk_dac2())
            .field("en_clk_dac2", &self.en_clk_dac2())
            .field("diva_clk_dac2", &self.diva_clk_dac2())
            .field("en_clk_chop_dac2", &self.en_clk_chop_dac2())
            .field("diva_clk_chop_dac2", &self.diva_clk_chop_dac2())
            .field("divb_clk_chop_dac2", &self.divb_clk_chop_dac2())
            .field("en_clk_chop_refgen", &self.en_clk_chop_refgen())
            .field("diva_clk_chop_refgen", &self.diva_clk_chop_refgen())
            .field("divb_clk_chop_refgen", &self.divb_clk_chop_refgen())
            .field("en_clk_chop_bg", &self.en_clk_chop_bg())
            .field("diva_clk_chop_bg", &self.diva_clk_chop_bg())
            .field("divb_clk_chop_bg", &self.divb_clk_chop_bg())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DIVB bg chop clk
    #[inline(always)]
    pub fn divb_clk_chop_bg(&mut self) -> DivbClkChopBgW<PLL_CFG5rs> {
        DivbClkChopBgW::new(self, 0)
    }
    ///Bits 2:6 - DIVA bg chop clk
    #[inline(always)]
    pub fn diva_clk_chop_bg(&mut self) -> DivaClkChopBgW<PLL_CFG5rs> {
        DivaClkChopBgW::new(self, 2)
    }
    ///Bit 7 - enable bg chop clk
    #[inline(always)]
    pub fn en_clk_chop_bg(&mut self) -> EnClkChopBgW<PLL_CFG5rs> {
        EnClkChopBgW::new(self, 7)
    }
    ///Bits 8:9 - DIVB ref chop clk
    #[inline(always)]
    pub fn divb_clk_chop_refgen(&mut self) -> DivbClkChopRefgenW<PLL_CFG5rs> {
        DivbClkChopRefgenW::new(self, 8)
    }
    ///Bits 10:14 - DIVA ref chop clk
    #[inline(always)]
    pub fn diva_clk_chop_refgen(&mut self) -> DivaClkChopRefgenW<PLL_CFG5rs> {
        DivaClkChopRefgenW::new(self, 10)
    }
    ///Bit 15 - enable ref chop clk
    #[inline(always)]
    pub fn en_clk_chop_refgen(&mut self) -> EnClkChopRefgenW<PLL_CFG5rs> {
        EnClkChopRefgenW::new(self, 15)
    }
    ///Bits 16:17 - DIVB dac2 chop clk
    #[inline(always)]
    pub fn divb_clk_chop_dac2(&mut self) -> DivbClkChopDac2W<PLL_CFG5rs> {
        DivbClkChopDac2W::new(self, 16)
    }
    ///Bits 18:20 - DIVA dac2 chop clk
    #[inline(always)]
    pub fn diva_clk_chop_dac2(&mut self) -> DivaClkChopDac2W<PLL_CFG5rs> {
        DivaClkChopDac2W::new(self, 18)
    }
    ///Bit 21 - enable dac2 chop clk
    #[inline(always)]
    pub fn en_clk_chop_dac2(&mut self) -> EnClkChopDac2W<PLL_CFG5rs> {
        EnClkChopDac2W::new(self, 21)
    }
    ///Bits 22:26 - DIVA dac2 clk
    #[inline(always)]
    pub fn diva_clk_dac2(&mut self) -> DivaClkDac2W<PLL_CFG5rs> {
        DivaClkDac2W::new(self, 22)
    }
    ///Bit 27 - enable dac2 clk
    #[inline(always)]
    pub fn en_clk_dac2(&mut self) -> EnClkDac2W<PLL_CFG5rs> {
        EnClkDac2W::new(self, 27)
    }
    ///Bit 28 - 1: select 9.6MHz as DAC clock
    #[inline(always)]
    pub fn sel_clk_dac2(&mut self) -> SelClkDac2W<PLL_CFG5rs> {
        SelClkDac2W::new(self, 28)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG5rs;
impl crate::RegisterSpec for PLL_CFG5rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg5::R`](R) reader structure
impl crate::Readable for PLL_CFG5rs {}
///`write(|w| ..)` method takes [`pll_cfg5::W`](W) writer structure
impl crate::Writable for PLL_CFG5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG5 to value 0
impl crate::Resettable for PLL_CFG5rs {
    const RESET_VALUE: u32 = 0;
}
