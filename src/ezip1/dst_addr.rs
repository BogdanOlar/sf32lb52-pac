///Register `DST_ADDR` reader
pub type R = crate::R<DST_ADDRrs>;
///Register `DST_ADDR` writer
pub type W = crate::W<DST_ADDRrs>;
///Field `DST_ADDR` reader - ezip decoder destination address(ahb_out mode)
pub type DstAddrR = crate::FieldReader<u32>;
///Field `DST_ADDR` writer - ezip decoder destination address(ahb_out mode)
pub type DstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ezip decoder destination address(ahb_out mode)
    #[inline(always)]
    pub fn dst_addr(&self) -> DstAddrR {
        DstAddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DST_ADDR")
            .field("dst_addr", &self.dst_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ezip decoder destination address(ahb_out mode)
    #[inline(always)]
    pub fn dst_addr(&mut self) -> DstAddrW<DST_ADDRrs> {
        DstAddrW::new(self, 0)
    }
}
///ezip decoder destination address
///
///You can [`read`](crate::Reg::read) this register and get [`dst_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DST_ADDRrs;
impl crate::RegisterSpec for DST_ADDRrs {
    type Ux = u32;
}
///`read()` method returns [`dst_addr::R`](R) reader structure
impl crate::Readable for DST_ADDRrs {}
///`write(|w| ..)` method takes [`dst_addr::W`](W) writer structure
impl crate::Writable for DST_ADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DST_ADDR to value 0
impl crate::Resettable for DST_ADDRrs {
    const RESET_VALUE: u32 = 0;
}
