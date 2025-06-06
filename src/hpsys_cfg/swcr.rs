///Register `SWCR` reader
pub type R = crate::R<SWCRrs>;
///Register `SWCR` writer
pub type W = crate::W<SWCRrs>;
///Field `SWSEL` reader - reserved for debug
pub type SwselR = crate::BitReader;
///Field `SWSEL` writer - reserved for debug
pub type SwselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn swsel(&self) -> SwselR {
        SwselR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWCR")
            .field("swsel", &self.swsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn swsel(&mut self) -> SwselW<SWCRrs> {
        SwselW::new(self, 0)
    }
}
///SW Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`swcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SWCRrs;
impl crate::RegisterSpec for SWCRrs {
    type Ux = u32;
}
///`read()` method returns [`swcr::R`](R) reader structure
impl crate::Readable for SWCRrs {}
///`write(|w| ..)` method takes [`swcr::W`](W) writer structure
impl crate::Writable for SWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWCR to value 0
impl crate::Resettable for SWCRrs {
    const RESET_VALUE: u32 = 0;
}
