///Register `AR1` reader
pub type R = crate::R<AR1rs>;
///Register `AR1` writer
pub type W = crate::W<AR1rs>;
///Field `ADDR` reader - Address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - Address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR1").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<AR1rs> {
        AddrW::new(self, 0)
    }
}
///Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AR1rs;
impl crate::RegisterSpec for AR1rs {
    type Ux = u32;
}
///`read()` method returns [`ar1::R`](R) reader structure
impl crate::Readable for AR1rs {}
///`write(|w| ..)` method takes [`ar1::W`](W) writer structure
impl crate::Writable for AR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AR1 to value 0
impl crate::Resettable for AR1rs {
    const RESET_VALUE: u32 = 0;
}
