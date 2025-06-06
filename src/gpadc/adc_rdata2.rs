///Register `ADC_RDATA2` reader
pub type R = crate::R<ADC_RDATA2rs>;
///Register `ADC_RDATA2` writer
pub type W = crate::W<ADC_RDATA2rs>;
///Field `SLOT4_RDATA` reader -
pub type Slot4RdataR = crate::FieldReader<u16>;
///Field `SLOT4_RDATA` writer -
pub type Slot4RdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SLOT5_RDATA` reader -
pub type Slot5RdataR = crate::FieldReader<u16>;
///Field `SLOT5_RDATA` writer -
pub type Slot5RdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn slot4_rdata(&self) -> Slot4RdataR {
        Slot4RdataR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn slot5_rdata(&self) -> Slot5RdataR {
        Slot5RdataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_RDATA2")
            .field("slot5_rdata", &self.slot5_rdata())
            .field("slot4_rdata", &self.slot4_rdata())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn slot4_rdata(&mut self) -> Slot4RdataW<ADC_RDATA2rs> {
        Slot4RdataW::new(self, 0)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn slot5_rdata(&mut self) -> Slot5RdataW<ADC_RDATA2rs> {
        Slot5RdataW::new(self, 16)
    }
}
///ADC Read Data2
///
///You can [`read`](crate::Reg::read) this register and get [`adc_rdata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_rdata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_RDATA2rs;
impl crate::RegisterSpec for ADC_RDATA2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_rdata2::R`](R) reader structure
impl crate::Readable for ADC_RDATA2rs {}
///`write(|w| ..)` method takes [`adc_rdata2::W`](W) writer structure
impl crate::Writable for ADC_RDATA2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_RDATA2 to value 0
impl crate::Resettable for ADC_RDATA2rs {
    const RESET_VALUE: u32 = 0;
}
