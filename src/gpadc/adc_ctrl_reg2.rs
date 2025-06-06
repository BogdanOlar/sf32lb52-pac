///Register `ADC_CTRL_REG2` reader
pub type R = crate::R<ADC_CTRL_REG2rs>;
///Register `ADC_CTRL_REG2` writer
pub type W = crate::W<ADC_CTRL_REG2rs>;
///Field `SAMP_WIDTH` reader -
pub type SampWidthR = crate::FieldReader<u32>;
///Field `SAMP_WIDTH` writer -
pub type SampWidthW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `CONV_WIDTH` reader -
pub type ConvWidthR = crate::FieldReader;
///Field `CONV_WIDTH` writer -
pub type ConvWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23
    #[inline(always)]
    pub fn samp_width(&self) -> SampWidthR {
        SampWidthR::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn conv_width(&self) -> ConvWidthR {
        ConvWidthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CTRL_REG2")
            .field("conv_width", &self.conv_width())
            .field("samp_width", &self.samp_width())
            .finish()
    }
}
impl W {
    ///Bits 0:23
    #[inline(always)]
    pub fn samp_width(&mut self) -> SampWidthW<ADC_CTRL_REG2rs> {
        SampWidthW::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn conv_width(&mut self) -> ConvWidthW<ADC_CTRL_REG2rs> {
        ConvWidthW::new(self, 24)
    }
}
///ADC Control Register2
///
///You can [`read`](crate::Reg::read) this register and get [`adc_ctrl_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctrl_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_CTRL_REG2rs;
impl crate::RegisterSpec for ADC_CTRL_REG2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_ctrl_reg2::R`](R) reader structure
impl crate::Readable for ADC_CTRL_REG2rs {}
///`write(|w| ..)` method takes [`adc_ctrl_reg2::W`](W) writer structure
impl crate::Writable for ADC_CTRL_REG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CTRL_REG2 to value 0x0130
impl crate::Resettable for ADC_CTRL_REG2rs {
    const RESET_VALUE: u32 = 0x0130;
}
