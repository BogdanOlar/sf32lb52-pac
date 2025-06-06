///Register `ADC_RDATA1` reader
pub type R = crate::R<ADC_RDATA1rs>;
///Register `ADC_RDATA1` writer
pub type W = crate::W<ADC_RDATA1rs>;
///Field `SLOT2_RDATA` reader -
pub type Slot2RdataR = crate::FieldReader<u16>;
///Field `SLOT2_RDATA` writer -
pub type Slot2RdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SLOT3_RDATA` reader -
pub type Slot3RdataR = crate::FieldReader<u16>;
///Field `SLOT3_RDATA` writer -
pub type Slot3RdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn slot2_rdata(&self) -> Slot2RdataR {
        Slot2RdataR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn slot3_rdata(&self) -> Slot3RdataR {
        Slot3RdataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_RDATA1")
            .field("rsvd", &self.rsvd())
            .field("slot3_rdata", &self.slot3_rdata())
            .field("rsvd2", &self.rsvd2())
            .field("slot2_rdata", &self.slot2_rdata())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn slot2_rdata(&mut self) -> Slot2RdataW<ADC_RDATA1rs> {
        Slot2RdataW::new(self, 0)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ADC_RDATA1rs> {
        Rsvd2W::new(self, 12)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn slot3_rdata(&mut self) -> Slot3RdataW<ADC_RDATA1rs> {
        Slot3RdataW::new(self, 16)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ADC_RDATA1rs> {
        RsvdW::new(self, 28)
    }
}
///ADC Read Data1
///
///You can [`read`](crate::Reg::read) this register and get [`adc_rdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_rdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_RDATA1rs;
impl crate::RegisterSpec for ADC_RDATA1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_rdata1::R`](R) reader structure
impl crate::Readable for ADC_RDATA1rs {}
///`write(|w| ..)` method takes [`adc_rdata1::W`](W) writer structure
impl crate::Writable for ADC_RDATA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_RDATA1 to value 0
impl crate::Resettable for ADC_RDATA1rs {
    const RESET_VALUE: u32 = 0;
}
