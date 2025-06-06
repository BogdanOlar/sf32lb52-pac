///Register `CFG` reader
pub type R = crate::R<CFGrs>;
///Register `CFG` writer
pub type W = crate::W<CFGrs>;
///Field `ADC_ENABLE` reader - adc codec enable
pub type AdcEnableR = crate::BitReader;
///Field `ADC_ENABLE` writer - adc codec enable
pub type AdcEnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_ENABLE` reader - dac codec enable
pub type DacEnableR = crate::BitReader;
///Field `DAC_ENABLE` writer - dac codec enable
pub type DacEnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_1K_MODE` reader - codec dac sine 1k mode
pub type Dac1kModeR = crate::BitReader;
///Field `DAC_1K_MODE` writer - codec dac sine 1k mode
pub type Dac1kModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_EN_DLY_SEL` reader - codec adc enable delay count 0: no delay 1: 32 pclk 2: 64 pclk 3: 128 pclk
pub type AdcEnDlySelR = crate::FieldReader;
///Field `ADC_EN_DLY_SEL` writer - codec adc enable delay count 0: no delay 1: 32 pclk 2: 64 pclk 3: 128 pclk
pub type AdcEnDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - adc codec enable
    #[inline(always)]
    pub fn adc_enable(&self) -> AdcEnableR {
        AdcEnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - dac codec enable
    #[inline(always)]
    pub fn dac_enable(&self) -> DacEnableR {
        DacEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - codec dac sine 1k mode
    #[inline(always)]
    pub fn dac_1k_mode(&self) -> Dac1kModeR {
        Dac1kModeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - codec adc enable delay count 0: no delay 1: 32 pclk 2: 64 pclk 3: 128 pclk
    #[inline(always)]
    pub fn adc_en_dly_sel(&self) -> AdcEnDlySelR {
        AdcEnDlySelR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("adc_en_dly_sel", &self.adc_en_dly_sel())
            .field("dac_1k_mode", &self.dac_1k_mode())
            .field("dac_enable", &self.dac_enable())
            .field("adc_enable", &self.adc_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - adc codec enable
    #[inline(always)]
    pub fn adc_enable(&mut self) -> AdcEnableW<CFGrs> {
        AdcEnableW::new(self, 0)
    }
    ///Bit 1 - dac codec enable
    #[inline(always)]
    pub fn dac_enable(&mut self) -> DacEnableW<CFGrs> {
        DacEnableW::new(self, 1)
    }
    ///Bit 2 - codec dac sine 1k mode
    #[inline(always)]
    pub fn dac_1k_mode(&mut self) -> Dac1kModeW<CFGrs> {
        Dac1kModeW::new(self, 2)
    }
    ///Bits 3:4 - codec adc enable delay count 0: no delay 1: 32 pclk 2: 64 pclk 3: 128 pclk
    #[inline(always)]
    pub fn adc_en_dly_sel(&mut self) -> AdcEnDlySelW<CFGrs> {
        AdcEnDlySelW::new(self, 3)
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
