///Register `PLL_CFG3` reader
pub type R = crate::R<PLL_CFG3rs>;
///Register `PLL_CFG3` writer
pub type W = crate::W<PLL_CFG3rs>;
///Field `SDIN` reader - sdm input
pub type SdinR = crate::FieldReader<u32>;
///Field `SDIN` writer - sdm input
pub type SdinW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `FCW` reader - FCW
pub type FcwR = crate::FieldReader;
///Field `FCW` writer - FCW
pub type FcwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SDM_UPDATE` reader - write 1 to update FCW and SDIN value
pub type SdmUpdateR = crate::BitReader;
///Field `SDM_UPDATE` writer - write 1 to update FCW and SDIN value
pub type SdmUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMIN_BYPASS` reader - 1: bypass FCW and SDIN sdm control signal
pub type SdminBypassR = crate::BitReader;
///Field `SDMIN_BYPASS` writer - 1: bypass FCW and SDIN sdm control signal
pub type SdminBypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDM_MODE` reader - sdm mode
pub type SdmModeR = crate::BitReader;
///Field `SDM_MODE` writer - sdm mode
pub type SdmModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_SDM_DITHER` reader - enable sdm dither
pub type EnSdmDitherR = crate::BitReader;
///Field `EN_SDM_DITHER` writer - enable sdm dither
pub type EnSdmDitherW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDM_DITHER` reader - sdm dither
pub type SdmDitherR = crate::BitReader;
///Field `SDM_DITHER` writer - sdm dither
pub type SdmDitherW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_SDM` reader - enable sdm
pub type EnSdmR = crate::BitReader;
///Field `EN_SDM` writer - enable sdm
pub type EnSdmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMCLK_POL` reader - sdm dig clk polarity
pub type SdmclkPolR = crate::BitReader;
///Field `SDMCLK_POL` writer - sdm dig clk polarity
pub type SdmclkPolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:19 - sdm input
    #[inline(always)]
    pub fn sdin(&self) -> SdinR {
        SdinR::new(self.bits & 0x000f_ffff)
    }
    ///Bits 20:24 - FCW
    #[inline(always)]
    pub fn fcw(&self) -> FcwR {
        FcwR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 25 - write 1 to update FCW and SDIN value
    #[inline(always)]
    pub fn sdm_update(&self) -> SdmUpdateR {
        SdmUpdateR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - 1: bypass FCW and SDIN sdm control signal
    #[inline(always)]
    pub fn sdmin_bypass(&self) -> SdminBypassR {
        SdminBypassR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - sdm mode
    #[inline(always)]
    pub fn sdm_mode(&self) -> SdmModeR {
        SdmModeR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - enable sdm dither
    #[inline(always)]
    pub fn en_sdm_dither(&self) -> EnSdmDitherR {
        EnSdmDitherR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - sdm dither
    #[inline(always)]
    pub fn sdm_dither(&self) -> SdmDitherR {
        SdmDitherR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable sdm
    #[inline(always)]
    pub fn en_sdm(&self) -> EnSdmR {
        EnSdmR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - sdm dig clk polarity
    #[inline(always)]
    pub fn sdmclk_pol(&self) -> SdmclkPolR {
        SdmclkPolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG3")
            .field("sdmclk_pol", &self.sdmclk_pol())
            .field("en_sdm", &self.en_sdm())
            .field("sdm_dither", &self.sdm_dither())
            .field("en_sdm_dither", &self.en_sdm_dither())
            .field("sdm_mode", &self.sdm_mode())
            .field("sdmin_bypass", &self.sdmin_bypass())
            .field("sdm_update", &self.sdm_update())
            .field("fcw", &self.fcw())
            .field("sdin", &self.sdin())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - sdm input
    #[inline(always)]
    pub fn sdin(&mut self) -> SdinW<PLL_CFG3rs> {
        SdinW::new(self, 0)
    }
    ///Bits 20:24 - FCW
    #[inline(always)]
    pub fn fcw(&mut self) -> FcwW<PLL_CFG3rs> {
        FcwW::new(self, 20)
    }
    ///Bit 25 - write 1 to update FCW and SDIN value
    #[inline(always)]
    pub fn sdm_update(&mut self) -> SdmUpdateW<PLL_CFG3rs> {
        SdmUpdateW::new(self, 25)
    }
    ///Bit 26 - 1: bypass FCW and SDIN sdm control signal
    #[inline(always)]
    pub fn sdmin_bypass(&mut self) -> SdminBypassW<PLL_CFG3rs> {
        SdminBypassW::new(self, 26)
    }
    ///Bit 27 - sdm mode
    #[inline(always)]
    pub fn sdm_mode(&mut self) -> SdmModeW<PLL_CFG3rs> {
        SdmModeW::new(self, 27)
    }
    ///Bit 28 - enable sdm dither
    #[inline(always)]
    pub fn en_sdm_dither(&mut self) -> EnSdmDitherW<PLL_CFG3rs> {
        EnSdmDitherW::new(self, 28)
    }
    ///Bit 29 - sdm dither
    #[inline(always)]
    pub fn sdm_dither(&mut self) -> SdmDitherW<PLL_CFG3rs> {
        SdmDitherW::new(self, 29)
    }
    ///Bit 30 - enable sdm
    #[inline(always)]
    pub fn en_sdm(&mut self) -> EnSdmW<PLL_CFG3rs> {
        EnSdmW::new(self, 30)
    }
    ///Bit 31 - sdm dig clk polarity
    #[inline(always)]
    pub fn sdmclk_pol(&mut self) -> SdmclkPolW<PLL_CFG3rs> {
        SdmclkPolW::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG3rs;
impl crate::RegisterSpec for PLL_CFG3rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg3::R`](R) reader structure
impl crate::Readable for PLL_CFG3rs {}
///`write(|w| ..)` method takes [`pll_cfg3::W`](W) writer structure
impl crate::Writable for PLL_CFG3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG3 to value 0
impl crate::Resettable for PLL_CFG3rs {
    const RESET_VALUE: u32 = 0;
}
