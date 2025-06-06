///Register `PLL_CFG4` reader
pub type R = crate::R<PLL_CFG4rs>;
///Register `PLL_CFG4` writer
pub type W = crate::W<PLL_CFG4rs>;
///Field `DIVB_CLK_CHOP_DAC` reader - DIVB dac chop clk
pub type DivbClkChopDacR = crate::FieldReader;
///Field `DIVB_CLK_CHOP_DAC` writer - DIVB dac chop clk
pub type DivbClkChopDacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIVA_CLK_CHOP_DAC` reader - DIVA dac chop clk
pub type DivaClkChopDacR = crate::FieldReader;
///Field `DIVA_CLK_CHOP_DAC` writer - DIVA dac chop clk
pub type DivaClkChopDacW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_CLK_CHOP_DAC` reader - enable dac chop clk
pub type EnClkChopDacR = crate::BitReader;
///Field `EN_CLK_CHOP_DAC` writer - enable dac chop clk
pub type EnClkChopDacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVA_CLK_DAC` reader - DIVA dac clk
pub type DivaClkDacR = crate::FieldReader;
///Field `DIVA_CLK_DAC` writer - DIVA dac clk
pub type DivaClkDacW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EN_CLK_DAC` reader - enable dac clk
pub type EnClkDacR = crate::BitReader;
///Field `EN_CLK_DAC` writer - enable dac clk
pub type EnClkDacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_DAC` reader - 1: select 9.6MHz as DAC clock
pub type SelClkDacR = crate::BitReader;
///Field `SEL_CLK_DAC` writer - 1: select 9.6MHz as DAC clock
pub type SelClkDacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_CLK_DAC_SOURCE` reader - 0: xtal 1: pll
pub type SelClkDacSourceR = crate::FieldReader;
///Field `SEL_CLK_DAC_SOURCE` writer - 0: xtal 1: pll
pub type SelClkDacSourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_CLK_DIG` reader - select dig clk 0: pll 1: 24MHz from xtal
pub type SelClkDigR = crate::BitReader;
///Field `SEL_CLK_DIG` writer - select dig clk 0: pll 1: 24MHz from xtal
pub type SelClkDigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DIG_STR` reader - strength
pub type ClkDigStrR = crate::FieldReader;
///Field `CLK_DIG_STR` writer - strength
pub type ClkDigStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIVA_CLK_DIG` reader - DIVA dig clk
pub type DivaClkDigR = crate::FieldReader;
///Field `DIVA_CLK_DIG` writer - DIVA dig clk
pub type DivaClkDigW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EN_CLK_DIG` reader - enable dig clk
pub type EnClkDigR = crate::BitReader;
///Field `EN_CLK_DIG` writer - enable dig clk
pub type EnClkDigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - DIVB dac chop clk
    #[inline(always)]
    pub fn divb_clk_chop_dac(&self) -> DivbClkChopDacR {
        DivbClkChopDacR::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - DIVA dac chop clk
    #[inline(always)]
    pub fn diva_clk_chop_dac(&self) -> DivaClkChopDacR {
        DivaClkChopDacR::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bit 5 - enable dac chop clk
    #[inline(always)]
    pub fn en_clk_chop_dac(&self) -> EnClkChopDacR {
        EnClkChopDacR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - DIVA dac clk
    #[inline(always)]
    pub fn diva_clk_dac(&self) -> DivaClkDacR {
        DivaClkDacR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - enable dac clk
    #[inline(always)]
    pub fn en_clk_dac(&self) -> EnClkDacR {
        EnClkDacR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 1: select 9.6MHz as DAC clock
    #[inline(always)]
    pub fn sel_clk_dac(&self) -> SelClkDacR {
        SelClkDacR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - 0: xtal 1: pll
    #[inline(always)]
    pub fn sel_clk_dac_source(&self) -> SelClkDacSourceR {
        SelClkDacSourceR::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - select dig clk 0: pll 1: 24MHz from xtal
    #[inline(always)]
    pub fn sel_clk_dig(&self) -> SelClkDigR {
        SelClkDigR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - strength
    #[inline(always)]
    pub fn clk_dig_str(&self) -> ClkDigStrR {
        ClkDigStrR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:22 - DIVA dig clk
    #[inline(always)]
    pub fn diva_clk_dig(&self) -> DivaClkDigR {
        DivaClkDigR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - enable dig clk
    #[inline(always)]
    pub fn en_clk_dig(&self) -> EnClkDigR {
        EnClkDigR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG4")
            .field("rsvd", &self.rsvd())
            .field("en_clk_dig", &self.en_clk_dig())
            .field("diva_clk_dig", &self.diva_clk_dig())
            .field("clk_dig_str", &self.clk_dig_str())
            .field("sel_clk_dig", &self.sel_clk_dig())
            .field("sel_clk_dac_source", &self.sel_clk_dac_source())
            .field("sel_clk_dac", &self.sel_clk_dac())
            .field("en_clk_dac", &self.en_clk_dac())
            .field("diva_clk_dac", &self.diva_clk_dac())
            .field("en_clk_chop_dac", &self.en_clk_chop_dac())
            .field("diva_clk_chop_dac", &self.diva_clk_chop_dac())
            .field("divb_clk_chop_dac", &self.divb_clk_chop_dac())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DIVB dac chop clk
    #[inline(always)]
    pub fn divb_clk_chop_dac(&mut self) -> DivbClkChopDacW<PLL_CFG4rs> {
        DivbClkChopDacW::new(self, 0)
    }
    ///Bits 2:4 - DIVA dac chop clk
    #[inline(always)]
    pub fn diva_clk_chop_dac(&mut self) -> DivaClkChopDacW<PLL_CFG4rs> {
        DivaClkChopDacW::new(self, 2)
    }
    ///Bit 5 - enable dac chop clk
    #[inline(always)]
    pub fn en_clk_chop_dac(&mut self) -> EnClkChopDacW<PLL_CFG4rs> {
        EnClkChopDacW::new(self, 5)
    }
    ///Bits 6:10 - DIVA dac clk
    #[inline(always)]
    pub fn diva_clk_dac(&mut self) -> DivaClkDacW<PLL_CFG4rs> {
        DivaClkDacW::new(self, 6)
    }
    ///Bit 11 - enable dac clk
    #[inline(always)]
    pub fn en_clk_dac(&mut self) -> EnClkDacW<PLL_CFG4rs> {
        EnClkDacW::new(self, 11)
    }
    ///Bit 12 - 1: select 9.6MHz as DAC clock
    #[inline(always)]
    pub fn sel_clk_dac(&mut self) -> SelClkDacW<PLL_CFG4rs> {
        SelClkDacW::new(self, 12)
    }
    ///Bits 13:14 - 0: xtal 1: pll
    #[inline(always)]
    pub fn sel_clk_dac_source(&mut self) -> SelClkDacSourceW<PLL_CFG4rs> {
        SelClkDacSourceW::new(self, 13)
    }
    ///Bit 15 - select dig clk 0: pll 1: 24MHz from xtal
    #[inline(always)]
    pub fn sel_clk_dig(&mut self) -> SelClkDigW<PLL_CFG4rs> {
        SelClkDigW::new(self, 15)
    }
    ///Bits 16:17 - strength
    #[inline(always)]
    pub fn clk_dig_str(&mut self) -> ClkDigStrW<PLL_CFG4rs> {
        ClkDigStrW::new(self, 16)
    }
    ///Bits 18:22 - DIVA dig clk
    #[inline(always)]
    pub fn diva_clk_dig(&mut self) -> DivaClkDigW<PLL_CFG4rs> {
        DivaClkDigW::new(self, 18)
    }
    ///Bit 23 - enable dig clk
    #[inline(always)]
    pub fn en_clk_dig(&mut self) -> EnClkDigW<PLL_CFG4rs> {
        EnClkDigW::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PLL_CFG4rs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG4rs;
impl crate::RegisterSpec for PLL_CFG4rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg4::R`](R) reader structure
impl crate::Readable for PLL_CFG4rs {}
///`write(|w| ..)` method takes [`pll_cfg4::W`](W) writer structure
impl crate::Writable for PLL_CFG4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG4 to value 0
impl crate::Resettable for PLL_CFG4rs {
    const RESET_VALUE: u32 = 0;
}
