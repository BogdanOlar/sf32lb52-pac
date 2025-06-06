///Register `Y_SRC` reader
pub type R = crate::R<Y_SRCrs>;
///Register `Y_SRC` writer
pub type W = crate::W<Y_SRCrs>;
///Field `ADDR` reader - y vector address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - y vector address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - y vector address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Y_SRC").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - y vector address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<Y_SRCrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`y_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Y_SRCrs;
impl crate::RegisterSpec for Y_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`y_src::R`](R) reader structure
impl crate::Readable for Y_SRCrs {}
///`write(|w| ..)` method takes [`y_src::W`](W) writer structure
impl crate::Writable for Y_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets Y_SRC to value 0
impl crate::Resettable for Y_SRCrs {
    const RESET_VALUE: u32 = 0;
}
