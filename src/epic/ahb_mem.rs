///Register `AHB_MEM` reader
pub type R = crate::R<AHB_MEMrs>;
///Register `AHB_MEM` writer
pub type W = crate::W<AHB_MEMrs>;
///Field `ADDR` reader - AHB RAM/AHB LCD target address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - AHB RAM/AHB LCD target address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AHB RAM/AHB LCD target address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_MEM")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AHB RAM/AHB LCD target address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<AHB_MEMrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ahb_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AHB_MEMrs;
impl crate::RegisterSpec for AHB_MEMrs {
    type Ux = u32;
}
///`read()` method returns [`ahb_mem::R`](R) reader structure
impl crate::Readable for AHB_MEMrs {}
///`write(|w| ..)` method takes [`ahb_mem::W`](W) writer structure
impl crate::Writable for AHB_MEMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_MEM to value 0
impl crate::Resettable for AHB_MEMrs {
    const RESET_VALUE: u32 = 0;
}
