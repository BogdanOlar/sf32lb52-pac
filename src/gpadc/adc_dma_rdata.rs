///Register `ADC_DMA_RDATA` reader
pub type R = crate::R<ADC_DMA_RDATArs>;
///Register `ADC_DMA_RDATA` writer
pub type W = crate::W<ADC_DMA_RDATArs>;
///Field `DMA_RDATA` reader -
pub type DmaRdataR = crate::FieldReader<u16>;
///Field `DMA_RDATA` writer -
pub type DmaRdataW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMA_RDATA_RAW` reader -
pub type DmaRdataRawR = crate::FieldReader<u16>;
///Field `DMA_RDATA_RAW` writer -
pub type DmaRdataRawW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:12
    #[inline(always)]
    pub fn dma_rdata(&self) -> DmaRdataR {
        DmaRdataR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:28
    #[inline(always)]
    pub fn dma_rdata_raw(&self) -> DmaRdataRawR {
        DmaRdataRawR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    ///Bits 29:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DMA_RDATA")
            .field("rsvd", &self.rsvd())
            .field("dma_rdata_raw", &self.dma_rdata_raw())
            .field("rsvd2", &self.rsvd2())
            .field("dma_rdata", &self.dma_rdata())
            .finish()
    }
}
impl W {
    ///Bits 0:12
    #[inline(always)]
    pub fn dma_rdata(&mut self) -> DmaRdataW<ADC_DMA_RDATArs> {
        DmaRdataW::new(self, 0)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ADC_DMA_RDATArs> {
        Rsvd2W::new(self, 13)
    }
    ///Bits 16:28
    #[inline(always)]
    pub fn dma_rdata_raw(&mut self) -> DmaRdataRawW<ADC_DMA_RDATArs> {
        DmaRdataRawW::new(self, 16)
    }
    ///Bits 29:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ADC_DMA_RDATArs> {
        RsvdW::new(self, 29)
    }
}
///ADC Read Data For DMA
///
///You can [`read`](crate::Reg::read) this register and get [`adc_dma_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dma_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_DMA_RDATArs;
impl crate::RegisterSpec for ADC_DMA_RDATArs {
    type Ux = u32;
}
///`read()` method returns [`adc_dma_rdata::R`](R) reader structure
impl crate::Readable for ADC_DMA_RDATArs {}
///`write(|w| ..)` method takes [`adc_dma_rdata::W`](W) writer structure
impl crate::Writable for ADC_DMA_RDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_DMA_RDATA to value 0
impl crate::Resettable for ADC_DMA_RDATArs {
    const RESET_VALUE: u32 = 0;
}
