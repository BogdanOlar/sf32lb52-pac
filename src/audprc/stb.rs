///Register `STB` reader
pub type R = crate::R<STBrs>;
///Register `STB` writer
pub type W = crate::W<STBrs>;
///Field `DAC_DIV` reader - dac strobe divider
pub type DacDivR = crate::FieldReader<u16>;
///Field `DAC_DIV` writer - dac strobe divider
pub type DacDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ADC_DIV` reader - adc strobe divider
pub type AdcDivR = crate::FieldReader<u16>;
///Field `ADC_DIV` writer - adc strobe divider
pub type AdcDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - dac strobe divider
    #[inline(always)]
    pub fn dac_div(&self) -> DacDivR {
        DacDivR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - adc strobe divider
    #[inline(always)]
    pub fn adc_div(&self) -> AdcDivR {
        AdcDivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STB")
            .field("adc_div", &self.adc_div())
            .field("dac_div", &self.dac_div())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - dac strobe divider
    #[inline(always)]
    pub fn dac_div(&mut self) -> DacDivW<STBrs> {
        DacDivW::new(self, 0)
    }
    ///Bits 16:31 - adc strobe divider
    #[inline(always)]
    pub fn adc_div(&mut self) -> AdcDivW<STBrs> {
        AdcDivW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`stb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STBrs;
impl crate::RegisterSpec for STBrs {
    type Ux = u32;
}
///`read()` method returns [`stb::R`](R) reader structure
impl crate::Readable for STBrs {}
///`write(|w| ..)` method takes [`stb::W`](W) writer structure
impl crate::Writable for STBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STB to value 0x0001_0001
impl crate::Resettable for STBrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
