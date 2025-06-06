///Register `AHB_STRIDE` reader
pub type R = crate::R<AHB_STRIDErs>;
///Register `AHB_STRIDE` writer
pub type W = crate::W<AHB_STRIDErs>;
///Field `OFFSET` reader -
pub type OffsetR = crate::FieldReader<u16>;
///Field `OFFSET` writer -
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_STRIDE")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<AHB_STRIDErs> {
        OffsetW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ahb_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AHB_STRIDErs;
impl crate::RegisterSpec for AHB_STRIDErs {
    type Ux = u32;
}
///`read()` method returns [`ahb_stride::R`](R) reader structure
impl crate::Readable for AHB_STRIDErs {}
///`write(|w| ..)` method takes [`ahb_stride::W`](W) writer structure
impl crate::Writable for AHB_STRIDErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_STRIDE to value 0
impl crate::Resettable for AHB_STRIDErs {
    const RESET_VALUE: u32 = 0;
}
