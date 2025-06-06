///Register `U_SRC` reader
pub type R = crate::R<U_SRCrs>;
///Register `U_SRC` writer
pub type W = crate::W<U_SRCrs>;
///Field `ADDR` reader - u vector address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - u vector address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - u vector address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_SRC").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - u vector address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<U_SRCrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`u_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct U_SRCrs;
impl crate::RegisterSpec for U_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`u_src::R`](R) reader structure
impl crate::Readable for U_SRCrs {}
///`write(|w| ..)` method takes [`u_src::W`](W) writer structure
impl crate::Writable for U_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets U_SRC to value 0
impl crate::Resettable for U_SRCrs {
    const RESET_VALUE: u32 = 0;
}
