///Register `ADC_ANA_CFG` reader
pub type R = crate::R<ADC_ANA_CFGrs>;
///Register `ADC_ANA_CFG` writer
pub type W = crate::W<ADC_ANA_CFGrs>;
///Field `MICBIAS_CHOP_EN` reader - micbias chopping enable
pub type MicbiasChopEnR = crate::BitReader;
///Field `MICBIAS_CHOP_EN` writer - micbias chopping enable
pub type MicbiasChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MICBIAS_EN` reader - micbias enable
pub type MicbiasEnR = crate::BitReader;
///Field `MICBIAS_EN` writer - micbias enable
pub type MicbiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAPCODE` reader - ADC cap code
pub type CapcodeR = crate::FieldReader;
///Field `CAPCODE` writer - ADC cap code
pub type CapcodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - micbias chopping enable
    #[inline(always)]
    pub fn micbias_chop_en(&self) -> MicbiasChopEnR {
        MicbiasChopEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - micbias enable
    #[inline(always)]
    pub fn micbias_en(&self) -> MicbiasEnR {
        MicbiasEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - ADC cap code
    #[inline(always)]
    pub fn capcode(&self) -> CapcodeR {
        CapcodeR::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_ANA_CFG")
            .field("capcode", &self.capcode())
            .field("micbias_en", &self.micbias_en())
            .field("micbias_chop_en", &self.micbias_chop_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - micbias chopping enable
    #[inline(always)]
    pub fn micbias_chop_en(&mut self) -> MicbiasChopEnW<ADC_ANA_CFGrs> {
        MicbiasChopEnW::new(self, 0)
    }
    ///Bit 1 - micbias enable
    #[inline(always)]
    pub fn micbias_en(&mut self) -> MicbiasEnW<ADC_ANA_CFGrs> {
        MicbiasEnW::new(self, 1)
    }
    ///Bits 2:6 - ADC cap code
    #[inline(always)]
    pub fn capcode(&mut self) -> CapcodeW<ADC_ANA_CFGrs> {
        CapcodeW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ana_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ana_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_ANA_CFGrs;
impl crate::RegisterSpec for ADC_ANA_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ana_cfg::R`](R) reader structure
impl crate::Readable for ADC_ANA_CFGrs {}
///`write(|w| ..)` method takes [`adc_ana_cfg::W`](W) writer structure
impl crate::Writable for ADC_ANA_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_ANA_CFG to value 0
impl crate::Resettable for ADC_ANA_CFGrs {
    const RESET_VALUE: u32 = 0;
}
