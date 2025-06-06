///Register `LAYER0_FILTER` reader
pub type R = crate::R<LAYER0_FILTERrs>;
///Register `LAYER0_FILTER` writer
pub type W = crate::W<LAYER0_FILTERrs>;
///Field `FILTER_B` reader - filter b color
pub type FilterBR = crate::FieldReader;
///Field `FILTER_B` writer - filter b color
pub type FilterBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FILTER_G` reader - filter g color
pub type FilterGR = crate::FieldReader;
///Field `FILTER_G` writer - filter g color
pub type FilterGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FILTER_R` reader - filter r color
pub type FilterRR = crate::FieldReader;
///Field `FILTER_R` writer - filter r color
pub type FilterRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FILTER_MASK` reader - layer color filter mask
pub type FilterMaskR = crate::FieldReader;
///Field `FILTER_MASK` writer - layer color filter mask
pub type FilterMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - filter b color
    #[inline(always)]
    pub fn filter_b(&self) -> FilterBR {
        FilterBR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - filter g color
    #[inline(always)]
    pub fn filter_g(&self) -> FilterGR {
        FilterGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - filter r color
    #[inline(always)]
    pub fn filter_r(&self) -> FilterRR {
        FilterRR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - layer color filter mask
    #[inline(always)]
    pub fn filter_mask(&self) -> FilterMaskR {
        FilterMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER0_FILTER")
            .field("filter_mask", &self.filter_mask())
            .field("filter_r", &self.filter_r())
            .field("filter_g", &self.filter_g())
            .field("filter_b", &self.filter_b())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - filter b color
    #[inline(always)]
    pub fn filter_b(&mut self) -> FilterBW<LAYER0_FILTERrs> {
        FilterBW::new(self, 0)
    }
    ///Bits 8:15 - filter g color
    #[inline(always)]
    pub fn filter_g(&mut self) -> FilterGW<LAYER0_FILTERrs> {
        FilterGW::new(self, 8)
    }
    ///Bits 16:23 - filter r color
    #[inline(always)]
    pub fn filter_r(&mut self) -> FilterRW<LAYER0_FILTERrs> {
        FilterRW::new(self, 16)
    }
    ///Bits 24:31 - layer color filter mask
    #[inline(always)]
    pub fn filter_mask(&mut self) -> FilterMaskW<LAYER0_FILTERrs> {
        FilterMaskW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER0_FILTERrs;
impl crate::RegisterSpec for LAYER0_FILTERrs {
    type Ux = u32;
}
///`read()` method returns [`layer0_filter::R`](R) reader structure
impl crate::Readable for LAYER0_FILTERrs {}
///`write(|w| ..)` method takes [`layer0_filter::W`](W) writer structure
impl crate::Writable for LAYER0_FILTERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER0_FILTER to value 0
impl crate::Resettable for LAYER0_FILTERrs {
    const RESET_VALUE: u32 = 0;
}
