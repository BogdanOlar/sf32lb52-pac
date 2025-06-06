///Register `DEBUG_LOOP` reader
pub type R = crate::R<DEBUG_LOOPrs>;
///Register `DEBUG_LOOP` writer
pub type W = crate::W<DEBUG_LOOPrs>;
///Field `DA2AD_LOOP_BACK` reader - TX-->RX Loop debug control: 0: disable 1: enable, internally connect TX SDTO to RX SDTI
pub type Da2adLoopBackR = crate::BitReader;
///Field `DA2AD_LOOP_BACK` writer - TX-->RX Loop debug control: 0: disable 1: enable, internally connect TX SDTO to RX SDTI
pub type Da2adLoopBackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AD2DA_LOOP_BACK` reader - RX-->TX Loop debug control: 0: disable 1: enable, internally connect RX Resampled PCM to TX Resample PCM input
pub type Ad2daLoopBackR = crate::BitReader;
///Field `AD2DA_LOOP_BACK` writer - RX-->TX Loop debug control: 0: disable 1: enable, internally connect RX Resampled PCM to TX Resample PCM input
pub type Ad2daLoopBackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SP_CLK_SEL` reader - clock select 0: xtal clock 1: pll clock
pub type SpClkSelR = crate::BitReader;
///Field `SP_CLK_SEL` writer - clock select 0: xtal clock 1: pll clock
pub type SpClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SP_CLK_DIV_UPDATE` reader - update sp clock divider
pub type SpClkDivUpdateR = crate::BitReader;
///Field `SP_CLK_DIV_UPDATE` writer - update sp clock divider
pub type SpClkDivUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SP_CLK_DIV` reader - sp clock divider value
pub type SpClkDivR = crate::FieldReader;
///Field `SP_CLK_DIV` writer - sp clock divider value
pub type SpClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - TX-->RX Loop debug control: 0: disable 1: enable, internally connect TX SDTO to RX SDTI
    #[inline(always)]
    pub fn da2ad_loop_back(&self) -> Da2adLoopBackR {
        Da2adLoopBackR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX-->TX Loop debug control: 0: disable 1: enable, internally connect RX Resampled PCM to TX Resample PCM input
    #[inline(always)]
    pub fn ad2da_loop_back(&self) -> Ad2daLoopBackR {
        Ad2daLoopBackR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - clock select 0: xtal clock 1: pll clock
    #[inline(always)]
    pub fn sp_clk_sel(&self) -> SpClkSelR {
        SpClkSelR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:7
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bit 8 - update sp clock divider
    #[inline(always)]
    pub fn sp_clk_div_update(&self) -> SpClkDivUpdateR {
        SpClkDivUpdateR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:23 - sp clock divider value
    #[inline(always)]
    pub fn sp_clk_div(&self) -> SpClkDivR {
        SpClkDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_LOOP")
            .field("rsvd", &self.rsvd())
            .field("sp_clk_div", &self.sp_clk_div())
            .field("rsvd2", &self.rsvd2())
            .field("sp_clk_div_update", &self.sp_clk_div_update())
            .field("rsvd3", &self.rsvd3())
            .field("sp_clk_sel", &self.sp_clk_sel())
            .field("ad2da_loop_back", &self.ad2da_loop_back())
            .field("da2ad_loop_back", &self.da2ad_loop_back())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX-->RX Loop debug control: 0: disable 1: enable, internally connect TX SDTO to RX SDTI
    #[inline(always)]
    pub fn da2ad_loop_back(&mut self) -> Da2adLoopBackW<DEBUG_LOOPrs> {
        Da2adLoopBackW::new(self, 0)
    }
    ///Bit 1 - RX-->TX Loop debug control: 0: disable 1: enable, internally connect RX Resampled PCM to TX Resample PCM input
    #[inline(always)]
    pub fn ad2da_loop_back(&mut self) -> Ad2daLoopBackW<DEBUG_LOOPrs> {
        Ad2daLoopBackW::new(self, 1)
    }
    ///Bit 2 - clock select 0: xtal clock 1: pll clock
    #[inline(always)]
    pub fn sp_clk_sel(&mut self) -> SpClkSelW<DEBUG_LOOPrs> {
        SpClkSelW::new(self, 2)
    }
    ///Bits 3:7
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<DEBUG_LOOPrs> {
        Rsvd3W::new(self, 3)
    }
    ///Bit 8 - update sp clock divider
    #[inline(always)]
    pub fn sp_clk_div_update(&mut self) -> SpClkDivUpdateW<DEBUG_LOOPrs> {
        SpClkDivUpdateW::new(self, 8)
    }
    ///Bits 9:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DEBUG_LOOPrs> {
        Rsvd2W::new(self, 9)
    }
    ///Bits 16:23 - sp clock divider value
    #[inline(always)]
    pub fn sp_clk_div(&mut self) -> SpClkDivW<DEBUG_LOOPrs> {
        SpClkDivW::new(self, 16)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DEBUG_LOOPrs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`debug_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DEBUG_LOOPrs;
impl crate::RegisterSpec for DEBUG_LOOPrs {
    type Ux = u32;
}
///`read()` method returns [`debug_loop::R`](R) reader structure
impl crate::Readable for DEBUG_LOOPrs {}
///`write(|w| ..)` method takes [`debug_loop::W`](W) writer structure
impl crate::Writable for DEBUG_LOOPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEBUG_LOOP to value 0
impl crate::Resettable for DEBUG_LOOPrs {
    const RESET_VALUE: u32 = 0;
}
