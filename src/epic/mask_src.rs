///Register `MASK_SRC` reader
pub type R = crate::R<MASK_SRCrs>;
///Register `MASK_SRC` writer
pub type W = crate::W<MASK_SRCrs>;
///Field `ADDR` reader - mask data address\[31:0\]. This is byte address, even for A4, this address is byte aligned.
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - mask data address\[31:0\]. This is byte address, even for A4, this address is byte aligned.
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - mask data address\[31:0\]. This is byte address, even for A4, this address is byte aligned.
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_SRC")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - mask data address\[31:0\]. This is byte address, even for A4, this address is byte aligned.
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<MASK_SRCrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`mask_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MASK_SRCrs;
impl crate::RegisterSpec for MASK_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`mask_src::R`](R) reader structure
impl crate::Readable for MASK_SRCrs {}
///`write(|w| ..)` method takes [`mask_src::W`](W) writer structure
impl crate::Writable for MASK_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MASK_SRC to value 0
impl crate::Resettable for MASK_SRCrs {
    const RESET_VALUE: u32 = 0;
}
