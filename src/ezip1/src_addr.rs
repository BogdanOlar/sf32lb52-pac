///Register `SRC_ADDR` reader
pub type R = crate::R<SRC_ADDRrs>;
///Register `SRC_ADDR` writer
pub type W = crate::W<SRC_ADDRrs>;
///Field `SRC_ADDR` reader - ezip decoder source address
pub type SrcAddrR = crate::FieldReader<u32>;
///Field `SRC_ADDR` writer - ezip decoder source address
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ezip decoder source address
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC_ADDR")
            .field("src_addr", &self.src_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ezip decoder source address
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<SRC_ADDRrs> {
        SrcAddrW::new(self, 0)
    }
}
///ezip decoder source address
///
///You can [`read`](crate::Reg::read) this register and get [`src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRC_ADDRrs;
impl crate::RegisterSpec for SRC_ADDRrs {
    type Ux = u32;
}
///`read()` method returns [`src_addr::R`](R) reader structure
impl crate::Readable for SRC_ADDRrs {}
///`write(|w| ..)` method takes [`src_addr::W`](W) writer structure
impl crate::Writable for SRC_ADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRC_ADDR to value 0
impl crate::Resettable for SRC_ADDRrs {
    const RESET_VALUE: u32 = 0;
}
